use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::derive::{
    attributes::{ContainerAttributes, FieldAttributes},
    data::{FieldNamed, FieldUnnamed, StructData},
};

pub fn impl_into_arma(attributes: &ContainerAttributes, data: &StructData) -> TokenStream {
    match &data {
        StructData::Map(fields) => map_struct(attributes, fields),
        StructData::Tuple(fields) => tuple_struct(attributes, fields),
        StructData::NewType(field) => newtype_struct(
            attributes,
            &field.index.to_token_stream(),
            &field.attributes,
        ),
    }
}

fn map_struct(attributes: &ContainerAttributes, fields: &[FieldNamed]) -> TokenStream {
    if *attributes.transparent.value() {
        let field = fields.first().unwrap();
        return newtype_struct(
            attributes,
            &field.ident.to_token_stream(),
            &field.attributes,
        );
    }

    let field_bodies = fields.iter().map(|field| {
        let (ident, name) = (&field.ident, &field.name);

        let (key, value) = if *field.attributes.to_string.value() {
            (quote!(#name.to_string()), quote!(self.#ident.to_string()))
        } else {
            (quote!(#name.to_string()), quote!(self.#ident))
        };

        quote!((#key, arma_rs::IntoArma::to_arma(&#value)))
    });

    quote! {
        std::collections::HashMap::<String, arma_rs::Value>::from([
            #(#field_bodies),*
        ]).to_arma()
    }
}

fn tuple_struct(_attributes: &ContainerAttributes, fields: &[FieldUnnamed]) -> TokenStream {
    let field_bodies = fields.iter().map(|field| {
        let index = &field.index;

        if *field.attributes.to_string.value() {
            quote!(self.#index.to_string())
        } else {
            quote!(self.#index)
        }
    });

    quote! {
        Vec::<arma_rs::Value>::from([
            #(arma_rs::IntoArma::to_arma(&#field_bodies)),*
        ]).to_arma()
    }
}

fn newtype_struct(
    _attributes: &ContainerAttributes,
    field_token: &TokenStream,
    field_attributes: &FieldAttributes,
) -> TokenStream {
    let field_body = if *field_attributes.to_string.value() {
        quote!(self.#field_token.to_string())
    } else {
        quote!(self.#field_token)
    };

    quote! {
        arma_rs::IntoArma::to_arma(&#field_body)
    }
}
