use std::collections::HashMap;
use std::marker::PhantomData;
use std::path::PathBuf;

/// Phantom type: index has been built but not yet validated.
pub struct Unvalidated;

/// Phantom type: index has passed validation checks.
pub struct Validated;

/// Sum type for all extractable public items.
#[derive(Debug, Clone)]
pub enum ItemKind {
    Function {
        is_async: bool,
        is_unsafe: bool,
    },
    Struct {
        fields: FieldKind,
    },
    Enum {
        variants: Vec<VariantInfo>,
    },
    Trait {
        supertraits: Vec<String>,
    },
    TypeAlias,
    Const {
        ty: String,
    },
    Impl {
        self_type: String,
        trait_name: Option<String>,
    },
    ReExport {
        target: String,
    },
}

/// Field structure for structs.
#[derive(Debug, Clone)]
pub enum FieldKind {
    Named(Vec<FieldInfo>),
    Tuple(Vec<String>),
    Unit,
}

#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub name: String,
    pub ty: String,
    pub doc_comment: String,
    pub visibility: Visibility,
}

#[derive(Debug, Clone)]
pub struct VariantInfo {
    pub name: String,
    pub fields: FieldKind,
    pub doc_comment: String,
}

#[derive(Debug, Clone)]
pub enum Visibility {
    Public,
    Crate,
    Restricted(String),
    Private,
}

/// A single public API item extracted from source.
#[derive(Debug, Clone)]
pub struct ApiItem {
    pub kind: ItemKind,
    pub name: String,
    pub full_path: String,
    pub signature: String,
    pub doc_comment: String,
    pub feature_gate: Option<String>,
    pub generics: Option<String>,
    pub source_file: PathBuf,
    pub line_number: usize,
}

/// Information about a parsed crate.
#[derive(Debug, Clone)]
pub struct CrateInfo {
    pub name: String,
    pub alias: Option<String>,
    pub feature_gate: Option<String>,
    pub source_dir: PathBuf,
    pub modules: Vec<ModuleInfo>,
    pub module_docs: String,
}

/// Information about a parsed module.
#[derive(Debug, Clone)]
pub struct ModuleInfo {
    pub name: String,
    pub path: PathBuf,
    pub module_docs: String,
    pub items: Vec<ApiItem>,
    pub submodules: Vec<ModuleInfo>,
    pub feature_gate: Option<String>,
}

/// Statistics about the indexed API surface.
#[derive(Debug)]
pub struct IndexStats {
    pub crate_count: usize,
    pub module_count: usize,
    pub item_count: usize,
}

/// Report of validation errors.
#[derive(Debug)]
pub struct ValidationReport {
    pub errors: Vec<String>,
}

impl std::fmt::Display for ValidationReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for err in &self.errors {
            writeln!(f, "{err}")?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationReport {}

/// The API index, parameterized by validation state.
///
/// `ApiIndex<Unvalidated>` is built by the parser.
/// `ApiIndex<Validated>` is produced by `validate()` and is required
/// by MCP tool handlers — this ensures tools can only operate on
/// a successfully-parsed index.
pub struct ApiIndex<State = Validated> {
    pub library_name: String,
    pub crates: Vec<CrateInfo>,
    pub items_by_name: HashMap<String, Vec<ApiItem>>,
    pub parse_errors: Vec<String>,
    _state: PhantomData<State>,
}

/// Recursively collect all items from a module tree into the flat index.
pub fn collect_items_from_modules(
    modules: &[ModuleInfo],
    index: &mut HashMap<String, Vec<ApiItem>>,
) {
    for module in modules {
        for item in &module.items {
            index
                .entry(item.name.clone())
                .or_default()
                .push(item.clone());
        }
        collect_items_from_modules(&module.submodules, index);
    }
}

impl ApiIndex<Unvalidated> {
    /// Create an empty unvalidated index.
    pub fn empty() -> Self {
        Self {
            library_name: String::new(),
            crates: Vec::new(),
            items_by_name: HashMap::new(),
            parse_errors: Vec::new(),
            _state: PhantomData,
        }
    }

    /// Create an unvalidated index with parsed data.
    pub fn new(
        library_name: String,
        crates: Vec<CrateInfo>,
        items_by_name: HashMap<String, Vec<ApiItem>>,
        parse_errors: Vec<String>,
    ) -> Self {
        Self {
            library_name,
            crates,
            items_by_name,
            parse_errors,
            _state: PhantomData,
        }
    }

    /// Validate the index, transitioning to the Validated state.
    /// Parse errors are reported but don't block validation — partial
    /// indexes are still useful.
    pub fn validate(self) -> Result<ApiIndex<Validated>, ValidationReport> {
        if !self.parse_errors.is_empty() {
            tracing::warn!(
                "{} parse errors during indexing (index still usable)",
                self.parse_errors.len()
            );
            for err in &self.parse_errors {
                tracing::warn!("  {err}");
            }
        }

        Ok(ApiIndex {
            library_name: self.library_name,
            crates: self.crates,
            items_by_name: self.items_by_name,
            parse_errors: self.parse_errors,
            _state: PhantomData,
        })
    }
}

impl ApiIndex<Validated> {
    /// Get statistics about the indexed API surface.
    pub fn stats(&self) -> IndexStats {
        let module_count: usize = self.crates.iter().map(|c| count_modules(&c.modules)).sum();

        IndexStats {
            crate_count: self.crates.len(),
            module_count,
            item_count: self.items_by_name.values().map(|v| v.len()).sum(),
        }
    }

    /// Search items by name, full path, or doc comment substring (case-insensitive).
    pub fn search(&self, query: &str) -> Vec<&ApiItem> {
        let query_lower = query.to_lowercase();
        self.items_by_name
            .values()
            .flatten()
            .filter(|item| {
                item.name.to_lowercase().contains(&query_lower)
                    || item.full_path.to_lowercase().contains(&query_lower)
                    || item.doc_comment.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Get all items gated by a specific feature.
    pub fn feature_items(&self, feature: &str) -> Vec<&ApiItem> {
        self.items_by_name
            .values()
            .flatten()
            .filter(|item| {
                item.feature_gate
                    .as_ref()
                    .map(|fg| fg.contains(feature))
                    .unwrap_or(false)
            })
            .collect()
    }

    /// Get all items in a specific crate (by name or alias).
    pub fn crate_items(&self, crate_name: &str) -> Vec<&ApiItem> {
        self.crates
            .iter()
            .find(|c| c.name == crate_name || c.alias.as_deref() == Some(crate_name))
            .map(|c| collect_all_items(&c.modules))
            .unwrap_or_default()
    }

    /// Get a crate by name or alias.
    pub fn get_crate(&self, name: &str) -> Option<&CrateInfo> {
        self.crates
            .iter()
            .find(|c| c.name == name || c.alias.as_deref() == Some(name))
    }
}

fn collect_all_items(modules: &[ModuleInfo]) -> Vec<&ApiItem> {
    let mut items = Vec::new();
    for module in modules {
        items.extend(module.items.iter());
        items.extend(collect_all_items(&module.submodules));
    }
    items
}

fn count_modules(modules: &[ModuleInfo]) -> usize {
    modules
        .iter()
        .map(|m| 1 + count_modules(&m.submodules))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_index_validates_successfully() {
        let index = ApiIndex::<Unvalidated>::empty();
        let validated = index.validate();
        assert!(validated.is_ok());
    }

    #[test]
    fn validated_index_reports_zero_stats() {
        let index = ApiIndex::<Unvalidated>::empty();
        let validated = index.validate().unwrap();
        let stats = validated.stats();
        assert_eq!(stats.crate_count, 0);
        assert_eq!(stats.module_count, 0);
        assert_eq!(stats.item_count, 0);
    }

    #[test]
    fn search_on_empty_index_returns_empty() {
        let index = ApiIndex::<Unvalidated>::empty();
        let validated = index.validate().unwrap();
        let results = validated.search("anything");
        assert!(results.is_empty());
    }

    #[test]
    fn phantom_types_enforce_state_at_compile_time() {
        // This test verifies the type system design:
        // ApiIndex<Unvalidated> has validate() but not search()
        // ApiIndex<Validated> has search() but not validate()
        let unvalidated = ApiIndex::<Unvalidated>::empty();
        // unvalidated.search("x"); // Would not compile
        let validated = unvalidated.validate().unwrap();
        let _ = validated.search("x"); // Compiles
                                       // validated.validate(); // Would not compile
    }

    fn make_test_item(name: &str, full_path: &str, doc: &str) -> ApiItem {
        ApiItem {
            kind: ItemKind::Function {
                is_async: false,
                is_unsafe: false,
            },
            name: name.to_string(),
            full_path: full_path.to_string(),
            signature: format!("pub fn {name}()"),
            doc_comment: doc.to_string(),
            feature_gate: None,
            generics: None,
            source_file: PathBuf::from("test.rs"),
            line_number: 1,
        }
    }

    fn make_index_with_items(items: Vec<ApiItem>) -> ApiIndex<Validated> {
        let mut by_name: HashMap<String, Vec<ApiItem>> = HashMap::new();
        for item in items {
            by_name.entry(item.name.clone()).or_default().push(item);
        }
        ApiIndex::<Unvalidated>::new("test".to_string(), vec![], by_name, vec![])
            .validate()
            .unwrap()
    }

    #[test]
    fn search_matches_item_name() {
        let index = make_index_with_items(vec![make_test_item(
            "kl_polynomial",
            "amari::enumerative::kazhdan_lusztig::kl_polynomial",
            "Compute a Kazhdan-Lusztig polynomial",
        )]);
        let results = index.search("kl_polynomial");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "kl_polynomial");
    }

    #[test]
    fn search_matches_full_path() {
        let index = make_index_with_items(vec![make_test_item(
            "kl_polynomial",
            "amari::enumerative::kazhdan_lusztig::kl_polynomial",
            "Compute a KL polynomial",
        )]);
        let results = index.search("kazhdan_lusztig");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "kl_polynomial");
    }

    #[test]
    fn search_matches_doc_comment() {
        let index = make_index_with_items(vec![make_test_item(
            "kl_polynomial",
            "amari::enumerative::kazhdan_lusztig::kl_polynomial",
            "Compute a Kazhdan-Lusztig polynomial",
        )]);
        let results = index.search("Kazhdan");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "kl_polynomial");
    }

    #[test]
    fn search_is_case_insensitive() {
        let index = make_index_with_items(vec![make_test_item(
            "kl_polynomial",
            "amari::enumerative::kazhdan_lusztig::kl_polynomial",
            "Compute a Kazhdan-Lusztig polynomial",
        )]);
        let results = index.search("KAZHDAN");
        assert_eq!(results.len(), 1);
    }
}
