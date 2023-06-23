use std::collections::HashSet;

use syn::{Error, Result};

#[derive(PartialEq, Eq)]
pub enum ContainerAttribute {
    Transparent,
}

pub trait ParseAttrMeta {
    fn parse_attr_meta(meta: syn::Meta) -> Result<Self>
    where
        Self: Sized;
}

impl ParseAttrMeta for ContainerAttribute {
    fn parse_attr_meta(meta: syn::Meta) -> Result<Self>
    where
        Self: Sized,
    {
        meta.path()
            .get_ident()
            .and_then(|ident| match ident.to_string().as_str() {
                "transparent" => Some(Self::Transparent),
                _ => None,
            })
            .ok_or_else(|| {
                Error::new_spanned(
                    &meta,
                    format!("unknown attribute `{}`", path_to_string(meta.path())),
                )
            })
    }
}

pub fn parse_attrs<T>(attrs: &[syn::Attribute]) -> Result<Vec<T>>
where
    T: ParseAttrMeta,
{
    combine_attrs(attrs)?
        .into_iter()
        .map(ParseAttrMeta::parse_attr_meta)
        .collect()
}

fn combine_attrs(attrs: &[syn::Attribute]) -> Result<Vec<syn::Meta>> {
    let mut combined = Vec::new();
    for attr in filter_attrs(attrs) {
        combined.extend(parse_nested_meta(attr)?);
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

    let nested_metas: Vec<_> = match meta.parse_meta()? {
        syn::Meta::List(list) => list.nested.into_iter().map(nested_meta).collect(),
        meta => Err(Error::new_spanned(meta, "expected #[arma(...)]")),
    }?;

    let mut attr_paths = HashSet::new();
    for nested in &nested_metas {
        let path = path_to_string(nested.path());
        if !attr_paths.insert(path.clone()) {
            return Err(Error::new_spanned(
                nested,
                format!("duplicate attribute `{path}`"),
            ));
        }
    }

    Ok(nested_metas)
}

fn path_to_string(path: &syn::Path) -> String {
    path.segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}
