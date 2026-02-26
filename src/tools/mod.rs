pub mod api_search;
pub mod browse_docs;
pub mod dependency_graph;
pub mod feature_map;
pub mod module_overview;
pub mod type_info;
pub mod usage_examples;

use crate::config::LibraryManifest;
use crate::parser::index::{ApiIndex, Validated};
use pmcp::ToolInfo;
use serde_json::Value;
use std::sync::Arc;

/// Construct a ToolInfo. pmcp marks ToolInfo as #[non_exhaustive], so we
/// can't use struct literal syntax. This helper builds one from Default.
pub fn tool_info(name: &str, description: &str, input_schema: Value) -> ToolInfo {
    let mut info = ToolInfo::default();
    info.name = name.to_string();
    info.description = Some(description.to_string());
    info.input_schema = input_schema;
    info
}

/// Shared state passed to all tool handlers.
pub struct SharedState {
    pub index: ApiIndex<Validated>,
    pub manifest: LibraryManifest,
}

impl SharedState {
    pub fn new(index: ApiIndex<Validated>, manifest: LibraryManifest) -> Arc<Self> {
        Arc::new(Self { index, manifest })
    }
}
