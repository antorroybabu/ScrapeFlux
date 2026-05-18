/*
 * =============================================================================
 * Project: ScrapeFlux - Advanced Unified Web Scraping Framework
 * Author: Antor Roy
 * Email: antorroybabu@gmail.com
 * License: MIT
 * 
 * Description:
 *     A high-performance web scraping framework that combines the best features
 *     from ScrapeGraphAI, Crawl4AI, Scrapling, Stagehand, and Browser-use.
 *     Built with Rust for maximum speed and Python bindings for ease of use.
 * 
 * Features:
 *     - Ultra-fast HTML parsing with advanced DOM traversal
 *     - Async HTTP with connection pooling and rate limiting
 *     - Smart crawling with Bloom filter deduplication
 *     - Schema-based extraction with pattern recognition
 *     - LRU caching for optimal memory usage
 *     - Python bindings via PyO3
 * 
 * Architecture:
 *     - Rust core: Maximum performance for critical operations
 *     - Python API: Easy-to-use high-level interface
 *     - Hybrid design: Best of both worlds
 * =============================================================================
 */

pub mod html;
pub mod http;
pub mod crawler;
pub mod extractor;
pub mod cache;
pub mod utils;
pub mod advanced_data;
pub mod super_crawler;
pub mod unified;
pub mod llm_engine;
pub mod browser_automation;
pub mod ai_agent;
pub mod google_extractor;

#[cfg(feature = "python")]
pub mod python;

pub use html::HtmlParser;
pub use http::HttpClient;
pub use crawler::Crawler;
pub use extractor::Extractor;
pub use cache::UrlCache;
pub use super_crawler::{SuperCrawler, SuperCrawledPage, CrawlResult as SuperCrawlResult};
pub use advanced_data::*;
pub use unified::*;
pub use llm_engine::*;
pub use browser_automation::*;
pub use ai_agent::*;
pub use google_extractor::*;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHOR: &str = "Antor Roy <antorroybabu@gmail.com>";
pub const GITHUB: &str = "https://github.com/antorroybabu/ScrapeFlux";

#[cfg(test)]
mod tests {
    #[test]
    fn test_version() {
        assert_eq!(super::VERSION, "0.1.0");
    }
}