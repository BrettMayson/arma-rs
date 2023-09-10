mod attributes;
mod data;
mod r#struct;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Result};

use attributes::{Attribute, ContainerAttributes, FieldAttributes};
use data::{ContainerData, Data, DataStruct, FieldNamed, FieldUnnamed};

pub fn generate_into_arma(input: DeriveInput) -> Result<TokenStream> {
    let container = ContainerData::from_input(input)?;
    let body = match container.data {
        Data::Struct(data) => {
            r#struct::validate_attributes(&data, &container.attributes)?;
            r#struct::into_impl_body(&data, &container.attributes)
        }
    };

    let ident = container.ident;
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

pub fn generate_from_arma(input: DeriveInput) -> Result<TokenStream> {
    let container = ContainerData::from_input(input)?;
    let body = match container.data {
        Data::Struct(data) => {
            r#struct::validate_attributes(&data, &container.attributes)?;
            r#struct::from_impl_body(&data, &container.attributes)
        }
    };

    let ident = container.ident;
    let (impl_generics, ty_generics, where_clause) = container.generics.split_for_impl();
    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics arma_rs::FromArma for #ident #ty_generics #where_clause {
            fn from_arma(input_string: String) -> Result<Self, arma_rs::FromArmaError> {
                #body
            }
        }
    })
}
