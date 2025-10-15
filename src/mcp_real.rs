use anyhow::Result;
use rmcp::{
    handler::server::router::tool::ToolRouter,
    model::*,
    tool, tool_router,
    ServiceExt,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use schemars::JsonSchema;
use tracing::info;

use crate::tools::*;

/// Request types for our mathematical tools
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MultivectorRequest {
    pub coefficients: Vec<f64>,
    #[serde(default = "default_dimensions")]
    pub dimensions: Option<usize>,
    #[serde(default = "default_signature")]
    pub signature: Option<Vec<usize>>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GeometricProductRequest {
    pub a: Vec<f64>,
    pub b: Vec<f64>,
    #[serde(default = "default_signature")]
    pub signature: Option<Vec<usize>>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RotorRotationRequest {
    pub vector: Vec<f64>,
    pub axis: Vec<f64>,
    pub angle: f64,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TropicalMatrixRequest {
    pub matrix_a: Vec<Vec<f64>>,
    pub matrix_b: Vec<Vec<f64>>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ShortestPathRequest {
    pub adjacency_matrix: Vec<Vec<f64>>,
    pub source: usize,
    pub target: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GradientRequest {
    pub expression: String,
    pub variables: Vec<String>,
    pub values: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CellularAutomataRequest {
    pub initial_state: Vec<Vec<f64>>,
    pub rule: String,
    pub steps: usize,
    pub grid_width: usize,
    pub grid_height: usize,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FisherInformationRequest {
    pub distribution: String,
    pub parameters: Vec<f64>,
    pub data: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GpuBatchRequest {
    pub operation: String,
    pub data: Value,
    pub batch_size: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SaveComputationRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub computation_type: String,
    pub result: Value,
    pub metadata: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LoadComputationRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CayleyTableRequest {
    pub signature: Vec<usize>,
    #[serde(default)]
    pub force_recompute: bool,
}

// Helper functions for default values
fn default_dimensions() -> Option<usize> {
    Some(3)
}

fn default_signature() -> Option<Vec<usize>> {
    Some(vec![3, 0, 0])
}

/// Convert option signature to the format expected by our tools
fn convert_signature(sig: Option<Vec<usize>>) -> Value {
    match sig {
        Some(s) => serde_json::to_value(s).unwrap_or_else(|_| serde_json::json!([3, 0, 0])),
        None => serde_json::json!([3, 0, 0]),
    }
}

/// Real MCP server implementation using the official rmcp SDK
#[derive(Clone)]
pub struct AmariMcpServer {
    gpu_enabled: bool,
    #[cfg(feature = "database")]
    db_pool: Option<sqlx::PgPool>,
    tool_router: ToolRouter<Self>,
}

impl AmariMcpServer {
    pub async fn new(
        gpu_enabled: bool,
        #[cfg(feature = "database")] db_pool: Option<sqlx::PgPool>,
    ) -> Result<Self> {
        let server = Self {
            gpu_enabled,
            #[cfg(feature = "database")]
            db_pool,
            tool_router: Self::tool_router(),
        };

        Ok(server)
    }
}

#[tool_router]
impl AmariMcpServer {
    /// Create a multivector from coefficients in geometric algebra
    #[tool(description = "Create a multivector from coefficients in geometric algebra")]
    async fn create_multivector(
        &self,
        params: Parameters<MultivectorRequest>,
    ) -> Result<CallToolResult, McpError> {
        info!("🔧 Creating multivector");

        let request = params.into_inner();
        let args = serde_json::json!({
            "coefficients": request.coefficients,
            "dimensions": request.dimensions.unwrap_or(3),
            "signature": convert_signature(request.signature)
        });

        match geometric::create_multivector(args).await {
            Ok(result) => Ok(CallToolResult::Json(result)),
            Err(e) => Err(McpError::internal_error(&e.to_string())),
        }
    }

    /// Compute geometric product of two multivectors
    #[tool(description = "Compute geometric product of two multivectors")]
    async fn geometric_product(
        &self,
        params: Parameters<GeometricProductRequest>,
    ) -> Result<CallToolResult, McpError> {
        info!("🔧 Computing geometric product");

        let request = params.into_inner();
        let args = serde_json::json!({
            "a": request.a,
            "b": request.b,
            "signature": convert_signature(request.signature)
        });

        match geometric::geometric_product(args).await {
            Ok(result) => Ok(CallToolResult::Json(result)),
            Err(e) => Err(McpError::internal_error(&e.to_string())),
        }
    }

    /// Apply rotation to vector using rotor in geometric algebra
    #[tool(description = "Apply rotation to vector using rotor in geometric algebra")]
    async fn rotor_rotation(
        &self,
        params: Parameters<RotorRotationRequest>,
    ) -> Result<CallToolResult, McpError> {
        info!("🔧 Applying rotor rotation");

        let request = params.into_inner();
        let args = serde_json::json!({
            "vector": request.vector,
            "axis": request.axis,
            "angle": request.angle
        });

        match geometric::rotor_rotation(args).await {
            Ok(result) => Ok(CallToolResult::Json(result)),
            Err(e) => Err(McpError::internal_error(&e.to_string())),
        }
    }

    /// Multiply two matrices using tropical algebra (min-plus)
    #[tool(description = "Multiply two matrices using tropical algebra (min-plus)")]
    async fn tropical_matrix_multiply(
        &self,
        params: Parameters<TropicalMatrixRequest>,
    ) -> Result<CallToolResult, McpError> {
        info!("🔧 Computing tropical matrix multiplication");

        let request = params.into_inner();
        let args = serde_json::json!({
            "matrix_a": request.matrix_a,
            "matrix_b": request.matrix_b
        });

        match tropical::matrix_multiply(args).await {
            Ok(result) => Ok(CallToolResult::Json(result)),
            Err(e) => Err(McpError::internal_error(&e.to_string())),
        }
    }

    /// Find shortest paths in graph using tropical algebra
    #[tool(description = "Find shortest paths in graph using tropical algebra")]
    async fn shortest_path(
        &self,
        params: Parameters<ShortestPathRequest>,
    ) -> Result<CallToolResult, McpError> {
        info!("🔧 Computing shortest paths");

        let request = params.into_inner();
        let args = serde_json::json!({
            "adjacency_matrix": request.adjacency_matrix,
            "source": request.source,
            "target": request.target
        });

        match tropical::shortest_path(args).await {
            Ok(result) => Ok(CallToolResult::Json(result)),
            Err(e) => Err(McpError::internal_error(&e.to_string())),
        }
    }

    /// Compute gradient using forward-mode automatic differentiation
    #[tool(description = "Compute gradient using forward-mode automatic differentiation")]
    async fn compute_gradient(
        &self,
        params: Parameters<GradientRequest>,
    ) -> Result<CallToolResult, McpError> {
        info!("🔧 Computing gradient");

        let request = params.into_inner();
        let args = serde_json::json!({
            "expression": request.expression,
            "variables": request.variables,
            "values": request.values
        });

        match autodiff::compute_gradient(args).await {
            Ok(result) => Ok(CallToolResult::Json(result)),
            Err(e) => Err(McpError::internal_error(&e.to_string())),
        }
    }

    /// Evolve geometric cellular automata
    #[tool(description = "Evolve geometric cellular automata")]
    async fn ca_evolution(
        &self,
        params: Parameters<CellularAutomataRequest>,
    ) -> Result<CallToolResult, McpError> {
        info!("🔧 Evolving cellular automata");

        let request = params.into_inner();
        let args = serde_json::json!({
            "initial_state": request.initial_state,
            "rule": request.rule,
            "steps": request.steps,
            "grid_width": request.grid_width,
            "grid_height": request.grid_height
        });

        match cellular_automata::evolve(args).await {
            Ok(result) => Ok(CallToolResult::Json(result)),
            Err(e) => Err(McpError::internal_error(&e.to_string())),
        }
    }

    /// Compute Fisher information matrix
    #[tool(description = "Compute Fisher information matrix")]
    async fn fisher_information(
        &self,
        params: Parameters<FisherInformationRequest>,
    ) -> Result<CallToolResult, McpError> {
        info!("🔧 Computing Fisher information");

        let request = params.into_inner();
        let args = serde_json::json!({
            "distribution": request.distribution,
            "parameters": request.parameters,
            "data": request.data
        });

        match info_geometry::fisher_information(args).await {
            Ok(result) => Ok(CallToolResult::Json(result)),
            Err(e) => Err(McpError::internal_error(&e.to_string())),
        }
    }

    /// Perform batch computations on GPU (if GPU enabled)
    #[tool(description = "Perform batch computations on GPU")]
    async fn gpu_batch_compute(
        &self,
        params: Parameters<GpuBatchRequest>,
    ) -> Result<CallToolResult, McpError> {
        if !self.gpu_enabled {
            return Err(McpError::method_not_found("GPU acceleration not enabled"));
        }

        info!("🔧 GPU batch computation");

        let request = params.into_inner();
        let args = serde_json::json!({
            "operation": request.operation,
            "data": request.data,
            "batch_size": request.batch_size.unwrap_or(256)
        });

        match gpu::batch_compute(args).await {
            Ok(result) => Ok(CallToolResult::Json(result)),
            Err(e) => Err(McpError::internal_error(&e.to_string())),
        }
    }

    /// Save computation result to database (if database enabled)
    #[cfg(feature = "database")]
    #[tool(description = "Save computation result to database")]
    async fn save_computation(
        &self,
        params: Parameters<SaveComputationRequest>,
    ) -> Result<CallToolResult, McpError> {
        if self.db_pool.is_none() {
            return Err(McpError::method_not_found("Database not configured"));
        }

        info!("🔧 Saving computation to database");

        let request = params.into_inner();
        let args = serde_json::json!({
            "name": request.name,
            "type": request.computation_type,
            "result": request.result,
            "metadata": request.metadata
        });

        match database::save_computation(args).await {
            Ok(result) => Ok(CallToolResult::Json(result)),
            Err(e) => Err(McpError::internal_error(&e.to_string())),
        }
    }

    /// Load saved computation from database (if database enabled)
    #[cfg(feature = "database")]
    #[tool(description = "Load saved computation from database")]
    async fn load_computation(
        &self,
        params: Parameters<LoadComputationRequest>,
    ) -> Result<CallToolResult, McpError> {
        if self.db_pool.is_none() {
            return Err(McpError::method_not_found("Database not configured"));
        }

        info!("🔧 Loading computation from database");

        let request = params.into_inner();
        let args = serde_json::json!({
            "name": request.name
        });

        match database::load_computation(args).await {
            Ok(result) => Ok(CallToolResult::Json(result)),
            Err(e) => Err(McpError::internal_error(&e.to_string())),
        }
    }

    /// Cache and retrieve Cayley tables for geometric algebra operations
    #[tool(description = "Cache and retrieve Cayley tables for geometric algebra operations")]
    async fn get_cayley_table(
        &self,
        params: Parameters<CayleyTableRequest>,
    ) -> Result<CallToolResult, McpError> {
        info!("🔧 Retrieving Cayley table");

        let request = params.into_inner();
        let args = serde_json::json!({
            "signature": request.signature,
            "force_recompute": request.force_recompute
        });

        match cayley_tables::get_cayley_table(args).await {
            Ok(result) => Ok(CallToolResult::Json(result)),
            Err(e) => Err(McpError::internal_error(&e.to_string())),
        }
    }
}