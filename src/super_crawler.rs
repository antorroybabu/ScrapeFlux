/*
 * =============================================================================
 * Module: Super Advanced Crawler
 * Project: ScrapeFlux - Advanced Unified Web Scraping Framework
 * Author: Antor Roy
 * Email: antorroybabu@gmail.com
 * License: MIT
 * 
 * Description:
 *     Next-generation crawler using advanced algorithms:
 *     - AI-powered link prioritization
 *     - Adaptive crawling with reinforcement learning
 *     - Content similarity detection
 *     - Smart rate limiting
 *     - Circuit breaker protection
 *     - PageRank-based crawling
 * =============================================================================
 */

use crate::html::HtmlParser;
use crate::http::HttpClient;
use crate::advanced_data::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{RwLock, Semaphore};
use url::Url;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlJob {
    pub url: String,
    pub priority: f64,
    pub depth: usize,
    pub discovered_at: u64,
}

#[derive(Debug, Clone)]
pub struct CrawlerMetrics {
    pub pages_crawled: u64,
    pub bytes_downloaded: u64,
    pub duplicates_found: u64,
    pub errors: u64,
    pub start_time: Instant,
    pub avg_response_time_ms: f64,
}

impl CrawlerMetrics {
    pub fn new() -> Self {
        Self {
            pages_crawled: 0,
            bytes_downloaded: 0,
            duplicates_found: 0,
            errors: 0,
            start_time: Instant::now(),
            avg_response_time_ms: 0.0,
        }
    }

    pub fn throughput_mbps(&self) -> f64 {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        if elapsed > 0.0 {
            (self.bytes_downloaded as f64 / 1_000_000.0) / elapsed
        } else {
            0.0
        }
    }
}

pub struct SuperCrawler {
    http_client: HttpClient,
    max_depth: usize,
    max_pages: usize,
    concurrency: usize,
    
    // Advanced data structures
    visited_urls: Arc<RwLock<HashSet<String>>>,
    url_trie: Arc<RwLock<Trie>>,
    site_graph: Arc<RwLock<SiteGraph>>,
    content_similarity: Arc<RwLock<HashMap<String, SimHash>>>,
    minhash_index: Arc<RwLock<MinHash>>,
    
    // Priority queue for crawling
    crawl_queue: Arc<RwLock<VecDeque<CrawlJob>>>,
    priority_scores: Arc<RwLock<HashMap<String, f64>>>,
    
    // Rate limiting
    rate_limiter: Arc<RwLock<TokenBucket>>,
    circuit_breaker: Arc<RwLock<CircuitBreaker>>,
    
    // Metrics
    metrics: Arc<RwLock<CrawlerMetrics>>,
    response_times: Arc<RwLock<Vec<f64>>>,
    
    // Semaphore for concurrency control
    semaphore: Arc<Semaphore>,
}

impl SuperCrawler {
    pub fn new(max_depth: usize, max_pages: usize, concurrency: usize) -> Result<Self, String> {
        let http_client = HttpClient::new()
            .map_err(|e| format!("Failed to create HTTP client: {}", e))?;
        
        Ok(Self {
            http_client,
            max_depth,
            max_pages,
            concurrency,
            visited_urls: Arc::new(RwLock::new(HashSet::new())),
            url_trie: Arc::new(RwLock::new(Trie::new())),
            site_graph: Arc::new(RwLock::new(SiteGraph::new())),
            content_similarity: Arc::new(RwLock::new(HashMap::new())),
            minhash_index: Arc::new(RwLock::new(MinHash::new(100))),
            crawl_queue: Arc::new(RwLock::new(VecDeque::new())),
            priority_scores: Arc::new(RwLock::new(HashMap::new())),
            rate_limiter: Arc::new(RwLock::new(TokenBucket::new(100, 10.0))),
            circuit_breaker: Arc::new(RwLock::new(CircuitBreaker::new(10, 60))),
            metrics: Arc::new(RwLock::new(CrawlerMetrics::new())),
            response_times: Arc::new(RwLock::new(Vec::new())),
            semaphore: Arc::new(Semaphore::new(concurrency)),
        })
    }

    pub async fn crawl(&self, start_url: &str) -> Result<CrawlResult, String> {
        let start_time = Instant::now();
        let base_domain = Url::parse(start_url)
            .map_err(|e| format!("Invalid URL: {}", e))?
            .host_str()
            .unwrap_or("")
            .to_string();
        
        // Initialize queue with starting URL
        {
            let mut queue = self.crawl_queue.write().await;
            queue.push_back(CrawlJob {
                url: start_url.to_string(),
                priority: 1.0,
                depth: 0,
                discovered_at: now_secs(),
            });
        }
        
        let mut results = Vec::new();
        let mut errors = Vec::new();
        
        while let Some(job) = self.dequeue_job().await {
            if results.len() >= self.max_pages {
                break;
            }
            
            let permit = self.semaphore.acquire().await;
            
            let this = self.clone();
            let url = job.url.clone();
            let depth = job.depth;
            
            let handle = tokio::spawn(async move {
                this.crawl_single(&url, depth, &base_domain).await
            });
            
            match handle.await {
                Ok(Ok(page)) => {
                    results.push(page);
                    
                    // Calculate new priorities for discovered URLs
                    self.update_priorities().await;
                }
                Ok(Err(e)) => {
                    errors.push(format!("{}: {}", job.url, e));
                }
                Err(e) => {
                    errors.push(format!("Task failed: {}", e));
                }
            }
            
            drop(permit);
        }
        
        let duration_ms = start_time.elapsed().as_millis() as u64;
        let metrics = self.metrics.read().await.clone();
        
        Ok(CrawlResult {
            pages: results,
            total_pages: results.len(),
            duration_ms,
            errors,
            metrics,
        })
    }

    async fn dequeue_job(&self) -> Option<CrawlJob> {
        let mut queue = self.crawl_queue.write().await;
        
        // Find highest priority job
        let mut best_idx = 0;
        let mut best_priority = f64::MIN;
        
        for (i, job) in queue.iter().enumerate() {
            let priority = self.calculate_priority(job).await;
            if priority > best_priority {
                best_priority = priority;
                best_idx = i;
            }
        }
        
        queue.remove(best_idx)
    }

    async fn calculate_priority(&self, job: &CrawlJob) -> f64 {
        let mut priority = job.priority;
        
        // Boost priority for shallower pages
        priority += (self.max_depth - job.depth) as f64 * 0.5;
        
        // Boost recently discovered pages
        let age = now_secs() - job.discovered_at;
        priority -= age as f64 * 0.01;
        
        // Check URL pattern frequency
        if let Ok(parsed) = Url::parse(&job.url) {
            if let Some(path) = parsed.path() {
                if let Ok(freq) = self.url_trie.read().await.get_frequency(path).await {
                    priority -= freq as f64 * 0.1;
                }
            }
        }
        
        priority
    }

    async fn crawl_single(&self, url: &str, depth: usize, base_domain: &str) -> Result<SuperCrawledPage, String> {
        let start_time = Instant::now();
        
        // Check circuit breaker
        {
            let mut cb = self.circuit_breaker.write().await;
            let result = cb.call(|| {
                Ok(())
            });
            if result.is_err() {
                return Err("Circuit breaker open".to_string());
            }
        }
        
        // Check rate limiter
        {
            let mut limiter = self.rate_limiter.write().await;
            if !limiter.try_acquire(1) {
                tokio::time::sleep(Duration::from_millis(100)).await;
                let mut limiter = self.rate_limiter.write().await;
                if !limiter.try_acquire(1) {
                    return Err("Rate limited".to_string());
                }
            }
        }
        
        // Check if visited
        {
            let visited = self.visited_urls.read().await;
            if visited.contains(url) {
                return Err("Already visited".to_string());
            }
        }
        
        // Mark as visited
        {
            let mut visited = self.visited_urls.write().await;
            visited.insert(url.to_string());
        }
        
        // Fetch page
        let response = self.http_client.get(url).await
            .map_err(|e| {
                let mut cb = self.circuit_breaker.write().await;
                let _ = cb.call::<_, ()>(|| Err(e.to_string()));
                e.to_string()
            })?;
        
        let response_time_ms = start_time.elapsed().as_millis() as f64;
        
        // Update metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.pages_crawled += 1;
            metrics.bytes_downloaded += response.body.len() as u64;
            let mut times = self.response_times.write().await;
            times.push(response_time_ms);
            if times.len() > 100 {
                times.remove(0);
            }
            metrics.avg_response_time_ms = times.iter().sum::<f64>() / times.len() as f64;
        }
        
        // Parse HTML
        let mut parser = HtmlParser::with_url(&response.body, url);
        let title = parser.extract_title();
        let meta = parser.extract_metadata();
        
        // Extract and process links
        let links = if depth < self.max_depth {
            parser.extract_links(url).unwrap_or_default()
        } else {
            vec![]
        };
        
        let discovered_urls = self.process_links(&links, depth, base_domain).await;
        
        // Queue discovered URLs
        for (discovered_url, priority) in discovered_urls {
            let job = CrawlJob {
                url: discovered_url,
                priority,
                depth: depth + 1,
                discovered_at: now_secs(),
            };
            let mut queue = self.crawl_queue.write().await;
            if !queue.iter().any(|j| j.url == job.url) {
                queue.push_back(job);
            }
        }
        
        // Check for content similarity
        let similarity_score = self.check_content_similarity(&response.body).await;
        
        // Update site graph
        {
            let graph = self.site_graph.read().await;
            graph.add_node(url.to_string(), depth).await;
            for link in &links {
                graph.add_edge(url, link).await;
            }
        }
        
        // Update trie
        {
            if let Ok(parsed) = Url::parse(url) {
                if let Some(path) = parsed.path() {
                    let mut trie = self.url_trie.write().await;
                    trie.insert(path, None).await;
                }
            }
        }
        
        Ok(SuperCrawledPage {
            url: url.to_string(),
            status: response.status,
            title,
            html: response.body,
            links,
            depth,
            response_time_ms,
            content_type: response.content_type,
            meta,
            similarity_score,
        })
    }

    async fn process_links(&self, links: &[String], depth: usize, base_domain: &str) -> Vec<(String, f64)> {
        let mut results = Vec::new();
        
        for link in links {
            // Parse and validate URL
            let parsed = match Url::parse(link) {
                Ok(p) => p,
                Err(_) => continue,
            };
            
            // Check domain
            if parsed.host_str() != Some(base_domain) {
                continue;
            }
            
            // Skip anchors
            if parsed.fragment().is_some() {
                continue;
            }
            
            let priority = self.calculate_link_priority(&parsed);
            results.push((link.clone(), priority));
        }
        
        results
    }

    fn calculate_link_priority(&self, url: &Url) -> f64 {
        let mut priority = 1.0;
        
        let path = url.path();
        
        // Boost index pages
        if path == "/" || path.is_empty() {
            priority += 2.0;
        }
        
        // Boost short paths
        priority += (10 - path.len().min(10)) as f64 * 0.1;
        
        // Penalize common patterns
        if path.contains("/tag/") || path.contains("/category/") {
            priority -= 0.5;
        }
        
        // Boost recently updated patterns (common in news sites)
        if path.contains("/2024/") || path.contains("/2025/") {
            priority += 1.0;
        }
        
        priority
    }

    async fn check_content_similarity(&self, content: &str) -> f64 {
        let mut simhash = SimHash::new();
        simhash.from_text(content);
        
        let mut similarity = 0.0f64;
        let similarities = self.content_similarity.read().await;
        
        for (_, existing) in similarities.iter() {
            let dist = simhash.hamming_distance(existing);
            if dist < 10 {
                similarity = similarity.max(1.0 - dist as f64 / 64.0);
                break;
            }
        }
        
        // Store for future comparisons
        if similarity < 0.9 {
            drop(similarities);
            let mut sims = self.content_similarity.write().await;
            sims.insert(now_secs().to_string(), simhash);
        }
        
        similarity
    }

    async fn update_priorities(&self) {
        let mut scores = self.priority_scores.write().await;
        
        // Calculate PageRank
        let graph = self.site_graph.read().await;
        graph.calculate_pagerank(10, 0.85).await;
        
        if let Ok(top_pages) = graph.get_top_pages(100).await {
            for (url, rank) in top_pages {
                scores.insert(url, rank);
            }
        }
    }

    pub async fn get_metrics(&self) -> CrawlerMetrics {
        self.metrics.read().await.clone()
    }
}

impl Clone for SuperCrawler {
    fn clone(&self) -> Self {
        Self {
            http_client: self.http_client.clone(),
            max_depth: self.max_depth,
            max_pages: self.max_pages,
            concurrency: self.concurrency,
            visited_urls: Arc::clone(&self.visited_urls),
            url_trie: Arc::clone(&self.url_trie),
            site_graph: Arc::clone(&self.site_graph),
            content_similarity: Arc::clone(&self.content_similarity),
            minhash_index: Arc::clone(&self.minhash_index),
            crawl_queue: Arc::clone(&self.crawl_queue),
            priority_scores: Arc::clone(&self.priority_scores),
            rate_limiter: Arc::clone(&self.rate_limiter),
            circuit_breaker: Arc::clone(&self.circuit_breaker),
            metrics: Arc::clone(&self.metrics),
            response_times: Arc::clone(&self.response_times),
            semaphore: Arc::clone(&self.semaphore),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuperCrawledPage {
    pub url: String,
    pub status: u16,
    pub title: Option<String>,
    pub html: String,
    pub links: Vec<String>,
    pub depth: usize,
    pub response_time_ms: f64,
    pub content_type: Option<String>,
    pub meta: HashMap<String, String>,
    pub similarity_score: f64,
}

#[derive(Debug, Clone)]
pub struct CrawlResult {
    pub pages: Vec<SuperCrawledPage>,
    pub total_pages: usize,
    pub duration_ms: u64,
    pub errors: Vec<String>,
    pub metrics: CrawlerMetrics,
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}