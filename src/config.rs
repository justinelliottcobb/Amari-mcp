use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Top-level library manifest loaded from a TOML file.
#[derive(Debug, Deserialize)]
pub struct LibraryManifest {
    pub library: LibraryInfo,
    pub workspace: WorkspaceInfo,
    pub crates: CrateGroups,
    pub aliases: HashMap<String, String>,
}

/// Metadata about the target library.
#[derive(Debug, Deserialize)]
pub struct LibraryInfo {
    pub name: String,
    pub display_name: String,
    pub version: String,
    pub description: String,
    pub repository: Option<String>,
    pub docs_url: Option<String>,
    pub source_path: String,
}

/// Workspace layout information.
#[derive(Debug, Deserialize)]
pub struct WorkspaceInfo {
    pub root_cargo_toml: String,
    pub umbrella_crate: String,
}

/// Categorized workspace crate groups.
#[derive(Debug, Deserialize)]
pub struct CrateGroups {
    pub default: CrateList,
    pub optional: HashMap<String, String>,
    pub internal: Option<CrateList>,
}

/// A list of crate directory names.
#[derive(Debug, Deserialize)]
pub struct CrateList {
    pub members: Vec<String>,
}

/// A resolved crate entry with its feature gate and alias.
#[derive(Debug, Clone)]
pub struct ResolvedCrate {
    pub dir_name: String,
    pub alias: Option<String>,
    pub feature_gate: Option<String>,
}

impl LibraryManifest {
    /// Load and parse a manifest from a TOML file.
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read manifest at {}", path.display()))?;
        let manifest: Self = toml::from_str(&content)
            .with_context(|| format!("Failed to parse manifest at {}", path.display()))?;
        Ok(manifest)
    }

    /// Resolve the library source path relative to the manifest file's directory.
    pub fn resolve_source_path(&self, manifest_path: &Path) -> PathBuf {
        let manifest_dir = manifest_path.parent().unwrap_or_else(|| Path::new("."));
        manifest_dir.join(&self.library.source_path)
    }

    /// Get all user-facing crates (default + optional), with aliases and feature gates.
    pub fn all_user_facing_crates(&self) -> Vec<ResolvedCrate> {
        let mut crates: Vec<ResolvedCrate> = self
            .crates
            .default
            .members
            .iter()
            .map(|dir_name| ResolvedCrate {
                dir_name: dir_name.clone(),
                alias: self.aliases.get(dir_name).cloned(),
                feature_gate: None,
            })
            .collect();

        for (feature, dir_name) in &self.crates.optional {
            crates.push(ResolvedCrate {
                dir_name: dir_name.clone(),
                alias: self.aliases.get(dir_name).cloned(),
                feature_gate: Some(feature.clone()),
            });
        }

        crates
    }

    /// Get internal crates (proc-macros, wasm bindings, etc.).
    pub fn internal_crates(&self) -> Vec<&str> {
        self.crates
            .internal
            .as_ref()
            .map(|list| list.members.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }

    /// Look up the alias for a crate directory name.
    pub fn alias_for(&self, dir_name: &str) -> Option<&str> {
        self.aliases.get(dir_name).map(|s| s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_toml() -> &'static str {
        r#"
[library]
name = "testlib"
display_name = "Test Library"
version = "1.0.0"
description = "A test library"
source_path = "../testlib"

[workspace]
root_cargo_toml = "Cargo.toml"
umbrella_crate = "src/lib.rs"

[crates.default]
members = ["testlib-core", "testlib-utils"]

[crates.optional]
gpu = "testlib-gpu"
extra = "testlib-extra"

[crates.internal]
members = ["testlib-macros"]

[aliases]
testlib-core = "core"
testlib-utils = "utils"
testlib-gpu = "gpu"
testlib-extra = "extra"
"#
    }

    fn parse_sample() -> LibraryManifest {
        toml::from_str(sample_toml()).expect("Failed to parse sample TOML")
    }

    #[test]
    fn parses_library_info() {
        let manifest = parse_sample();
        assert_eq!(manifest.library.name, "testlib");
        assert_eq!(manifest.library.display_name, "Test Library");
        assert_eq!(manifest.library.version, "1.0.0");
        assert_eq!(manifest.library.source_path, "../testlib");
        assert!(manifest.library.repository.is_none());
        assert!(manifest.library.docs_url.is_none());
    }

    #[test]
    fn parses_workspace_info() {
        let manifest = parse_sample();
        assert_eq!(manifest.workspace.root_cargo_toml, "Cargo.toml");
        assert_eq!(manifest.workspace.umbrella_crate, "src/lib.rs");
    }

    #[test]
    fn parses_default_crates() {
        let manifest = parse_sample();
        assert_eq!(
            manifest.crates.default.members,
            vec!["testlib-core", "testlib-utils"]
        );
    }

    #[test]
    fn parses_optional_crates() {
        let manifest = parse_sample();
        assert_eq!(manifest.crates.optional.get("gpu").unwrap(), "testlib-gpu");
        assert_eq!(
            manifest.crates.optional.get("extra").unwrap(),
            "testlib-extra"
        );
    }

    #[test]
    fn parses_internal_crates() {
        let manifest = parse_sample();
        let internal = manifest.crates.internal.as_ref().unwrap();
        assert_eq!(internal.members, vec!["testlib-macros"]);
    }

    #[test]
    fn parses_aliases() {
        let manifest = parse_sample();
        assert_eq!(manifest.aliases.get("testlib-core").unwrap(), "core");
        assert_eq!(manifest.aliases.get("testlib-gpu").unwrap(), "gpu");
    }

    #[test]
    fn resolves_source_path_relative_to_manifest() {
        let manifest = parse_sample();
        let manifest_path = Path::new("/home/user/project/manifests/lib.toml");
        let resolved = manifest.resolve_source_path(manifest_path);
        assert_eq!(
            resolved,
            PathBuf::from("/home/user/project/manifests/../testlib")
        );
    }

    #[test]
    fn all_user_facing_crates_includes_default_and_optional() {
        let manifest = parse_sample();
        let crates = manifest.all_user_facing_crates();

        // 2 default + 2 optional = 4
        assert_eq!(crates.len(), 4);

        // Default crates have no feature gate
        let core = crates
            .iter()
            .find(|c| c.dir_name == "testlib-core")
            .unwrap();
        assert_eq!(core.alias.as_deref(), Some("core"));
        assert!(core.feature_gate.is_none());

        // Optional crates have a feature gate
        let gpu = crates.iter().find(|c| c.dir_name == "testlib-gpu").unwrap();
        assert_eq!(gpu.alias.as_deref(), Some("gpu"));
        assert_eq!(gpu.feature_gate.as_deref(), Some("gpu"));
    }

    #[test]
    fn internal_crates_returns_internal_members() {
        let manifest = parse_sample();
        let internal = manifest.internal_crates();
        assert_eq!(internal, vec!["testlib-macros"]);
    }

    #[test]
    fn alias_for_returns_correct_alias() {
        let manifest = parse_sample();
        assert_eq!(manifest.alias_for("testlib-core"), Some("core"));
        assert_eq!(manifest.alias_for("nonexistent"), None);
    }

    #[test]
    fn optional_fields_can_be_omitted() {
        let toml_str = r#"
[library]
name = "minimal"
display_name = "Minimal"
version = "0.1.0"
description = "Minimal manifest"
source_path = "."

[workspace]
root_cargo_toml = "Cargo.toml"
umbrella_crate = "src/lib.rs"

[crates.default]
members = []

[crates.optional]

[aliases]
"#;
        let manifest: LibraryManifest = toml::from_str(toml_str).unwrap();
        assert!(manifest.crates.internal.is_none());
        assert!(manifest.library.repository.is_none());
        assert!(manifest.crates.optional.is_empty());
    }

    #[test]
    fn load_from_file_works() {
        // Test loading the actual amari manifest
        let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("manifests/amari.toml");
        if manifest_path.exists() {
            let manifest = LibraryManifest::load(&manifest_path).unwrap();
            assert_eq!(manifest.library.name, "amari");
            assert_eq!(manifest.crates.default.members.len(), 9);
            assert_eq!(manifest.crates.optional.len(), 10);
            assert_eq!(manifest.aliases.len(), 19);
        }
    }
}
