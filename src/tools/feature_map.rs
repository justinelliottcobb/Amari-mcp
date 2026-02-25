use super::SharedState;
use async_trait::async_trait;
use pmcp::{Error as McpError, RequestHandlerExtra, ToolHandler, ToolInfo};
use serde_json::{json, Value};
use std::sync::Arc;

pub struct FeatureMapHandler {
    pub state: Arc<SharedState>,
}

#[async_trait]
impl ToolHandler for FeatureMapHandler {
    fn metadata(&self) -> Option<ToolInfo> {
        Some(ToolInfo {
            name: "feature_map".to_string(),
            description: Some(
                "Show which Cargo features enable which crates and types".to_string(),
            ),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "feature": {
                        "type": "string",
                        "description": "Specific feature to query, or omit for the full feature map"
                    }
                }
            }),
        })
    }

    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        let feature_filter = args.get("feature").and_then(|v| v.as_str());

        let default_crates: Vec<&str> = self
            .state
            .index
            .crates
            .iter()
            .filter(|c| c.feature_gate.is_none())
            .map(|c| c.name.as_str())
            .collect();

        let features: Vec<Value> = self
            .state
            .manifest
            .crates
            .optional
            .iter()
            .filter(|(feature, _)| feature_filter.is_none_or(|f| f == feature.as_str()))
            .map(|(feature, dir_name)| {
                let alias = self.state.manifest.alias_for(dir_name);
                let item_count = self.state.index.feature_items(feature).len();
                json!({
                    "feature": feature,
                    "crate_dir": dir_name,
                    "alias": alias,
                    "public_item_count": item_count,
                })
            })
            .collect();

        Ok(json!({
            "library": self.state.manifest.library.name,
            "default_crates": default_crates,
            "optional_features": features,
        }))
    }
}
