use super::SharedState;
use crate::parser::display;
use crate::parser::index::ModuleInfo;
use async_trait::async_trait;
use pmcp::{Error as McpError, RequestHandlerExtra, ToolHandler, ToolInfo};
use serde_json::{json, Value};
use std::sync::Arc;

pub struct ModuleOverviewHandler {
    pub state: Arc<SharedState>,
}

#[async_trait]
impl ToolHandler for ModuleOverviewHandler {
    fn metadata(&self) -> Option<ToolInfo> {
        Some(ToolInfo {
            name: "module_overview".to_string(),
            description: Some(
                "List all public items in a crate or module with brief descriptions".to_string(),
            ),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "crate": {
                        "type": "string",
                        "description": "Crate name or alias (e.g. 'amari-core' or 'core')"
                    },
                    "module": {
                        "type": "string",
                        "description": "Module path within the crate (e.g. 'rotor'). Omit for crate root."
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum items to return (default 50)"
                    }
                },
                "required": ["crate"]
            }),
        })
    }

    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        let crate_name = args["crate"]
            .as_str()
            .ok_or_else(|| McpError::invalid_params("crate is required"))?;

        let module_path = args.get("module").and_then(|v| v.as_str());
        let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(50) as usize;

        let crate_info = self.state.index.get_crate(crate_name);
        let Some(crate_info) = crate_info else {
            let available: Vec<_> = self
                .state
                .index
                .crates
                .iter()
                .map(|c| {
                    if let Some(ref alias) = c.alias {
                        format!("{} ({})", c.name, alias)
                    } else {
                        c.name.clone()
                    }
                })
                .collect();
            return Ok(json!({
                "error": format!("Crate '{crate_name}' not found"),
                "available_crates": available,
            }));
        };

        let target_modules: Vec<&ModuleInfo> = if let Some(path) = module_path {
            find_module(&crate_info.modules, path)
                .map(|m| vec![m])
                .unwrap_or_default()
        } else {
            crate_info.modules.iter().collect()
        };

        if target_modules.is_empty() {
            return Ok(
                json!({"error": format!("Module '{module_path:?}' not found in crate '{crate_name}'")}),
            );
        }

        let module = target_modules[0];
        let items: Vec<Value> = module
            .items
            .iter()
            .take(limit)
            .map(|item| {
                json!({
                    "kind": super::api_search::kind_label(&item.kind),
                    "name": item.name,
                    "signature": item.signature,
                    "doc_summary": display::first_sentence(&item.doc_comment),
                })
            })
            .collect();

        let submodules: Vec<Value> = module
            .submodules
            .iter()
            .map(|sub| {
                json!({
                    "name": sub.name,
                    "item_count": sub.items.len(),
                    "feature_gate": sub.feature_gate,
                })
            })
            .collect();

        Ok(json!({
            "crate": crate_name,
            "module": module.name,
            "module_docs": module.module_docs,
            "feature_gate": crate_info.feature_gate,
            "items": items,
            "submodules": submodules,
        }))
    }
}

fn find_module<'a>(modules: &'a [ModuleInfo], path: &str) -> Option<&'a ModuleInfo> {
    let parts: Vec<&str> = path.split("::").collect();
    find_module_recursive(modules, &parts)
}

fn find_module_recursive<'a>(modules: &'a [ModuleInfo], parts: &[&str]) -> Option<&'a ModuleInfo> {
    if parts.is_empty() {
        return None;
    }
    for module in modules {
        if module.name == parts[0] {
            if parts.len() == 1 {
                return Some(module);
            }
            return find_module_recursive(&module.submodules, &parts[1..]);
        }
    }
    None
}
