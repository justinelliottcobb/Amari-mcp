use syn::Attribute;

/// Extract module-level documentation (`//!` comments) from source text.
///
/// These correspond to inner doc attributes (`#![doc = "..."]`) which
/// `///!` desugars to.
pub fn extract_module_docs(source: &str) -> String {
    let mut docs = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("//!") {
            let doc_line = trimmed
                .strip_prefix("//! ")
                .unwrap_or(trimmed.strip_prefix("//!").unwrap_or(""));
            docs.push(doc_line.to_string());
        } else if trimmed.starts_with("#![") || trimmed.is_empty() || trimmed.starts_with("//") {
            // Skip other attributes, blank lines between doc blocks, and regular comments
            continue;
        } else {
            // Stop at first non-doc, non-attribute, non-empty line
            break;
        }
    }
    docs.join("\n").trim().to_string()
}

/// Extract doc comments from syn attributes (`/// ...` style).
///
/// These desugar to `#[doc = "..."]` attributes on items.
pub fn extract_doc_comment(attrs: &[Attribute]) -> String {
    let mut docs = Vec::new();
    for attr in attrs {
        if attr.path().is_ident("doc") {
            if let syn::Meta::NameValue(nv) = &attr.meta {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit_str),
                    ..
                }) = &nv.value
                {
                    docs.push(lit_str.value());
                }
            }
        }
    }
    let joined = docs.join("\n");
    // Trim leading space from each line (rustdoc convention)
    joined
        .lines()
        .map(|line| line.strip_prefix(' ').unwrap_or(line))
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_module_docs_from_source() {
        let source = r#"//! Module documentation
//! with multiple lines
//!
//! And a blank doc line

use std::collections::HashMap;
"#;
        let docs = extract_module_docs(source);
        assert_eq!(
            docs,
            "Module documentation\nwith multiple lines\n\nAnd a blank doc line"
        );
    }

    #[test]
    fn empty_source_returns_empty_docs() {
        assert_eq!(extract_module_docs(""), "");
    }

    #[test]
    fn source_without_doc_comments_returns_empty() {
        let source = "use std::path::Path;\nfn main() {}";
        assert_eq!(extract_module_docs(source), "");
    }

    #[test]
    fn extracts_doc_comment_from_attrs() {
        let source = r#"
            /// A documented item
            /// with two lines
            pub fn foo() {}
        "#;
        let file = syn::parse_file(source).unwrap();
        if let syn::Item::Fn(item_fn) = &file.items[0] {
            let doc = extract_doc_comment(&item_fn.attrs);
            assert_eq!(doc, "A documented item\nwith two lines");
        } else {
            panic!("Expected function item");
        }
    }

    #[test]
    fn no_doc_attrs_returns_empty() {
        let source = "pub fn foo() {}";
        let file = syn::parse_file(source).unwrap();
        if let syn::Item::Fn(item_fn) = &file.items[0] {
            let doc = extract_doc_comment(&item_fn.attrs);
            assert_eq!(doc, "");
        }
    }
}
