mod from;
mod into;
mod validate;

use proc_macro2::Span;
use syn::{Error, Result};

pub use from::impl_from_arma;
pub use into::impl_into_arma;
pub use validate::validate_attributes;

use crate::derive::{
    data::{DataStruct, FieldNamed, FieldUnnamed},
    CombinedError,
};

impl DataStruct {
    pub fn new(errors: &mut CombinedError, data: syn::DataStruct) -> Result<Self> {
        match data.fields {
            syn::Fields::Unit => Err(Error::new(
                Span::call_site(),
                "unit-like structs aren't supported",
            )),
            syn::Fields::Named(fields) => {
                if fields.named.is_empty() {
                    return Err(Error::new(
                        Span::call_site(),
                        "unit-like structs aren't supported",
                    ));
                }

                let fields = fields
                    .named
                    .into_iter()
                    .map(|f| FieldNamed::new(errors, f))
                    .collect::<_>();
                Ok(Self::Map(fields))
            }
            syn::Fields::Unnamed(fields) => {
                if fields.unnamed.is_empty() {
                    return Err(Error::new(
                        Span::call_site(),
                        "unit-like structs aren't supported",
                    ));
                }

                if fields.unnamed.len() == 1 {
                    let field = FieldUnnamed::new(errors, fields.unnamed[0].clone(), 0);
                    Ok(Self::NewType(field))
                } else {
                    let fields = fields
                        .unnamed
                        .into_iter()
                        .enumerate()
                        .map(|(i, f)| FieldUnnamed::new(errors, f, i))
                        .collect::<_>();
                    Ok(Self::Tuple(fields))
                }
            }
        }
    }
}
