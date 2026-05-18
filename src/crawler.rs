/*
 * =============================================================================
 * Module: Web Crawler Module
 * Project: ScrapeFlux - Advanced Unified Web Scraping Framework
 * Author: Antor Roy
 * Email: antorroybabu@gmail.com
 * License: MIT
 * 
 * Description:
 *     High-performance web crawler with Bloom filter deduplication,
 *     robots.txt compliance, and concurrent crawling support.
 * 
 * Features:
 *     - Bloom filter for O(1) URL deduplication
 *     - robots.txt compliance
 *     - Depth-limited crawling
 *     - Concurrent page fetching
 *     - Link extraction and normalization
 *     - Memory-efficient visited tracking
 * =============================================================================
 */

use crate::html::{HtmlParser, ElementInfo};
use crate::http::{HttpClient, HttpResponse};
use bloomfilter::Bloom;
use serde::{Deserialize, Serialize};
use std::collections::{HashSet, VecDeque, HashMap};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use url::Url;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CrawlerError {
    #[error("HTTP error: {0}")]
    HttpError(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Already visited")]
    AlreadyVisited,
    #[error("Disallowed by robots.txt")]
    RobotsDisallowed,
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    #[error("Max pages exceeded")]
    MaxPagesExceeded,
}

#[derive(Debug, Clone)]
pub struct CrawlerConfig {
    pub max_depth: usize,
    pub max_pages: usize,
    pub respect_robots_txt: bool,
    pub same_domain_only: bool,
    pub extract_links: bool,
    pub concurrent_requests: usize,
    pub bloom_filter_items: usize,
    pub bloom_filter_fp_rate: f64,
}

impl Default for CrawlerConfig {
    fn default() -> Self {
        Self {
            max_depth: 3,
            max_pages: 100,
            respect_robots_txt: true,
            same_domain_only: true,
            extract_links: true,
            concurrent_requests: 10,
            bloom_filter_items: 100000,
            bloom_filter_fp_rate: 0.001,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawledPage {
    pub url: String,
    pub status: u16,
    pub title: Option<String>,
    pub html: String,
    pub links: Vec<String>,
    pub depth: usize,
    pub fetch_time_ms: u64,
    pub content_type: Option<String>,
    pub meta: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlResult {
    pub pages: Vec<CrawledPage>,
    pub total_pages: usize,
    pub total_links: usize,
    pub duration_ms: u64,
    pub errors: Vec<String>,
    pub bandwidth_mb: f64,
}

pub struct Crawler {
    config: CrawlerConfig,
    http_client: HttpClient,
    visited: Arc<Bloom<String>>,
    robots_cache: Arc<RwLock<HashMap<String, bool>>>,
    visited_set: Arc<RwLock<HashSet<String>>>,
}

impl Crawler {
    pub fn new() -> Result<Self, String> {
        Self::with_config(CrawlerConfig::default())
    }

    pub fn with_config(config: CrawlerConfig) -> Result<Self, String> {
        let http_client = HttpClient::new()
            .map_err(|e| format!("Failed to create HTTP client: {}", e))?;
        
        let bloom = Bloom::new_for_testing(
            config.bloom_filter_items,
            config.bloom_filter_fp_rate
        );

        Ok(Self {
            config,
            http_client,
            visited: Arc::new(bloom),
            robots_cache: Arc::new(RwLock::new(HashMap::new())),
            visited_set: Arc::new(RwLock::new(HashSet::new())),
        })
    }

    pub async fn crawl(&self, url: &str) -> Result<CrawledPage, CrawlerError> {
        let start = Instant::now();
        
        if self.visited_set.read().await.contains(url) {
            return Err(CrawlerError::AlreadyVisited);
        }
        
        if self.config.respect_robots_txt {
            if !self.can_fetch(url).await? {
                return Err(CrawlerError::RobotsDisallowed);
            }
        }
        
        {
            let mut visited = self.visited_set.write().await;
            visited.insert(url.to_string());
        }
        self.visited.insert(url);

        let response = self.http_client.get(url).await
            .map_err(|e| CrawlerError::HttpError(e.to_string()))?;
        
        if !response.is_success() {
            return Err(CrawlerError::HttpError(format!("Status: {}", response.status)));
        }

        let mut parser = HtmlParser::with_url(&response.body, url);
        let title = parser.extract_title();
        let meta = parser.extract_metadata();
        
        let links = if self.config.extract_links {
            parser.extract_links(url).unwrap_or_default()
        } else {
            vec![]
        };

        let fetch_time_ms = start.elapsed().as_millis() as u64;

        Ok(CrawledPage {
            url: url.to_string(),
            status: response.status,
            title,
            html: response.body,
            links,
            depth: 0,
            fetch_time_ms,
            content_type: response.content_type,
            meta,
        })
    }

    pub async fn crawl_site(&self, start_url: &str) -> Result<CrawlResult, CrawlerError> {
        let start = Instant::now();
        let mut pages = Vec::new();
        let mut errors = Vec::new();
        let mut total_bytes = 0u64;
        
        let base_domain = Url::parse(start_url)
            .map_err(|e| CrawlerError::InvalidUrl(e.to_string()))?
            .host_str()
            .unwrap_or("")
            .to_string();
        
        let mut queue: VecDeque<(String, usize)> = VecDeque::new();
        queue.push_back((start_url.to_string(), 0));
        
        while let Some((url, depth)) = queue.pop_front() {
            if pages.len() >= self.config.max_pages {
                break;
            }
            
            if depth > self.config.max_depth {
                continue;
            }
            
            if self.visited_set.read().await.contains(&url) {
                continue;
            }
            
            if self.config.same_domain_only {
                if let Ok(url_parsed) = Url::parse(&url) {
                    if url_parsed.host_str() != Some(&base_domain) {
                        continue;
                    }
                }
            }
            
            match self.crawl(&url).await {
                Ok(page) => {
                    total_bytes += page.html.len() as u64;
                    pages.push(page.clone());
                    
                    for link in page.links.iter().filter(|l| {
                        Url::parse(l).ok().map(|u| u.host_str() == Some(&base_domain)).unwrap_or(false)
                    }) {
                        if !self.visited_set.read().await.contains(link) && 
                           queue.iter().all(|(u, _)| u != link) {
                            queue.push_back((link.clone(), depth + 1));
                        }
                    }
                }
                Err(e) => {
                    errors.push(format!("{}: {}", url, e));
                }
            }
        }
        
        let duration_ms = start.elapsed().as_millis() as u64;
        let total_links = pages.iter().map(|p| p.links.len()).sum();
        let bandwidth_mb = total_bytes as f64 / (1024.0 * 1024.0);

        Ok(CrawlResult {
            pages,
            total_pages: pages.len(),
            total_links,
            duration_ms,
            errors,
            bandwidth_mb,
        })
    }

    pub async fn crawl_batch(&self, urls: Vec<String>) -> Vec<Result<CrawledPage, CrawlerError>> {
        use futures::stream::{self, StreamExt};
        
        stream::iter(urls)
            .map(|url| {
                let crawler = self.clone();
                async move { crawler.crawl(&url).await }
            })
            .buffer_unordered(self.config.concurrent_requests)
            .collect()
            .await
    }

    async fn can_fetch(&self, url: &str) -> Result<bool, CrawlerError> {
        let parsed = Url::parse(url)
            .map_err(|e| CrawlerError::InvalidUrl(e.to_string()))?;
        
        let robots_url = format!("{}://{}/robots.txt", parsed.scheme(), parsed.host_str().unwrap_or(""));
        
        {
            let cache = self.robots_cache.read().await;
            if let Some(&allowed) = cache.get(&robots_url) {
                return Ok(allowed);
            }
        }
        
        match self.http_client.get(&robots_url).await {
            Ok(response) => {
                let allowed = self.parse_robots_txt(&response.body, url);
                let mut cache = self.robots_cache.write().await;
                cache.insert(robots_url, allowed);
                Ok(allowed)
            }
            Err(_) => Ok(true),
        }
    }

    fn parse_robots_txt(&self, content: &str, url: &str) -> bool {
        let url_path = Url::parse(url).ok()
            .and_then(|u| u.path().parse::<String>().ok())
            .unwrap_or_default();
        
        let mut allow_current = true;
        
        for line in content.lines() {
            let line = line.trim();
            let lower = line.to_lowercase();
            
            if lower.starts_with("disallow:") {
                let path = line.split(':').nth(1).unwrap_or("/").trim();
                if url_path.starts_with(path) {
                    allow_current = false;
                }
            } else if lower.starts_with("allow:") {
                let path = line.split(':').nth(1).unwrap_or("/").trim();
                if url_path.starts_with(path) {
                    allow_current = true;
                }
            }
        }
        
        allow_current
    }

    pub async fn clear_cache(&self) {
        self.visited_set.write().await.clear();
        self.robots_cache.write().await.clear();
    }

    pub async fn visited_count(&self) -> usize {
        self.visited_set.read().await.len()
    }
}

impl Clone for Crawler {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            http_client: self.http_client.clone(),
            visited: Arc::clone(&self.visited),
            robots_cache: Arc::clone(&self.robots_cache),
            visited_set: Arc::clone(&self.visited_set),
        }
    }
}

impl Default for Crawler {
    fn default() -> Self {
        Self::new().expect("Failed to create crawler")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_crawl_single() {
        let crawler = Crawler::new().unwrap();
        let result = crawler.crawl("https://example.com").await;
        
        assert!(result.is_ok());
        let page = result.unwrap();
        assert_eq!(page.status, 200);
    }
}