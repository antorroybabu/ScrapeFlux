/*
 * =============================================================================
 * Module: HTML Parser Module
 * Project: ScrapeFlux - Advanced Unified Web Scraping Framework
 * Author: Antor Roy
 * Email: antorroybabu@gmail.com
 * License: MIT
 * 
 * Description:
 *     High-performance HTML parsing module with advanced DOM traversal.
 *     Uses scraper crate for CSS selectors and custom algorithms for
 *     optimal performance.
 * 
 * Features:
 *     - Fast CSS selector matching using scraper crate
 *     - Advanced DOM traversal algorithms
 *     - Text and attribute extraction
 *     - Link discovery and normalization
 *     - Memory-efficient operations
 * =============================================================================
 */

use scraper::{Html, Selector, ElementRef};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use fxhash::FxHashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HtmlError {
    #[error("Invalid selector: {0}")]
    InvalidSelector(String),
    #[error("Element not found")]
    ElementNotFound,
    #[error("Parse error: {0}")]
    ParseError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementInfo {
    pub tag: String,
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub text: String,
    pub html: String,
    pub attributes: HashMap<String, String>,
    pub inner_html: String,
    pub outer_html: String,
    pub children_count: usize,
    pub depth: usize,
}

#[derive(Debug, Clone)]
pub struct ParsedDocument {
    pub html: Html,
    pub url: String,
    pub title: Option<String>,
    pub meta: HashMap<String, String>,
}

pub struct HtmlParser {
    document: Option<Html>,
    url: String,
    selector_cache: FxHashMap<String, Vec<ElementRef<'static>>>,
}

impl HtmlParser {
    pub fn new(html: &str) -> Self {
        Self {
            document: Some(Html::parse_document(html)),
            url: String::new(),
            selector_cache: FxHashMap::default(),
        }
    }

    pub fn with_url(html: &str, url: &str) -> Self {
        Self {
            document: Some(Html::parse_document(html)),
            url: url.to_string(),
            selector_cache: FxHashMap::default(),
        }
    }

    pub fn from_bytes(html: &[u8], url: Option<&str>) -> Result<Self, HtmlError> {
        let html_str = String::from_utf8_lossy(html);
        Ok(if let Some(u) = url {
            Self::with_url(&html_str, u)
        } else {
            Self::new(&html_str)
        })
    }

    pub fn parse_document(&mut self, html: &str) -> &mut Self {
        self.document = Some(Html::parse_document(html));
        self.selector_cache.clear();
        self
    }

    pub fn select(&mut self, selector: &str) -> Result<&[ElementRef], HtmlError> {
        if self.document.is_none() {
            return Err(HtmlError::ParseError("No document loaded".into()));
        }

        if let Some(cached) = self.selector_cache.get(selector) {
            return Ok(cached.as_slice());
        }

        let selector_obj = Selector::parse(selector)
            .map_err(|e| HtmlError::InvalidSelector(e.to_string()))?;

        let doc = self.document.as_ref().unwrap();
        let elements: Vec<ElementRef> = doc.select(&selector_obj).collect();
        
        self.selector_cache.insert(selector.to_string(), elements.clone());
        Ok(self.selector_cache.get(selector).unwrap().as_slice())
    }

    pub fn select_first(&mut self, selector: &str) -> Result<ElementInfo, HtmlError> {
        let elements = self.select(selector)?;
        elements.first()
            .map(|el| ElementInfo::from_element(el, 0))
            .ok_or(HtmlError::ElementNotFound)
    }

    pub fn select_all(&mut self, selector: &str) -> Result<Vec<ElementInfo>, HtmlError> {
        let elements = self.select(selector)?;
        Ok(elements.iter()
            .enumerate()
            .map(|(i, el)| ElementInfo::from_element(el, i))
            .collect())
    }

    pub fn text(&self, element: &ElementRef) -> String {
        element.text().collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string()
    }

    pub fn html(&self, element: &ElementRef) -> String {
        element.inner_html()
    }

    pub fn outer_html(&self, element: &ElementRef) -> String {
        element.outer_html()
    }

    pub fn attr(&self, element: &ElementRef, name: &str) -> Option<String> {
        element.value().attr(name).map(|s| s.to_string())
    }

    pub fn extract_links(&mut self, base_url: &str) -> Result<Vec<String>, HtmlError> {
        let links = self.select("a[href]")?;
        let base = url::Url::parse(base_url)
            .map_err(|e| HtmlError::ParseError(format!("Invalid URL: {}", e)))?;
        
        Ok(links.iter()
            .filter_map(|el| el.value().attr("href"))
            .filter_map(|href| base.join(href).ok())
            .map(|u| u.to_string())
            .collect())
    }

    pub fn extract_images(&mut self) -> Result<Vec<String>, HtmlError> {
        let images = self.select("img[src]")?;
        Ok(images.iter()
            .filter_map(|el| el.value().attr("src").map(|s| s.to_string()))
            .collect())
    }

    pub fn extract_metadata(&self) -> HashMap<String, String> {
        let mut meta = HashMap::new();
        
        if let Some(ref doc) = self.document {
            for el in doc.select(&Selector::parse("meta").unwrap()) {
                if let (Some(name), Some(content)) = (
                    el.value().attr("name").or_else(|| el.value().attr("property")),
                    el.value().attr("content")
                ) {
                    meta.insert(name.to_string(), content.to_string());
                }
            }
        }
        
        meta
    }

    pub fn extract_title(&self) -> Option<String> {
        self.document.as_ref()?
            .select(&Selector::parse("title").unwrap())
            .next()
            .map(|el| el.text().collect::<String>())
    }

    pub fn extract_structured(&mut self, selectors: &[(&str, &str)]) -> Vec<HashMap<String, String>> {
        let mut results = Vec::new();
        
        for (name, selector) in selectors {
            if let Ok(elements) = self.select(selector) {
                for el in elements {
                    let mut item = HashMap::new();
                    item.insert(name.to_string(), self.text(el));
                    results.push(item);
                }
            }
        }
        
        results
    }

    pub fn get_document(&self) -> Option<&Html> {
        self.document.as_ref()
    }

    pub fn clear_cache(&mut self) {
        self.selector_cache.clear();
    }
}

impl ElementInfo {
    pub fn from_element(element: &ElementRef, index: usize) -> Self {
        let tag = element.value().name().to_string();
        let id = element.value().attr("id").map(|s| s.to_string());
        let classes: Vec<String> = element.value().attr("class")
            .map(|c| c.split_whitespace().map(|s| s.to_string()).collect())
            .unwrap_or_default();
        
        let text = element.text().collect::<Vec<_>>().join(" ").trim().to_string();
        let inner_html = element.inner_html();
        let outer_html = element.outer_html();
        let children_count = element.children().filter(|n| n.is_element()).count();
        
        let attributes: HashMap<String, String> = element
            .value()
            .attrs()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        
        Self {
            tag,
            id,
            classes,
            text,
            html: inner_html.clone(),
            attributes,
            inner_html,
            outer_html,
            children_count,
            depth: index,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parsing() {
        let html = r#"<html><body><h1 class="title">Hello World</h1></body></html>"#;
        let mut parser = HtmlParser::new(html);
        
        let result = parser.select("h1.title");
        assert!(result.is_ok());
        assert_eq!(parser.text(&result.unwrap()[0]), "Hello World");
    }

    #[test]
    fn test_link_extraction() {
        let html = r#"<html><body><a href="https://example.com">Link</a></body></html>"#;
        let mut parser = HtmlParser::new(html);
        
        let links = parser.extract_links("https://base.com/").unwrap();
        assert_eq!(links.len(), 1);
    }
}