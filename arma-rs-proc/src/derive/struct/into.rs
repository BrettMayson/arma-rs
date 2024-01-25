use proc_macro2::TokenStream;
use quote::quote;

use crate::derive::{
    attributes::ContainerAttributes,
    data::{DataStruct, Field, FieldNamed, FieldUnnamed},
};

pub fn impl_into_arma(attributes: &ContainerAttributes, data: &DataStruct) -> TokenStream {
    match &data {
        DataStruct::Map(fields) => map_struct(attributes, fields),
        DataStruct::Tuple(fields) => tuple_struct(attributes, fields),
        DataStruct::NewType(field) => newtype_struct(attributes, field),
    }
}

fn map_struct(attributes: &ContainerAttributes, fields: &[FieldNamed]) -> TokenStream {
    if *attributes.transparent.value() {
        return newtype_struct(attributes, fields.first().unwrap());
    }

    let (idents, names): (Vec<_>, Vec<_>) = fields.iter().map(|f| (&f.ident, &f.name)).unzip();
    quote! {
        std::collections::HashMap::<String, arma_rs::Value>::from([#(
            (#names.to_string(), self.#idents.to_arma()),
        )*]).to_arma()
    }
}

fn tuple_struct(_attributes: &ContainerAttributes, fields: &[FieldUnnamed]) -> TokenStream {
    let indexes = fields.iter().map(|f| &f.index);
    quote! {
        Vec::<arma_rs::Value>::from([#(
            self.#indexes.to_arma(),
        )*]).to_arma()
    }
}

fn newtype_struct(_attributes: &ContainerAttributes, field: &impl Field) -> TokenStream {
    let token = &field.token();

    quote! {
        self.#token.to_arma()
    }
}
