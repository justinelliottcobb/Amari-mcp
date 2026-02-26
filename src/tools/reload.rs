use super::SharedState;
use async_trait::async_trait;
use pmcp::{Error as McpError, RequestHandlerExtra, ToolHandler};
use serde_json::{json, Value};
use std::sync::Arc;

pub struct ReloadHandler {
    pub state: Arc<SharedState>,
}

#[async_trait]
impl ToolHandler for ReloadHandler {
    fn metadata(&self) -> Option<pmcp::ToolInfo> {
        Some(super::tool_info(
            "reload",
            "Re-parse the library source and refresh the API index without restarting the server. Use this after the library has been updated.",
            json!({
                "type": "object",
                "properties": {}
            }),
        ))
    }

    async fn handle(&self, _args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        match self.state.reload() {
            Ok(report) => {
                let item_diff = report.new_items as i64 - report.old_items as i64;
                let diff_label = if item_diff > 0 {
                    format!("+{item_diff}")
                } else {
                    format!("{item_diff}")
                };

                Ok(json!({
                    "status": "reloaded",
                    "previous": {
                        "crates": report.old_crates,
                        "modules": report.old_modules,
                        "items": report.old_items,
                    },
                    "current": {
                        "crates": report.new_crates,
                        "modules": report.new_modules,
                        "items": report.new_items,
                    },
                    "item_diff": diff_label,
                }))
            }
            Err(e) => Ok(json!({
                "status": "error",
                "error": e,
            })),
        }
    }
}
