"""
HTTP Client API
===============

Author: Antor Roy
Email: antorroybabu@gmail.com

Provides async HTTP client with connection pooling.
"""

from typing import Dict, Optional, Any

try:
    from scrapeflux import PyHttpClient as _RustHttpClient
    RUST_AVAILABLE = True
except ImportError:
    RUST_AVAILABLE = False


class HttpClient:
    """
    High-performance HTTP client.
    
    Example usage:
        client = HttpClient()
        response = client.get("https://example.com")
        print(response["body"])
    """
    
    def __init__(self, timeout: int = 30, max_retries: int = 3):
        """
        Initialize the HTTP client.
        
        Args:
            timeout: Request timeout in seconds
            max_retries: Maximum number of retries
        """
        self.timeout = timeout
        self.max_retries = max_retries
        
        if RUST_AVAILABLE:
            self._client = _RustHttpClient.with_config(timeout, max_retries)
        else:
            self._client = None
            self._setup_pure_python()
    
    def _setup_pure_python(self):
        """Setup pure Python fallback."""
        import urllib.request
        import ssl
        
        self._context = ssl.create_default_context()
        self._opener = urllib.request.build_opener(
            urllib.request.HTTPSHandler(context=self._context)
        )
    
    def get(self, url: str) -> Dict[str, Any]:
        """
        Perform GET request.
        
        Args:
            url: URL to fetch
            
        Returns:
            Dict with status, body, headers, etc.
        """
        if RUST_AVAILABLE:
            return self._client.get(url)
        else:
            return self._get_pure_python(url)
    
    def _get_pure_python(self, url: str) -> Dict[str, Any]:
        """Pure Python GET request."""
        try:
            response = self._opener.open(url, timeout=self.timeout)
            return {
                "status": response.getcode(),
                "body": response.read().decode('utf-8', errors='ignore'),
                "url": url,
                "headers": dict(response.headers),
            }
        except Exception as e:
            raise Exception(f"Request failed: {e}")