/*
 * =============================================================================
 * Module: Cache Module (LRU Cache Implementation)
 * Project: ScrapeFlux - Advanced Unified Web Scraping Framework
 * Author: Antor Roy
 * Email: antorroybabu@gmail.com
 * License: MIT
 * 
 * Description:
 *     High-performance LRU (Least Recently Used) cache implementation
 *     for optimal memory management and fast lookups.
 * 
 * Features:
 *     - O(1) get and put operations
 *     - Configurable capacity
 *     - Memory-efficient storage
 *     - Thread-safe operations
 *     - Automatic eviction of least recently used items
 * =============================================================================
 */

use lru::LruCache;
use std::hash::Hash;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct UrlCache {
    cache: Arc<RwLock<LruCache<String, CachedResponse>>>,
    max_capacity: usize,
}

#[derive(Clone)]
pub struct CachedResponse {
    pub body: String,
    pub status: u16,
    pub content_type: Option<String>,
    pub cached_at: u64,
    pub expires_at: u64,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
}

impl UrlCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: Arc::new(RwLock::new(LruCache::new(capacity))),
            max_capacity: capacity,
        }
    }

    pub async fn get(&self, url: &str) -> Option<CachedResponse> {
        let cache = self.cache.read().await;
        cache.get(url).cloned()
    }

    pub async fn put(&self, url: String, response: CachedResponse) {
        let mut cache = self.cache.write().await;
        if cache.len() >= self.max_capacity {
            cache.pop_lru();
        }
        cache.put(url, response);
    }

    pub async fn contains(&self, url: &str) -> bool {
        let cache = self.cache.read().await;
        cache.contains(url)
    }

    pub async fn remove(&self, url: &str) {
        let mut cache = self.cache.write().await;
        cache.pop(url);
    }

    pub async fn clear(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }

    pub async fn len(&self) -> usize {
        let cache = self.cache.read().await;
        cache.len()
    }

    pub async fn is_empty(&self) -> bool {
        let cache = self.cache.read().await;
        cache.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.max_capacity
    }
}

impl CachedResponse {
    pub fn new(body: String, status: u16, cache_duration_secs: u64) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        Self {
            body,
            status,
            content_type: None,
            cached_at: now,
            expires_at: now + cache_duration_secs,
            etag: None,
            last_modified: None,
        }
    }

    pub fn is_expired(&self) -> bool {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        now > self.expires_at
    }

    pub fn with_content_type(mut self, content_type: String) -> Self {
        self.content_type = Some(content_type);
        self
    }

    pub fn with_etag(mut self, etag: String) -> Self {
        self.etag = Some(etag);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_operations() {
        let cache = UrlCache::new(10);
        
        let response = CachedResponse::new("test body".to_string(), 200, 3600);
        cache.put("https://example.com".to_string(), response).await;
        
        let cached = cache.get("https://example.com").await;
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().body, "test body");
    }

    #[tokio::test]
    async fn test_cache_eviction() {
        let cache = UrlCache::new(2);
        
        for i in 0..3 {
            let url = format!("https://example{}.com", i);
            let response = CachedResponse::new(format!("body{}", i), 200, 3600);
            cache.put(url, response).await;
        }
        
        let first = cache.get("https://example0.com").await;
        assert!(first.is_none());
    }
}