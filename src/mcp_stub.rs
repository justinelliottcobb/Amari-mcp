// Stub MCP implementation until the real MCP framework is available
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub homepage: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ToolDescription {
    pub name: String,
    pub description: String,
    pub parameters: Value,
}

pub type ToolHandler = Box<
    dyn Fn(Value) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Value>> + Send>>
        + Send
        + Sync,
>;

// Helper to create tool handlers from async functions
pub fn tool_handler<F, Fut>(f: F) -> ToolHandler
where
    F: Fn(Value) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = Result<Value>> + Send + 'static,
{
    Box::new(move |params| Box::pin(f(params)))
}

pub struct Server {
    pub info: ServerInfo,
    pub tools: HashMap<String, (ToolDescription, ToolHandler)>,
}

impl Server {
    pub fn new(info: ServerInfo) -> Self {
        Self {
            info,
            tools: HashMap::new(),
        }
    }

    pub fn add_tool(
        &mut self,
        name: &str,
        description: ToolDescription,
        handler: ToolHandler,
    ) -> Result<()> {
        self.tools.insert(name.to_string(), (description, handler));
        Ok(())
    }

    pub async fn listen(&self, _addr: &str) -> Result<()> {
        println!("🚀 Amari MCP Server started");
        println!("   Name: {}", self.info.name);
        println!("   Version: {}", self.info.version);
        println!("   Tools registered: {}", self.tools.len());

        for (name, (desc, _)) in &self.tools {
            println!("   - {}: {}", name, desc.description);
        }

        println!("\n⚠️  Note: This is a stub implementation");
        println!("   Replace with actual MCP framework when available");

        // In a real implementation, this would start the MCP server
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        Ok(())
    }
}
