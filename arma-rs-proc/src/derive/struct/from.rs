use proc_macro2::TokenStream;
use quote::quote;

use crate::derive::{
    attributes::ContainerAttributes,
    data::{DataStruct, Field, FieldNamed, FieldUnnamed},
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
        return newtype_struct(attributes, fields.first().unwrap());
    }

    let mut setup = TokenStream::new();
    setup.extend(quote! {
        let mut input_as_values = std::collections::HashMap::<String, arma_rs::Value>::default();

        let input_pairs: Vec<(String, arma_rs::Value)> = FromArma::from_arma(func_input)?;
        for (k, v) in input_pairs {
            if input_as_values.insert(k.clone(), v).is_some() {
                return Err(arma_rs::FromArmaError::DuplicateField(k));
            }
        }
    });
    if *attributes.default.value() {
        setup.extend(quote! {
            let container_default = Self::default();
        });
    };

    let field_assignments = fields.iter().map(|field| {
        let (ident, name, ty) = (&field.ident, &field.name, &field.ty);

        let some_match = if *field.attributes.stringify.value() {
            quote!(input_value.to_string().parse::<#ty>().map_err(arma_rs::FromArmaError::custom)?)
        } else {
            quote!(arma_rs::FromArma::from_arma(input_value.to_string())?)
        };

        let none_match = if *field.attributes.default.value() {
            quote!(#ty::default())
        } else if *attributes.default.value() {
            quote!(container_default.#ident)
        } else {
            quote!(return Err(arma_rs::FromArmaError::MissingField(#name.to_string())))
        };

        quote! {
            #ident: match input_as_values.remove(#name) {
                Some(input_value) => #some_match,
                None => #none_match,
            }
        }
    });

    let check_unknown = quote! {
        if let Some(unknown) = input_as_values.keys().next() {
            return Err(arma_rs::FromArmaError::UnknownField(unknown.clone()));
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
        let input_as_values: Vec<arma_rs::Value> = arma_rs::FromArma::from_arma(func_input)?;
        let mut input_as_values = input_as_values.into_iter();
    });
    if *attributes.default.value() {
        setup.extend(quote! {
            let container_default = Self::default();
        });
    };

    let expected_len = fields.len();
    let field_assignments = fields.iter().map(|field| {
        let (index, ty) = (&field.index, &field.ty);

        let some_match = if *field.attributes.stringify.value() {
            quote!(input_value.to_string().parse::<#ty>().map_err(arma_rs::FromArmaError::custom)?)
        } else {
            quote!(arma_rs::FromArma::from_arma(input_value.to_string())?)
        };

        let none_match = if *field.attributes.default.value() {
            quote!(#ty::default())
        } else if *attributes.default.value() {
            quote!(container_default.#index)
        } else {
            quote!(return Err(arma_rs::FromArmaError::InvalidLength {
                expected: #expected_len,
                actual: #index,
            }))
        };

        quote! {
            match input_as_values.next() {
                Some(input_value) => #some_match,
                None => #none_match,
            }
        }
    });

    let check_unknown = quote! {
        if let Some(unknown) = input_as_values.next() {
            return Err(arma_rs::FromArmaError::InvalidLength {
                expected: #expected_len,
                actual: #expected_len + 1 + input_as_values.len(),
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

fn newtype_struct(_attributes: &ContainerAttributes, field: &impl Field) -> TokenStream {
    let token = field.token();
    if *field.attributes().stringify.value() {
        let ty = &field.ty();
        return quote! {
            Ok(Self {
                #token: func_input.parse::<#ty>().map_err(arma_rs::FromArmaError::custom)?
            })
        };
    }

    quote! {
        Ok(Self {
            #token: arma_rs::FromArma::from_arma(func_input)?
        })
    }
}
