use proc_macro2::TokenStream;
use quote::quote;

use crate::derive::{
    attributes::ContainerAttributes,
    data::{DataStruct, FieldNamed, FieldUnnamed},
};

pub fn impl_from_arma(attributes: &ContainerAttributes, data: &DataStruct) -> TokenStream {
    match &data {
        DataStruct::Map(fields) => map_struct(attributes, fields),
        DataStruct::Tuple(fields) => tuple_struct(attributes, fields),
        DataStruct::NewType(field) => newtype_struct(attributes, field),
    }
}

fn map_struct(attributes: &ContainerAttributes, fields: &[FieldNamed]) -> TokenStream {
    if *attributes.transparent.value() {
        let field = fields.first().unwrap();
        let ident = &field.ident;
        return quote! {
            Ok(Self {
                #ident: arma_rs::FromArma::from_arma(input_string)?,
            })
        };
    }

    let mut setup = TokenStream::new();
    setup.extend(quote! {
        let mut input_values: std::collections::HashMap<String, arma_rs::Value> =
            arma_rs::FromArma::from_arma(input_string)?;
    });
    if *attributes.default.value() {
        setup.extend(quote! {
            let container_default = Self::default();
        });
    };

    let field_assignments = fields.iter().map(|field| {
        let (ident, name, ty) = (&field.ident, &field.name, &field.ty);
        let none_match = if *field.attributes.default.value() {
            quote!(#ty::default())
        } else if *attributes.default.value() {
            quote!(container_default.#ident)
        } else {
            quote!(return Err(arma_rs::FromArmaError::MapMissingField(#name.to_string())))
        };

        quote! {
            #ident: match input_values.remove(#name) {
                Some(value) => arma_rs::FromArma::from_arma(value.to_string())?,
                None => #none_match,
            }
        }
    });

    let check_unknown = quote! {
        if let Some(unknown) = input_values.keys().next() {
            return Err(arma_rs::FromArmaError::MapUnknownField(unknown.clone()));
        }
    };
    quote! {
        #setup
        let result = Self {
            #(#field_assignments),*
        };

        #check_unknown
        Ok(result)
    }
}

fn tuple_struct(attributes: &ContainerAttributes, fields: &[FieldUnnamed]) -> TokenStream {
    let mut setup = TokenStream::new();
    setup.extend(quote! {
        let input_values: Vec<arma_rs::Value> = arma_rs::FromArma::from_arma(input_string)?;
        let mut input_values = input_values.into_iter();
    });
    if *attributes.default.value() {
        setup.extend(quote! {
            let container_default = Self::default();
        });
    };

    let expected_len = fields.len();
    let field_assignments = fields.iter().map(|field| {
        let (index, ty) = (&field.index, &field.ty);
        let none_match = if *field.attributes.default.value() {
            quote!(#ty::default())
        } else if *attributes.default.value() {
            quote!(container_default.#index)
        } else {
            quote!(return Err(arma_rs::FromArmaError::SizeMismatch {
                expected: #expected_len,
                actual: #index,
            }))
        };

        quote! {
            match input_values.next() {
                Some(value) => arma_rs::FromArma::from_arma(value.to_string())?,
                None => #none_match,
            }
        }
    });

    let check_unknown = quote! {
        if let Some(unknown) = input_values.next() {
            return Err(arma_rs::FromArmaError::SizeMismatch {
                expected: #expected_len,
                actual: #expected_len + 1 + input_values.len(),
            });
        }
    };
    quote! {
        #setup
        let result = Self (
            #(#field_assignments),*
        );

        #check_unknown
        Ok(result)
    }
}

fn newtype_struct(_attributes: &ContainerAttributes, _field: &FieldUnnamed) -> TokenStream {
    quote! {
        Ok(Self (arma_rs::FromArma::from_arma(input_string)?))
    }
}
