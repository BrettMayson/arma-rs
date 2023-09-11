mod attributes;
mod data;
mod r#struct;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Result};

use attributes::{Attribute, ContainerAttributes, FieldAttributes};
use data::{ContainerData, Data, DataStruct, FieldNamed, FieldUnnamed};

pub struct CombinedError {
    root: Option<syn::Error>,
}

impl CombinedError {
    fn new() -> Self {
        Self { root: None }
    }

    pub fn add(&mut self, error: syn::Error) {
        match &mut self.root {
            Some(root) => root.combine(error.clone()),
            None => self.root = Some(error.clone()),
        }
    }

    pub fn into_result(self) -> Result<()> {
        match self.root {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }
}

pub fn generate_into_arma(input: DeriveInput) -> Result<TokenStream> {
    let container = parse_container_data(input)?;
    let body = match container.data {
        Data::Struct(data) => r#struct::into_impl_body(&data, &container.attributes),
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
    let container = parse_container_data(input)?;
    let body = match container.data {
        Data::Struct(data) => r#struct::from_impl_body(&data, &container.attributes),
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

fn parse_container_data(input: DeriveInput) -> Result<ContainerData> {
    let mut errors = CombinedError::new();
    let container = ContainerData::from_input(&mut errors, input)?;
    match container.data {
        Data::Struct(ref data) => {
            r#struct::validate_attributes(&mut errors, data, &container.attributes);
        }
    }
    errors.into_result().map(|_| container)
}
