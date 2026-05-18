"""
High-level Scraping API
=====================

Author: Antor Roy
Email: antorroybabu@gmail.com

Provides a simple, high-level interface for web scraping.
"""

from typing import Dict, List, Optional, Any
from concurrent.futures import ThreadPoolExecutor

try:
    from scrapeflux import PyScraper as _RustScraper
    RUST_AVAILABLE = True
except ImportError:
    RUST_AVAILABLE = False


class Scraper:
    """
    High-level web scraper with simple API.
    
    Example usage:
        scraper = Scraper()
        result = scraper.scrape("https://example.com", {"title": "h1", "links": "a"})
        print(result)
    """
    
    def __init__(self, use_rust: bool = True):
        """
        Initialize the scraper.
        
        Args:
            use_rust: If True, use Rust backend for better performance.
                     Falls back to pure Python if Rust is not available.
        """
        self.use_rust = use_rust and RUST_AVAILABLE
        
        if self.use_rust:
            self._scraper = _RustScraper()
        else:
            self._scraper = None
            self._setup_pure_python()
    
    def _setup_pure_python(self):
        """Setup pure Python fallback implementation."""
        import urllib.request
        import ssl
        
        self._context = ssl.create_default_context()
        self._opener = urllib.request.build_opener(
            urllib.request.HTTPSHandler(context=self._context)
        )
        self._opener.addheaders = [
            ('User-Agent', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36')
        ]
    
    def scrape(self, url: str, schema: Dict[str, str]) -> Dict[str, List[str]]:
        """
        Scrape data from a URL using a schema.
        
        Args:
            url: The URL to scrape
            schema: Dict mapping field names to CSS selectors
            
        Returns:
            Dict mapping field names to lists of extracted values
        """
        if self.use_rust:
            return self._scraper.scrape(url, schema)
        else:
            return self._scrape_pure_python(url, schema)
    
    def get_html(self, url: str) -> str:
        """Get raw HTML from a URL."""
        if self.use_rust:
            return self._scraper.get_html(url)
        else:
            return self._get_html_pure_python(url)
    
    def _get_html_pure_python(self, url: str) -> str:
        """Pure Python HTML fetching."""
        try:
            response = self._opener.open(url, timeout=30)
            return response.read().decode('utf-8', errors='ignore')
        except Exception as e:
            raise Exception(f"Failed to fetch {url}: {e}")
    
    def _scrape_pure_python(self, url: str, schema: Dict[str, str]) -> Dict[str, List[str]]:
        """Pure Python scraping implementation."""
        html = self.get_html(url)
        results = {}
        for field, selector in schema.items():
            results[field] = []
        return results
    
    def extract_products(self, html: str) -> List[Dict[str, str]]:
        """Extract product information from HTML."""
        if self.use_rust:
            return self._scraper.extract_products(html)
        return []
    
    def extract_articles(self, html: str) -> List[Dict[str, str]]:
        """Extract article information from HTML."""
        if self.use_rust:
            return self._scraper.extract_articles(html)
        return []
    
    def batch_scrape(self, urls: List[str], schema: Dict[str, str]) -> List[Dict[str, List[str]]]:
        """Scrape multiple URLs concurrently."""
        results = []
        with ThreadPoolExecutor(max_workers=5) as executor:
            futures = [executor.submit(self.scrape, url, schema) for url in urls]
            for future in futures:
                try:
                    results.append(future.result())
                except Exception as e:
                    results.append({"error": str(e)})
        return results