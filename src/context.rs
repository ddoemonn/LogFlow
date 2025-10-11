use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogContext {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub level: u32,
    pub module: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub target: String,
    pub subtitle: Option<String>,
    pub fields: HashMap<String, serde_json::Value>,
    pub parent_id: Option<String>,
}

impl LogContext {
    pub fn new(target: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            level: 0,
            module: None,
            file: None,
            line: None,
            target,
            subtitle: None,
            fields: HashMap::new(),
            parent_id: None,
        }
    }

    pub fn with_level(mut self, level: u32) -> Self {
        self.level = level;
        self
    }

    pub fn with_module(mut self, module: &str) -> Self {
        self.module = Some(module.to_string());
        self
    }

    pub fn with_file_line(mut self, file: &str, line: u32) -> Self {
        self.file = Some(file.to_string());
        self.line = Some(line);
        self
    }

    pub fn with_field<T>(mut self, key: &str, value: T) -> Self
    where
        T: Serialize,
    {
        if let Ok(json_value) = serde_json::to_value(value) {
            self.fields.insert(key.to_string(), json_value);
        }
        self
    }

    pub fn with_subtitle(mut self, subtitle: &str) -> Self {
        self.subtitle = Some(subtitle.to_string());
        self
    }

    pub fn with_parent(mut self, parent_id: String) -> Self {
        self.parent_id = Some(parent_id);
        self.level = self.parent_id.as_ref().map_or(0, |_| self.level + 1);
        self
    }

    pub fn child(&self, target: String) -> Self {
        LogContext::new(target)
            .with_level(self.level + 1)
            .with_parent(self.id.clone())
    }

    pub fn get_field(&self, key: &str) -> Option<&serde_json::Value> {
        self.fields.get(key)
    }

    pub fn is_nested(&self) -> bool {
        self.parent_id.is_some()
    }

    pub fn nesting_level(&self) -> u32 {
        self.level
    }
}

#[derive(Debug, Clone)]
pub struct ContextStack {
    contexts: Arc<std::sync::Mutex<Vec<LogContext>>>,
}

impl ContextStack {
    pub fn new() -> Self {
        Self {
            contexts: Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }

    pub fn push(&self, context: LogContext) {
        if let Ok(mut contexts) = self.contexts.lock() {
            contexts.push(context);
        }
    }

    pub fn pop(&self) -> Option<LogContext> {
        if let Ok(mut contexts) = self.contexts.lock() {
            contexts.pop()
        } else {
            None
        }
    }

    pub fn current(&self) -> Option<LogContext> {
        if let Ok(contexts) = self.contexts.lock() {
            contexts.last().cloned()
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        if let Ok(contexts) = self.contexts.lock() {
            contexts.is_empty()
        } else {
            true
        }
    }

    pub fn depth(&self) -> usize {
        if let Ok(contexts) = self.contexts.lock() {
            contexts.len()
        } else {
            0
        }
    }
}

impl Default for ContextStack {
    fn default() -> Self {
        Self::new()
    }
}
