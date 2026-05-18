"""
Data Extractor API
=================

Author: Antor Roy
Email: antorroybabu@gmail.com

Provides schema-based data extraction from HTML.
"""

from typing import Dict, List, Optional, Any

try:
    from scrapeflux import PyExtractor as _RustExtractor
    RUST_AVAILABLE = True
except ImportError:
    RUST_AVAILABLE = False


class Extractor:
    """
    High-performance data extractor with schema support.
    
    Example usage:
        extractor = Extractor()
        result = extractor.extract(html, {"title": "h1", "links": "a"}, "https://example.com")
        print(result["data"]["title"])
    """
    
    def __init__(self):
        """Initialize the extractor."""
        if RUST_AVAILABLE:
            self._extractor = _RustExtractor()
        else:
            self._extractor = None
    
    def extract(self, html: str, schema: Dict[str, str], url: str = "") -> Dict[str, Any]:
        """
        Extract data from HTML using a schema.
        
        Args:
            html: HTML content to extract from
            schema: Dict mapping field names to CSS selectors
            url: Optional URL for metadata
            
        Returns:
            Dict with "data" key containing extracted values
        """
        if RUST_AVAILABLE:
            return self._extractor.extract(html, schema, url)
        else:
            raise NotImplementedError("Rust backend not available")
    
    def extract_products(self, html: str) -> List[Dict[str, str]]:
        """
        Extract product information from e-commerce HTML.
        
        Args:
            html: HTML content
            
        Returns:
            List of product dictionaries with title, price, image
        """
        if RUST_AVAILABLE:
            return self._extractor.extract_products(html)
        else:
            raise NotImplementedError("Rust backend not available")
    
    def extract_articles(self, html: str) -> List[Dict[str, str]]:
        """
        Extract article information from blog/news HTML.
        
        Args:
            html: HTML content
            
        Returns:
            List of article dictionaries with title, content, date
        """
        if RUST_AVAILABLE:
            return self._extractor.extract_articles(html)
        else:
            raise NotImplementedError("Rust backend not available")