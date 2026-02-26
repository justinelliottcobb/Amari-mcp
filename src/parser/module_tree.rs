use crate::parser::docs;
use crate::parser::features;
use crate::parser::index::ModuleInfo;
use crate::parser::items;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Walk a crate directory starting from `src/lib.rs` and build
/// the full module tree with extracted API items.
pub fn walk_crate(crate_dir: &Path) -> Result<Vec<ModuleInfo>> {
    let lib_path = crate_dir.join("src/lib.rs");
    if !lib_path.exists() {
        // Some crates might use src/main.rs or have no src/lib.rs
        return Ok(Vec::new());
    }

    let content = std::fs::read_to_string(&lib_path)
        .with_context(|| format!("Failed to read {}", lib_path.display()))?;

    let file = syn::parse_file(&content)
        .with_context(|| format!("Failed to parse {}", lib_path.display()))?;

    let crate_name = crate_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    // Extract items from lib.rs itself
    let lib_items = items::extract_items(&file, &lib_path, crate_name, None);

    // Discover and recurse into submodules
    let src_dir = crate_dir.join("src");
    let submodules = discover_submodules(&file, &src_dir, crate_name)?;

    // The root module contains lib.rs items plus all submodules
    let root = ModuleInfo {
        name: "lib".to_string(),
        path: lib_path,
        module_docs: docs::extract_module_docs(&content),
        items: lib_items,
        submodules,
        feature_gate: None,
    };

    Ok(vec![root])
}

/// Discover submodules declared in a parsed file and recursively walk them.
fn discover_submodules(
    file: &syn::File,
    src_dir: &Path,
    module_prefix: &str,
) -> Result<Vec<ModuleInfo>> {
    let mut modules = Vec::new();

    for item in &file.items {
        if let syn::Item::Mod(item_mod) = item {
            let mod_name = item_mod.ident.to_string();
            let feature_gate = features::extract_feature_gate(&item_mod.attrs);

            if let Some((_, ref items)) = item_mod.content {
                // Inline module: mod foo { ... }
                let inline_file = syn::File {
                    shebang: None,
                    attrs: item_mod.attrs.clone(),
                    items: items.clone(),
                };

                let prefix = format!("{module_prefix}::{mod_name}");
                let mod_items = items::extract_items(
                    &inline_file,
                    // Use parent path since inline modules don't have their own file
                    &src_dir.join("lib.rs"),
                    &prefix,
                    feature_gate.as_deref(),
                );

                let submodules = discover_submodules(&inline_file, src_dir, &prefix)?;

                modules.push(ModuleInfo {
                    name: mod_name,
                    path: src_dir.join("lib.rs"),
                    module_docs: String::new(),
                    items: mod_items,
                    submodules,
                    feature_gate,
                });
            } else {
                // External module: mod foo; — look for foo.rs or foo/mod.rs
                let mod_path = resolve_mod_path(src_dir, &mod_name, &item_mod.attrs);

                if let Some(path) = mod_path {
                    match parse_module_file(
                        &path,
                        module_prefix,
                        &mod_name,
                        feature_gate.as_deref(),
                    ) {
                        Ok(module_info) => modules.push(module_info),
                        Err(e) => {
                            tracing::warn!("Failed to parse module {mod_name}: {e}");
                        }
                    }
                }
            }
        }
    }

    Ok(modules)
}

/// Resolve the filesystem path for an external `mod foo;` declaration.
fn resolve_mod_path(src_dir: &Path, mod_name: &str, attrs: &[syn::Attribute]) -> Option<PathBuf> {
    // Check for #[path = "..."] attribute
    for attr in attrs {
        if attr.path().is_ident("path") {
            if let syn::Meta::NameValue(nv) = &attr.meta {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit_str),
                    ..
                }) = &nv.value
                {
                    let custom_path = src_dir.join(lit_str.value());
                    if custom_path.exists() {
                        return Some(custom_path);
                    }
                }
            }
        }
    }

    // Standard resolution: foo.rs or foo/mod.rs
    let file_path = src_dir.join(format!("{mod_name}.rs"));
    if file_path.exists() {
        return Some(file_path);
    }

    let dir_path = src_dir.join(mod_name).join("mod.rs");
    if dir_path.exists() {
        return Some(dir_path);
    }

    None
}

/// Parse a module file and recursively discover its submodules.
fn parse_module_file(
    path: &Path,
    parent_prefix: &str,
    mod_name: &str,
    feature_gate: Option<&str>,
) -> Result<ModuleInfo> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read module {}", path.display()))?;

    let file = syn::parse_file(&content)
        .with_context(|| format!("Failed to parse module {}", path.display()))?;

    let prefix = format!("{parent_prefix}::{mod_name}");
    let mod_items = items::extract_items(&file, path, &prefix, feature_gate);

    // Determine the directory for submodule resolution
    let mod_dir = if path.file_name().map(|f| f == "mod.rs").unwrap_or(false) {
        path.parent().unwrap_or(path).to_path_buf()
    } else {
        // For foo.rs, submodules live in foo/
        let parent = path.parent().unwrap_or(path);
        parent.join(mod_name)
    };

    let submodules = if mod_dir.is_dir() {
        discover_submodules(&file, &mod_dir, &prefix)?
    } else {
        // Still discover inline modules even if no directory exists
        discover_submodules(&file, path.parent().unwrap_or(path), &prefix)?
    };

    Ok(ModuleInfo {
        name: mod_name.to_string(),
        path: path.to_path_buf(),
        module_docs: docs::extract_module_docs(&content),
        items: mod_items,
        submodules,
        feature_gate: feature_gate.map(|s| s.to_string()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_crate(dir: &Path) {
        let src = dir.join("src");
        fs::create_dir_all(&src).unwrap();

        fs::write(
            src.join("lib.rs"),
            r#"
//! Test crate documentation

pub mod utils;
pub mod types;

/// A root function
pub fn root_fn() -> i32 { 42 }
"#,
        )
        .unwrap();

        fs::write(
            src.join("utils.rs"),
            r#"
//! Utility module

/// A helper function
pub fn helper() -> bool { true }
"#,
        )
        .unwrap();

        fs::write(
            src.join("types.rs"),
            r#"
//! Type definitions

/// A point in 2D space
pub struct Point {
    pub x: f64,
    pub y: f64,
}
"#,
        )
        .unwrap();
    }

    #[test]
    fn walks_simple_crate() {
        let tmp = TempDir::new().unwrap();
        let crate_dir = tmp.path().join("test-crate");
        create_test_crate(&crate_dir);

        let modules = walk_crate(&crate_dir).unwrap();
        assert_eq!(modules.len(), 1); // root module

        let root = &modules[0];
        assert_eq!(root.name, "lib");
        assert!(root.module_docs.contains("Test crate documentation"));

        // root_fn should be in root items
        assert!(root.items.iter().any(|i| i.name == "root_fn"));

        // Two submodules
        assert_eq!(root.submodules.len(), 2);

        let utils = root.submodules.iter().find(|m| m.name == "utils").unwrap();
        assert!(utils.module_docs.contains("Utility module"));
        assert!(utils.items.iter().any(|i| i.name == "helper"));

        let types = root.submodules.iter().find(|m| m.name == "types").unwrap();
        assert!(types.items.iter().any(|i| i.name == "Point"));
    }

    #[test]
    fn handles_nested_modules() {
        let tmp = TempDir::new().unwrap();
        let crate_dir = tmp.path().join("nested-crate");
        let src = crate_dir.join("src");
        let inner_dir = src.join("outer");
        fs::create_dir_all(&inner_dir).unwrap();

        fs::write(src.join("lib.rs"), "pub mod outer;\n").unwrap();
        fs::write(
            src.join("outer.rs"),
            "pub mod inner;\npub fn outer_fn() {}\n",
        )
        .unwrap();
        fs::write(inner_dir.join("inner.rs"), "pub fn inner_fn() {}\n").unwrap();

        let modules = walk_crate(&crate_dir).unwrap();
        let root = &modules[0];
        let outer = &root.submodules[0];
        assert_eq!(outer.name, "outer");
        assert!(outer.items.iter().any(|i| i.name == "outer_fn"));

        // inner should be a submodule of outer
        assert_eq!(outer.submodules.len(), 1);
        assert_eq!(outer.submodules[0].name, "inner");
        assert!(outer.submodules[0]
            .items
            .iter()
            .any(|i| i.name == "inner_fn"));
    }

    #[test]
    fn handles_mod_rs_style() {
        let tmp = TempDir::new().unwrap();
        let crate_dir = tmp.path().join("modrs-crate");
        let src = crate_dir.join("src");
        let sub_dir = src.join("sub");
        fs::create_dir_all(&sub_dir).unwrap();

        fs::write(src.join("lib.rs"), "pub mod sub;\n").unwrap();
        fs::write(sub_dir.join("mod.rs"), "pub fn sub_fn() {}\n").unwrap();

        let modules = walk_crate(&crate_dir).unwrap();
        let root = &modules[0];
        assert_eq!(root.submodules.len(), 1);
        assert_eq!(root.submodules[0].name, "sub");
        assert!(root.submodules[0].items.iter().any(|i| i.name == "sub_fn"));
    }

    #[test]
    fn handles_missing_lib_rs() {
        let tmp = TempDir::new().unwrap();
        let crate_dir = tmp.path().join("no-lib");
        fs::create_dir_all(&crate_dir).unwrap();

        let modules = walk_crate(&crate_dir).unwrap();
        assert!(modules.is_empty());
    }

    #[test]
    fn handles_inline_module() {
        let tmp = TempDir::new().unwrap();
        let crate_dir = tmp.path().join("inline-crate");
        let src = crate_dir.join("src");
        fs::create_dir_all(&src).unwrap();

        fs::write(
            src.join("lib.rs"),
            r#"
pub mod inline {
    pub fn inline_fn() -> bool { true }
}
"#,
        )
        .unwrap();

        let modules = walk_crate(&crate_dir).unwrap();
        let root = &modules[0];
        assert_eq!(root.submodules.len(), 1);
        assert_eq!(root.submodules[0].name, "inline");
        assert!(root.submodules[0]
            .items
            .iter()
            .any(|i| i.name == "inline_fn"));
    }
}
