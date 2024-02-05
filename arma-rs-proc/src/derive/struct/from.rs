use proc_macro2::TokenStream;
use quote::quote;

use crate::derive::{
    attributes::ContainerAttributes,
    data::{Field, FieldNamed, FieldUnnamed, StructData},
};

pub fn impl_from_arma(attributes: &ContainerAttributes, data: &StructData) -> TokenStream {
    // For simplicity sake we assume that theres no conflicts and everything has already been validated
    match &data {
        StructData::Map(fields) => map_struct(attributes, fields),
        StructData::Tuple(fields) => tuple_struct(attributes, fields),
        StructData::NewType(field) => newtype_struct(attributes, field),
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
            let container_default: Self = std::default::Default::default();
        });
    };

    let field_bodies = fields.iter().map(|field| {
        let (ident, name) = (&field.ident, &field.name);

        let some_match = if *field.attributes.from_str.value() {
            quote!(input_value
                .to_string()
                .parse()
                .map_err(arma_rs::FromArmaError::custom)?)
        } else {
            quote!(arma_rs::FromArma::from_arma(input_value.to_string())?)
        };

        let none_match = if *field.attributes.default.value() {
            quote!(std::default::Default::default())
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
            #(#field_bodies),*
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
            let container_default: Self = std::default::Default::default();
        });
    };

    let expected_len = fields.len();
    let field_bodies = fields.iter().map(|field| {
        let index = &field.index;

        let some_match = if *field.attributes.from_str.value() {
            quote!(input_value
                .to_string()
                .parse()
                .map_err(arma_rs::FromArmaError::custom)?)
        } else {
            quote!(arma_rs::FromArma::from_arma(input_value.to_string())?)
        };

        let none_match = if *field.attributes.default.value() {
            quote!(std::default::Default::default())
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
        let remaining = input_as_values.len();
        if remaining > 0 {
            return Err(arma_rs::FromArmaError::InvalidLength {
                expected: #expected_len,
                actual: #expected_len + remaining,
            });
        }
    };
    quote! {
        #setup
        let result = Self (
            #(#field_bodies),*
        );

        #check_unknown
        Ok(result)
    }
}

fn newtype_struct(_attributes: &ContainerAttributes, field: &impl Field) -> TokenStream {
    let token = field.token();

    let field_body = if *field.attributes().from_str.value() {
        quote!(func_input.parse().map_err(arma_rs::FromArmaError::custom)?)
    } else {
        quote!(arma_rs::FromArma::from_arma(func_input)?)
    };

    quote! {
        Ok(Self {
            #token: #field_body
        })
    }
}
