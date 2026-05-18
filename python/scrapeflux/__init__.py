"""
ScrapeFlux - Advanced Unified Web Scraping Framework
=====================================================

Author: Antor Roy
Email: antorroybabu@gmail.com
License: MIT

A high-performance web scraping framework that combines the best features
from ScrapeGraphAI, Crawl4AI, Scrapling, Stagehand, and Browser-use.
Built with Rust for maximum speed and Python bindings for ease of use.

Features:
    - Ultra-fast HTML parsing with advanced DOM traversal
    - Async HTTP with connection pooling and rate limiting
    - Smart crawling with Bloom filter deduplication
    - Schema-based extraction with pattern recognition
    - LRU caching for optimal memory usage
    - Python bindings via PyO3

Example:
    >>> from scrapeflux import Scraper
    >>> scraper = Scraper()
    >>> result = scraper.scrape("https://example.com", {"title": "h1", "links": "a"})
    >>> print(result)
"""

from .scraper import Scraper
from .crawler import Crawler
from .extractor import Extractor
from .http_client import HttpClient

__version__ = "0.1.0"
__author__ = "Antor Roy <antorroybabu@gmail.com>"

__all__ = ["Scraper", "Crawler", "Extractor", "HttpClient"]