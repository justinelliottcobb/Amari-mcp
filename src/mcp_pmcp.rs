use anyhow::Result;
use async_trait::async_trait;
use pmcp::{Error as McpError, RequestHandlerExtra, Server, ServerCapabilities, ToolHandler};
use serde_json::Value;
use tracing::{info, warn};

use crate::tools::*;

/// Tool handler for geometric algebra operations
struct GeometricToolHandler;

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
struct GeometricProductHandler;

#[async_trait]
impl ToolHandler for GeometricProductHandler {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        info!("🔧 Computing geometric product");
        geometric::geometric_product(args)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))
    }
}

/// Tool handler for rotor rotation operations
struct RotorRotationHandler;

#[async_trait]
impl ToolHandler for RotorRotationHandler {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        info!("🔧 Applying rotor rotation");
        geometric::rotor_rotation(args)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))
    }
}

/// Tool handler for tropical matrix multiplication
struct TropicalMatrixHandler;

#[async_trait]
impl ToolHandler for TropicalMatrixHandler {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        info!("🔧 Computing tropical matrix multiplication");
        tropical::matrix_multiply(args)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))
    }
}

/// Tool handler for shortest path computation
struct ShortestPathHandler;

#[async_trait]
impl ToolHandler for ShortestPathHandler {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        info!("🔧 Computing shortest paths");
        tropical::shortest_path(args)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))
    }
}

/// Tool handler for gradient computation
struct GradientHandler;

#[async_trait]
impl ToolHandler for GradientHandler {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        info!("🔧 Computing gradient");
        autodiff::compute_gradient(args)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))
    }
}

/// Tool handler for cellular automata evolution
struct CellularAutomataHandler;

#[async_trait]
impl ToolHandler for CellularAutomataHandler {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        info!("🔧 Evolving cellular automata");
        cellular_automata::evolve(args)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))
    }
}

/// Tool handler for Fisher information computation
struct FisherInformationHandler;

#[async_trait]
impl ToolHandler for FisherInformationHandler {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        info!("🔧 Computing Fisher information");
        info_geometry::fisher_information(args)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))
    }
}

/// Tool handler for GPU batch computation
struct GpuBatchHandler {
    gpu_enabled: bool,
}

impl GpuBatchHandler {
    fn new(gpu_enabled: bool) -> Self {
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
struct SaveComputationHandler {
    db_available: bool,
}

#[cfg(feature = "database")]
impl SaveComputationHandler {
    fn new(db_available: bool) -> Self {
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
struct LoadComputationHandler {
    db_available: bool,
}

#[cfg(feature = "database")]
impl LoadComputationHandler {
    fn new(db_available: bool) -> Self {
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
struct CayleyTableHandler;

#[async_trait]
impl ToolHandler for CayleyTableHandler {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        info!("🔧 Retrieving Cayley table");
        cayley_tables::get_cayley_table(args)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))
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
        info!("   💾 Adding database tools");
        server_builder = server_builder
            .tool("save_computation", SaveComputationHandler::new(db_available))
            .tool("load_computation", LoadComputationHandler::new(db_available));
    }

    let server = server_builder
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to build MCP server: {}", e))?;

    info!("✅ Amari MCP Server created successfully");

    Ok(server)
}