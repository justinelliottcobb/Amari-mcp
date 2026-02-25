use super::SharedState;
use crate::parser::workspace;
use async_trait::async_trait;
use pmcp::{Error as McpError, RequestHandlerExtra, ToolHandler, ToolInfo};
use serde_json::{json, Value};
use std::sync::Arc;

pub struct DependencyGraphHandler {
    pub state: Arc<SharedState>,
}

#[async_trait]
impl ToolHandler for DependencyGraphHandler {
    fn metadata(&self) -> Option<ToolInfo> {
        Some(ToolInfo {
            name: "dependency_graph".to_string(),
            description: Some(
                "Show inter-crate dependency relationships within the workspace".to_string(),
            ),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "crate": {
                        "type": "string",
                        "description": "Show dependencies for a specific crate, or omit for the full graph"
                    }
                }
            }),
        })
    }

    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        let crate_filter = args.get("crate").and_then(|v| v.as_str());

        let crate_dirs: Vec<(String, &std::path::Path)> = self
            .state
            .index
            .crates
            .iter()
            .map(|c| (c.name.clone(), c.source_dir.as_path()))
            .collect();

        let graph = workspace::build_dependency_graph(&crate_dirs);

        if let Some(name) = crate_filter {
            let deps = graph.get(name).cloned().unwrap_or_default();
            let depended_by: Vec<&str> = graph
                .iter()
                .filter(|(_, deps)| deps.contains(&name.to_string()))
                .map(|(name, _)| name.as_str())
                .collect();

            Ok(json!({
                "crate": name,
                "depends_on": deps,
                "depended_by": depended_by,
            }))
        } else {
            let entries: Vec<Value> = graph
                .iter()
                .map(|(name, deps)| {
                    let depended_by: Vec<&str> = graph
                        .iter()
                        .filter(|(_, d)| d.contains(name))
                        .map(|(n, _)| n.as_str())
                        .collect();
                    json!({
                        "crate": name,
                        "depends_on": deps,
                        "depended_by": depended_by,
                    })
                })
                .collect();
            Ok(json!({ "graph": entries }))
        }
    }
}
