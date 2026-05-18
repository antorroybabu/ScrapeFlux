"""
Web Crawler API
==============

Author: Antor Roy
Email: antorroybabu@gmail.com

Provides functionality for crawling websites with depth control.
"""

from typing import Dict, List, Optional, Any

try:
    from scrapeflux import PyCrawler as _RustCrawler
    RUST_AVAILABLE = True
except ImportError:
    RUST_AVAILABLE = False


class Crawler:
    """
    High-performance web crawler.
    
    Example usage:
        crawler = Crawler()
        result = crawler.crawl("https://example.com")
        print(result["url"], result["title"])
    """
    
    def __init__(self, max_depth: int = 3, max_pages: int = 100):
        """
        Initialize the crawler.
        
        Args:
            max_depth: Maximum crawling depth
            max_pages: Maximum number of pages to crawl
        """
        self.max_depth = max_depth
        self.max_pages = max_pages
        
        if RUST_AVAILABLE:
            self._crawler = _RustCrawler()
        else:
            self._crawler = None
    
    def crawl(self, url: str) -> Dict[str, Any]:
        """
        Crawl a single URL.
        
        Args:
            url: The URL to crawl
            
        Returns:
            Dict with url, status, title, html, links, etc.
        """
        if RUST_AVAILABLE:
            return self._crawler.crawl(url)
        else:
            raise NotImplementedError("Rust backend not available")
    
    def crawl_site(self, url: str) -> Dict[str, Any]:
        """
        Crawl entire website starting from URL.
        
        Args:
            url: Starting URL for crawling
            
        Returns:
            Dict with pages list, total_pages, total_links, duration_ms, errors
        """
        if RUST_AVAILABLE:
            return self._crawler.crawl_site(url)
        else:
            raise NotImplementedError("Rust backend not available")
    
    def crawl_batch(self, urls: List[str]) -> List[Dict[str, Any]]:
        """
        Crawl multiple URLs concurrently.
        
        Args:
            urls: List of URLs to crawl
            
        Returns:
            List of results for each URL
        """
        if RUST_AVAILABLE:
            return self._crawler.crawl_batch(urls)
        else:
            raise NotImplementedError("Rust backend not available")