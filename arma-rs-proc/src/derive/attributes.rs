use syn::{Error, Result};

use crate::derive::CombinedError;

pub struct Attribute<T> {
    value: T,
    path: Option<syn::Path>,
}

impl<T> Attribute<T> {
    fn new(default: T) -> Self {
        Self {
            value: default,
            path: None,
        }
    }

    fn set(&mut self, meta: &syn::meta::ParseNestedMeta, value: T) -> Result<()> {
        if self.is_set() {
            return Err(meta.error(format!(
                "duplicate arma attribute `{}`",
                path_to_string(&meta.path)
            )));
        }
        self.value = value;
        self.path = Some(meta.path.clone());
        Ok(())
    }

    pub fn is_set(&self) -> bool {
        self.path.is_some()
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    #[must_use]
    pub fn error(&self, message: &str) -> Error {
        Error::new_spanned(self.path.as_ref().unwrap(), message)
    }
}

pub trait ParseAttr {
    fn parse_attr(&mut self, meta: syn::meta::ParseNestedMeta) -> Result<()>;
}

pub fn parse_attributes<T>(errors: &mut CombinedError, attrs: &[syn::Attribute]) -> T
where
    T: ParseAttr + Default + Sized,
{
    attrs.iter().fold(T::default(), |mut attributes, attr| {
        if !attr.path().is_ident("arma") {
            return attributes;
        }

        let result = attr.parse_nested_meta(|meta| {
            if let Err(err) = attributes.parse_attr(meta) {
                errors.add(err);
            }
            Ok(())
        });

        if let Err(err) = result {
            errors.add(err);
        }
        attributes
    })
}

pub struct ContainerAttributes {
    pub transparent: Attribute<bool>,
    pub default: Attribute<bool>,
}

impl Default for ContainerAttributes {
    fn default() -> Self {
        Self {
            transparent: Attribute::new(false),
            default: Attribute::new(false),
        }
    }
}

impl ParseAttr for ContainerAttributes {
    fn parse_attr(&mut self, meta: syn::meta::ParseNestedMeta) -> Result<()> {
        if meta.path.is_ident("transparent") {
            return self.transparent.set(&meta, true);
        }

        if meta.path.is_ident("default") {
            return self.default.set(&meta, true);
        }

        Err(meta.error(format!(
            "unknown arma container attribute `{}`",
            path_to_string(&meta.path)
        )))
    }
}

pub struct FieldAttributes {
    pub stringify: Attribute<bool>,
    pub default: Attribute<bool>,
}

impl Default for FieldAttributes {
    fn default() -> Self {
        Self {
            stringify: Attribute::new(false),
            default: Attribute::new(false),
        }
    }
}

impl ParseAttr for FieldAttributes {
    fn parse_attr(&mut self, meta: syn::meta::ParseNestedMeta) -> Result<()> {
        if meta.path.is_ident("stringify") {
            return self.stringify.set(&meta, true);
        }

        if meta.path.is_ident("default") {
            return self.default.set(&meta, true);
        }

        Err(meta.error(format!(
            "unknown arma field attribute `{}`",
            path_to_string(&meta.path)
        )))
    }
}

fn path_to_string(path: &syn::Path) -> String {
    path.segments
        .iter()
        .map(|s| s.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}
