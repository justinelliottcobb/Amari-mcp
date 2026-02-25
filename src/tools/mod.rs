pub mod api_search;
pub mod browse_docs;
pub mod dependency_graph;
pub mod feature_map;
pub mod module_overview;
pub mod type_info;
pub mod usage_examples;

use crate::config::LibraryManifest;
use crate::parser::index::{ApiIndex, Validated};
use std::sync::Arc;

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
