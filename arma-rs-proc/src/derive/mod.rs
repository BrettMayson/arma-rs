mod data;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{DeriveInput, Error, Result};

use data::*;

pub fn generate_into_arma(input: DeriveInput) -> Result<TokenStream> {
    let input = ContainerData::from(input);
    match input.data {
        Data::Enum => Err(Error::new(Span::call_site(), "Enums aren't supported")),
        Data::Struct(data) => {
            let ident = input.ident;
            let body = struct_into_arma(&data)?;
            let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
            Ok(quote! {
                #[automatically_derived]
                impl #impl_generics arma_rs::IntoArma for #ident #ty_generics #where_clause {
                    fn to_arma(&self) -> arma_rs::Value {
                        #body
                    }
                }
            })
        }
        Data::Union => Err(Error::new(Span::call_site(), "Unions aren't supported")),
    }
}

pub fn generate_from_arma(input: DeriveInput) -> Result<TokenStream> {
    let input = ContainerData::from(input);
    match input.data {
        Data::Enum => Err(Error::new(Span::call_site(), "Enums aren't supported")),
        Data::Struct(data) => {
            let ident = input.ident;
            let body = struct_from_arma_body(&data)?;
            let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
            Ok(quote! {
                #[automatically_derived]
                impl #impl_generics arma_rs::FromArma for #ident #ty_generics #where_clause {
                    fn from_arma(source: String) -> Result<Self, arma_rs::FromArmaError> {
                        #body
                    }
                }
            })
        }
        Data::Union => Err(Error::new(Span::call_site(), "Unions aren't supported")),
    }
}

fn struct_into_arma(data: &DataStruct) -> Result<TokenStream> {
    match &data {
        DataStruct::Unit => Err(Error::new(
            Span::call_site(),
            "Unit struct's aren't supported",
        )),
        DataStruct::Map(fields) => {
            let idents = fields.idents();
            let names = fields.names();
            Ok(quote! {
                std::collections::HashMap::<String, arma_rs::Value>::from([#(
                    (#names.to_string(), self.#idents.to_arma() ),
                )*]).to_arma()
            })
        }
        DataStruct::Tuple(fields) => {
            let indices: Vec<_> = fields.iter().map(|f| &f.index).collect();
            Ok(quote! {
                Vec::<arma_rs::Value>::from([#(
                    self.#indices.to_arma(),
                )*]).to_arma()
            })
        }
        DataStruct::NewType(_) => Ok(quote! {
            self.0.to_arma()
        }),
    }
}

fn struct_from_arma_body(data: &DataStruct) -> Result<TokenStream> {
    match &data {
        DataStruct::Unit => Err(Error::new(
            Span::call_site(),
            "Unit struct's aren't supported",
        )),
        DataStruct::Map(fields) => {
            let idents = fields.idents();
            let names = fields.names();
            let count = fields.len();
            Ok(quote! {
                let values: std::collections::HashMap<String, String> = arma_rs::FromArma::from_arma(source)?;
                if values.len() != #count {
                    return Err(arma_rs::FromArmaError::SizeMismatch {
                        expected: #count,
                        actual: values.len(),
                    })
                }
                Ok(Self {#(
                    #idents: arma_rs::FromArma::from_arma(values[#names].clone())?,
                )*})
            })
        }
        DataStruct::Tuple(fields) => {
            let indexes = fields.indexes();
            let types = fields.types();
            Ok(quote! {
                    let values: (#(
                        #types,
                    )*) = arma_rs::FromArma::from_arma(source)?;
                    Ok(Self (#(
                        values.#indexes,
                    )*))
            })
        }
        DataStruct::NewType(_) => Ok(quote! {
            Ok(Self (arma_rs::FromArma::from_arma(source)?))
        }),
    }
}
