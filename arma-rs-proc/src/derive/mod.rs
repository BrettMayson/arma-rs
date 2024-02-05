mod attributes;
mod data;
mod r#struct;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Result};

use data::ContainerData;

pub fn generate_from_arma(input: DeriveInput) -> Result<TokenStream> {
    let container = parse_container_data(input)?;
    let body = container.impl_from_arma();

    let ident = container.ident;
    let (impl_generics, ty_generics, where_clause) = container.generics.split_for_impl();
    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics arma_rs::FromArma for #ident #ty_generics #where_clause {
            fn from_arma(func_input: String) -> Result<Self, arma_rs::FromArmaError> {
                #body
            }
        }
    })
}

pub fn generate_into_arma(input: DeriveInput) -> Result<TokenStream> {
    let container = parse_container_data(input)?;
    let body = container.impl_into_arma();

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

fn parse_container_data(input: DeriveInput) -> Result<ContainerData> {
    let mut errors = CombinedErrors::new();
    let container = ContainerData::from_input(&mut errors, input)?;
    container.validate_attributes(&mut errors);
    errors.into_result().and(Ok(container))
}

pub struct CombinedErrors {
    root: Option<syn::Error>,
}

impl CombinedErrors {
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
