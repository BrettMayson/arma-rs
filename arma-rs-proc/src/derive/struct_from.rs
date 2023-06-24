use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, Result};

use super::*;

pub fn from_impl_body(data: &DataStruct, attributes: &ContainerAttributes) -> Result<TokenStream> {
    match &data {
        DataStruct::Unit => Err(Error::new(
            Span::call_site(),
            "Unit struct's aren't supported",
        )),
        DataStruct::Map(fields) => map_struct(fields, attributes),
        DataStruct::Tuple(fields) => tuple_struct(fields, attributes),
        DataStruct::NewType(field) => newtype_struct(field, attributes),
    }
}

fn map_struct(
    fields: &Fields<FieldNamed>,
    attributes: &ContainerAttributes,
) -> Result<TokenStream> {
    let idents = fields.idents();
    let names = fields.names();
    let count = fields.len();

    if attributes.transparent {
        if count > 1 {
            return Err(Error::new(
                Span::call_site(),
                "#[arma(transparent)] structs must have exactly one field",
            ));
        }

        if attributes.default {
            return Err(Error::new(
                Span::call_site(),
                "#[arma(transparent)] and #[arma(default)] cannot be used together",
            ));
        }

        let ident = idents[0];
        return Ok(quote! {
            Ok(Self {
                #ident: arma_rs::FromArma::from_arma(source)?,
            })
        });
    }

    let values = quote! {
        let values: std::collections::HashMap<String, String> = arma_rs::FromArma::from_arma(source)?;
        for value in values.keys() {
            if ![#(#names),*].contains(&value.as_str()) {
                return Err(arma_rs::FromArmaError::MapUnknownField(value.clone()));
            }
        }
    };
    Ok(match attributes.default {
        true => quote! {
            #values

            let default = Self::default();
            Ok(Self {
                #(#idents: match values.get(#names) {
                    Some(value) => arma_rs::FromArma::from_arma(value.clone())?,
                    None => default.#idents,
                }),*
            })
        },
        false => quote! {
            #values

            Ok(Self {
                #(#idents: match values.get(#names) {
                    Some(value) => arma_rs::FromArma::from_arma(value.clone())?,
                    None => return Err(arma_rs::FromArmaError::MapMissingField(#names.to_string())),
                }),*
            })
        },
    })
}

fn tuple_struct(
    fields: &Fields<FieldUnnamed>,
    attributes: &ContainerAttributes,
) -> Result<TokenStream> {
    let indexes = fields.indexes();
    let types = fields.types();

    if attributes.default {
        return Err(Error::new(
            Span::call_site(),
            "#[arma(default)] can only be used on structs with named fields",
        ));
    }

    Ok(quote! {
        let values: (#(
            #types,
        )*) = arma_rs::FromArma::from_arma(source)?;
        Ok(Self (#(
            values.#indexes,
        )*))
    })
}

fn newtype_struct(_field: &FieldUnnamed, attributes: &ContainerAttributes) -> Result<TokenStream> {
    if attributes.default {
        return Err(Error::new(
            Span::call_site(),
            "#[arma(default)] can only be used on structs with named fields",
        ));
    }

    Ok(quote! {
        Ok(Self (arma_rs::FromArma::from_arma(source)?))
    })
}
