/*
 * =============================================================================
 * Module: AI Agent Engine
 * Project: ScrapeFlux - Ultra-Advanced Web Scraping Framework
 * Author: Antor Roy
 * Email: antorroybabu@gmail.com
 * License: MIT
 * 
 * Description:
 *     Custom AI agent framework for autonomous web scraping tasks.
 *     This is ScrapeFlux's own implementation using advanced algorithms.
 *     
 *     Features:
 *     - Autonomous task planning
 *     - Multi-step workflow execution
 *     - Memory management (short-term + long-term)
 *     - Tool calling and reasoning
 *     - Self-correction and learning
 * =============================================================================
 */

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::advanced_data::{AdaptiveHashMap, CircuitBreaker, SiteGraph};

// ============================================================================
// AGENT CONFIGURATION
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub model: ModelConfig,
    pub max_steps: u32,
    pub timeout_secs: u64,
    pub memory_config: MemoryConfig,
    pub toolset: Vec<ToolDefinition>,
    pub reasoning_enabled: bool,
    pub self_correction_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub provider: String,
    pub model_name: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub api_endpoint: Option<String>,
    pub api_key: Option<String>,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            provider: "openai".to_string(),
            model_name: "gpt-4".to_string(),
            temperature: 0.7,
            max_tokens: 2048,
            api_endpoint: None,
            api_key: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    pub short_term_capacity: usize,
    pub long_term_capacity: usize,
    pub working_memory_capacity: usize,
    pub auto_archive: bool,
    pub archive_after_secs: u64,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            short_term_capacity: 100,
            long_term_capacity: 10000,
            working_memory_capacity: 50,
            auto_archive: true,
            archive_after_secs: 3600,
        }
    }
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            model: ModelConfig::default(),
            max_steps: 50,
            timeout_secs: 300,
            memory_config: MemoryConfig::default(),
            toolset: vec![
                ToolDefinition {
                    name: "navigate".to_string(),
                    description: "Navigate to a URL".to_string(),
                    parameters: vec![
                        ToolParameter { name: "url".to_string(), param_type: "string".to_string(), required: true, description: "URL to navigate to".to_string() },
                    ],
                },
                ToolDefinition {
                    name: "click".to_string(),
                    description: "Click an element".to_string(),
                    parameters: vec![
                        ToolParameter { name: "selector".to_string(), param_type: "string".to_string(), required: true, description: "CSS selector for element".to_string() },
                    ],
                },
                ToolDefinition {
                    name: "type".to_string(),
                    description: "Type text into an element".to_string(),
                    parameters: vec![
                        ToolParameter { name: "selector".to_string(), param_type: "string".to_string(), required: true, description: "CSS selector for element".to_string() },
                        ToolParameter { name: "text".to_string(), param_type: "string".to_string(), required: true, description: "Text to type".to_string() },
                    ],
                },
                ToolDefinition {
                    name: "extract".to_string(),
                    description: "Extract data from page".to_string(),
                    parameters: vec![
                        ToolParameter { name: "schema".to_string(), param_type: "object".to_string(), required: true, description: "Extraction schema".to_string() },
                    ],
                },
                ToolDefinition {
                    name: "screenshot".to_string(),
                    description: "Take a screenshot".to_string(),
                    parameters: vec![],
                },
                ToolDefinition {
                    name: "wait".to_string(),
                    description: "Wait for a condition".to_string(),
                    parameters: vec![
                        ToolParameter { name: "selector".to_string(), param_type: "string".to_string(), required: false, description: "Element to wait for".to_string() },
                        ToolParameter { name: "timeout_ms".to_string(), param_type: "number".to_string(), required: false, description: "Timeout in milliseconds".to_string() },
                    ],
                },
                ToolDefinition {
                    name: "scroll".to_string(),
                    description: "Scroll the page".to_string(),
                    parameters: vec![
                        ToolParameter { name: "direction".to_string(), param_type: "string".to_string(), required: false, description: "up or down".to_string() },
                        ToolParameter { name: "amount".to_string(), param_type: "number".to_string(), required: false, description: "Pixels to scroll".to_string() },
                    ],
                },
            ],
            reasoning_enabled: true,
            self_correction_enabled: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: Vec<ToolParameter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolParameter {
    pub name: String,
    pub param_type: String,
    pub required: bool,
    pub description: String,
}

// ============================================================================
// TASK DEFINITIONS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTask {
    pub id: String,
    pub description: String,
    pub goal: String,
    pub context: HashMap<String, String>,
    pub constraints: Vec<String>,
    pub success_criteria: Vec<String>,
}

impl AgentTask {
    pub fn new(description: &str, goal: &str) -> Self {
        Self {
            id: uuid_simple(),
            description: description.to_string(),
            goal: goal.to_string(),
            context: HashMap::new(),
            constraints: Vec::new(),
            success_criteria: Vec::new(),
        }
    }

    pub fn with_context(mut self, key: &str, value: &str) -> Self {
        self.context.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_constraint(mut self, constraint: &str) -> Self {
        self.constraints.push(constraint.to_string());
        self
    }

    pub fn with_success_criteria(mut self, criteria: &str) -> Self {
        self.success_criteria.push(criteria.to_string());
        self
    }
}

// ============================================================================
// MEMORY MANAGEMENT
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMemory {
    pub short_term: VecDeque<MemoryEntry>,
    pub long_term: VecDeque<MemoryEntry>,
    pub working_memory: HashMap<String, MemoryValue>,
    pub max_short_term: usize,
    pub max_long_term: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub id: String,
    pub content: String,
    pub content_type: MemoryType,
    pub timestamp: u64,
    pub importance: f32,
    pub source_step: u32,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryType {
    Observation,
    Action,
    Thought,
    Result,
    Error,
    Reflection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Object(serde_json::Value),
}

impl AgentMemory {
    pub fn new(config: &MemoryConfig) -> Self {
        Self {
            short_term: VecDeque::new(),
            long_term: VecDeque::new(),
            working_memory: HashMap::new(),
            max_short_term: config.short_term_capacity,
            max_long_term: config.long_term_capacity,
        }
    }

    pub fn add_to_short_term(&mut self, entry: MemoryEntry) {
        // Calculate importance and archive if needed
        if self.short_term.len() >= self.max_short_term {
            self.archive_oldest();
        }
        
        self.short_term.push_back(entry);
    }

    pub fn add_to_working(&mut self, key: &str, value: MemoryValue) {
        if self.working_memory.len() >= self.max_short_term {
            // Remove least important item
            if let Some(oldest) = self.working_memory.keys().next().cloned() {
                self.working_memory.remove(&oldest);
            }
        }
        
        self.working_memory.insert(key.to_string(), value);
    }

    pub fn get_from_working(&self, key: &str) -> Option<&MemoryValue> {
        self.working_memory.get(key)
    }

    pub fn archive_oldest(&mut self) {
        if let Some(entry) = self.short_term.pop_front() {
            if self.long_term.len() < self.max_long_term {
                self.long_term.push_back(entry);
            }
        }
    }

    pub fn search(&self, query: &str) -> Vec<&MemoryEntry> {
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();
        
        // Search short term
        for entry in &self.short_term {
            if entry.content.to_lowercase().contains(&query_lower) {
                results.push(entry);
            }
        }
        
        // Search long term
        for entry in &self.long_term {
            if entry.content.to_lowercase().contains(&query_lower) {
                results.push(entry);
            }
        }
        
        results
    }

    pub fn get_recent(&self, count: usize) -> Vec<MemoryEntry> {
        self.short_term.iter()
            .rev()
            .take(count)
            .cloned()
            .collect()
    }

    pub fn clear_short_term(&mut self) {
        self.short_term.clear();
    }

    pub fn clear_all(&mut self) {
        self.short_term.clear();
        self.long_term.clear();
        self.working_memory.clear();
    }
}

// ============================================================================
// REASONING ENGINE
// ============================================================================

#[derive(Debug, Clone)]
pub struct ReasoningEngine {
    config: AgentConfig,
    reasoning_history: Vec<ReasoningStep>,
}

#[derive(Debug, Clone)]
pub struct ReasoningStep {
    pub step_number: u32,
    pub thought: String,
    pub action: Option<String>,
    pub observation: Option<String>,
    pub confidence: f32,
    pub correction: Option<String>,
}

impl ReasoningEngine {
    pub fn new(config: AgentConfig) -> Self {
        Self {
            config,
            reasoning_history: Vec::new(),
        }
    }

    pub fn think(&mut self, context: &str, task: &AgentTask) -> String {
        let mut thought = format!(
            "Analyzing task: {}\nGoal: {}\nContext: {}\n\n",
            task.description,
            task.goal,
            context
        );

        // Add constraint considerations
        if !task.constraints.is_empty() {
            thought.push_str("Constraints to consider:\n");
            for constraint in &task.constraints {
                thought.push_str(&format!("- {}\n", constraint));
            }
        }

        // Add success criteria
        if !task.success_criteria.is_empty() {
            thought.push_str("Success criteria:\n");
            for criteria in &task.success_criteria {
                thought.push_str(&format!("- {}\n", criteria));
            }
        }

        // Consider available tools
        thought.push_str("\nAvailable tools:\n");
        for tool in &self.config.toolset {
            thought.push_str(&format!("- {}: {}\n", tool.name, tool.description));
        }

        // Generate reasoning steps
        self.reasoning_history.push(ReasoningStep {
            step_number: self.reasoning_history.len() as u32 + 1,
            thought: thought.clone(),
            action: None,
            observation: None,
            confidence: 0.8,
            correction: None,
        });

        thought
    }

    pub fn observe(&mut self, observation: &str) {
        if let Some(last) = self.reasoning_history.last_mut() {
            last.observation = Some(observation.to_string());
        }
    }

    pub fn correct(&mut self, correction: &str) {
        if let Some(last) = self.reasoning_history.last_mut() {
            last.correction = Some(correction.to_string());
            last.confidence = (last.confidence * 0.5).max(0.3); // Reduce confidence on correction
        }
    }

    pub fn get_history(&self) -> &[ReasoningStep] {
        &self.reasoning_history
    }

    pub fn get_recent_reasoning(&self, count: usize) -> Vec<String> {
        self.reasoning_history.iter()
            .rev()
            .take(count)
            .map(|s| s.thought.clone())
            .collect()
    }
}

// ============================================================================
// AGENT EXECUTION
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStep {
    pub step_number: u32,
    pub thought: String,
    pub action: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub observation: Option<String>,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResult {
    pub task_id: String,
    pub task_completed: bool,
    pub steps: Vec<AgentStep>,
    pub final_result: Option<serde_json::Value>,
    pub memory_snapshot: MemorySnapshot,
    pub total_steps: u32,
    pub total_time_ms: u64,
    pub success_rate: f32,
    pub error_summary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySnapshot {
    pub short_term_count: usize,
    pub long_term_count: usize,
    pub working_memory_keys: Vec<String>,
}

pub struct AIAgent {
    config: AgentConfig,
    memory: AgentMemory,
    reasoning: ReasoningEngine,
    circuit_breaker: CircuitBreaker,
    execution_history: VecDeque<AgentStep>,
}

impl AIAgent {
    pub fn new(config: AgentConfig) -> Self {
        Self {
            circuit_breaker: CircuitBreaker::new(10, 60),
            memory: AgentMemory::new(&config.memory_config),
            reasoning: ReasoningEngine::new(config.clone()),
            config,
            execution_history: VecDeque::new(),
        }
    }

    pub async fn execute(&mut self, task: AgentTask) -> Result<AgentResult, String> {
        let start_time = std::time::Instant::now();
        
        // Store task context in working memory
        for (key, value) in &task.context {
            self.memory.add_to_working(key, MemoryValue::String(value.clone()));
        }

        // Generate initial reasoning
        let context = self.generate_context_summary();
        let initial_thought = self.reasoning.think(&context, &task);

        let mut steps = Vec::new();
        let mut step_number = 0u32;
        let mut task_completed = false;
        let mut error_summary = None;

        // Main execution loop
        while step_number < self.config.max_steps {
            step_number += 1;

            // Check circuit breaker
            {
                let result = self.circuit_breaker.call::<_, ()>(|| Ok(()));
                if result.is_err() {
                    error_summary = Some("Circuit breaker triggered".to_string());
                    break;
                }
            }

            // Generate next action based on reasoning
            let action = self.decide_next_action(&task, step_number);
            
            let step = AgentStep {
                step_number,
                thought: format!("Step {}: {}", step_number, initial_thought),
                action: action.name.clone(),
                parameters: action.parameters,
                observation: None,
                success: false,
                error: None,
            };

            // Execute action (simulated)
            let exec_result = self.execute_action(&action).await;
            
            let mut final_step = step;
            final_step.observation = Some(exec_result.observation.clone());
            final_step.success = exec_result.success;
            final_step.error = exec_result.error.clone();

            // Record observation
            self.reasoning.observe(&exec_result.observation);
            
            // Add to memory
            self.memory.add_to_short_term(MemoryEntry {
                id: uuid_simple(),
                content: exec_result.observation.clone(),
                content_type: if exec_result.success { MemoryType::Result } else { MemoryType::Error },
                timestamp: now_secs(),
                importance: 0.8,
                source_step: step_number,
                tags: vec![action.name.clone()],
            });

            // Self-correction
            if !exec_result.success && self.config.self_correction_enabled {
                let correction = self.generate_correction(&exec_result.error);
                self.reasoning.correct(&correction);
                final_step.thought = format!("{}\n\nCorrection: {}", final_step.thought, correction);
            }

            steps.push(final_step);
            
            // Check if task is complete
            if self.check_success(&task, &exec_result.observation) {
                task_completed = true;
                break;
            }
        }

        // Calculate success rate
        let success_count = steps.iter().filter(|s| s.success).count();
        let success_rate = if steps.is_empty() {
            0.0
        } else {
            success_count as f32 / steps.len() as f32
        };

        // Generate final result
        let final_result = self.generate_final_result(&steps);

        Ok(AgentResult {
            task_id: task.id,
            task_completed,
            steps,
            final_result,
            memory_snapshot: MemorySnapshot {
                short_term_count: self.memory.short_term.len(),
                long_term_count: self.memory.long_term.len(),
                working_memory_keys: self.memory.working_memory.keys().cloned().collect(),
            },
            total_steps: step_number,
            total_time_ms: start_time.elapsed().as_millis() as u64,
            success_rate,
            error_summary,
        })
    }

    fn generate_context_summary(&self) -> String {
        let mut context = String::new();
        
        // Add recent observations
        let recent = self.memory.get_recent(5);
        if !recent.is_empty() {
            context.push_str("Recent observations:\n");
            for entry in &recent {
                context.push_str(&format!("- [{}] {}\n", entry.content_type as u8, entry.content));
            }
        }

        // Add working memory
        if !self.memory.working_memory.is_empty() {
            context.push_str("\nWorking memory:\n");
            for (key, value) in &self.memory.working_memory {
                context.push_str(&format!("- {}: {:?}\n", key, value));
            }
        }

        context
    }

    fn decide_next_action(&self, task: &AgentTask, _step: u32) -> ToolCall {
        // Simple decision logic - in production, this would call an LLM
        // For now, we'll use rule-based logic
        
        if task.goal.contains("extract") || task.goal.contains("scrape") {
            ToolCall {
                name: "extract".to_string(),
                parameters: HashMap::new(),
            }
        } else if task.goal.contains("click") || task.goal.contains("navigate") {
            // Check if URL is in context
            if let Some(MemoryValue::String(url)) = self.memory.get_from_working("url") {
                ToolCall {
                    name: "navigate".to_string(),
                    parameters: {
                        let mut p = HashMap::new();
                        p.insert("url".to_string(), serde_json::json!(url));
                        p
                    },
                }
            } else {
                ToolCall {
                    name: "screenshot".to_string(),
                    parameters: HashMap::new(),
                }
            }
        } else {
            ToolCall {
                name: "screenshot".to_string(),
                parameters: HashMap::new(),
            }
        }
    }

    async fn execute_action(&self, action: &ToolCall) -> ActionResult {
        // Simulated action execution
        // In production, this would actually perform the actions
        
        match action.name.as_str() {
            "navigate" => {
                if let Some(url) = action.parameters.get("url") {
                    ActionResult {
                        success: true,
                        observation: format!("Navigated to {}", url),
                        error: None,
                    }
                } else {
                    ActionResult {
                        success: false,
                        observation: String::new(),
                        error: Some("URL parameter missing".to_string()),
                    }
                }
            }
            "extract" => {
                ActionResult {
                    success: true,
                    observation: "Extracted data from page".to_string(),
                    error: None,
                }
            }
            "click" => {
                ActionResult {
                    success: true,
                    observation: "Clicked element".to_string(),
                    error: None,
                }
            }
            _ => {
                ActionResult {
                    success: true,
                    observation: format!("Executed action: {}", action.name),
                    error: None,
                }
            }
        }
    }

    fn check_success(&self, task: &AgentTask, observation: &str) -> bool {
        // Check if any success criteria is met
        for criteria in &task.success_criteria {
            if observation.contains(criteria) {
                return true;
            }
        }
        
        // Default: check if we have extracted data
        task.goal.contains("extract") && observation.contains("Extracted")
    }

    fn generate_correction(&self, error: &Option<String>) -> String {
        let error_msg = error.as_deref().unwrap_or("Unknown error");
        format!(
            "An error occurred: {}. Attempting to correct by using alternative approach.",
            error_msg
        )
    }

    fn generate_final_result(&self, steps: &[AgentStep]) -> Option<serde_json::Value> {
        let results: Vec<serde_json::Value> = steps
            .iter()
            .filter(|s| s.success && s.observation.is_some())
            .map(|s| {
                serde_json::json!({
                    "step": s.step_number,
                    "action": s.action,
                    "observation": s.observation
                })
            })
            .collect();

        if results.is_empty() {
            None
        } else {
            Some(serde_json::json!({
                "total_steps": steps.len(),
                "successful_steps": steps.iter().filter(|s| s.success).count(),
                "results": results
            }))
        }
    }

    pub fn get_memory(&self) -> &AgentMemory {
        &self.memory
    }

    pub fn get_reasoning(&self) -> &ReasoningEngine {
        &self.reasoning
    }
}

#[derive(Debug, Clone)]
pub struct ToolCall {
    pub name: String,
    pub parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct ActionResult {
    pub success: bool,
    pub observation: String,
    pub error: Option<String>,
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:x}", nanos)
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_task() {
        let task = AgentTask::new(
            "Scrape product information from e-commerce site",
            "Extract product names, prices, and descriptions"
        )
        .with_constraint("Do not click on ads")
        .with_context("url", "https://example.com/products")
        .with_success_criteria("Extracted 10+ products");

        assert!(task.description.contains("Scrape"));
        assert_eq!(task.context.get("url"), Some(&"https://example.com/products".to_string()));
    }

    #[test]
    fn test_agent_memory() {
        let config = MemoryConfig::default();
        let mut memory = AgentMemory::new(&config);

        memory.add_to_working("test", MemoryValue::String("value".to_string()));
        assert_eq!(memory.get_from_working("test"), Some(&MemoryValue::String("value".to_string())));

        memory.add_to_short_term(MemoryEntry {
            id: "1".to_string(),
            content: "Test observation".to_string(),
            content_type: MemoryType::Observation,
            timestamp: now_secs(),
            importance: 0.8,
            source_step: 1,
            tags: vec![],
        });

        assert_eq!(memory.short_term.len(), 1);
    }

    #[test]
    fn test_reasoning_engine() {
        let config = AgentConfig::default();
        let mut reasoning = ReasoningEngine::new(config);

        let task = AgentTask::new("Test task", "Test goal");
        let thought = reasoning.think("context", &task);

        assert!(thought.contains("Test task"));
        assert!(thought.contains("Test goal"));
    }
}