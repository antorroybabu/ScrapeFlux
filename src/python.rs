/*
 * =============================================================================
 * Module: Python Bindings Module
 * Project: ScrapeFlux - Advanced Unified Web Scraping Framework
 * Author: Antor Roy
 * Email: antorroybabu@gmail.com
 * License: MIT
 * 
 * Description:
 *     PyO3 bindings for exposing Rust functionality to Python.
 *     Provides high-performance Python API for web scraping.
 * =============================================================================
 */

use crate::html::HtmlParser;
use crate::http::{HttpClient, HttpConfig};
use crate::crawler::{Crawler, CrawlerConfig, CrawledPage};
use crate::extractor::{Extractor, ExtractorConfig, Schema};
use crate::cache::UrlCache;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

#[pyclass]
pub struct PyHtmlParser {
    parser: Option<HtmlParser>,
    html_content: String,
}

#[pymethods]
impl PyHtmlParser {
    #[new]
    fn new(html: &str) -> Self {
        Self {
            parser: Some(HtmlParser::new(html)),
            html_content: html.to_string(),
        }
    }

    fn select(&mut self, selector: &str) -> PyResult<Vec<String>> {
        if let Some(ref mut parser) = self.parser {
            match parser.select(selector) {
                Ok(elements) => Ok(elements.iter()
                    .map(|el| parser.text(el))
                    .collect()),
                Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("No HTML loaded"))
        }
    }

    fn extract_links(&mut self, base_url: &str) -> PyResult<Vec<String>> {
        if let Some(ref mut parser) = self.parser {
            match parser.extract_links(base_url) {
                Ok(links) => Ok(links),
                Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("No HTML loaded"))
        }
    }

    fn text(&self, selector: &str) -> PyResult<String> {
        if let Some(ref mut parser) = self.parser {
            match parser.select(selector) {
                Ok(elements) => Ok(elements.first()
                    .map(|el| parser.text(el))
                    .unwrap_or_default()),
                Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("No HTML loaded"))
        }
    }

    fn html(&self, selector: &str) -> PyResult<String> {
        if let Some(ref mut parser) = self.parser {
            match parser.select(selector) {
                Ok(elements) => Ok(elements.first()
                    .map(|el| parser.html(el))
                    .unwrap_or_default()),
                Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("No HTML loaded"))
        }
    }
}

#[pyclass]
pub struct PyHttpClient {
    client: HttpClient,
}

#[pymethods]
impl PyHttpClient {
    #[new]
    fn new() -> PyResult<Self> {
        match HttpClient::new() {
            Ok(client) => Ok(Self { client }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())),
        }
    }

    #[args(timeout = 30, max_retries = 3)]
    fn with_config(timeout: u64, max_retries: u32) -> PyResult<Self> {
        let config = HttpConfig {
            timeout_secs: timeout,
            max_retries,
            ..Default::default()
        };
        match HttpClient::with_config(config) {
            Ok(client) => Ok(Self { client }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())),
        }
    }

    fn get(&self, url: &str) -> PyResult<Py<PyDict>> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let response = rt.block_on(self.client.get(url));
        
        Python::with_gil(|py| {
            match response {
                Ok(resp) => {
                    let dict = PyDict::new(py);
                    dict.set_item("status", resp.status)?;
                    dict.set_item("body", resp.body)?;
                    dict.set_item("url", resp.url)?;
                    dict.set_item("headers", resp.headers)?;
                    dict.set_item("response_time_ms", resp.response_time_ms)?;
                    Ok(dict.into())
                }
                Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())),
            }
        })
    }

    fn get_sync(&self, url: &str) -> PyResult<String> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let response = rt.block_on(self.client.get(url));
        
        match response {
            Ok(resp) => Ok(resp.body),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())),
        }
    }
}

#[pyclass]
pub struct PyCrawler {
    crawler: Crawler,
}

#[pymethods]
impl PyCrawler {
    #[new]
    fn new() -> PyResult<Self> {
        match Crawler::new() {
            Ok(crawler) => Ok(Self { crawler }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e)),
        }
    }

    fn crawl(&self, url: &str) -> PyResult<Py<PyDict>> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(self.crawler.crawl(url));
        
        Python::with_gil(|py| {
            match result {
                Ok(page) => {
                    let dict = PyDict::new(py);
                    dict.set_item("url", page.url)?;
                    dict.set_item("status", page.status)?;
                    dict.set_item("title", page.title)?;
                    dict.set_item("html", page.html)?;
                    dict.set_item("links", page.links)?;
                    dict.set_item("depth", page.depth)?;
                    dict.set_item("fetch_time_ms", page.fetch_time_ms)?;
                    dict.set_item("meta", page.meta)?;
                    Ok(dict.into())
                }
                Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())),
            }
        })
    }

    fn crawl_site(&self, url: &str) -> PyResult<Py<PyDict>> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(self.crawler.crawl_site(url));
        
        Python::with_gil(|py| {
            match result {
                Ok(crawl_result) => {
                    let dict = PyDict::new(py);
                    dict.set_item("total_pages", crawl_result.total_pages)?;
                    dict.set_item("total_links", crawl_result.total_links)?;
                    dict.set_item("duration_ms", crawl_result.duration_ms)?;
                    dict.set_item("bandwidth_mb", crawl_result.bandwidth_mb)?;
                    dict.set_item("errors", crawl_result.errors)?;
                    
                    let pages_list: Vec<Py<PyDict>> = crawl_result.pages.iter()
                        .map(|page| {
                            let page_dict = PyDict::new(py);
                            page_dict.set_item("url", &page.url).unwrap();
                            page_dict.set_item("status", page.status).unwrap();
                            page_dict.set_item("title", &page.title).unwrap();
                            page_dict.set_item("html", &page.html).unwrap();
                            page_dict.set_item("links", &page.links).unwrap();
                            page_dict.into()
                        })
                        .collect();
                    
                    dict.set_item("pages", pages_list)?;
                    Ok(dict.into())
                }
                Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())),
            }
        })
    }
}

#[pyclass]
pub struct PyExtractor {
    extractor: Extractor,
}

#[pymethods]
impl PyExtractor {
    #[new]
    fn new() -> Self {
        Self {
            extractor: Extractor::new(),
        }
    }

    fn extract(&self, html: &str, schema: &PyDict, url: &str) -> PyResult<Py<PyDict>> {
        let fields: Vec<(&str, &str)> = schema.iter()
            .filter_map(|(key, value)| {
                Some((key.to_string().as_str(), value.extract::<String>().ok()?))
            })
            .collect();
        
        let schema_obj = Schema::simple(fields);
        
        match self.extractor.extract(html, &schema_obj, url) {
            Ok(result) => {
                Python::with_gil(|py| {
                    let dict = PyDict::new(py);
                    let data_dict = PyDict::new(py);
                    for (key, values) in &result.data {
                        data_dict.set_item(key, values)?;
                    }
                    dict.set_item("data", data_dict)?;
                    dict.set_item("confidence", result.confidence)?;
                    Ok(dict.into())
                })
            }
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())),
        }
    }

    fn extract_products(&self, html: &str) -> PyResult<Vec<HashMap<String, String>>> {
        Ok(self.extractor.extract_products(html))
    }

    fn extract_articles(&self, html: &str) -> PyResult<Vec<HashMap<String, String>>> {
        Ok(self.extractor.extract_articles(html))
    }
}

#[pyclass]
pub struct PyScraper {
    http: HttpClient,
    extractor: Extractor,
}

#[pymethods]
impl PyScraper {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            http: HttpClient::new().map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())
            })?,
            extractor: Extractor::new(),
        })
    }

    fn scrape(&self, url: &str, schema: &PyDict) -> PyResult<Py<PyDict>> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        
        let result = rt.block_on(async {
            let response = self.http.get(url).await?;
            let mut parser = HtmlParser::new(&response.body);
            
            let fields: Vec<(&str, &str)> = schema.iter()
                .filter_map(|(key, value)| {
                    Some((key.to_string().as_str(), value.extract::<String>().ok()?))
                })
                .collect();
            
            let schema_obj = Schema::simple(fields);
            
            let mut data: HashMap<String, Vec<String>> = HashMap::new();
            
            for (name, selector) in &fields {
                if let Ok(elements) = parser.select(selector) {
                    let values: Vec<String> = elements.iter()
                        .map(|el| parser.text(el))
                        .collect();
                    data.insert(name.to_string(), values);
                }
            }
            
            Ok::<_, String>(data)
        });
        
        Python::with_gil(|py| {
            match result {
                Ok(data) => {
                    let dict = PyDict::new(py);
                    for (key, values) in &data {
                        dict.set_item(key, values)?;
                    }
                    Ok(dict.into())
                }
                Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e)),
            }
        })
    }

    fn get_html(&self, url: &str) -> PyResult<String> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let response = rt.block_on(self.http.get(url));
        
        match response {
            Ok(resp) => Ok(resp.body),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())),
        }
    }
}

#[pymodule]
fn scrapeflux(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyHtmlParser>()?;
    m.add_class::<PyHttpClient>()?;
    m.add_class::<PyCrawler>()?;
    m.add_class::<PyExtractor>()?;
    m.add_class::<PyScraper>()?;
    
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__author__", "Antor Roy <antorroybabu@gmail.com>")?;
    
    Ok(())
}