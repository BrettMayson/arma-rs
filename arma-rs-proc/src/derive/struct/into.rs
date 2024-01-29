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

    let fields = fields.iter().map(|field| {
        let (ident, name) = (&field.ident, &field.name);

        if *field.attributes.stringify.value() {
            quote!((#name.to_string(), self.#ident.to_string().to_arma()))
        } else {
            quote!((#name.to_string(), self.#ident.to_arma()))
        }
    });

    quote! {
        std::collections::HashMap::<String, arma_rs::Value>::from([
            #(#fields),*
        ]).to_arma()
    }
}

fn tuple_struct(_attributes: &ContainerAttributes, fields: &[FieldUnnamed]) -> TokenStream {
    let fields = fields.iter().map(|field| {
        let index = &field.index;

        if *field.attributes.stringify.value() {
            quote!(self.#index.to_string().to_arma())
        } else {
            quote!(self.#index.to_arma())
        }
    });

    quote! {
        Vec::<arma_rs::Value>::from([
            #(#fields),*
        ]).to_arma()
    }
}

fn newtype_struct(_attributes: &ContainerAttributes, field: &impl Field) -> TokenStream {
    let token = &field.token();
    if *field.attributes().stringify.value() {
        return quote! {
            self.#token.to_string().to_arma()
        };
    }

    quote! {
        self.#token.to_arma()
    }
}
