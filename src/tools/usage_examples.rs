use super::SharedState;
use async_trait::async_trait;
use pmcp::{Error as McpError, RequestHandlerExtra, ToolHandler, ToolInfo};
use serde_json::{json, Value};
use std::sync::Arc;

pub struct UsageExamplesHandler {
    pub state: Arc<SharedState>,
}

#[async_trait]
impl ToolHandler for UsageExamplesHandler {
    fn metadata(&self) -> Option<ToolInfo> {
        Some(ToolInfo {
            name: "usage_examples".to_string(),
            description: Some(
                "Extract code examples from doc comments for a type or function".to_string(),
            ),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Type or function name to find examples for"
                    }
                },
                "required": ["name"]
            }),
        })
    }

    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> Result<Value, McpError> {
        let name = args["name"]
            .as_str()
            .ok_or_else(|| McpError::invalid_params("name is required"))?;

        let matching: Vec<_> = self
            .state
            .index
            .search(name)
            .into_iter()
            .filter(|i| i.name == name || i.full_path.ends_with(name))
            .collect();

        if matching.is_empty() {
            return Ok(json!({"error": format!("No items found matching '{name}'")}));
        }

        let mut examples = Vec::new();
        for item in &matching {
            let code_blocks = extract_code_blocks(&item.doc_comment);
            if !code_blocks.is_empty() {
                examples.push(json!({
                    "item": item.full_path,
                    "source_file": item.source_file.display().to_string(),
                    "examples": code_blocks,
                }));
            }
        }

        Ok(json!({
            "name": name,
            "doc_examples": examples,
        }))
    }
}

fn extract_code_blocks(doc: &str) -> Vec<String> {
    let mut blocks = Vec::new();
    let mut in_block = false;
    let mut current_block = Vec::new();

    for line in doc.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            if in_block {
                blocks.push(current_block.join("\n"));
                current_block.clear();
                in_block = false;
            } else {
                in_block = true;
            }
        } else if in_block {
            current_block.push(line.to_string());
        }
    }

    blocks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_code_blocks_from_doc() {
        let doc = "Example:\n\n```rust\nlet mv = Multivector::new(vec![1.0]);\n```\n\nAnother:\n\n```\nlet x = 42;\n```\n";
        let blocks = extract_code_blocks(doc);
        assert_eq!(blocks.len(), 2);
        assert!(blocks[0].contains("Multivector::new"));
        assert!(blocks[1].contains("let x = 42"));
    }

    #[test]
    fn no_code_blocks_returns_empty() {
        assert!(extract_code_blocks("Just a plain doc.").is_empty());
    }
}
