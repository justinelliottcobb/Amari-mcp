use anyhow::Result;
use async_trait::async_trait;
use pmcp::{Error as McpError, RequestHandlerExtra, Server, ServerCapabilities, ToolHandler};
use serde_json::Value;
use tracing::info;

use crate::tools::*;

/// Tool handler for geometric algebra operations
pub struct GeometricToolHandler;

#[async_trait]
impl ToolHandler for GeometricToolHandler {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        info!("🔧 Executing geometric tool");
        geometric::create_multivector(args)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))
    }
}

/// Tool handler for geometric product operations
pub struct GeometricProductHandler;

#[async_trait]
impl ToolHandler for GeometricProductHandler {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        info!("Computing geometric product");
        geometric::geometric_product(args)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))
    }
}

/// Tool handler for rotor rotation operations
pub struct RotorRotationHandler;

#[async_trait]
impl ToolHandler for RotorRotationHandler {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        info!("Applying rotor rotation");
        geometric::rotor_rotation(args)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))
    }
}

/// Tool handler for tropical matrix multiplication
pub struct TropicalMatrixHandler;

#[async_trait]
impl ToolHandler for TropicalMatrixHandler {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        info!("Computing tropical matrix multiplication");
        tropical::matrix_multiply(args)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))
    }
}

/// Tool handler for shortest path computation
pub struct ShortestPathHandler;

#[async_trait]
impl ToolHandler for ShortestPathHandler {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        info!("Computing shortest paths");
        tropical::shortest_path(args)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))
    }
}

/// Tool handler for gradient computation
pub struct GradientHandler;

#[async_trait]
impl ToolHandler for GradientHandler {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        info!("Computing gradient");
        autodiff::compute_gradient(args)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))
    }
}

/// Tool handler for cellular automata evolution
pub struct CellularAutomataHandler;

#[async_trait]
impl ToolHandler for CellularAutomataHandler {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        info!("Evolving cellular automata");
        cellular_automata::evolve(args)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))
    }
}

/// Tool handler for Fisher information computation
pub struct FisherInformationHandler;

#[async_trait]
impl ToolHandler for FisherInformationHandler {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        info!("Computing Fisher information");
        info_geometry::fisher_information(args)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))
    }
}

/// Tool handler for GPU batch computation
pub struct GpuBatchHandler {
    gpu_enabled: bool,
}

impl GpuBatchHandler {
    pub fn new(gpu_enabled: bool) -> Self {
        Self { gpu_enabled }
    }
}

#[async_trait]
impl ToolHandler for GpuBatchHandler {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        if !self.gpu_enabled {
            return Err(McpError::Internal(
                "GPU acceleration not enabled".to_string(),
            ));
        }

        info!("🔧 GPU batch computation");
        gpu::batch_compute(args)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))
    }
}

// Database handlers removed - MCP servers should be simple and stateless

/// Tool handler for Cayley table operations
pub struct CayleyTableHandler;

/// Tool handler for library documentation browsing
pub struct LibraryDocsHandler;

/// Tool handler for code analysis
pub struct CodeAnalysisHandler;

/// Tool handler for project scaffolding
pub struct ProjectScaffoldHandler;

/// Tool handler for code generation
pub struct CodeGenerationHandler;

/// Tool handler for pattern searching
pub struct PatternSearchHandler;

#[async_trait]
impl ToolHandler for CayleyTableHandler {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        info!("Retrieving Cayley table");
        cayley_tables::get_cayley_table(args)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))
    }
}

#[async_trait]
impl ToolHandler for LibraryDocsHandler {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        info!("📚 Browsing Amari library documentation");
        library_access::browse_docs(args)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))
    }
}

#[async_trait]
impl ToolHandler for CodeAnalysisHandler {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        info!("🔍 Analyzing Amari code structure");
        library_access::analyze_code(args)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))
    }
}

#[async_trait]
impl ToolHandler for ProjectScaffoldHandler {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        info!("🏗️ Scaffolding Amari project");
        library_access::scaffold_project(args)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))
    }
}

#[async_trait]
impl ToolHandler for CodeGenerationHandler {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        info!("💻 Generating Amari code examples");
        library_access::generate_code(args)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))
    }
}

#[async_trait]
impl ToolHandler for PatternSearchHandler {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        info!("🔎 Searching Amari codebase patterns");
        library_access::search_patterns(args)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))
    }
}

/// Create and configure the Amari MCP server
pub async fn create_amari_mcp_server(gpu_enabled: bool) -> Result<Server> {
    info!("🧮 Creating Amari MCP Server with pmcp");

    let mut server_builder = Server::builder()
        .name("amari-mcp")
        .version(env!("CARGO_PKG_VERSION"))
        .capabilities(ServerCapabilities::default());

    // Register core mathematical tools
    server_builder = server_builder
        .tool("create_multivector", GeometricToolHandler)
        .tool("geometric_product", GeometricProductHandler)
        .tool("rotor_rotation", RotorRotationHandler)
        .tool("tropical_matrix_multiply", TropicalMatrixHandler)
        .tool("shortest_path", ShortestPathHandler)
        .tool("compute_gradient", GradientHandler)
        .tool("ca_evolution", CellularAutomataHandler)
        .tool("fisher_information", FisherInformationHandler)
        .tool("get_cayley_table", CayleyTableHandler);

    // Register library access and development tools
    server_builder = server_builder
        .tool("browse_docs", LibraryDocsHandler)
        .tool("analyze_code", CodeAnalysisHandler)
        .tool("scaffold_project", ProjectScaffoldHandler)
        .tool("generate_code", CodeGenerationHandler)
        .tool("search_patterns", PatternSearchHandler);

    // Add GPU tools if enabled
    if gpu_enabled {
        info!("   🚀 Adding GPU acceleration tools");
        server_builder =
            server_builder.tool("gpu_batch_compute", GpuBatchHandler::new(gpu_enabled));
    }

    // Database tools removed - caching should be handled by Amari core library

    let server = server_builder
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to build MCP server: {}", e))?;

    info!("✅ Amari MCP Server created successfully");

    Ok(server)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pmcp::RequestHandlerExtra;
    use serde_json::json;

    fn mock_extra() -> RequestHandlerExtra {
        use std::collections::HashMap;

        RequestHandlerExtra {
            cancellation_token: tokio_util::sync::CancellationToken::new(),
            request_id: "test-123".to_string(),
            session_id: Some("session-123".to_string()),
            auth_info: None,
            auth_context: None,
            metadata: HashMap::new(),
        }
    }

    #[tokio::test]
    async fn test_create_amari_mcp_server() {
        let server_result = create_amari_mcp_server(false).await;

        assert!(server_result.is_ok());
    }

    #[tokio::test]
    async fn test_geometric_tool_handler() {
        let handler = GeometricToolHandler;
        let args = json!({
            "coefficients": [1.0, 2.0, 3.0, 4.0],
            "signature": [2, 0, 0]
        });

        let result = handler.handle(args, mock_extra()).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response["success"], true);
    }

    #[tokio::test]
    async fn test_cayley_table_handler() {
        let handler = CayleyTableHandler;
        let args = json!({
            "signature": [3, 0, 0]
        });

        let result = handler.handle(args, mock_extra()).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response["success"], true);
        assert_eq!(response["signature"], json!([3, 0, 0]));
    }

    #[tokio::test]
    async fn test_gpu_batch_handler_disabled() {
        let handler = GpuBatchHandler::new(false); // GPU disabled
        let args = json!({
            "operation": "geometric_product",
            "data": [],
            "batch_size": 1024
        });

        let result = handler.handle(args, mock_extra()).await;
        assert!(result.is_err());

        if let Err(pmcp::Error::Internal(msg)) = result {
            assert!(msg.contains("GPU acceleration not enabled"));
        } else {
            panic!("Expected Internal error with specific message");
        }
    }

    #[tokio::test]
    async fn test_invalid_tool_arguments() {
        let handler = GeometricToolHandler;
        let invalid_args = json!({
            "invalid_field": "invalid_value"
        });

        let result = handler.handle(invalid_args, mock_extra()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_missing_required_arguments() {
        let handler = CayleyTableHandler;
        let empty_args = json!({});

        let result = handler.handle(empty_args, mock_extra()).await;
        assert!(result.is_err());
    }

    // Database tests removed - MCP servers should be simple and stateless

    #[tokio::test]
    async fn test_library_docs_handler() {
        let handler = LibraryDocsHandler;
        let args = json!({
            "module": "core"
        });

        let result = handler.handle(args, mock_extra()).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response["success"].is_boolean());
    }

    #[tokio::test]
    async fn test_library_docs_handler_unknown_module() {
        let handler = LibraryDocsHandler;
        let args = json!({
            "module": "unknown_module"
        });

        let result = handler.handle(args, mock_extra()).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response["success"], false);
        assert!(response["error"].as_str().unwrap().contains("Unknown module"));
    }

    #[tokio::test]
    async fn test_code_analysis_handler() {
        let handler = CodeAnalysisHandler;
        let args = json!({
            "target": "structure"
        });

        let result = handler.handle(args, mock_extra()).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response["success"].is_boolean());
    }

    #[tokio::test]
    async fn test_code_analysis_handler_unknown_target() {
        let handler = CodeAnalysisHandler;
        let args = json!({
            "target": "unknown_target"
        });

        let result = handler.handle(args, mock_extra()).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response["success"], false);
        assert!(response["error"].as_str().unwrap().contains("Unknown analysis target"));
    }

    #[tokio::test]
    async fn test_project_scaffold_handler() {
        let handler = ProjectScaffoldHandler;
        let args = json!({
            "type": "basic",
            "name": "test-project"
        });

        let result = handler.handle(args, mock_extra()).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response["success"], true);
        assert_eq!(response["project_type"], "basic");
        assert_eq!(response["name"], "test-project");
    }

    #[tokio::test]
    async fn test_project_scaffold_handler_unknown_type() {
        let handler = ProjectScaffoldHandler;
        let args = json!({
            "type": "unknown_type",
            "name": "test-project"
        });

        let result = handler.handle(args, mock_extra()).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response["success"], false);
        assert!(response["error"].as_str().unwrap().contains("Unknown project type"));
    }

    #[tokio::test]
    async fn test_code_generation_handler() {
        let handler = CodeGenerationHandler;
        let args = json!({
            "operation": "multivector",
            "context": "basic"
        });

        let result = handler.handle(args, mock_extra()).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response["success"], true);
        assert_eq!(response["operation"], "multivector");
        assert_eq!(response["context"], "basic");
        assert!(response["code"].is_string());
    }

    #[tokio::test]
    async fn test_code_generation_handler_unknown_operation() {
        let handler = CodeGenerationHandler;
        let args = json!({
            "operation": "unknown_operation"
        });

        let result = handler.handle(args, mock_extra()).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response["success"], false);
        assert!(response["error"].as_str().unwrap().contains("Unknown operation"));
    }

    #[tokio::test]
    async fn test_pattern_search_handler() {
        let handler = PatternSearchHandler;
        let args = json!({
            "pattern": "geometric_product",
            "scope": "core"
        });

        let result = handler.handle(args, mock_extra()).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response["success"], true);
        assert_eq!(response["pattern"], "geometric_product");
        assert_eq!(response["scope"], "core");
    }

    #[tokio::test]
    async fn test_pattern_search_handler_empty_pattern() {
        let handler = PatternSearchHandler;
        let args = json!({
            "pattern": "",
            "scope": "all"
        });

        let result = handler.handle(args, mock_extra()).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response["success"], false);
        assert!(response["error"].as_str().unwrap().contains("Pattern cannot be empty"));
    }
}
