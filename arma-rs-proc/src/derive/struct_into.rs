use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, Result};

use super::*;

pub fn into_impl_body(data: &DataStruct, attributes: &ContainerAttributes) -> Result<TokenStream> {
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

    if attributes.transparent {
        if fields.len() > 1 {
            return Err(Error::new(
                Span::call_site(),
                "#[arma(transparent)] structs must have exactly one field",
            ));
        }

        let ident = idents[0];
        Ok(quote! {
            self.#ident.to_arma()
        })
    } else {
        let names = fields.names();

        Ok(quote! {
            std::collections::HashMap::<String, arma_rs::Value>::from([#(
                (#names.to_string(), self.#idents.to_arma()),
            )*]).to_arma()
        })
    }
}

fn tuple_struct(fields: &[FieldUnnamed], _attributes: &ContainerAttributes) -> Result<TokenStream> {
    let indices: Vec<_> = fields.iter().map(|f| &f.index).collect();

    Ok(quote! {
        Vec::<arma_rs::Value>::from([#(
            self.#indices.to_arma(),
        )*]).to_arma()
    })
}

fn newtype_struct(_field: &FieldUnnamed, _attributes: &ContainerAttributes) -> Result<TokenStream> {
    Ok(quote! {
        self.0.to_arma()
    })
}