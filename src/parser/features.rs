use syn::Attribute;

/// Extract a feature gate from `#[cfg(feature = "...")]` attributes.
///
/// Returns the feature name if a simple feature gate is found,
/// or a stringified version for compound gates.
pub fn extract_feature_gate(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if !attr.path().is_ident("cfg") {
            continue;
        }

        // Parse the cfg(...) content
        if let Ok(nested) = attr.parse_args::<syn::Meta>() {
            if let Some(feature) = extract_from_meta(&nested) {
                return Some(feature);
            }
        }
    }
    None
}

fn extract_from_meta(meta: &syn::Meta) -> Option<String> {
    match meta {
        syn::Meta::NameValue(nv) if nv.path.is_ident("feature") => {
            if let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(lit_str),
                ..
            }) = &nv.value
            {
                return Some(lit_str.value());
            }
            None
        }
        syn::Meta::List(list) if list.path.is_ident("any") || list.path.is_ident("all") => {
            // Compound: any(feature = "foo", feature = "bar")
            // Collect all features
            let mut features = Vec::new();
            let _ = list.parse_nested_meta(|nested_meta| {
                if nested_meta.path.is_ident("feature") {
                    let value = nested_meta.value()?;
                    let lit: syn::LitStr = value.parse()?;
                    features.push(lit.value());
                }
                Ok(())
            });
            if features.is_empty() {
                None
            } else if features.len() == 1 {
                Some(features.into_iter().next().unwrap())
            } else {
                let combiner = if list.path.is_ident("any") {
                    " | "
                } else {
                    " + "
                };
                Some(features.join(combiner))
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn extracts_simple_feature_gate() {
        let attr: Attribute = parse_quote!(#[cfg(feature = "gpu")]);
        let gate = extract_feature_gate(&[attr]);
        assert_eq!(gate, Some("gpu".to_string()));
    }

    #[test]
    fn no_cfg_returns_none() {
        let attr: Attribute = parse_quote!(#[derive(Debug)]);
        let gate = extract_feature_gate(&[attr]);
        assert!(gate.is_none());
    }

    #[test]
    fn non_feature_cfg_returns_none() {
        let attr: Attribute = parse_quote!(#[cfg(test)]);
        let gate = extract_feature_gate(&[attr]);
        assert!(gate.is_none());
    }

    #[test]
    fn extracts_from_multiple_attrs() {
        let attrs: Vec<Attribute> = vec![
            parse_quote!(#[derive(Debug)]),
            parse_quote!(#[cfg(feature = "topology")]),
        ];
        let gate = extract_feature_gate(&attrs);
        assert_eq!(gate, Some("topology".to_string()));
    }

    #[test]
    fn extracts_any_compound_gate() {
        let attr: Attribute = parse_quote!(#[cfg(any(feature = "a", feature = "b"))]);
        let gate = extract_feature_gate(&[attr]);
        assert_eq!(gate, Some("a | b".to_string()));
    }

    #[test]
    fn extracts_all_compound_gate() {
        let attr: Attribute = parse_quote!(#[cfg(all(feature = "x", feature = "y"))]);
        let gate = extract_feature_gate(&[attr]);
        assert_eq!(gate, Some("x + y".to_string()));
    }
}
