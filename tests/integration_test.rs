//! Integration tests against the actual Amari library source.
//!
//! These tests are ignored by default and only run when the Amari
//! source is available at the expected location (../../amari relative
//! to the manifest).

use std::path::Path;

const MANIFEST_PATH: &str = "manifests/amari.toml";

fn amari_source_available() -> bool {
    // The manifest resolves source_path relative to itself.
    // manifests/amari.toml says source_path = "../../amari"
    // so from the manifest dir: manifests/../../amari = ../amari
    // But from the repo root (where tests run): ../../amari
    let manifest_path = Path::new(MANIFEST_PATH);
    if !manifest_path.exists() {
        return false;
    }
    let manifest = amari_mcp::config::LibraryManifest::load(manifest_path);
    match manifest {
        Ok(m) => {
            let source = m.resolve_source_path(manifest_path);
            source.join("Cargo.toml").exists()
        }
        Err(_) => false,
    }
}

#[test]
fn manifest_loads_correctly() {
    let manifest = amari_mcp::config::LibraryManifest::load(Path::new(MANIFEST_PATH))
        .expect("Failed to load amari manifest");
    assert_eq!(manifest.library.name, "amari");
    assert_eq!(manifest.library.display_name, "Amari");

    let crates = manifest.all_user_facing_crates();
    assert!(
        crates.len() >= 19,
        "Expected at least 19 user-facing crates, got {}",
        crates.len()
    );
}

#[test]
fn full_index_build_and_validate() {
    if !amari_source_available() {
        eprintln!("Skipping: Amari source not available");
        return;
    }

    let manifest = amari_mcp::config::LibraryManifest::load(Path::new(MANIFEST_PATH))
        .expect("Failed to load manifest");

    let index = amari_mcp::parser::build_index(&manifest, Path::new(MANIFEST_PATH))
        .expect("Failed to build index");

    let parse_errors = index.parse_errors.len();
    let validated = index.validate().expect("Validation failed");
    let stats = validated.stats();

    eprintln!("Index stats: {stats:#?}",);
    eprintln!(
        "Parsed {} crates, {} modules, {} items ({} parse warnings)",
        stats.crate_count, stats.module_count, stats.item_count, parse_errors
    );

    // We expect all 19 user-facing crates to be discovered
    assert!(
        stats.crate_count >= 19,
        "Expected at least 19 crates, got {}",
        stats.crate_count
    );

    // At minimum we should find a non-trivial number of items
    assert!(
        stats.item_count > 100,
        "Expected >100 items, got {}",
        stats.item_count
    );
}

#[test]
fn multivector_found_with_const_generics() {
    if !amari_source_available() {
        eprintln!("Skipping: Amari source not available");
        return;
    }

    let manifest = amari_mcp::config::LibraryManifest::load(Path::new(MANIFEST_PATH))
        .expect("Failed to load manifest");

    let index = amari_mcp::parser::build_index(&manifest, Path::new(MANIFEST_PATH))
        .expect("Failed to build index");
    let validated = index.validate().expect("Validation failed");

    let results = validated.search("Multivector");
    assert!(
        !results.is_empty(),
        "Multivector should be found in the index"
    );

    // Find the struct definition (not impl methods)
    let mv_struct = results
        .iter()
        .find(|item| {
            item.name == "Multivector"
                && matches!(item.kind, amari_mcp::parser::index::ItemKind::Struct { .. })
        })
        .expect("Multivector struct should exist");

    assert!(
        mv_struct.full_path.contains("core"),
        "Multivector should be in the core crate, got: {}",
        mv_struct.full_path
    );

    // Verify const generics are captured in the signature
    assert!(
        mv_struct.signature.contains("const")
            || mv_struct
                .generics
                .as_ref()
                .is_some_and(|g| g.contains("const")),
        "Multivector should have const generic parameters"
    );
}

#[test]
fn feature_gated_crates_detected() {
    if !amari_source_available() {
        eprintln!("Skipping: Amari source not available");
        return;
    }

    let manifest = amari_mcp::config::LibraryManifest::load(Path::new(MANIFEST_PATH))
        .expect("Failed to load manifest");

    let index = amari_mcp::parser::build_index(&manifest, Path::new(MANIFEST_PATH))
        .expect("Failed to build index");
    let validated = index.validate().expect("Validation failed");

    // Check that feature-gated crates have their feature gate set
    let measure_crate = validated.get_crate("measure");
    if let Some(measure) = measure_crate {
        assert!(
            measure.feature_gate.is_some(),
            "amari-measure should have a feature gate"
        );
    }

    // Check that default crates do NOT have feature gates
    let core_crate = validated
        .get_crate("core")
        .or_else(|| validated.get_crate("amari-core"));
    if let Some(core) = core_crate {
        assert!(
            core.feature_gate.is_none(),
            "amari-core should not have a feature gate"
        );
    }
}

#[test]
fn module_docs_extracted() {
    if !amari_source_available() {
        eprintln!("Skipping: Amari source not available");
        return;
    }

    let manifest = amari_mcp::config::LibraryManifest::load(Path::new(MANIFEST_PATH))
        .expect("Failed to load manifest");

    let index = amari_mcp::parser::build_index(&manifest, Path::new(MANIFEST_PATH))
        .expect("Failed to build index");
    let validated = index.validate().expect("Validation failed");

    // At least one crate should have module-level documentation
    let has_docs = validated.crates.iter().any(|c| !c.module_docs.is_empty());
    assert!(
        has_docs,
        "At least one crate should have module-level documentation"
    );
}

#[test]
fn check_mode_runs_successfully() {
    if !amari_source_available() {
        eprintln!("Skipping: Amari source not available");
        return;
    }

    // Simulate what the `check` subcommand does
    let manifest = amari_mcp::config::LibraryManifest::load(Path::new(MANIFEST_PATH))
        .expect("Failed to load manifest");

    let index = amari_mcp::parser::build_index(&manifest, Path::new(MANIFEST_PATH))
        .expect("build_index should succeed");
    let validated = index.validate().expect("validate should succeed");
    let stats = validated.stats();

    // Print summary (visible in test output with --nocapture)
    eprintln!("=== Check Mode Summary ===");
    eprintln!("Library: {}", validated.library_name);
    eprintln!(
        "Crates: {}, Modules: {}, Items: {}",
        stats.crate_count, stats.module_count, stats.item_count
    );
    for c in &validated.crates {
        let fg = c
            .feature_gate
            .as_ref()
            .map(|f| format!(" [feature: {f}]"))
            .unwrap_or_default();
        eprintln!("  {}{fg}", c.name);
    }
}
