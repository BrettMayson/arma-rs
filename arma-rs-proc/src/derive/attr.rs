use std::collections::HashSet;

use syn::{Error, Result};

pub trait FromMetas: Sized {
    fn parse_meta(metas: &[syn::Meta]) -> Result<Self>;
}

pub fn parse_attributes<T: FromMetas>(attrs: &[syn::Attribute]) -> Result<T> {
    let nested_metas = collect_nested_metas(attrs)?;
    check_duplicate_metas(&nested_metas)?;
    T::parse_meta(&nested_metas)
}

pub struct ContainerAttributes {
    pub transparent: bool,
    pub default: bool,
}

impl FromMetas for ContainerAttributes {
    fn parse_meta(metas: &[syn::Meta]) -> Result<Self> {
        let mut result = Self {
            transparent: false,
            default: false,
        };

        for meta in metas {
            match path_to_string(meta.path()).as_str() {
                "transparent" => {
                    result.transparent = true;
                }
                "default" => {
                    result.default = true;
                }
                unknown => {
                    return Err(Error::new_spanned(
                        meta,
                        format!("unknown attribute `{unknown}`"),
                    ))
                }
            }
        }
        Ok(result)
    }
}

pub struct FieldAttributes {
    pub default: bool,
}

impl FromMetas for FieldAttributes {
    fn parse_meta(metas: &[syn::Meta]) -> Result<Self> {
        let mut result = Self { default: false };

        for meta in metas {
            match path_to_string(meta.path()).as_str() {
                "default" => {
                    result.default = true;
                }
                unknown => {
                    return Err(Error::new_spanned(
                        meta,
                        format!("unknown attribute `{unknown}`"),
                    ))
                }
            }
        }
        Ok(result)
    }
}

fn check_duplicate_metas(attrs: &[syn::Meta]) -> Result<()> {
    let mut seen = HashSet::new();
    attrs.iter().try_for_each(|attr| {
        let path = path_to_string(attr.path());
        if !seen.insert(path.clone()) {
            return Err(Error::new_spanned(
                attr,
                format!("duplicate attribute `{path}`"),
            ));
        }
        Ok(())
    })
}

fn collect_nested_metas(attrs: &[syn::Attribute]) -> Result<Vec<syn::Meta>> {
    attrs
        .iter()
        .filter(move |attr| attr.path.is_ident("arma"))
        .try_fold(Vec::new(), |mut acc, attr| {
            let nested_metas = parse_nested_meta(attr)?;
            acc.extend(nested_metas);
            Ok(acc)
        })
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
        .map(|s| s.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}
