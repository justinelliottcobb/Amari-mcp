pub mod display;
pub mod docs;
pub mod features;
pub mod index;
pub mod items;
pub mod module_tree;
pub mod workspace;

use crate::config::LibraryManifest;
use anyhow::Result;
use index::{ApiIndex, CrateInfo, Unvalidated};
use rayon::prelude::*;
use std::path::Path;

/// Build an API index from the library described by the manifest.
///
/// `manifest_path` is the path to the TOML manifest file, used to resolve
/// the relative `source_path` within it. Discovers workspace crates, then
/// parses each crate in parallel via rayon to extract the full public API surface.
pub fn build_index(
    manifest: &LibraryManifest,
    manifest_path: &Path,
) -> Result<ApiIndex<Unvalidated>> {
    let manifest_path =
        std::fs::canonicalize(manifest_path).unwrap_or_else(|_| manifest_path.to_path_buf());
    let source_root = manifest.resolve_source_path(&manifest_path);

    let resolved_crates = manifest.all_user_facing_crates();

    let crate_results: Vec<Result<CrateInfo>> = resolved_crates
        .par_iter()
        .map(|resolved| {
            let crate_dir = source_root.join(&resolved.dir_name);
            if !crate_dir.exists() {
                anyhow::bail!("Crate directory not found: {}", crate_dir.display());
            }

            let lib_path = crate_dir.join("src/lib.rs");
            let module_docs = if lib_path.exists() {
                let content = std::fs::read_to_string(&lib_path)?;
                docs::extract_module_docs(&content)
            } else {
                String::new()
            };

            let modules = module_tree::walk_crate(&crate_dir)?;

            Ok(CrateInfo {
                name: resolved.dir_name.clone(),
                alias: resolved.alias.clone(),
                feature_gate: resolved.feature_gate.clone(),
                source_dir: crate_dir,
                modules,
                module_docs,
            })
        })
        .collect();

    let mut crates = Vec::new();
    let mut errors = Vec::new();
    for result in crate_results {
        match result {
            Ok(info) => crates.push(info),
            Err(e) => errors.push(format!("{e}")),
        }
    }

    let mut items_by_name = std::collections::HashMap::new();
    for crate_info in &crates {
        index::collect_items_from_modules(&crate_info.modules, &mut items_by_name);
    }

    Ok(ApiIndex::new(
        manifest.library.name.clone(),
        crates,
        items_by_name,
        errors,
    ))
}

/// Build an index from a manifest file path.
pub fn build_index_from_path(manifest_path: &Path) -> Result<ApiIndex<Unvalidated>> {
    let manifest = LibraryManifest::load(manifest_path)?;
    build_index(&manifest, manifest_path)
}
