pub mod api_search;
pub mod browse_docs;
pub mod dependency_graph;
pub mod feature_map;
pub mod module_overview;
pub mod reload;
pub mod type_info;
pub mod usage_examples;

use crate::config::LibraryManifest;
use crate::parser::index::{ApiIndex, Validated};
use pmcp::ToolInfo;
use serde_json::Value;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

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
///
/// The index and manifest are wrapped in `RwLock` so the `reload` tool
/// can swap them at runtime without restarting the server process.
pub struct SharedState {
    pub index: RwLock<ApiIndex<Validated>>,
    pub manifest: RwLock<LibraryManifest>,
    pub manifest_path: PathBuf,
}

impl SharedState {
    pub fn new(
        index: ApiIndex<Validated>,
        manifest: LibraryManifest,
        manifest_path: PathBuf,
    ) -> Arc<Self> {
        Arc::new(Self {
            index: RwLock::new(index),
            manifest: RwLock::new(manifest),
            manifest_path,
        })
    }

    /// Re-parse the library source and swap in a fresh index.
    pub fn reload(&self) -> Result<ReloadReport, String> {
        let manifest = LibraryManifest::load(&self.manifest_path)
            .map_err(|e| format!("Failed to load manifest: {e}"))?;

        let unvalidated = crate::parser::build_index(&manifest, &self.manifest_path)
            .map_err(|e| format!("Failed to build index: {e}"))?;

        let old_stats = {
            let guard = self
                .index
                .read()
                .map_err(|e| format!("Lock poisoned: {e}"))?;
            guard.stats()
        };

        let validated = unvalidated
            .validate()
            .map_err(|report| format!("Validation failed: {:?}", report.errors))?;

        let new_stats = validated.stats();

        {
            let mut guard = self
                .index
                .write()
                .map_err(|e| format!("Lock poisoned: {e}"))?;
            *guard = validated;
        }
        {
            let mut guard = self
                .manifest
                .write()
                .map_err(|e| format!("Lock poisoned: {e}"))?;
            *guard = manifest;
        }

        Ok(ReloadReport {
            old_crates: old_stats.crate_count,
            old_modules: old_stats.module_count,
            old_items: old_stats.item_count,
            new_crates: new_stats.crate_count,
            new_modules: new_stats.module_count,
            new_items: new_stats.item_count,
        })
    }
}

pub struct ReloadReport {
    pub old_crates: usize,
    pub old_modules: usize,
    pub old_items: usize,
    pub new_crates: usize,
    pub new_modules: usize,
    pub new_items: usize,
}
