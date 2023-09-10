use syn::Result;

struct AttrValue<T> {
    value: Option<T>,
}

impl<T> AttrValue<T> {
    fn none() -> Self {
        Self { value: None }
    }

    fn set(&mut self, meta: syn::meta::ParseNestedMeta, value: T) -> Result<()> {
        if self.value.is_some() {
            return Err(meta.error(format!(
                "duplicate arma attribute `{}`",
                path_to_string(&meta.path)
            )));
        }
        self.value = Some(value);
        Ok(())
    }

    fn get(&self) -> &Option<T> {
        &self.value
    }
}

pub struct ContainerAttributes {
    pub transparent: bool,
    pub default: bool,
}

impl ContainerAttributes {
    pub fn from_attrs(attrs: &[syn::Attribute]) -> Result<Self> {
        let mut transparent = AttrValue::none();
        let mut default = AttrValue::none();

        for attr in attrs {
            if !attr.path().is_ident("arma") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("transparent") {
                    transparent.set(meta, true)?;
                    return Ok(());
                }

                if meta.path.is_ident("default") {
                    default.set(meta, true)?;
                    return Ok(());
                }

                Err(meta.error(format!(
                    "unknown arma container attribute `{}`",
                    path_to_string(&meta.path)
                )))
            })?;
        }

        Ok(Self {
            transparent: transparent.get().unwrap_or(false),
            default: default.get().unwrap_or(false),
        })
    }
}

pub struct FieldAttributes {
    pub default: bool,
}

impl FieldAttributes {
    pub fn from_attrs(attrs: &[syn::Attribute]) -> Result<Self> {
        let mut default = AttrValue::none();

        for attr in attrs {
            if !attr.path().is_ident("arma") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("default") {
                    default.set(meta, true)?;
                    return Ok(());
                }

                Err(meta.error(format!(
                    "unknown arma field attribute `{}`",
                    path_to_string(&meta.path)
                )))
            })?;
        }

        Ok(Self {
            default: default.get().unwrap_or(false),
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
