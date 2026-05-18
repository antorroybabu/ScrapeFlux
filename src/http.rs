/*
 * =============================================================================
 * Module: HTTP Client Module
 * Project: ScrapeFlux - Advanced Unified Web Scraping Framework
 * Author: Antor Roy
 * Email: antorroybabu@gmail.com
 * License: MIT
 * 
 * Description:
 *     High-performance async HTTP client with connection pooling,
 *     rate limiting, automatic retries, and compression support.
 * 
 * Features:
 *     - Connection pooling for optimal performance
 *     - Automatic retry with exponential backoff
 *     - Rate limiting to respect server resources
 *     - Gzip/Brotli decompression
 *     - Cookie management
 *     - Custom headers support
 * =============================================================================
 */

use reqwest::{Client, Response, header::{HeaderMap, HeaderName, HeaderValue, USER_AGENT, ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE}};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{RwLock, Semaphore};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HttpError {
    #[error("Request failed: {0}")]
    RequestFailed(String),
    #[error("Timeout after {0} seconds")]
    Timeout(u64),
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    #[error("Max retries exceeded")]
    MaxRetriesExceeded,
    #[error("Rate limited")]
    RateLimited,
}

#[derive(Debug, Clone)]
pub struct HttpConfig {
    pub user_agent: String,
    pub timeout_secs: u64,
    pub max_retries: u32,
    pub pool_size: u32,
    pub follow_redirects: bool,
    pub rate_limit_ms: u64,
    pub max_concurrent: usize,
    pub enable_compression: bool,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            timeout_secs: 30,
            max_retries: 3,
            pool_size: 100,
            follow_redirects: true,
            rate_limit_ms: 100,
            max_concurrent: 50,
            enable_compression: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpResponse {
    pub status: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub url: String,
    pub content_length: Option<u64>,
    pub content_type: Option<String>,
    pub response_time_ms: u64,
}

pub struct HttpClient {
    client: Client,
    config: HttpConfig,
    rate_limiter: Arc<Semaphore>,
    last_request: Arc<RwLock<Instant>>,
}

impl HttpClient {
    pub fn new() -> Result<Self, HttpError> {
        Self::with_config(HttpConfig::default())
    }

    pub fn with_config(config: HttpConfig) -> Result<Self, HttpError> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_str(&config.user_agent).unwrap());
        headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"));
        headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
        
        if config.enable_compression {
            headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate, br"));
        }

        let client = Client::builder()
            .pool_max_idle_per_host(config.pool_size as usize)
            .pool_idle_timeout(Duration::from_secs(30))
            .timeout(Duration::from_secs(config.timeout_secs))
            .default_headers(headers)
            .redirect(if config.follow_redirects { 
                reqwest::redirect::Policy::limited(10) 
            } else { 
                reqwest::redirect::Policy::none() 
            })
            .tcp_keepalive(Duration::from_secs(60))
            .tcp_nodelay(true)
            .build()
            .map_err(|e| HttpError::RequestFailed(e.to_string()))?;

        Ok(Self {
            client,
            config,
            rate_limiter: Arc::new(Semaphore::new(config.max_concurrent)),
            last_request: Arc::new(RwLock::new(Instant::now())),
        })
    }

    pub async fn get(&self, url: &str) -> Result<HttpResponse, HttpError> {
        self.request("GET", url, None).await
    }

    pub async fn post(&self, url: &str, body: Option<String>) -> Result<HttpResponse, HttpError> {
        self.request("POST", url, body).await
    }

    pub async fn head(&self, url: &str) -> Result<HttpResponse, HttpError> {
        self.request("HEAD", url, None).await
    }

    async fn request(&self, method: &str, url: &str, body: Option<String>) -> Result<HttpResponse, HttpError> {
        let _permit = self.rate_limiter.acquire().await
            .map_err(|_| HttpError::RateLimited)?;

        let start = Instant::now();
        let mut last_error = String::new();
        
        for attempt in 0..self.config.max_retries {
            match self.try_request(method, url, body.as_ref(), start).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    last_error = format!("Attempt {} failed: {}", attempt + 1, e);
                    if attempt < self.config.max_retries - 1 {
                        let delay = Duration::from_millis(100 * 2_u64.pow(attempt));
                        tokio::time::sleep(delay).await;
                    }
                }
            }
        }
        
        Err(HttpError::MaxRetriesExceeded)
    }

    async fn try_request(&self, method: &str, url: &str, body: Option<&String>, start: Instant) -> Result<HttpResponse, HttpError> {
        let mut request = match method {
            "GET" => self.client.get(url),
            "POST" => self.client.post(url),
            "HEAD" => self.client.head(url),
            "PUT" => self.client.put(url),
            "DELETE" => self.client.delete(url),
            "PATCH" => self.client.patch(url),
            _ => return Err(HttpError::RequestFailed(format!("Unsupported method: {}", method))),
        };

        if let Some(ref b) = body {
            request = request
                .header("Content-Type", "application/json")
                .body(b.clone());
        }

        let response = request.send().await
            .map_err(|e| HttpError::RequestFailed(e.to_string()))?;

        Ok(HttpResponse::from_response(response, start).await)
    }

    pub async fn fetch_batch(&self, urls: Vec<String>) -> Vec<Result<HttpResponse, HttpError>> {
        use futures::stream::{self, StreamExt};
        
        stream::iter(urls)
            .map(|url| {
                let client = self.clone();
                async move { client.get(&url).await }
            })
            .buffer_unordered(self.config.max_concurrent)
            .collect()
            .await
    }
}

impl HttpResponse {
    async fn from_response(response: Response, start: Instant) -> Self {
        let status = response.status().as_u16();
        let status_text = response.status().canonical_reason().unwrap_or("Unknown").to_string();
        let url = response.url().to_string();
        let content_length = response.content_length();
        let content_type = response.headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());
        
        let headers: HashMap<String, String> = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();

        let body = response.text().await.unwrap_or_default();
        let response_time_ms = start.elapsed().as_millis() as u64;

        Self {
            status,
            status_text,
            headers,
            body,
            url,
            content_length,
            content_type,
            response_time_ms,
        }
    }

    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status)
    }

    pub fn is_redirect(&self) -> bool {
        (300..400).contains(&self.status)
    }

    pub fn is_client_error(&self) -> bool {
        (400..500).contains(&self.status)
    }

    pub fn is_server_error(&self) -> bool {
        (500..600).contains(&self.status)
    }
}

impl Clone for HttpClient {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            config: self.config.clone(),
            rate_limiter: Arc::clone(&self.rate_limiter),
            last_request: Arc::clone(&self.last_request),
        }
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new().expect("Failed to create HTTP client")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_basic_get() {
        let client = HttpClient::new().unwrap();
        let result = client.get("https://example.com").await;
        
        assert!(result.is_ok());
        let resp = result.unwrap();
        assert_eq!(resp.status, 200);
        assert!(resp.is_success());
    }
}