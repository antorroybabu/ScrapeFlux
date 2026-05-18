/*
 * =============================================================================
 * Module: Data Extractor Module
 * Project: ScrapeFlux - Advanced Unified Web Scraping Framework
 * Author: Antor Roy
 * Email: antorroybabu@gmail.com
 * License: MIT
 * 
 * Description:
 *     High-performance data extraction with schema-based approach,
 *     pattern recognition, and specialized extractors for common use cases.
 * 
 * Features:
 *     - Schema-based extraction
 *     - Product extraction (e-commerce)
 *     - Article extraction (blogs/news)
 *     - Link extraction with metadata
 *     - Pattern-based field matching
 *     - Multi-field extraction
 * =============================================================================
 */

use crate::html::{HtmlParser, ElementInfo};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::time::Instant;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExtractorError {
    #[error("Invalid schema: {0}")]
    InvalidSchema(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Element not found")]
    ElementNotFound,
}

#[derive(Debug, Clone)]
pub struct ExtractorConfig {
    pub strict_mode: bool,
    pub include_metadata: bool,
    pub fallback_enabled: bool,
    pub confidence_threshold: f64,
}

impl Default for ExtractorConfig {
    fn default() -> Self {
        Self {
            strict_mode: false,
            include_metadata: true,
            fallback_enabled: true,
            confidence_threshold: 0.7,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionResult {
    pub data: HashMap<String, Vec<String>>,
    pub metadata: Option<ExtractionMetadata>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionMetadata {
    pub url: String,
    pub selectors_used: Vec<String>,
    pub elements_found: usize,
    pub extraction_time_ms: u64,
    pub extraction_date: String,
}

#[derive(Debug, Clone)]
pub struct Schema {
    pub fields: Vec<SchemaField>,
}

#[derive(Debug, Clone)]
pub struct SchemaField {
    pub name: String,
    pub selector: String,
    pub attr: Option<String>,
    pub multiple: bool,
    pub transform: Option<TransformType>,
    pub fallback_selectors: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum TransformType {
    Trim,
    Lowercase,
    Uppercase,
    Strip,
    Regex(String),
}

pub struct Extractor {
    config: ExtractorConfig,
}

impl Extractor {
    pub fn new() -> Self {
        Self::with_config(ExtractorConfig::default())
    }

    pub fn with_config(config: ExtractorConfig) -> Self {
        Self { config }
    }

    pub fn extract(&self, html: &str, schema: &Schema, url: &str) -> Result<ExtractionResult, ExtractorError> {
        let start = Instant::now();
        let mut parser = HtmlParser::new(html);
        let mut data: HashMap<String, Vec<String>> = HashMap::new();
        let mut selectors_used = Vec::new();
        let mut total_elements = 0;

        for field in &schema.fields {
            let mut values = self.extract_field(&mut parser, field)?;
            total_elements += values.len();
            selectors_used.push(field.selector.clone());
            data.insert(field.name.clone(), values);
        }

        let metadata = if self.config.include_metadata {
            Some(ExtractionMetadata {
                url: url.to_string(),
                selectors_used,
                elements_found: total_elements,
                extraction_time_ms: start.elapsed().as_millis() as u64,
                extraction_date: chrono_lite_date(),
            })
        } else {
            None
        };

        let confidence = self.calculate_confidence(&data, schema);

        Ok(ExtractionResult {
            data,
            metadata,
            confidence,
        })
    }

    fn extract_field(&self, parser: &mut HtmlParser, field: &SchemaField) -> Result<Vec<String>, ExtractorError> {
        match parser.select(&field.selector) {
            Ok(elements) if !elements.is_empty() => {
                let values: Vec<String> = if field.multiple {
                    elements.iter()
                        .map(|el| self.apply_transform(&self.get_value(parser, el, &field.attr), &field.transform))
                        .collect()
                } else {
                    elements.first()
                        .map(|el| self.apply_transform(&self.get_value(parser, el, &field.attr), &field.transform))
                        .into_iter()
                        .collect()
                };
                Ok(values)
            }
            _ if self.config.fallback_enabled && !field.fallback_selectors.is_empty() => {
                for fallback in &field.fallback_selectors {
                    if let Ok(elements) = parser.select(fallback) {
                        if !elements.is_empty() {
                            return Ok(elements.iter()
                                .map(|el| self.get_value(parser, el, &field.attr))
                                .collect());
                        }
                    }
                }
                Ok(vec![])
            }
            _ => Ok(vec![]),
        }
    }

    fn get_value(&self, parser: &HtmlParser, element: &scraper::ElementRef, attr: &Option<String>) -> String {
        match attr {
            Some(attr_name) => parser.attr(element, attr_name).unwrap_or_default(),
            None => parser.text(element),
        }
    }

    fn apply_transform(&self, value: &str, transform: &Option<TransformType>) -> String {
        let mut result = value.to_string();
        
        if let Some(t) = transform {
            result = match t {
                TransformType::Trim => result.trim().to_string(),
                TransformType::Lowercase => result.to_lowercase(),
                TransformType::Uppercase => result.to_uppercase(),
                TransformType::Strip => result.split_whitespace().collect::<Vec<_>>().join(" "),
                TransformType::Regex(pattern) => {
                    if let Ok(re) = regex_lite::Regex::new(pattern) {
                        re.find(&result).map(|m| m.as_str().to_string()).unwrap_or_default()
                    } else {
                        result
                    }
                }
            };
        }
        
        result
    }

    fn calculate_confidence(&self, data: &HashMap<String, Vec<String>>, schema: &Schema) -> f64 {
        let mut total = 0.0;
        let mut found = 0.0;

        for field in &schema.fields {
            total += 1.0;
            if let Some(values) = data.get(&field.name) {
                if !values.is_empty() && !values.iter().all(|v| v.is_empty()) {
                    found += 1.0;
                }
            }
        }

        if total > 0.0 { found / total } else { 0.0 }
    }

    pub fn extract_selectors(&self, html: &str, selectors: &[(String, String)]) -> HashMap<String, Vec<String>> {
        let mut parser = HtmlParser::new(html);
        let mut result = HashMap::new();

        for (name, selector) in selectors {
            if let Ok(elements) = parser.select(selector) {
                let values: Vec<String> = elements.iter()
                    .map(|el| parser.text(el))
                    .collect();
                result.insert(name.clone(), values);
            }
        }

        result
    }

    pub fn extract_products(&self, html: &str) -> Vec<HashMap<String, String>> {
        let mut parser = HtmlParser::new(html);
        let mut products = Vec::new();

        let container_selectors = [
            "[class*='product']",
            "[class*='item'][class*='card']",
            "[class*='merchandise']",
            "[data-product]",
            ".product-card",
            ".product-item",
            "[itemtype*='Product']",
        ];

        for selector in &container_selectors {
            if let Ok(elements) = parser.select(selector) {
                for el in elements {
                    let mut product = HashMap::new();

                    if let Ok(titles) = parser.select("h1, h2, h3, [class*='title'], [class*='name']") {
                        if let Some(t) = titles.first() {
                            product.insert("title".to_string(), parser.text(t));
                        }
                    }

                    if let Ok(prices) = parser.select("[class*='price'], .price, [itemprop='price']") {
                        if let Some(p) = prices.first() {
                            product.insert("price".to_string(), parser.text(p));
                        }
                    }

                    if let Ok(images) = parser.select("img[src], [class*='image'] img") {
                        if let Some(img) = images.first() {
                            if let Some(src) = parser.attr(img, "src") {
                                product.insert("image".to_string(), src);
                            }
                        }
                    }

                    if let Ok(links) = parser.select("a[href]") {
                        if let Some(link) = links.first() {
                            if let Some(href) = parser.attr(link, "href") {
                                product.insert("url".to_string(), href);
                            }
                        }
                    }

                    if !product.is_empty() {
                        products.push(product);
                    }
                }

                if !products.is_empty() {
                    break;
                }
            }
        }

        products
    }

    pub fn extract_articles(&self, html: &str) -> Vec<HashMap<String, String>> {
        let mut parser = HtmlParser::new(html);
        let mut articles = Vec::new();

        let article_selectors = [
            "article",
            "[class*='post']",
            "[class*='article']",
            "[class*='entry']",
            "[itemtype*='Article']",
            ".post-card",
            ".article-card",
        ];

        for selector in &article_selectors {
            if let Ok(elements) = parser.select(selector) {
                for el in elements {
                    let mut article = HashMap::new();

                    if let Ok(titles) = parser.select("h1, h2, [class*='title']") {
                        if let Some(t) = titles.first() {
                            article.insert("title".to_string(), parser.text(t));
                        }
                    }

                    if let Ok(contents) = parser.select("p") {
                        let text: String = contents.iter()
                            .map(|p| parser.text(p))
                            .take(5)
                            .collect::<Vec<_>>()
                            .join(" ");
                        article.insert("content".to_string(), text);
                    }

                    if let Ok(dates) = parser.select("[class*='date'], time, [class*='published'], [datetime]") {
                        if let Some(d) = dates.first() {
                            let date = parser.attr(&d, "datetime")
                                .unwrap_or_else(|| parser.text(d));
                            article.insert("date".to_string(), date);
                        }
                    }

                    if let Ok(authors) = parser.select("[class*='author'], [rel='author'], [itemprop='author']") {
                        if let Some(a) = authors.first() {
                            article.insert("author".to_string(), parser.text(a));
                        }
                    }

                    if !article.is_empty() {
                        articles.push(article);
                    }
                }

                if !articles.is_empty() {
                    break;
                }
            }
        }

        articles
    }

    pub fn extract_links_with_meta(&self, html: &str, base_url: &str) -> Vec<HashMap<String, String>> {
        let mut parser = HtmlParser::new(html);
        let mut links = Vec::new();

        if let Ok(anchor_elements) = parser.select("a[href]") {
            for el in anchor_elements {
                let mut link_info = HashMap::new();

                link_info.insert("url".to_string(), parser.attr(&el, "href").unwrap_or_default());
                link_info.insert("text".to_string(), parser.text(&el));

                if let Some(title) = parser.attr(&el, "title") {
                    link_info.insert("title".to_string(), title);
                }

                if let Some(rel) = parser.attr(&el, "rel") {
                    link_info.insert("rel".to_string(), rel);
                }

                if let Some(class) = parser.attr(&el, "class") {
                    link_info.insert("class".to_string(), class);
                }

                links.push(link_info);
            }
        }

        links
    }
}

impl Default for Extractor {
    fn default() -> Self {
        Self::new()
    }
}

impl Schema {
    pub fn new(fields: Vec<SchemaField>) -> Self {
        Self { fields }
    }

    pub fn simple(fields: Vec<(&str, &str)>) -> Self {
        Self {
            fields: fields.into_iter()
                .map(|(name, selector)| SchemaField {
                    name: name.to_string(),
                    selector: selector.to_string(),
                    attr: None,
                    multiple: false,
                    transform: None,
                    fallback_selectors: vec![],
                })
                .collect(),
        }
    }

    pub fn with_fallbacks(fields: Vec<(&str, &str, Vec<&str>)>) -> Self {
        Self {
            fields: fields.into_iter()
                .map(|(name, selector, fallbacks)| SchemaField {
                    name: name.to_string(),
                    selector: selector.to_string(),
                    attr: None,
                    multiple: false,
                    transform: None,
                    fallback_selectors: fallbacks.into_iter().map(|s| s.to_string()).collect(),
                })
                .collect(),
        }
    }
}

fn chrono_lite_date() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();
    format!("{}", secs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_extraction() {
        let html = r#"<html><body><h1 class="title">Test Title</h1><span class="price">$99</span></body></html>"#;
        let extractor = Extractor::new();
        let schema = Schema::simple(vec![("title", "h1.title"), ("price", "span.price")]);
        
        let result = extractor.extract(html, &schema, "https://example.com").unwrap();
        assert_eq!(result.data.get("title").unwrap().first().unwrap(), "Test Title");
    }

    #[test]
    fn test_product_extraction() {
        let html = r#"<html><body><div class="product"><h2>Product 1</h2><span class="price">$49.99</span></div></body></html>"#;
        let extractor = Extractor::new();
        let products = extractor.extract_products(html);
        assert!(!products.is_empty());
    }
}