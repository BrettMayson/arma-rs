mod attr;
mod data;
mod struct_from;
mod struct_into;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{DeriveInput, Error, Result};

use attr::*;
use data::*;
use struct_from::from_impl_body;
use struct_into::into_impl_body;

pub fn generate_into_arma(input: DeriveInput) -> Result<TokenStream> {
    let container = ContainerData::from_input(&input)?;
    match container.data {
        Data::Enum => Err(Error::new(Span::call_site(), "Enums aren't supported")),
        Data::Struct(data) => {
            let ident = container.ident;
            let body = into_impl_body(&data, &container.attributes)?;
            let (impl_generics, ty_generics, where_clause) = container.generics.split_for_impl();

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
    let container = ContainerData::from_input(&input)?;
    match container.data {
        Data::Enum => Err(Error::new(Span::call_site(), "Enums aren't supported")),
        Data::Struct(data) => {
            let ident = container.ident;
            let body = from_impl_body(&data, &container.attributes)?;
            let (impl_generics, ty_generics, where_clause) = container.generics.split_for_impl();

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
