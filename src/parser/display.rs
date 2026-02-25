use crate::parser::index::{ApiItem, ItemKind};

/// Format an API item as a brief one-line summary for search results.
pub fn brief_summary(item: &ApiItem) -> String {
    let kind_str = match &item.kind {
        ItemKind::Function {
            is_async,
            is_unsafe,
        } => {
            let mut prefix = String::new();
            if *is_unsafe {
                prefix.push_str("unsafe ");
            }
            if *is_async {
                prefix.push_str("async ");
            }
            format!("{prefix}fn")
        }
        ItemKind::Struct { .. } => "struct".to_string(),
        ItemKind::Enum { .. } => "enum".to_string(),
        ItemKind::Trait { .. } => "trait".to_string(),
        ItemKind::TypeAlias => "type".to_string(),
        ItemKind::Const { .. } => "const".to_string(),
        ItemKind::Impl { trait_name, .. } => match trait_name {
            Some(t) => format!("impl {t}"),
            None => "impl".to_string(),
        },
        ItemKind::ReExport { .. } => "use".to_string(),
    };

    let doc_summary = first_sentence(&item.doc_comment);

    if doc_summary.is_empty() {
        format!("[{kind_str}] {}", item.full_path)
    } else {
        format!("[{kind_str}] {} — {doc_summary}", item.full_path)
    }
}

/// Extract the first sentence from a doc comment.
pub fn first_sentence(doc: &str) -> String {
    let first_line = doc.lines().next().unwrap_or("");
    // Take up to first period followed by space or end
    if let Some(dot_pos) = first_line.find(". ") {
        first_line[..=dot_pos].to_string()
    } else {
        first_line.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::index::FieldKind;
    use std::path::PathBuf;

    fn make_item(kind: ItemKind, name: &str, doc: &str) -> ApiItem {
        ApiItem {
            kind,
            name: name.to_string(),
            full_path: format!("lib::{name}"),
            signature: format!("pub struct {name}"),
            doc_comment: doc.to_string(),
            feature_gate: None,
            generics: None,
            source_file: PathBuf::from("src/lib.rs"),
            line_number: 1,
        }
    }

    #[test]
    fn brief_summary_struct_with_doc() {
        let item = make_item(
            ItemKind::Struct {
                fields: FieldKind::Unit,
            },
            "Foo",
            "A foo thing. It does stuff.",
        );
        let summary = brief_summary(&item);
        assert_eq!(summary, "[struct] lib::Foo — A foo thing.");
    }

    #[test]
    fn brief_summary_without_doc() {
        let item = make_item(
            ItemKind::Struct {
                fields: FieldKind::Unit,
            },
            "Bar",
            "",
        );
        let summary = brief_summary(&item);
        assert_eq!(summary, "[struct] lib::Bar");
    }

    #[test]
    fn brief_summary_async_unsafe_fn() {
        let item = make_item(
            ItemKind::Function {
                is_async: true,
                is_unsafe: true,
            },
            "dangerous",
            "Don't call this.",
        );
        let summary = brief_summary(&item);
        assert!(summary.starts_with("[unsafe async fn]"));
    }

    #[test]
    fn first_sentence_extracts_correctly() {
        assert_eq!(first_sentence("Hello world. More stuff."), "Hello world.");
        assert_eq!(first_sentence("No period here"), "No period here");
        assert_eq!(first_sentence("Ends with period."), "Ends with period.");
        assert_eq!(first_sentence(""), "");
    }
}
