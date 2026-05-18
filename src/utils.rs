/*
 * =============================================================================
 * Module: Utilities Module
 * Project: ScrapeFlux - Advanced Unified Web Scraping Framework
 * Author: Antor Roy
 * Email: antorroybabu@gmail.com
 * License: MIT
 * 
 * Description:
 *     Utility functions and helpers for web scraping operations.
 * 
 * Features:
 *     - URL normalization and validation
 *     - Text cleaning and normalization
 *     - Common patterns
 *     - Performance metrics
 * =============================================================================
 */

use std::collections::HashSet;
use url::Url;

pub struct UrlUtils;

impl UrlUtils {
    pub fn normalize(url: &str) -> Result<String, String> {
        let mut parsed = Url::parse(url)
            .map_err(|e| format!("Invalid URL: {}", e))?;
        
        parsed.set_fragment(None);
        
        if parsed.path() == "" {
            parsed.set_path("/");
        }
        
        Ok(parsed.to_string())
    }

    pub fn is_valid(url: &str) -> bool {
        Url::parse(url).is_ok()
    }

    pub fn is_same_domain(url1: &str, url2: &str) -> bool {
        let (domain1, domain2) = match (Url::parse(url1), Url::parse(url2)) {
            (Ok(u1), Ok(u2)) => (u1.host_str(), u2.host_str()),
            _ => return false,
        };
        
        domain1 == domain2
    }

    pub fn extract_domain(url: &str) -> Option<String> {
        Url::parse(url).ok().and_then(|u| u.host_str().map(|s| s.to_string()))
    }

    pub fn filter_same_domain(urls: Vec<String>, base_url: &str) -> Vec<String> {
        let base_domain = Self::extract_domain(base_url);
        
        urls.into_iter()
            .filter(|url| {
                Self::extract_domain(url) == base_domain
            })
            .collect()
    }

    pub fn make_absolute(relative: &str, base: &str) -> Option<String> {
        let base_url = Url::parse(base).ok()?;
        base_url.join(relative).ok().map(|u| u.to_string())
    }

    pub fn remove_fragments(urls: Vec<String>) -> Vec<String> {
        urls.into_iter()
            .filter_map(|url| Self::normalize(&url).ok())
            .collect()
    }
}

pub struct TextUtils;

impl TextUtils {
    pub fn clean(text: &str) -> String {
        text.split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string()
    }

    pub fn remove_html_tags(html: &str) -> String {
        let mut result = String::new();
        let mut in_tag = false;
        
        for ch in html.chars() {
            match ch {
                '<' => in_tag = true,
                '>' => in_tag = false,
                _ if !in_tag => result.push(ch),
                _ => {}
            }
        }
        
        Self::clean(&result)
    }

    pub fn truncate(text: &str, max_len: usize) -> String {
        if text.len() <= max_len {
            text.to_string()
        } else {
            format!("{}...", &text[..max_len.saturating_sub(3)])
        }
    }

    pub fn extract_numbers(text: &str) -> Vec<f64> {
        text.split_whitespace()
            .filter_map(|word| {
                word.replace(&[',', '$', '€', '£'][..], "")
                    .parse::<f64>()
                    .ok()
            })
            .collect()
    }

    pub fn normalize_whitespace(text: &str) -> String {
        text.split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn to_snake_case(text: &str) -> String {
        let mut result = String::new();
        
        for (i, ch) in text.chars().enumerate() {
            if ch.is_uppercase() && i > 0 {
                result.push('_');
            }
            result.push(ch.to_lowercase().next().unwrap_or(ch));
        }
        
        result
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_')
            .collect()
    }
}

pub struct PerformanceMetrics {
    pub requests: usize,
    pub total_bytes: u64,
    pub duration_ms: u64,
    pub errors: usize,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            requests: 0,
            total_bytes: 0,
            duration_ms: 0,
            errors: 0,
        }
    }

    pub fn throughput_mbps(&self) -> f64 {
        if self.duration_ms > 0 {
            (self.total_bytes as f64 / 1_000_000.0) / (self.duration_ms as f64 / 1000.0)
        } else {
            0.0
        }
    }

    pub fn requests_per_second(&self) -> f64 {
        if self.duration_ms > 0 {
            (self.requests as f64) / (self.duration_ms as f64 / 1000.0)
        } else {
            0.0
        }
    }

    pub fn success_rate(&self) -> f64 {
        let total = self.requests + self.errors;
        if total > 0 {
            (self.requests as f64) / (total as f64)
        } else {
            0.0
        }
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self::new()
    }
}

pub fn extract_emails(text: &str) -> Vec<String> {
    let email_pattern = regex_lite::Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap();
    
    email_pattern.find_iter(text)
        .map(|m| m.as_str().to_string())
        .collect()
}

pub fn extract_urls(text: &str) -> Vec<String> {
    let url_pattern = regex_lite::Regex::new(r"https?://[^\s<>\"]+").unwrap();
    
    url_pattern.find_iter(text)
        .map(|m| m.as_str().to_string())
        .collect()
}

pub fn hash_url(url: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_normalization() {
        let url = "https://example.com/page#section";
        let normalized = UrlUtils::normalize(url).unwrap();
        assert!(!normalized.contains("#"));
    }

    #[test]
    fn test_text_cleaning() {
        let text = "  Hello   World  ";
        let cleaned = TextUtils::clean(text);
        assert_eq!(cleaned, "Hello World");
    }

    #[test]
    fn test_html_tag_removal() {
        let html = "<p>Hello <b>World</b></p>";
        let text = TextUtils::remove_html_tags(html);
        assert_eq!(text, "Hello World");
    }
}