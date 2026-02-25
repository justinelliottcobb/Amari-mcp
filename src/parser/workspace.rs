use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::Path;

/// Parsed dependency information from a workspace member's Cargo.toml.
#[derive(Debug)]
pub struct CrateDeps {
    pub name: String,
    pub sibling_deps: Vec<String>,
}

/// Read a crate's Cargo.toml and extract its name and sibling dependencies.
pub fn parse_crate_cargo_toml(crate_dir: &Path) -> Result<CrateDeps> {
    let cargo_path = crate_dir.join("Cargo.toml");
    let content = std::fs::read_to_string(&cargo_path)
        .with_context(|| format!("Failed to read {}", cargo_path.display()))?;

    let parsed: toml::Value = content
        .parse()
        .with_context(|| format!("Failed to parse {}", cargo_path.display()))?;

    let name = parsed
        .get("package")
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .unwrap_or("unknown")
        .to_string();

    let mut sibling_deps = Vec::new();

    if let Some(deps) = parsed.get("dependencies") {
        if let Some(table) = deps.as_table() {
            for (dep_name, dep_value) in table {
                // Sibling deps use workspace = true or path = "../..."
                let is_sibling = match dep_value {
                    toml::Value::Table(t) => {
                        t.get("workspace")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false)
                            || t.get("path").is_some()
                    }
                    _ => false,
                };
                if is_sibling {
                    sibling_deps.push(dep_name.clone());
                }
            }
        }
    }

    Ok(CrateDeps { name, sibling_deps })
}

/// Build a dependency graph for all crates in the workspace.
/// Returns a map from crate name to its sibling dependencies.
pub fn build_dependency_graph(crate_dirs: &[(String, &Path)]) -> HashMap<String, Vec<String>> {
    let mut graph = HashMap::new();

    for (name, dir) in crate_dirs {
        match parse_crate_cargo_toml(dir) {
            Ok(deps) => {
                graph.insert(name.clone(), deps.sibling_deps);
            }
            Err(e) => {
                tracing::warn!("Failed to parse Cargo.toml for {name}: {e}");
                graph.insert(name.clone(), Vec::new());
            }
        }
    }

    graph
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn parses_simple_cargo_toml() {
        let tmp = TempDir::new().unwrap();
        let cargo_content = r#"
[package]
name = "my-crate"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"
sibling-crate = { workspace = true }
local-dep = { path = "../local" }
"#;
        fs::write(tmp.path().join("Cargo.toml"), cargo_content).unwrap();

        let deps = parse_crate_cargo_toml(tmp.path()).unwrap();
        assert_eq!(deps.name, "my-crate");
        assert!(deps.sibling_deps.contains(&"sibling-crate".to_string()));
        assert!(deps.sibling_deps.contains(&"local-dep".to_string()));
        assert!(!deps.sibling_deps.contains(&"serde".to_string()));
    }

    #[test]
    fn handles_no_dependencies() {
        let tmp = TempDir::new().unwrap();
        let cargo_content = r#"
[package]
name = "no-deps"
version = "0.1.0"
"#;
        fs::write(tmp.path().join("Cargo.toml"), cargo_content).unwrap();

        let deps = parse_crate_cargo_toml(tmp.path()).unwrap();
        assert_eq!(deps.name, "no-deps");
        assert!(deps.sibling_deps.is_empty());
    }

    #[test]
    fn builds_dependency_graph() {
        let tmp = TempDir::new().unwrap();

        let crate_a = tmp.path().join("crate-a");
        let crate_b = tmp.path().join("crate-b");
        fs::create_dir_all(&crate_a).unwrap();
        fs::create_dir_all(&crate_b).unwrap();

        fs::write(
            crate_a.join("Cargo.toml"),
            r#"
[package]
name = "crate-a"
version = "0.1.0"
"#,
        )
        .unwrap();

        fs::write(
            crate_b.join("Cargo.toml"),
            r#"
[package]
name = "crate-b"
version = "0.1.0"

[dependencies]
crate-a = { workspace = true }
"#,
        )
        .unwrap();

        let dirs: Vec<(String, &Path)> = vec![
            ("crate-a".to_string(), crate_a.as_path()),
            ("crate-b".to_string(), crate_b.as_path()),
        ];

        let graph = build_dependency_graph(&dirs);
        assert!(graph["crate-a"].is_empty());
        assert!(graph["crate-b"].contains(&"crate-a".to_string()));
    }
}
