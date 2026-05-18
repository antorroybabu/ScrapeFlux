/*
 * =============================================================================
 * Module: LLM-Powered Extraction Engine
 * Project: ScrapeFlux - Ultra-Advanced Web Scraping Framework
 * Author: Antor Roy
 * Email: antorroybabu@gmail.com
 * License: MIT
 * 
 * Description:
 *     Custom LLM-powered extraction inspired by modern AI patterns.
 *     This is ScrapeFlux's own implementation using advanced algorithms.
 *     
 *     Features:
 *     - Natural language schema generation
 *     - Prompt engineering with few-shot learning
 *     - Structured output parsing
 *     - Schema validation
 *     - Multi-format export (JSON, XML, CSV)
 * =============================================================================
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::advanced_data::{Trie, AdaptiveHashMap};

// ============================================================================
// SCHEMA DEFINITIONS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    pub model: String,
    pub api_endpoint: Option<String>,
    pub api_key: Option<String>,
    pub temperature: f32,
    pub max_tokens: u32,
    pub timeout_secs: u64,
}

impl Default for LLMConfig {
    fn default() -> Self {
        Self {
            model: "gpt-4".to_string(),
            api_endpoint: None,
            api_key: None,
            temperature: 0.7,
            max_tokens: 4096,
            timeout_secs: 30,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionSchema {
    pub name: String,
    pub description: String,
    pub fields: Vec<SchemaField>,
    pub examples: Vec<Example>,
    pub constraints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaField {
    pub name: String,
    pub field_type: FieldType,
    pub description: String,
    pub selector_hint: Option<String>,
    pub examples: Vec<String>,
    pub is_required: bool,
    pub is_array: bool,
    pub constraints: FieldConstraints,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldType {
    Text,
    Number,
    Boolean,
    Url,
    Email,
    Phone,
    Date,
    Price,
    Image,
    Array,
    Object,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldConstraints {
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub pattern: Option<String>,
    pub enum_values: Option<Vec<String>>,
}

impl Default for FieldConstraints {
    fn default() -> Self {
        Self {
            min_length: None,
            max_length: None,
            pattern: None,
            enum_values: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Example {
    pub input: String,
    pub output: serde_json::Value,
}

// ============================================================================
// PROMPT ENGINEERING
// ============================================================================

#[derive(Debug, Clone)]
pub struct PromptBuilder {
    schema: ExtractionSchema,
    context: HashMap<String, String>,
    few_shot_examples: Vec<(String, String)>,
}

impl PromptBuilder {
    pub fn new(schema: ExtractionSchema) -> Self {
        Self {
            schema,
            context: HashMap::new(),
            few_shot_examples: Vec::new(),
        }
    }

    pub fn with_context(mut self, key: &str, value: &str) -> Self {
        self.context.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_example(mut self, input: &str, output: &str) -> Self {
        self.few_shot_examples.push((input.to_string(), output.to_string()));
        self
    }

    pub fn build(&self) -> String {
        let mut prompt = String::new();
        
        // System prompt
        prompt.push_str("You are a data extraction expert for a web scraping framework called ScrapeFlux.\n\n");
        
        // Task description
        prompt.push_str(&format!("Task: {}\n\n", self.schema.description));
        
        // Fields specification
        prompt.push_str("Extract the following fields:\n");
        for field in &self.schema.fields {
            prompt.push_str(&format!("- {}: {}\n", field.name, field.description));
            if let Some(selector) = &field.selector_hint {
                prompt.push_str(&format!("  Hint: Look for CSS selector '{}'\n", selector));
            }
        }
        
        // Examples
        if !self.few_shot_examples.is_empty() {
            prompt.push_str("\nExamples:\n");
            for (input, output) in &self.few_shot_examples {
                prompt.push_str(&format!("Input:\n{}\n\nOutput:\n{}\n\n", input, output));
            }
        }
        
        // Context
        if !self.context.is_empty() {
            prompt.push_str("\nContext:\n");
            for (key, value) in &self.context {
                prompt.push_str(&format!("- {}: {}\n", key, value));
            }
        }
        
        // Output format
        prompt.push_str("\nOutput format: JSON object with the extracted fields.\n");
        prompt.push_str("Return ONLY the JSON, no additional text.\n");
        
        prompt
    }

    pub fn build_json_schema_prompt(&self) -> String {
        let mut prompt = String::new();
        
        prompt.push_str("Generate a JSON schema for web scraping based on the following description:\n\n");
        prompt.push_str(&format!("Description: {}\n\n", self.schema.description));
        
        prompt.push_str("The schema should include:\n");
        for field in &self.schema.fields {
            prompt.push_str(&format!("- {} ({:?}): {}\n", 
                field.name, field.field_type, field.description));
        }
        
        prompt.push_str("\nReturn a JSON schema in the following format:\n");
        prompt.push_str(r#"{
  "type": "object",
  "properties": {
    "field_name": {
      "type": "string",
      "description": "..."
    }
  }
}"#);
        
        prompt
    }
}

// ============================================================================
// RESPONSE PARSING
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedResponse {
    pub data: serde_json::Value,
    pub confidence: f32,
    pub parsing_errors: Vec<String>,
    pub validation_results: Vec<ValidationResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub field: String,
    pub valid: bool,
    pub message: String,
}

pub struct ResponseParser {
    schema: ExtractionSchema,
    strict_mode: bool,
}

impl ResponseParser {
    pub fn new(schema: ExtractionSchema) -> Self {
        Self {
            schema,
            strict_mode: false,
        }
    }

    pub fn with_strict_mode(mut self, strict: bool) -> Self {
        self.strict_mode = strict;
        self
    }

    pub fn parse(&self, raw_response: &str) -> Result<ParsedResponse, String> {
        let mut errors = Vec::new();
        
        // Try to parse JSON
        let data = match serde_json::from_str::<serde_json::Value>(raw_response) {
            Ok(v) => v,
            Err(e) => {
                // Try to extract JSON from markdown code blocks
                if let Some(json_start) = raw_response.find("```json") {
                    let start = json_start + 7;
                    let remaining = &raw_response[start..];
                    if let Some(json_end) = remaining.find("```") {
                        let json_str = &remaining[..json_end].trim();
                        match serde_json::from_str::<serde_json::Value>(json_str) {
                            Ok(v) => v,
                            Err(e2) => {
                                errors.push(format!("JSON parse error: {}", e2));
                                return Err(errors.join("; "));
                            }
                        }
                    } else {
                        errors.push(format!("No closing code block: {}", e));
                        return Err(errors.join("; "));
                    }
                } else if let Some(json_start) = raw_response.find("```") {
                    let start = json_start + 3;
                    let remaining = &raw_response[start..];
                    if let Some(json_end) = remaining.find("```") {
                        let json_str = &remaining[..json_end].trim();
                        match serde_json::from_str::<serde_json::Value>(json_str) {
                            Ok(v) => v,
                            Err(e2) => {
                                errors.push(format!("JSON parse error: {}", e2));
                                return Err(errors.join("; "));
                            }
                        }
                    } else {
                        errors.push(format!("Parse error: {}", e));
                        return Err(errors.join("; "));
                    }
                } else {
                    errors.push(format!("JSON parse error: {}", e));
                    return Err(errors.join("; "));
                }
            }
        };
        
        // Validate against schema
        let validation_results = self.validate(&data);
        
        // Calculate confidence
        let valid_count = validation_results.iter().filter(|r| r.valid).count();
        let confidence = if self.schema.fields.is_empty() {
            0.5
        } else {
            valid_count as f32 / self.schema.fields.len() as f32
        };
        
        Ok(ParsedResponse {
            data,
            confidence,
            parsing_errors: errors,
            validation_results,
        })
    }

    fn validate(&self, data: &serde_json::Value) -> Vec<ValidationResult> {
        let mut results = Vec::new();
        
        if let Some(obj) = data.as_object() {
            for field in &self.schema.fields {
                let value = obj.get(&field.name);
                
                let valid = match (value, field.is_required) {
                    (None, false) => true,
                    (None, true) => false,
                    (Some(v), _) => self.validate_field(&field, v),
                };
                
                results.push(ValidationResult {
                    field: field.name.clone(),
                    valid,
                    message: if valid {
                        "Valid".to_string()
                    } else {
                        format!("Field '{}' validation failed", field.name)
                    },
                });
            }
        }
        
        results
    }

    fn validate_field(&self, field: &SchemaField, value: &serde_json::Value) -> bool {
        match &field.field_type {
            FieldType::Text | FieldType::Url | FieldType::Email => {
                if let Some(s) = value.as_str() {
                    if let Some(min) = field.constraints.min_length {
                        if s.len() < min { return false; }
                    }
                    if let Some(max) = field.constraints.max_length {
                        if s.len() > max { return false; }
                    }
                    true
                } else {
                    false
                }
            }
            FieldType::Number | FieldType::Price => {
                value.is_number()
            }
            FieldType::Boolean => {
                value.is_boolean()
            }
            FieldType::Array => {
                value.is_array()
            }
            _ => true,
        }
    }
}

// ============================================================================
// STRUCTURED EXTRACTION
// ============================================================================

#[derive(Debug, Clone)]
pub struct StructuredExtractor {
    schema: ExtractionSchema,
    parser: ResponseParser,
    prompt_builder: PromptBuilder,
    cache: AdaptiveHashMap<String, ParsedResponse>,
}

impl StructuredExtractor {
    pub fn new(schema: ExtractionSchema) -> Self {
        let parser = ResponseParser::new(schema.clone());
        let prompt_builder = PromptBuilder::new(schema.clone());
        let mut cache = AdaptiveHashMap::new();
        cache.threshold = 100; // Auto-split after 100 entries
        
        Self {
            schema,
            parser,
            prompt_builder,
            cache,
        }
    }

    pub fn extract(&mut self, html: &str, raw_llm_response: &str) -> Result<ParsedResponse, String> {
        // Check cache first
        let cache_key = format!("{}:{}", 
            html.len(), 
            if raw_llm_response.len() > 100 { &raw_llm_response[..100] } else { raw_llm_response }
        );
        
        if let Some(cached) = self.cache.get(&cache_key) {
            return Ok(cached.clone());
        }
        
        // Parse response
        let result = self.parser.parse(raw_llm_response)?;
        
        // Cache result
        self.cache.insert(cache_key, result.clone());
        
        Ok(result)
    }

    pub fn get_prompt(&self) -> String {
        self.prompt_builder.build()
    }

    pub fn generate_schema_prompt(&self) -> String {
        self.prompt_builder.build_json_schema_prompt()
    }
}

// ============================================================================
// MULTI-FORMAT EXPORT
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExportFormat {
    Json,
    Xml,
    Csv,
    Markdown,
    Html,
}

pub struct MultiFormatExporter;

impl MultiFormatExporter {
    pub fn to_json(data: &serde_json::Value, pretty: bool) -> String {
        if pretty {
            serde_json::to_string_pretty(data).unwrap_or_default()
        } else {
            serde_json::to_string(data).unwrap_or_default()
        }
    }

    pub fn to_xml(data: &serde_json::Value, root_element: &str) -> String {
        let mut xml = String::new();
        xml.push_str(&format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n"));
        Self::json_to_xml_internal(&mut xml, data, root_element, 0);
        xml
    }

    fn json_to_xml_internal(xml: &mut String, value: &serde_json::Value, tag: &str, indent: usize) {
        let spaces = "  ".repeat(indent);
        
        match value {
            serde_json::Value::Object(map) => {
                xml.push_str(&format!("{}<{}>\n", spaces, tag));
                for (key, val) in map {
                    Self::json_to_xml_internal(xml, val, key, indent + 1);
                }
                xml.push_str(&format!("{}</{}>\n", spaces, tag));
            }
            serde_json::Value::Array(arr) => {
                for item in arr {
                    Self::json_to_xml_internal(xml, item, tag, indent);
                }
            }
            serde_json::Value::String(s) => {
                let escaped = s.replace('&', "&")
                    .replace('<', "<")
                    .replace('>', ">")
                    .replace('"', """);
                xml.push_str(&format!("{}<{}><![CDATA[{}]]></{}>\n", spaces, tag, escaped, tag));
            }
            serde_json::Value::Number(n) => {
                xml.push_str(&format!("{}<{}>{}</{}>\n", spaces, tag, n, tag));
            }
            serde_json::Value::Bool(b) => {
                xml.push_str(&format!("{}<{}>{}</{}>\n", spaces, tag, b, tag));
            }
            _ => {
                xml.push_str(&format!("{}<{}></{}>\n", spaces, tag, tag));
            }
        }
    }

    pub fn to_csv(data: &serde_json::Value) -> String {
        let mut csv = String::new();
        
        if let Some(arr) = data.as_array() {
            if let Some(first) = arr.first() {
                if let Some(obj) = first.as_object() {
                    // Header
                    let headers: Vec<&str> = obj.keys().map(|s| s.as_str()).collect();
                    csv.push_str(&headers.join(","));
                    csv.push('\n');
                    
                    // Rows
                    for item in arr {
                        if let Some(row_obj) = item.as_object() {
                            let values: Vec<String> = headers.iter()
                                .map(|h| {
                                    row_obj.get(*h)
                                        .map(|v| match v {
                                            serde_json::Value::String(s) => format!("\"{}\"", s.replace('"', "\"\"")),
                                            _ => v.to_string(),
                                        })
                                        .unwrap_or_default()
                                })
                                .collect();
                            csv.push_str(&values.join(","));
                            csv.push('\n');
                        }
                    }
                }
            }
        } else if let Some(obj) = data.as_object() {
            // Single object - one row
            let headers: Vec<&str> = obj.keys().map(|s| s.as_str()).collect();
            csv.push_str(&headers.join(","));
            csv.push('\n');
            
            let values: Vec<String> = headers.iter()
                .map(|h| {
                    obj.get(*h)
                        .map(|v| match v {
                            serde_json::Value::String(s) => format!("\"{}\"", s.replace('"', "\"\"")),
                            _ => v.to_string(),
                        })
                        .unwrap_or_default()
                })
                .collect();
            csv.push_str(&values.join(","));
        }
        
        csv
    }

    pub fn to_markdown(data: &serde_json::Value) -> String {
        let mut md = String::new();
        
        match data {
            serde_json::Value::Object(map) => {
                for (key, value) in map {
                    match value {
                        serde_json::Value::Object(inner) => {
                            md.push_str(&format!("### {}\n\n"));
                            md.push_str(&format!("| Key | Value |\n"));
                            md.push_str("|------|--------|\n");
                            for (k, v) in inner {
                                md.push_str(&format!("| {} | {} |\n", k, v));
                            }
                            md.push('\n');
                        }
                        serde_json::Value::Array(arr) => {
                            md.push_str(&format!("### {}\n\n", key));
                            if let Some(first) = arr.first() {
                                if let Some(obj) = first.as_object() {
                                    md.push_str(&format!("| {} |\n", obj.keys().map(|s| s.as_str()).collect::<Vec<_>>().join(" | ")));
                                    md.push_str(&format!("| {} |\n", obj.values().map(|v| v.to_string()).collect::<Vec<_>>().join(" | ")));
                                }
                            }
                            md.push('\n');
                        }
                        _ => {
                            md.push_str(&format!("**{}**: {}\n\n", key, value));
                        }
                    }
                }
            }
            serde_json::Value::Array(arr) => {
                if let Some(first) = arr.first() {
                    if let Some(obj) = first.as_object() {
                        md.push_str(&format!("| {} |\n", obj.keys().map(|s| s.as_str()).collect::<Vec<_>>().join(" | ")));
                        md.push_str(&format!("| {} |\n", obj.values().map(|v| v.to_string()).collect::<Vec<_>>().join(" | ")));
                        
                        for item in arr.iter().skip(1) {
                            if let Some(row) = item.as_object() {
                                md.push_str(&format!("| {} |\n", row.values().map(|v| v.to_string()).collect::<Vec<_>>().join(" | ")));
                            }
                        }
                    }
                }
            }
            _ => {
                md.push_str(&data.to_string());
            }
        }
        
        md
    }

    pub fn to_html(data: &serde_json::Value) -> String {
        let mut html = String::new();
        
        html.push_str(r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>ScrapeFlux Extraction Results</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        table { border-collapse: collapse; width: 100%; margin: 20px 0; }
        th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
        th { background-color: #4CAF50; color: white; }
        tr:nth-child(even) { background-color: #f2f2f2; }
    </style>
</head>
<body>
    <h1>ScrapeFlux Extraction Results</h1>
"#);
        
        match data {
            serde_json::Value::Array(arr) => {
                if let Some(first) = arr.first() {
                    if let Some(obj) = first.as_object() {
                        html.push_str("<table>\n<thead>\n<tr>\n");
                        for key in obj.keys() {
                            html.push_str(&format!("<th>{}</th>\n", key));
                        }
                        html.push_str("</tr>\n</thead>\n<tbody>\n");
                        
                        for item in arr {
                            if let Some(row) = item.as_object() {
                                html.push_str("<tr>\n");
                                for value in row.values() {
                                    html.push_str(&format!("<td>{}</td>\n", value));
                                }
                                html.push_str("</tr>\n");
                            }
                        }
                        
                        html.push_str("</tbody>\n</table>\n");
                    }
                }
            }
            serde_json::Value::Object(map) => {
                html.push_str("<table>\n<tbody>\n");
                for (key, value) in map {
                    html.push_str(&format!("<tr><th>{}</th><td>{}</td></tr>\n", key, value));
                }
                html.push_str("</tbody>\n</table>\n");
            }
            _ => {
                html.push_str(&format!("<p>{}</p>", data));
            }
        }
        
        html.push_str("</body>\n</html>");
        html
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_builder() {
        let schema = ExtractionSchema {
            name: "Products".to_string(),
            description: "Extract product information from e-commerce pages".to_string(),
            fields: vec![
                SchemaField {
                    name: "title".to_string(),
                    field_type: FieldType::Text,
                    description: "Product title".to_string(),
                    selector_hint: Some(".product-title".to_string()),
                    examples: vec![],
                    is_required: true,
                    is_array: false,
                    constraints: FieldConstraints::default(),
                },
                SchemaField {
                    name: "price".to_string(),
                    field_type: FieldType::Price,
                    description: "Product price".to_string(),
                    selector_hint: Some(".price".to_string()),
                    examples: vec![],
                    is_required: true,
                    is_array: false,
                    constraints: FieldConstraints::default(),
                },
            ],
            examples: vec![],
            constraints: vec![],
        };
        
        let builder = PromptBuilder::new(schema);
        let prompt = builder.build();
        
        assert!(prompt.contains("Products"));
        assert!(prompt.contains("title"));
        assert!(prompt.contains("price"));
    }

    #[test]
    fn test_response_parser() {
        let schema = ExtractionSchema {
            name: "Test".to_string(),
            description: "Test schema".to_string(),
            fields: vec![
                SchemaField {
                    name: "name".to_string(),
                    field_type: FieldType::Text,
                    description: "Name field".to_string(),
                    selector_hint: None,
                    examples: vec![],
                    is_required: true,
                    is_array: false,
                    constraints: FieldConstraints::default(),
                },
            ],
            examples: vec![],
            constraints: vec![],
        };
        
        let parser = ResponseParser::new(schema);
        let result = parser.parse(r#"{"name": "John"}"#).unwrap();
        
        assert_eq!(result.confidence, 1.0);
        assert!(result.validation_results[0].valid);
    }

    #[test]
    fn test_multi_format_export() {
        let data = serde_json::json!({
            "name": "John",
            "age": 30,
            "email": "john@example.com"
        });
        
        let json = MultiFormatExporter::to_json(&data, true);
        assert!(json.contains("John"));
        
        let xml = MultiFormatExporter::to_xml(&data, "result");
        assert!(xml.contains("<result>"));
        
        let csv = MultiFormatExporter::to_csv(&data);
        assert!(csv.contains("name"));
    }
}