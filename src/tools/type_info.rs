use super::SharedState;
use crate::parser::index::{FieldKind, ItemKind};
use async_trait::async_trait;
use pmcp::{Error as McpError, RequestHandlerExtra, ToolHandler};
use serde_json::{json, Value};
use std::sync::Arc;

pub struct TypeInfoHandler {
    pub state: Arc<SharedState>,
}

#[async_trait]
impl ToolHandler for TypeInfoHandler {
    fn metadata(&self) -> Option<pmcp::ToolInfo> {
        Some(super::tool_info(
            "type_info",
            "Get full details on a specific type including signature, fields, methods, trait impls, and documentation",
            json!({
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Type name (e.g. 'Multivector' or 'amari::core::Multivector')"
                    }
                },
                "required": ["name"]
            }),
        ))
    }

    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        let name = args["name"]
            .as_str()
            .ok_or_else(|| McpError::invalid_params("name is required"))?;

        let index = self
            .state
            .index
            .read()
            .map_err(|_| McpError::internal("index lock poisoned"))?;
        let type_items: Vec<_> = index
            .search(name)
            .into_iter()
            .filter(|item| item.name == name || item.full_path.ends_with(name))
            .filter(|item| {
                matches!(
                    item.kind,
                    ItemKind::Struct { .. }
                        | ItemKind::Enum { .. }
                        | ItemKind::Trait { .. }
                        | ItemKind::TypeAlias
                )
            })
            .collect();

        if type_items.is_empty() {
            return Ok(json!({"error": format!("Type '{name}' not found")}));
        }

        let primary = type_items[0];

        let methods: Vec<Value> = index
            .search(&primary.name)
            .into_iter()
            .filter(|item| {
                if let ItemKind::Impl { ref self_type, .. } = item.kind {
                    self_type.contains(&primary.name)
                } else {
                    false
                }
            })
            .map(|item| {
                let trait_name = if let ItemKind::Impl { ref trait_name, .. } = item.kind {
                    trait_name.clone()
                } else {
                    None
                };
                json!({
                    "name": item.name,
                    "signature": item.signature,
                    "doc_summary": crate::parser::display::first_sentence(&item.doc_comment),
                    "trait_impl": trait_name,
                })
            })
            .collect();

        let fields_json = match &primary.kind {
            ItemKind::Struct { fields } => Some(render_fields(fields)),
            _ => None,
        };

        let variants_json: Option<Vec<Value>> = match &primary.kind {
            ItemKind::Enum { variants } => Some(
                variants
                    .iter()
                    .map(|v| {
                        json!({
                            "name": v.name,
                            "fields": render_fields(&v.fields),
                            "doc": v.doc_comment,
                        })
                    })
                    .collect(),
            ),
            _ => None,
        };

        Ok(json!({
            "name": primary.name,
            "kind": match &primary.kind {
                ItemKind::Struct { .. } => "struct",
                ItemKind::Enum { .. } => "enum",
                ItemKind::Trait { .. } => "trait",
                ItemKind::TypeAlias => "type_alias",
                _ => "other",
            },
            "full_path": primary.full_path,
            "signature": primary.signature,
            "doc_comment": primary.doc_comment,
            "generics": primary.generics,
            "feature_gate": primary.feature_gate,
            "fields": fields_json,
            "variants": variants_json,
            "methods": methods,
            "source_file": primary.source_file.display().to_string(),
            "line": primary.line_number,
        }))
    }
}

fn render_fields(fields: &FieldKind) -> Value {
    match fields {
        FieldKind::Named(fields) => {
            json!(fields
                .iter()
                .map(|f| { json!({"name": f.name, "type": f.ty, "doc": f.doc_comment}) })
                .collect::<Vec<_>>())
        }
        FieldKind::Tuple(types) => json!(types),
        FieldKind::Unit => json!(null),
    }
}
