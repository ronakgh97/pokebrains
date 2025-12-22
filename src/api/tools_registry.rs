use crate::dtos::Tool as Tool_Dtos;
use anyhow::{Result, anyhow};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> Value;
    fn tool_callback(&self) -> bool;
    async fn execute_tool(&self, args: Value) -> Result<String>;
}

#[derive(Clone)]
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool + Send + Sync>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register<T: Tool + 'static>(&mut self, tool: T) {
        self.tools.insert(tool.name().to_string(), Arc::new(tool));
    }

    pub fn check_tool_callback(&self, tool_name: &str) -> Result<bool> {
        match self.tools.get(tool_name) {
            Some(tool) => Ok(tool.tool_callback()),
            None => Err(anyhow!("Tool '{}' not found", tool_name)),
        }
    }

    pub async fn execute(&self, tool_name: &str, args: Value) -> Result<String> {
        match self.tools.get(tool_name) {
            Some(tool) => tool.execute_tool(args).await,
            None => Err(anyhow!("Tool '{}' not found", tool_name)),
        }
    }

    pub fn get_tool_definitions(&self) -> Vec<Tool_Dtos> {
        self.tools
            .values()
            .filter_map(|tool| {
                let desc = tool.description();
                serde_json::from_value(desc).ok()
            })
            .collect()
    }
}
