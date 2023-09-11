use syn::{Error, Result};

use super::CombinedError;

pub trait FromAttrs: Default + Sized {
    fn parse_attr(&mut self, errors: &mut CombinedError, attrs: &syn::Attribute) -> Result<()>;

    fn from_attrs(errors: &mut CombinedError, attrs: &[syn::Attribute]) -> Result<Self> {
        attrs.iter().try_fold(Self::default(), |mut result, attr| {
            if attr.path().is_ident("arma") {
                result.parse_attr(errors, attr)?;
            }
            Ok(result)
        })
    }
}

pub struct Attribute<T> {
    path: Option<syn::Path>,
    value: T,
}

impl<T> Attribute<T> {
    fn new(default: T) -> Self {
        Self {
            value: default,
            path: None,
        }
    }

    pub fn is_set(&self) -> bool {
        self.path.is_some()
    }

    fn set(&mut self, errors: &mut CombinedError, meta: &syn::meta::ParseNestedMeta, value: T) {
        if self.is_set() {
            errors.add(meta.error(format!(
                "duplicate arma attribute `{}`",
                path_to_string(&meta.path)
            )));
            return;
        }
        self.value = value;
        self.path = Some(meta.path.clone());
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    #[must_use]
    pub fn error(&self, message: &str) -> Error {
        Error::new_spanned(self.path.as_ref().unwrap(), message)
    }
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

impl FromAttrs for ContainerAttributes {
    fn parse_attr(&mut self, errors: &mut CombinedError, attr: &syn::Attribute) -> Result<()> {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("transparent") {
                self.transparent.set(errors, &meta, true);
                return Ok(());
            }

            if meta.path.is_ident("default") {
                self.default.set(errors, &meta, true);
                return Ok(());
            }

            Err(meta.error(format!(
                "unknown arma container attribute `{}`",
                path_to_string(&meta.path)
            )))
        })
    }
}

pub struct FieldAttributes {
    pub default: Attribute<bool>,
}

impl Default for FieldAttributes {
    fn default() -> Self {
        Self {
            default: Attribute::new(false),
        }
    }
}

impl FromAttrs for FieldAttributes {
    fn parse_attr(&mut self, errors: &mut CombinedError, attr: &syn::Attribute) -> Result<()> {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("default") {
                self.default.set(errors, &meta, true);
                return Ok(());
            }

            Err(meta.error(format!(
                "unknown arma field attribute `{}`",
                path_to_string(&meta.path)
            )))
        })
    }
}

fn path_to_string(path: &syn::Path) -> String {
    path.segments
        .iter()
        .map(|s| s.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}
