use anyhow::Result;
// use serde_json::Value;
use tracing::info;

use crate::mcp_stub::{Server, ServerInfo, ToolDescription, tool_handler};

#[cfg(feature = "database")]
use sqlx::PgPool;

use crate::tools::*;

/// Main Amari MCP Server
pub struct AmariMcpServer {
    gpu_enabled: bool,
    #[cfg(feature = "database")]
    db_pool: Option<PgPool>,
}

impl AmariMcpServer {
    pub async fn new(
        gpu_enabled: bool,
        #[cfg(feature = "database")] database_url: Option<String>,
    ) -> Result<Self> {
        #[cfg(feature = "database")]
        let db_pool = if let Some(url) = database_url {
            Some(PgPool::connect(&url).await?)
        } else {
            None
        };

        Ok(Self {
            gpu_enabled,
            #[cfg(feature = "database")]
            db_pool,
        })
    }

    pub async fn run(self, host: &str, port: u16) -> Result<()> {
        info!("🧮 Registering Amari mathematical tools...");

        let mut server = Server::new(ServerInfo {
            name: "amari-mcp".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: Some("Mathematical computing with Amari library".to_string()),
            author: Some("Amari Contributors".to_string()),
            homepage: Some("https://github.com/justinelliottcobb/amari-mcp".to_string()),
        });

        // Register all tools
        self.register_geometric_tools(&mut server)?;
        self.register_tropical_tools(&mut server)?;
        self.register_autodiff_tools(&mut server)?;
        self.register_cellular_automata_tools(&mut server)?;
        self.register_info_geometry_tools(&mut server)?;

        if self.gpu_enabled {
            self.register_gpu_tools(&mut server)?;
        }

        #[cfg(feature = "database")]
        if self.db_pool.is_some() {
            self.register_database_tools(&mut server)?;
        }

        info!("✅ All tools registered successfully");

        // Start the server
        let addr = format!("{}:{}", host, port);
        info!("🌐 Starting server on {}", addr);

        server.listen(&addr).await?;

        Ok(())
    }

    fn register_geometric_tools(&self, server: &mut Server) -> Result<()> {
        info!("   📐 Registering geometric algebra tools");

        server.add_tool(
            "create_multivector",
            ToolDescription {
                name: "create_multivector".to_string(),
                description: "Create a multivector from coefficients in geometric algebra".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "coefficients": {
                            "type": "array",
                            "items": {"type": "number"},
                            "description": "Array of coefficients for basis elements"
                        },
                        "dimensions": {
                            "type": "integer",
                            "description": "Number of spatial dimensions (default: 3)"
                        },
                        "signature": {
                            "type": "array",
                            "items": {"type": "integer"},
                            "description": "Metric signature [p, q, r] (default: [3, 0, 0])"
                        }
                    },
                    "required": ["coefficients"]
                }),
            },
            tool_handler(geometric::create_multivector),
        )?;

        server.add_tool(
            "geometric_product",
            ToolDescription {
                name: "geometric_product".to_string(),
                description: "Compute geometric product of two multivectors".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "a": {
                            "type": "array",
                            "items": {"type": "number"},
                            "description": "First multivector coefficients"
                        },
                        "b": {
                            "type": "array",
                            "items": {"type": "number"},
                            "description": "Second multivector coefficients"
                        },
                        "signature": {
                            "type": "array",
                            "items": {"type": "integer"},
                            "description": "Metric signature [p, q, r] (default: [3, 0, 0])"
                        }
                    },
                    "required": ["a", "b"]
                }),
            },
            tool_handler(geometric::geometric_product),
        )?;

        server.add_tool(
            "rotor_rotation",
            ToolDescription {
                name: "rotor_rotation".to_string(),
                description: "Apply rotation to vector using rotor in geometric algebra".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "vector": {
                            "type": "array",
                            "items": {"type": "number"},
                            "description": "Vector to rotate [x, y, z]"
                        },
                        "axis": {
                            "type": "array",
                            "items": {"type": "number"},
                            "description": "Rotation axis [x, y, z]"
                        },
                        "angle": {
                            "type": "number",
                            "description": "Rotation angle in radians"
                        }
                    },
                    "required": ["vector", "axis", "angle"]
                }),
            },
            tool_handler(geometric::rotor_rotation),
        )?;

        Ok(())
    }

    fn register_tropical_tools(&self, server: &mut Server) -> Result<()> {
        info!("   🏝️ Registering tropical algebra tools");

        server.add_tool(
            "tropical_matrix_multiply",
            ToolDescription {
                name: "tropical_matrix_multiply".to_string(),
                description: "Multiply two matrices using tropical algebra (min-plus)".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "matrix_a": {
                            "type": "array",
                            "items": {
                                "type": "array",
                                "items": {"type": "number"}
                            },
                            "description": "First matrix"
                        },
                        "matrix_b": {
                            "type": "array",
                            "items": {
                                "type": "array",
                                "items": {"type": "number"}
                            },
                            "description": "Second matrix"
                        }
                    },
                    "required": ["matrix_a", "matrix_b"]
                }),
            },
            tool_handler(tropical::matrix_multiply),
        )?;

        server.add_tool(
            "shortest_path",
            ToolDescription {
                name: "shortest_path".to_string(),
                description: "Find shortest paths in graph using tropical algebra".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "adjacency_matrix": {
                            "type": "array",
                            "items": {
                                "type": "array",
                                "items": {"type": "number"}
                            },
                            "description": "Graph adjacency matrix with edge weights"
                        },
                        "source": {
                            "type": "integer",
                            "description": "Source vertex index"
                        },
                        "target": {
                            "type": "integer",
                            "description": "Target vertex index (optional - if not provided, computes all distances)"
                        }
                    },
                    "required": ["adjacency_matrix", "source"]
                }),
            },
            tool_handler(tropical::shortest_path),
        )?;

        Ok(())
    }

    fn register_autodiff_tools(&self, server: &mut Server) -> Result<()> {
        info!("   🔄 Registering automatic differentiation tools");

        server.add_tool(
            "compute_gradient",
            ToolDescription {
                name: "compute_gradient".to_string(),
                description: "Compute gradient using forward-mode automatic differentiation".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "expression": {
                            "type": "string",
                            "description": "Mathematical expression to differentiate"
                        },
                        "variables": {
                            "type": "array",
                            "items": {"type": "string"},
                            "description": "Variable names"
                        },
                        "values": {
                            "type": "array",
                            "items": {"type": "number"},
                            "description": "Variable values at which to evaluate gradient"
                        }
                    },
                    "required": ["expression", "variables", "values"]
                }),
            },
            tool_handler(autodiff::compute_gradient),
        )?;

        Ok(())
    }

    fn register_cellular_automata_tools(&self, server: &mut Server) -> Result<()> {
        info!("   🔬 Registering cellular automata tools");

        server.add_tool(
            "ca_evolution",
            ToolDescription {
                name: "ca_evolution".to_string(),
                description: "Evolve geometric cellular automata".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "initial_state": {
                            "type": "array",
                            "items": {
                                "type": "array",
                                "items": {"type": "number"}
                            },
                            "description": "Initial grid state as array of multivector coefficients"
                        },
                        "rule": {
                            "type": "string",
                            "enum": ["geometric", "game_of_life", "conservative", "rotor"],
                            "description": "CA evolution rule"
                        },
                        "steps": {
                            "type": "integer",
                            "description": "Number of evolution steps"
                        },
                        "grid_width": {
                            "type": "integer",
                            "description": "Grid width"
                        },
                        "grid_height": {
                            "type": "integer",
                            "description": "Grid height"
                        }
                    },
                    "required": ["initial_state", "rule", "steps", "grid_width", "grid_height"]
                }),
            },
            tool_handler(cellular_automata::evolve),
        )?;

        Ok(())
    }

    fn register_info_geometry_tools(&self, server: &mut Server) -> Result<()> {
        info!("   📊 Registering information geometry tools");

        server.add_tool(
            "fisher_information",
            ToolDescription {
                name: "fisher_information".to_string(),
                description: "Compute Fisher information matrix".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "distribution": {
                            "type": "string",
                            "enum": ["gaussian", "exponential", "categorical"],
                            "description": "Probability distribution family"
                        },
                        "parameters": {
                            "type": "array",
                            "items": {"type": "number"},
                            "description": "Distribution parameters"
                        },
                        "data": {
                            "type": "array",
                            "items": {"type": "number"},
                            "description": "Observed data points"
                        }
                    },
                    "required": ["distribution", "parameters", "data"]
                }),
            },
            tool_handler(info_geometry::fisher_information),
        )?;

        Ok(())
    }

    fn register_gpu_tools(&self, server: &mut Server) -> Result<()> {
        info!("   🚀 Registering GPU acceleration tools");

        server.add_tool(
            "gpu_batch_compute",
            ToolDescription {
                name: "gpu_batch_compute".to_string(),
                description: "Perform batch computations on GPU".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "operation": {
                            "type": "string",
                            "enum": ["geometric_product", "tropical_multiply", "ca_evolution"],
                            "description": "GPU operation to perform"
                        },
                        "data": {
                            "type": "array",
                            "description": "Input data for batch processing"
                        },
                        "batch_size": {
                            "type": "integer",
                            "description": "Batch size for GPU processing"
                        }
                    },
                    "required": ["operation", "data"]
                }),
            },
            tool_handler(gpu::batch_compute),
        )?;

        Ok(())
    }

    #[cfg(feature = "database")]
    fn register_database_tools(&self, server: &mut Server) -> Result<()> {
        info!("   💾 Registering database tools");

        server.add_tool(
            "save_computation",
            ToolDescription {
                name: "save_computation".to_string(),
                description: "Save computation result to database".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "name": {
                            "type": "string",
                            "description": "Computation name/identifier"
                        },
                        "type": {
                            "type": "string",
                            "description": "Computation type"
                        },
                        "result": {
                            "description": "Computation result"
                        },
                        "metadata": {
                            "type": "object",
                            "description": "Additional metadata"
                        }
                    },
                    "required": ["name", "type", "result"]
                }),
            },
            tool_handler(database::save_computation),
        )?;

        server.add_tool(
            "load_computation",
            ToolDescription {
                name: "load_computation".to_string(),
                description: "Load saved computation from database".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "name": {
                            "type": "string",
                            "description": "Computation name/identifier"
                        }
                    },
                    "required": ["name"]
                }),
            },
            tool_handler(database::load_computation),
        )?;

        Ok(())
    }
}