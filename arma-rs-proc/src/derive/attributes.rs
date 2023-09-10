use syn::{Error, Result};

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

impl ContainerAttributes {
    pub fn from_attrs(attrs: &[syn::Attribute]) -> Result<Self> {
        let mut transparent = Attribute::new(false);
        let mut default = Attribute::new(false);

        for attr in attrs {
            if !attr.path().is_ident("arma") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("transparent") {
                    transparent.set(&meta, true)?;
                    return Ok(());
                }

                if meta.path.is_ident("default") {
                    default.set(&meta, true)?;
                    return Ok(());
                }

                Err(meta.error(format!(
                    "unknown arma container attribute `{}`",
                    path_to_string(&meta.path)
                )))
            })?;
        }
        Ok(Self {
            transparent,
            default,
        })
    }
}

pub struct FieldAttributes {
    pub default: Attribute<bool>,
}

impl FieldAttributes {
    pub fn from_attrs(attrs: &[syn::Attribute]) -> Result<Self> {
        let mut default = Attribute::new(false);

        for attr in attrs {
            if !attr.path().is_ident("arma") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("default") {
                    default.set(&meta, true)?;
                    return Ok(());
                }

                Err(meta.error(format!(
                    "unknown arma field attribute `{}`",
                    path_to_string(&meta.path)
                )))
            })?;
        }
        Ok(Self { default })
    }
}

fn path_to_string(path: &syn::Path) -> String {
    path.segments
        .iter()
        .map(|s| s.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}
