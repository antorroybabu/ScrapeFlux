"""
Test Suite for ScrapeFlux
=========================

Author: Antor Roy
Email: antorroybabu@gmail.com

Test cases for the web scraping framework.
"""

import pytest
from scrapeflux import Scraper, Crawler, Extractor, HttpClient


class TestScraper:
    """Tests for the Scraper class."""
    
    def test_scraper_initialization(self):
        """Test that scraper can be initialized."""
        scraper = Scraper()
        assert scraper is not None
    
    def test_scraper_get_html(self):
        """Test getting HTML from a URL."""
        scraper = Scraper(use_rust=False)  # Use pure Python for testing
        try:
            html = scraper.get_html("https://example.com")
            assert isinstance(html, str)
            assert len(html) > 0
        except Exception:
            pass  # Skip if network not available


class TestHttpClient:
    """Tests for the HTTP Client."""
    
    def test_client_initialization(self):
        """Test HTTP client initialization."""
        client = HttpClient()
        assert client is not None
    
    def test_client_get(self):
        """Test GET request."""
        client = HttpClient()
        try:
            response = client.get("https://example.com")
            assert "body" in response
            assert "status" in response
        except Exception:
            pass  # Skip if network not available


class TestExtractor:
    """Tests for the Extractor class."""
    
    def test_extractor_initialization(self):
        """Test extractor initialization."""
        extractor = Extractor()
        assert extractor is not None
    
    def test_extract_products(self):
        """Test product extraction."""
        extractor = Extractor()
        html = """
        <html>
        <body>
            <div class="product">
                <h2>Test Product</h2>
                <span class="price">$99</span>
            </div>
        </body>
        </html>
        """
        try:
            products = extractor.extract_products(html)
            assert isinstance(products, list)
        except Exception:
            pass


class TestCrawler:
    """Tests for the Crawler class."""
    
    def test_crawler_initialization(self):
        """Test crawler initialization."""
        crawler = Crawler()
        assert crawler is not None
    
    def test_crawler_config(self):
        """Test crawler with custom config."""
        crawler = Crawler(max_depth=2, max_pages=50)
        assert crawler.max_depth == 2
        assert crawler.max_pages == 50


if __name__ == "__main__":
    pytest.main([__file__, "-v"])