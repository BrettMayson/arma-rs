use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, Result};

use super::*;

pub fn from_impl_body(data: &DataStruct, attributes: &ContainerAttributes) -> Result<TokenStream> {
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

        let field = fields.first().unwrap();
        if attributes.default || field.attributes.default {
            return Err(Error::new(
                Span::call_site(),
                "#[arma(transparent)] and #[arma(default)] cannot be used together",
            ));
        }

        let ident = &field.ident;
        return Ok(quote! {
            Ok(Self {
                #ident: arma_rs::FromArma::from_arma(source)?,
            })
        });
    }

    let mut setup = TokenStream::new();
    setup.extend(quote! {
        let mut passed_values: std::collections::HashMap<String, String> =
            arma_rs::FromArma::from_arma(source)?;
    });
    if attributes.default {
        setup.extend(quote! {
            let container_default = Self::default();
        });
    };

    let field_assignments = fields.iter().map(|field| {
        let (ident, name, ty) = (&field.ident, &field.name, &field.ty);
        let none_match = if field.attributes.default {
            quote!(#ty::default())
        } else if attributes.default {
            quote!(container_default.#ident)
        } else {
            quote!(return Err(arma_rs::FromArmaError::MapMissingField(#name.to_string())))
        };

        quote! {
            #ident: match passed_values.remove(#name) {
                Some(value) => arma_rs::FromArma::from_arma(value)?,
                None => #none_match,
            }
        }
    });

    let check_unknown = quote! {
        if let Some(unknown) = passed_values.keys().next() {
            return Err(arma_rs::FromArmaError::MapUnknownField(unknown.clone()));
        }
    };
    Ok(quote! {
        #setup
        let result = Self {
            #(#field_assignments),*
        };

        #check_unknown
        Ok(result)
    })
}

fn tuple_struct(fields: &[FieldUnnamed], attributes: &ContainerAttributes) -> Result<TokenStream> {
    if attributes.default || fields.iter().any(|f| f.attributes.default) {
        return Err(Error::new(
            Span::call_site(),
            "#[arma(default)] can only be used on structs with named fields",
        ));
    }

    let (indexes, types): (Vec<_>, Vec<_>) = fields.iter().map(|f| (&f.index, &f.ty)).unzip();
    Ok(quote! {
        let values: (#(
            #types,
        )*) = arma_rs::FromArma::from_arma(source)?;
        Ok(Self (#(
            values.#indexes,
        )*))
    })
}

fn newtype_struct(field: &FieldUnnamed, attributes: &ContainerAttributes) -> Result<TokenStream> {
    if attributes.default || field.attributes.default {
        return Err(Error::new(
            Span::call_site(),
            "#[arma(default)] can only be used on structs with named fields",
        ));
    }

    Ok(quote! {
        Ok(Self (arma_rs::FromArma::from_arma(source)?))
    })
}
