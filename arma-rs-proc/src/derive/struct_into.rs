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
    if attributes.transparent {
        if fields.len() > 1 {
            return Err(Error::new(
                Span::call_site(),
                "#[arma(transparent)] structs must have exactly one field",
            ));
        }

        let ident = &fields.first().unwrap().ident;
        return Ok(quote! {
            self.#ident.to_arma()
        });
    }

    let (idents, names): (Vec<_>, Vec<_>) = fields.iter().map(|f| (&f.ident, &f.name)).unzip();
    Ok(quote! {
        std::collections::HashMap::<String, arma_rs::Value>::from([#(
            (#names.to_string(), self.#idents.to_arma()),
        )*]).to_arma()
    })
}

fn tuple_struct(fields: &[FieldUnnamed], _attributes: &ContainerAttributes) -> Result<TokenStream> {
    let indexes = fields.iter().map(|f| &f.index);
    Ok(quote! {
        Vec::<arma_rs::Value>::from([#(
            self.#indexes.to_arma(),
        )*]).to_arma()
    })
}

fn newtype_struct(_field: &FieldUnnamed, _attributes: &ContainerAttributes) -> Result<TokenStream> {
    Ok(quote! {
        self.0.to_arma()
    })
}
