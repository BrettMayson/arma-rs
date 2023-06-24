use std::collections::HashSet;

use syn::{Error, Result};

#[derive(PartialEq, Eq)]
pub struct ContainerAttributes {
    pub transparent: bool,
    pub default: bool,
}

impl ContainerAttributes {
    pub fn from_attrs(attrs: &[syn::Attribute]) -> Result<Self> {
        let nested_metas = combine_attrs(attrs)?;
        Self::default().update_from_metas(&nested_metas)
    }

    fn default() -> Self {
        Self {
            transparent: false,
            default: false,
        }
    }

    fn update_from_metas(mut self, metas: &[syn::Meta]) -> Result<Self> {
        for meta in metas {
            match path_to_string(meta.path()).as_str() {
                "transparent" => {
                    self.transparent = true;
                }
                "default" => {
                    self.default = true;
                }
                unknown => {
                    return Err(Error::new_spanned(
                        meta,
                        format!("unknown attribute `{unknown}`"),
                    ))
                }
            }
        }
        Ok(self)
    }
}

fn combine_attrs(attrs: &[syn::Attribute]) -> Result<Vec<syn::Meta>> {
    let mut combined = Vec::new();
    for attr in filter_attrs(attrs) {
        combined.extend(parse_nested_meta(attr)?);
    }

    let mut unique_attr_paths = HashSet::new();
    for nested in &combined {
        let path = path_to_string(nested.path());
        if !unique_attr_paths.insert(path.clone()) {
            return Err(Error::new_spanned(
                nested,
                format!("duplicate attribute `{path}`"),
            ));
        }
    }
    Ok(combined)
}

fn filter_attrs(attrs: &[syn::Attribute]) -> impl Iterator<Item = &syn::Attribute> {
    attrs.iter().filter(move |attr| attr.path.is_ident("arma"))
}

fn parse_nested_meta(meta: &syn::Attribute) -> Result<Vec<syn::Meta>> {
    fn nested_meta(nested: syn::NestedMeta) -> Result<syn::Meta> {
        match nested {
            syn::NestedMeta::Meta(meta) => Ok(meta),
            syn::NestedMeta::Lit(_) => Err(Error::new_spanned(
                nested,
                "unexpected literal in nested attribute",
            )),
        }
    }

    match meta.parse_meta()? {
        syn::Meta::List(list) => list.nested.into_iter().map(nested_meta).collect(),
        meta => Err(Error::new_spanned(meta, "expected #[arma(...)]")),
    }
}

fn path_to_string(path: &syn::Path) -> String {
    path.segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}
