use proc_macro2::TokenStream;
use quote::quote;

use super::{ContainerAttributes, DataStruct, FieldNamed, FieldUnnamed};

pub fn from_impl_body(data: &DataStruct, attributes: &ContainerAttributes) -> TokenStream {
    match &data {
        DataStruct::Map(fields) => map_struct(fields, attributes),
        DataStruct::Tuple(fields) => tuple_struct(fields, attributes),
        DataStruct::NewType(field) => newtype_struct(field, attributes),
    }
}

fn map_struct(fields: &[FieldNamed], attributes: &ContainerAttributes) -> TokenStream {
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

fn tuple_struct(fields: &[FieldUnnamed], attributes: &ContainerAttributes) -> TokenStream {
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

fn newtype_struct(_field: &FieldUnnamed, _attributes: &ContainerAttributes) -> TokenStream {
    quote! {
        Ok(Self (arma_rs::FromArma::from_arma(input_string)?))
    }
}
