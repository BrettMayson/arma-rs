use proc_macro2::TokenStream;
use quote::quote;

use super::{ContainerAttributes, DataStruct, FieldNamed, FieldUnnamed};

pub fn into_impl_body(data: &DataStruct, attributes: &ContainerAttributes) -> TokenStream {
    match &data {
        DataStruct::Map(fields) => map_struct(fields, attributes),
        DataStruct::Tuple(fields) => tuple_struct(fields, attributes),
        DataStruct::NewType(field) => newtype_struct(field, attributes),
    }
}

fn map_struct(fields: &[FieldNamed], attributes: &ContainerAttributes) -> TokenStream {
    if *attributes.transparent.value() {
        let ident = &fields.first().unwrap().ident;
        return quote! {
            self.#ident.to_arma()
        };
    }

    let (idents, names): (Vec<_>, Vec<_>) = fields.iter().map(|f| (&f.ident, &f.name)).unzip();
    quote! {
        std::collections::HashMap::<String, arma_rs::Value>::from([#(
            (#names.to_string(), self.#idents.to_arma()),
        )*]).to_arma()
    }
}

fn tuple_struct(fields: &[FieldUnnamed], _attributes: &ContainerAttributes) -> TokenStream {
    let indexes = fields.iter().map(|f| &f.index);
    quote! {
        Vec::<arma_rs::Value>::from([#(
            self.#indexes.to_arma(),
        )*]).to_arma()
    }
}

fn newtype_struct(_field: &FieldUnnamed, _attributes: &ContainerAttributes) -> TokenStream {
    quote! {
        self.0.to_arma()
    }
}
