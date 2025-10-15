use anyhow::Result;
use async_trait::async_trait;
use pmcp::{Error as McpError, RequestHandlerExtra, Server, ServerCapabilities, ToolHandler};
use serde_json::Value;
use tracing::{info, warn};

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
            return Err(McpError::Internal("GPU acceleration not enabled".to_string()));
        }

        info!("🔧 GPU batch computation");
        gpu::batch_compute(args)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))
    }
}

/// Tool handler for saving computations to database
#[cfg(feature = "database")]
pub struct SaveComputationHandler {
    db_available: bool,
}

#[cfg(feature = "database")]
impl SaveComputationHandler {
    pub fn new(db_available: bool) -> Self {
        Self { db_available }
    }
}

#[cfg(feature = "database")]
#[async_trait]
impl ToolHandler for SaveComputationHandler {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        if !self.db_available {
            return Err(McpError::Internal("Database not configured".to_string()));
        }

        info!("🔧 Saving computation to database");
        database::save_computation(args)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))
    }
}

/// Tool handler for loading computations from database
#[cfg(feature = "database")]
pub struct LoadComputationHandler {
    db_available: bool,
}

#[cfg(feature = "database")]
impl LoadComputationHandler {
    pub fn new(db_available: bool) -> Self {
        Self { db_available }
    }
}

#[cfg(feature = "database")]
#[async_trait]
impl ToolHandler for LoadComputationHandler {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        if !self.db_available {
            return Err(McpError::Internal("Database not configured".to_string()));
        }

        info!("🔧 Loading computation from database");
        database::load_computation(args)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))
    }
}

/// Tool handler for Cayley table operations
pub struct CayleyTableHandler;

#[async_trait]
impl ToolHandler for CayleyTableHandler {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        info!("Retrieving Cayley table");
        cayley_tables::get_cayley_table(args)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))
    }
}

/// Tool handler for precomputing essential Cayley tables
#[cfg(feature = "database")]
pub struct PrecomputeTablesHandler {
    db_available: bool,
}

#[cfg(feature = "database")]
impl PrecomputeTablesHandler {
    pub fn new(db_available: bool) -> Self {
        Self { db_available }
    }
}

#[cfg(feature = "database")]
#[async_trait]
impl ToolHandler for PrecomputeTablesHandler {
    async fn handle(&self, _args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        if !self.db_available {
            return Err(McpError::Internal("Database not configured".to_string()));
        }

        info!("🧮 Precomputing essential Cayley tables");
        // This is a placeholder - would need access to the database pool
        // In practice, this would be called during server initialization
        Ok(serde_json::json!({
            "success": true,
            "note": "Precomputation should be run during server initialization with database access"
        }))
    }
}

/// Tool handler for getting precomputation status
#[cfg(feature = "database")]
pub struct PrecomputeStatusHandler {
    db_available: bool,
}

#[cfg(feature = "database")]
impl PrecomputeStatusHandler {
    pub fn new(db_available: bool) -> Self {
        Self { db_available }
    }
}

#[cfg(feature = "database")]
#[async_trait]
impl ToolHandler for PrecomputeStatusHandler {
    async fn handle(&self, _args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        if !self.db_available {
            return Err(McpError::Internal("Database not configured".to_string()));
        }

        info!("📊 Getting Cayley table precomputation status");
        // This is a placeholder - would need access to the database pool
        Ok(serde_json::json!({
            "success": true,
            "note": "Status check requires database pool access"
        }))
    }
}

/// Create and configure the Amari MCP server
pub async fn create_amari_mcp_server(
    gpu_enabled: bool,
    #[cfg(feature = "database")] db_available: bool,
) -> Result<Server> {
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

    // Add GPU tools if enabled
    if gpu_enabled {
        info!("   🚀 Adding GPU acceleration tools");
        server_builder = server_builder.tool("gpu_batch_compute", GpuBatchHandler::new(gpu_enabled));
    }

    // Add database tools if enabled
    #[cfg(feature = "database")]
    if db_available {
        info!("   💾 Adding database tools and Cayley table management");
        server_builder = server_builder
            .tool("save_computation", SaveComputationHandler::new(db_available))
            .tool("load_computation", LoadComputationHandler::new(db_available))
            .tool("precompute_cayley_tables", PrecomputeTablesHandler::new(db_available))
            .tool("cayley_precompute_status", PrecomputeStatusHandler::new(db_available));
    }

    let server = server_builder
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to build MCP server: {}", e))?;

    info!("✅ Amari MCP Server created successfully");

    Ok(server)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use pmcp::RequestHandlerExtra;

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
        let server_result = create_amari_mcp_server(
            false, // GPU disabled
            #[cfg(feature = "database")]
            false, // DB disabled
        ).await;

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

    #[cfg(feature = "database")]
    mod database_tests {
        use super::*;

        #[tokio::test]
        async fn test_save_computation_handler_disabled() {
            let handler = SaveComputationHandler::new(false); // DB disabled
            let args = json!({
                "id": "test-computation",
                "result": {"value": 42},
                "metadata": {"type": "test"}
            });

            let result = handler.handle(args, mock_extra()).await;
            assert!(result.is_err());

            if let Err(pmcp::Error::Internal(msg)) = result {
                assert!(msg.contains("Database not configured"));
            } else {
                panic!("Expected Internal error with specific message");
            }
        }

        #[tokio::test]
        async fn test_precompute_tables_handler_disabled() {
            let handler = PrecomputeTablesHandler::new(false); // DB disabled
            let args = json!({});

            let result = handler.handle(args, mock_extra()).await;
            assert!(result.is_err());

            if let Err(pmcp::Error::Internal(msg)) = result {
                assert!(msg.contains("Database not configured"));
            } else {
                panic!("Expected Internal error with specific message");
            }
        }
    }
}