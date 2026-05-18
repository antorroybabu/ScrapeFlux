/*
 * =============================================================================
 * Module: Advanced Data Structures
 * Project: ScrapeFlux - Advanced Unified Web Scraping Framework
 * Author: Antor Roy
 * Email: antorroybabu@gmail.com
 * License: MIT
 * 
 * Description:
 *     Advanced data structures for high-performance web scraping:
 *     - Trie for URL path matching
 *     - Graph for site structure
 *     - Skip list for concurrent visited tracking
 *     - MinHash for content similarity
 *     - SimHash for deduplication
 *     - Adaptive hash maps
 * =============================================================================
 */

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

// ============================================================================
// TRIE DATA STRUCTURE FOR URL PATH MATCHING
// ============================================================================

#[derive(Debug, Clone)]
pub struct TrieNode {
    children: HashMap<char, Arc<RwLock<TrieNode>>>,
    is_end: bool,
    frequency: u64,
    metadata: Option<String>,
}

impl TrieNode {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_end: false,
            frequency: 0,
            metadata: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Trie {
    root: Arc<RwLock<TrieNode>>,
    total_insertions: u64,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: Arc::new(RwLock::new(TrieNode::new())),
            total_insertions: 0,
        }
    }

    pub async fn insert(&mut self, path: &str, metadata: Option<String>) {
        let mut node = self.root.write().await;
        for ch in path.chars() {
            node = if let Some(child) = node.children.get(&ch) {
                child.clone()
            } else {
                let new_node = Arc::new(RwLock::new(TrieNode::new()));
                node.children.insert(ch, new_node.clone());
                new_node
            };
            let mut node_guard = node.write().await;
            node_guard.frequency += 1;
        }
        node.write().await.is_end = true;
        if let Some(meta) = metadata {
            node.write().await.metadata = Some(meta);
        }
        self.total_insertions += 1;
    }

    pub async fn search(&self, path: &str) -> bool {
        let mut node = self.root.read().await;
        for ch in path.chars() {
            match node.children.get(&ch) {
                Some(child) => node = child.read().await,
                None => return false,
            }
        }
        node.is_end
    }

    pub async fn prefix_match(&self, prefix: &str) -> Vec<String> {
        let mut results = Vec::new();
        let mut node = self.root.read().await;
        
        for ch in prefix.chars() {
            match node.children.get(&ch) {
                Some(child) => node = child.read().await,
                None => return results,
            }
        }
        
        self.collect_paths(&node, prefix.to_string(), &mut results, 10);
        results
    }

    fn collect_paths(&self, node: &TrieNode, current: String, results: &mut Vec<String>, limit: usize) {
        if results.len() >= limit {
            return;
        }
        if node.is_end {
            results.push(current.clone());
        }
        for (ch, child) in &node.children {
            let child_guard = child.blocking_read();
            self.collect_paths(&child_guard, format!("{}{}", current, ch), results, limit);
        }
    }

    pub async fn get_frequency(&self, path: &str) -> u64 {
        let mut node = self.root.read().await;
        for ch in path.chars() {
            match node.children.get(&ch) {
                Some(child) => node = child.read().await,
                None => return 0,
            }
        }
        node.frequency
    }
}

// ============================================================================
// GRAPH DATA STRUCTURE FOR SITE STRUCTURE
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub url: String,
    pub depth: usize,
    pub page_rank: f64,
    pub in_degree: usize,
    pub out_degree: usize,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct SiteGraph {
    nodes: Arc<RwLock<HashMap<String, Arc<RwLock<GraphNode>>>>>,
    edges: Arc<RwLock<HashMap<String, HashSet<String>>>>,
    adjacency_list: Arc<RwLock<HashMap<String, Vec<String>>>>,
}

impl SiteGraph {
    pub fn new() -> Self {
        Self {
            nodes: Arc::new(RwLock::new(HashMap::new())),
            edges: Arc::new(RwLock::new(HashMap::new())),
            adjacency_list: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_node(&self, url: String, depth: usize) {
        let node = GraphNode {
            url: url.clone(),
            depth,
            page_rank: 1.0,
            in_degree: 0,
            out_degree: 0,
            metadata: HashMap::new(),
        };
        
        let mut nodes = self.nodes.write().await;
        nodes.insert(url.clone(), Arc::new(RwLock::new(node)));
        
        let mut adjacency = self.adjacency_list.write().await;
        adjacency.entry(url).or_insert_with(Vec::new);
    }

    pub async fn add_edge(&self, from: &str, to: &str) {
        // Add to adjacency list
        {
            let mut adjacency = self.adjacency_list.write().await;
            adjacency.entry(from.to_string())
                .or_insert_with(Vec::new)
                .push(to.to_string());
        }
        
        // Update out-degree
        {
            let nodes = self.nodes.read().await;
            if let Some(node) = nodes.get(from) {
                node.write().await.out_degree += 1;
            }
        }
        
        // Update in-degree
        {
            let nodes = self.nodes.read().await;
            if let Some(node) = nodes.get(to) {
                node.write().await.in_degree += 1;
            }
        }
        
        // Add to edges
        {
            let mut edges = self.edges.write().await;
            edges.entry(from.to_string())
                .or_insert_with(HashSet::new)
                .insert(to.to_string());
        }
    }

    pub async fn calculate_pagerank(&self, iterations: usize, damping: f64) {
        let nodes_count = {
            let nodes = self.nodes.read().await;
            nodes.len()
        };
        
        if nodes_count == 0 {
            return;
        }
        
        let initial_rank = 1.0 / nodes_count as f64;
        
        // Initialize
        {
            let nodes = self.nodes.read().await;
            for node in nodes.values() {
                node.write().await.page_rank = initial_rank;
            }
        }
        
        // Iterate
        for _ in 0..iterations {
            let ranks: HashMap<String, f64> = {
                let nodes = self.nodes.read().await;
                let mut new_ranks = HashMap::new();
                
                for (url, node) in nodes.iter() {
                    let mut rank = (1.0 - damping) / nodes_count as f64;
                    let node_guard = node.read().await;
                    
                    // Sum ranks from incoming edges
                    let edges = self.edges.read().await;
                    for (from_url, to_urls) in edges.iter() {
                        if to_urls.contains(url) {
                            if let Some(from_node) = nodes.get(from_url) {
                                let from_guard = from_node.read().await;
                                if from_guard.out_degree > 0 {
                                    rank += damping * from_guard.page_rank / from_guard.out_degree as f64;
                                }
                            }
                        }
                    }
                    new_ranks.insert(url.clone(), rank);
                }
                new_ranks
            };
            
            // Update ranks
            let nodes = self.nodes.read().await;
            for (url, rank) in ranks {
                if let Some(node) = nodes.get(&url) {
                    node.write().await.page_rank = rank;
                }
            }
        }
    }

    pub async fn get_top_pages(&self, limit: usize) -> Vec<(String, f64)> {
        let nodes = self.nodes.read().await;
        let mut rankings: Vec<(String, f64)> = nodes
            .iter()
            .map(|(url, node)| {
                let rank = node.read().await.page_rank;
                (url.clone(), rank)
            })
            .collect();
        
        rankings.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        rankings.truncate(limit);
        rankings
    }
}

// ============================================================================
// MINHASH FOR CONTENT SIMILARITY
// ============================================================================

#[derive(Debug, Clone)]
pub struct MinHash {
    num_hashes: usize,
    hash_values: Vec<u64>,
}

impl MinHash {
    pub fn new(num_hashes: usize) -> Self {
        Self {
            num_hashes,
            hash_values: vec![u64::MAX; num_hashes],
        }
    }

    pub fn add(&mut self, item: &str) {
        for i in 0..self.num_hashes {
            let hash = self.double_hash(item, i);
            self.hash_values[i] = self.hash_values[i].min(hash);
        }
    }

    fn double_hash(&self, item: &str, seed: usize) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        let h1 = hasher.finish();
        
        let mut hasher2 = DefaultHasher::new();
        h1.hash(&mut hasher2);
        seed.hash(&mut hasher2);
        let h2 = hasher2.finish();
        
        (h1.wrapping_add((seed as u64).wrapping_mul(h2))) % u64::MAX
    }

    pub fn jaccard_similarity(&self, other: &MinHash) -> f64 {
        if self.num_hashes != other.num_hashes {
            return 0.0;
        }
        
        let matches = self.hash_values
            .iter()
            .zip(other.hash_values.iter())
            .filter(|(a, b)| a == b)
            .count();
        
        matches as f64 / self.num_hashes as f64
    }
}

// ============================================================================
// SIMHASH FOR DEDUPLICATION
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct SimHash {
    hash: u64,
    features: Vec<u64>,
}

impl SimHash {
    pub fn new() -> Self {
        Self {
            hash: 0,
            features: Vec::new(),
        }
    }

    pub fn from_text(&mut self, text: &str) {
        self.features.clear();
        self.hash = 0;
        
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut v = vec![0i64; 64];
        
        for word in &words {
            let word_hash = self.hash_word(word);
            self.features.push(word_hash);
            
            for i in 0..64 {
                if (word_hash >> i) & 1 == 1 {
                    v[i] += 1;
                } else {
                    v[i] -= 1;
                }
            }
        }
        
        for i in 0..64 {
            if v[i] > 0 {
                self.hash |= 1 << i;
            }
        }
    }

    fn hash_word(&self, word: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        word.hash(&mut hasher);
        hasher.finish()
    }

    pub fn hamming_distance(&self, other: &SimHash) -> usize {
        let xor = self.hash ^ other.hash;
        xor.count_ones() as usize
    }

    pub fn is_duplicate(&self, other: &SimHash, threshold: usize) -> bool {
        self.hamming_distance(other) <= threshold
    }
}

// ============================================================================
// TOKEN BUCKET RATE LIMITER
// ============================================================================

#[derive(Debug, Clone)]
pub struct TokenBucket {
    capacity: u64,
    tokens: f64,
    refill_rate: f64,
    last_refill: std::time::Instant,
}

impl TokenBucket {
    pub fn new(capacity: u64, refill_rate: f64) -> Self {
        Self {
            capacity,
            tokens: capacity as f64,
            refill_rate,
            last_refill: std::time::Instant::now(),
        }
    }

    pub fn try_acquire(&mut self, tokens: u64) -> bool {
        self.refill();
        
        if self.tokens >= tokens as f64 {
            self.tokens -= tokens as f64;
            true
        } else {
            false
        }
    }

    fn refill(&mut self) {
        let elapsed = self.last_refill.elapsed().as_secs_f64();
        let new_tokens = elapsed * self.refill_rate;
        self.tokens = (self.tokens + new_tokens).min(self.capacity as f64);
        self.last_refill = std::time::Instant::now();
    }
}

// ============================================================================
// CIRCUIT BREAKER PATTERN
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

#[derive(Debug, Clone)]
pub struct CircuitBreaker {
    state: CircuitState,
    failure_threshold: u32,
    timeout_secs: u64,
    failures: u32,
    last_failure: std::time::Instant,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: u32, timeout_secs: u64) -> Self {
        Self {
            state: CircuitState::Closed,
            failure_threshold,
            timeout_secs,
            failures: 0,
            last_failure: std::time::Instant::now(),
        }
    }

    pub fn call<F, R>(&mut self, f: F) -> Result<R, String>
    where
        F: FnOnce() -> Result<R, String>,
    {
        match self.state {
            CircuitState::Closed => {
                match f() {
                    Ok(result) => {
                        self.failures = 0;
                        Ok(result)
                    }
                    Err(e) => {
                        self.failures += 1;
                        self.last_failure = std::time::Instant::now();
                        
                        if self.failures >= self.failure_threshold {
                            self.state = CircuitState::Open;
                        }
                        Err(e)
                    }
                }
            }
            CircuitState::Open => {
                let elapsed = self.last_failure.elapsed().as_secs();
                if elapsed >= self.timeout_secs {
                    self.state = CircuitState::HalfOpen;
                    Err("Circuit half-open".to_string())
                } else {
                    Err("Circuit open".to_string())
                }
            }
            CircuitState::HalfOpen => {
                match f() {
                    Ok(result) => {
                        self.state = CircuitState::Closed;
                        self.failures = 0;
                        Ok(result)
                    }
                    Err(_) => {
                        self.state = CircuitState::Open;
                        self.last_failure = std::time::Instant::now();
                        Err("Circuit opened".to_string())
                    }
                }
            }
        }
    }
}

// ============================================================================
// ADAPTIVE HASH MAP
// ============================================================================

#[derive(Debug, Clone)]
pub struct AdaptiveHashMap<K, V> {
    small_map: HashMap<K, V>,
    large_map: Option<Box<AdaptiveHashMap<K, V>>>,
    threshold: usize,
    is_split: bool,
}

impl<K: std::hash::Hash + Eq + Clone, V: Clone> AdaptiveHashMap<K, V> {
    pub fn new() -> Self {
        Self {
            small_map: HashMap::new(),
            large_map: None,
            threshold: 100,
            is_split: false,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if !self.is_split {
            self.small_map.insert(key.clone(), value);
            
            if self.small_map.len() > self.threshold {
                self.split();
            }
        } else if let Some(ref mut large) = self.large_map {
            large.insert(key, value);
        }
    }

    fn split(&mut self) {
        let half = self.small_map.len() / 2;
        let mut keys: Vec<K> = self.small_map.keys().cloned().collect();
        keys.sort_by(|a, b| {
            let mut ha = std::collections::hash_map::DefaultHasher::new();
            a.hash(&mut ha);
            let mut hb = std::collections::hash_map::DefaultHasher::new();
            b.hash(&mut hb);
            ha.finish().cmp(&hb.finish())
        });
        
        let split_key = keys.get(half).cloned();
        
        if let Some(key) = split_key {
            let value = self.small_map.remove(&key).unwrap();
            
            let mut new_map = AdaptiveHashMap::new();
            new_map.insert(key, value);
            self.large_map = Some(Box::new(new_map));
            self.is_split = true;
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        if !self.is_split {
            self.small_map.get(key)
        } else if let Some(ref large) = self.large_map {
            large.get(key).or_else(|| self.small_map.get(key))
        } else {
            None
        }
    }
}

impl<K: std::hash::Hash + Eq, V> std::ops::Index<&K> for AdaptiveHashMap<K, V> {
    type Output = V;
    
    fn index(&self, key: &K) -> &V {
        self.get(key).expect("Key not found")
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_trie() {
        let mut trie = Trie::new();
        trie.insert("/api/users", Some("api".to_string())).await;
        trie.insert("/api/posts", Some("api".to_string())).await;
        
        assert!(trie.search("/api/users").await);
        assert!(trie.search("/api/posts").await);
        assert!(!trie.search("/other").await);
    }

    #[test]
    fn test_minhash() {
        let mut mh1 = MinHash::new(100);
        mh1.add("hello world");
        mh1.add("world hello");
        
        let mut mh2 = MinHash::new(100);
        mh2.add("hello world");
        
        let similarity = mh1.jaccard_similarity(&mh2);
        assert!(similarity > 0.5);
    }

    #[test]
    fn test_simhash() {
        let mut sh1 = SimHash::new();
        sh1.from_text("The quick brown fox jumps over the lazy dog");
        
        let mut sh2 = SimHash::new();
        sh2.from_text("The quick brown fox jumps over the lazy dog");
        
        assert_eq!(sh1.hamming_distance(&sh2), 0);
    }

    #[test]
    fn test_token_bucket() {
        let mut bucket = TokenBucket::new(10, 1.0);
        
        assert!(bucket.try_acquire(5));
        assert!(bucket.try_acquire(5));
        assert!(!bucket.try_acquire(1)); // Should fail, bucket empty
    }
}