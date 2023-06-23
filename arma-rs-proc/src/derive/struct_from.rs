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

fn map_struct(fields: &[FieldNamed], attributes: &ContainerAttributes) -> Result<TokenStream> {
    let idents = fields.idents();
    let count = fields.len();

    if attributes.transparent {
        if count > 1 {
            return Err(Error::new(
                Span::call_site(),
                "#[arma(transparent)] structs must have exactly one field",
            ));
        }

        let ident = idents[0];
        Ok(quote! {
            Ok(Self {
                #ident: arma_rs::FromArma::from_arma(source)?,
            })
        })
    } else {
        let names = fields.names();

        Ok(quote! {
            let values: std::collections::HashMap<String, String> = arma_rs::FromArma::from_arma(source)?;

            let len = values.len();
            if len != #count {
                return Err(arma_rs::FromArmaError::SizeMismatch {
                    expected: #count,
                    actual: len,
                })
            }

            Ok(Self {#(
                #idents: arma_rs::FromArma::from_arma(values[#names].clone())?,
            )*})
        })
    }
}

fn tuple_struct(fields: &[FieldUnnamed], _attributes: &ContainerAttributes) -> Result<TokenStream> {
    let indexes = fields.indexes();
    let types = fields.types();

    Ok(quote! {
        let values: (#(
            #types,
        )*) = arma_rs::FromArma::from_arma(source)?;
        Ok(Self (#(
            values.#indexes,
        )*))
    })
}

fn newtype_struct(_field: &FieldUnnamed, _attributes: &ContainerAttributes) -> Result<TokenStream> {
    Ok(quote! {
        Ok(Self (arma_rs::FromArma::from_arma(source)?))
    })
}
