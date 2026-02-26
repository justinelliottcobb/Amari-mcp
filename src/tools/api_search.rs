use super::SharedState;
use crate::parser::display;
use crate::parser::index::ItemKind;
use async_trait::async_trait;
use pmcp::{Error as McpError, RequestHandlerExtra, ToolHandler};
use serde_json::{json, Value};
use std::sync::Arc;

pub struct ApiSearchHandler {
    pub state: Arc<SharedState>,
}

#[async_trait]
impl ToolHandler for ApiSearchHandler {
    fn metadata(&self) -> Option<pmcp::ToolInfo> {
        Some(super::tool_info(
            "api_search",
            "Search the library API for types, functions, traits, and more by name",
            json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Name or substring to search for"
                    },
                    "kind": {
                        "type": "string",
                        "description": "Filter by item kind",
                        "enum": ["function", "struct", "enum", "trait", "type", "const", "impl", "reexport"]
                    },
                    "crate": {
                        "type": "string",
                        "description": "Limit search to a specific crate (name or alias)"
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of results (default 20)"
                    }
                },
                "required": ["query"]
            }),
        ))
    }

    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        let query = args["query"]
            .as_str()
            .ok_or_else(|| McpError::invalid_params("query is required"))?;

        let kind_filter = args.get("kind").and_then(|v| v.as_str());
        let crate_filter = args.get("crate").and_then(|v| v.as_str());
        let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(20) as usize;

        let results: Vec<Value> = self
            .state
            .index
            .search(query)
            .into_iter()
            .filter(|item| kind_filter.is_none_or(|kind| matches_kind(&item.kind, kind)))
            .filter(|item| {
                crate_filter.is_none_or(|crate_name| item.full_path.contains(crate_name))
            })
            .take(limit)
            .map(|item| {
                json!({
                    "name": item.name,
                    "kind": kind_label(&item.kind),
                    "full_path": item.full_path,
                    "signature": item.signature,
                    "doc_summary": display::first_sentence(&item.doc_comment),
                    "feature_gate": item.feature_gate,
                    "source_file": item.source_file.display().to_string(),
                    "line": item.line_number,
                })
            })
            .collect();

        let total = results.len();
        Ok(json!({
            "results": results,
            "total_matches": total,
            "query": query,
        }))
    }
}

fn matches_kind(kind: &ItemKind, filter: &str) -> bool {
    matches!(
        (kind, filter),
        (ItemKind::Function { .. }, "function")
            | (ItemKind::Struct { .. }, "struct")
            | (ItemKind::Enum { .. }, "enum")
            | (ItemKind::Trait { .. }, "trait")
            | (ItemKind::TypeAlias, "type")
            | (ItemKind::Const { .. }, "const")
            | (ItemKind::Impl { .. }, "impl")
            | (ItemKind::ReExport { .. }, "reexport")
    )
}

pub fn kind_label(kind: &ItemKind) -> &'static str {
    match kind {
        ItemKind::Function { .. } => "function",
        ItemKind::Struct { .. } => "struct",
        ItemKind::Enum { .. } => "enum",
        ItemKind::Trait { .. } => "trait",
        ItemKind::TypeAlias => "type",
        ItemKind::Const { .. } => "const",
        ItemKind::Impl { .. } => "impl",
        ItemKind::ReExport { .. } => "reexport",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kind_filter_matches_correctly() {
        assert!(matches_kind(
            &ItemKind::Function {
                is_async: false,
                is_unsafe: false
            },
            "function"
        ));
        assert!(!matches_kind(
            &ItemKind::Function {
                is_async: false,
                is_unsafe: false
            },
            "struct"
        ));
        assert!(matches_kind(
            &ItemKind::Struct {
                fields: crate::parser::index::FieldKind::Unit
            },
            "struct"
        ));
    }
}
