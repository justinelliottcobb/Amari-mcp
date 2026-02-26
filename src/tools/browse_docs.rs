use super::SharedState;
use async_trait::async_trait;
use pmcp::{Error as McpError, RequestHandlerExtra, ToolHandler};
use serde_json::{json, Value};
use std::sync::Arc;

pub struct BrowseDocsHandler {
    pub state: Arc<SharedState>,
}

#[async_trait]
impl ToolHandler for BrowseDocsHandler {
    fn metadata(&self) -> Option<pmcp::ToolInfo> {
        Some(super::tool_info(
            "browse_docs",
            "Browse module-level and item-level documentation from the library source",
            json!({
                "type": "object",
                "properties": {
                    "crate": {
                        "type": "string",
                        "description": "Crate name or alias"
                    },
                    "module": {
                        "type": "string",
                        "description": "Module path (e.g. 'rotor'). Omit for crate-level docs."
                    },
                    "item": {
                        "type": "string",
                        "description": "Specific item name for full docs"
                    }
                },
                "required": ["crate"]
            }),
        ))
    }

    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        let crate_name = args["crate"]
            .as_str()
            .ok_or_else(|| McpError::invalid_params("crate is required"))?;

        let module_path = args.get("module").and_then(|v| v.as_str());
        let item_name = args.get("item").and_then(|v| v.as_str());

        let index = self
            .state
            .index
            .read()
            .map_err(|_| McpError::internal("index lock poisoned"))?;
        let crate_info = index.get_crate(crate_name);
        let Some(crate_info) = crate_info else {
            return Ok(json!({"error": format!("Crate '{crate_name}' not found")}));
        };

        if let Some(item) = item_name {
            let found = index
                .search(item)
                .into_iter()
                .find(|i| i.name == item && i.full_path.contains(crate_name));

            return if let Some(found) = found {
                Ok(json!({
                    "item": found.name,
                    "full_path": found.full_path,
                    "signature": found.signature,
                    "documentation": found.doc_comment,
                    "source_file": found.source_file.display().to_string(),
                    "line": found.line_number,
                }))
            } else {
                Ok(json!({"error": format!("Item '{item}' not found in crate '{crate_name}'")}))
            };
        }

        if let Some(path) = module_path {
            find_module_docs(&crate_info.modules, path)
                .map(|(name, docs)| {
                    Ok(json!({
                        "crate": crate_name,
                        "module": name,
                        "documentation": docs,
                    }))
                })
                .unwrap_or_else(|| {
                    Ok(json!({
                        "error": format!("Module '{path}' not found in crate '{crate_name}'"),
                    }))
                })
        } else {
            Ok(json!({
                "crate": crate_name,
                "alias": crate_info.alias,
                "feature_gate": crate_info.feature_gate,
                "documentation": crate_info.module_docs,
            }))
        }
    }
}

fn find_module_docs(
    modules: &[crate::parser::index::ModuleInfo],
    path: &str,
) -> Option<(String, String)> {
    let parts: Vec<&str> = path.split("::").collect();
    find_docs_recursive(modules, &parts)
}

fn find_docs_recursive(
    modules: &[crate::parser::index::ModuleInfo],
    parts: &[&str],
) -> Option<(String, String)> {
    if parts.is_empty() {
        return None;
    }
    for module in modules {
        if module.name == parts[0] {
            if parts.len() == 1 {
                return Some((module.name.clone(), module.module_docs.clone()));
            }
            return find_docs_recursive(&module.submodules, &parts[1..]);
        }
    }
    None
}
