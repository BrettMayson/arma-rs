use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{Error, Result};

use crate::derive::{
    attributes::{parse_attributes, ContainerAttributes, FieldAttributes},
    r#struct, CombinedError,
};

pub struct FieldNamed {
    pub attributes: FieldAttributes,
    pub ident: syn::Ident,
    pub name: String,
    pub ty: syn::Type,
}

pub struct FieldUnnamed {
    pub attributes: FieldAttributes,
    pub index: syn::Index,
    pub ty: syn::Type,
}

pub trait Field {
    fn attributes(&self) -> &FieldAttributes;
    fn token(&self) -> TokenStream;
    fn ty(&self) -> &syn::Type;
}

impl FieldNamed {
    pub fn new(errors: &mut CombinedError, field: syn::Field) -> Self {
        let ident = field.ident.unwrap();
        let name = ident.to_string();
        Self {
            attributes: parse_attributes::<FieldAttributes>(errors, &field.attrs),
            ident,
            name,
            ty: field.ty,
        }
    }
}

impl Field for FieldNamed {
    fn attributes(&self) -> &FieldAttributes {
        &self.attributes
    }

    fn token(&self) -> TokenStream {
        self.ident.to_token_stream()
    }

    fn ty(&self) -> &syn::Type {
        &self.ty
    }
}

impl FieldUnnamed {
    pub fn new(errors: &mut CombinedError, field: syn::Field, index: usize) -> Self {
        Self {
            attributes: parse_attributes::<FieldAttributes>(errors, &field.attrs),
            index: syn::Index::from(index),
            ty: field.ty,
        }
    }
}

impl Field for FieldUnnamed {
    fn attributes(&self) -> &FieldAttributes {
        &self.attributes
    }

    fn token(&self) -> TokenStream {
        self.index.to_token_stream()
    }

    fn ty(&self) -> &syn::Type {
        &self.ty
    }
}

pub struct ContainerData {
    pub attributes: ContainerAttributes,
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub data: Data,
}

pub enum Data {
    Struct(DataStruct),
}

pub enum DataStruct {
    Map(Vec<FieldNamed>),
    Tuple(Vec<FieldUnnamed>),
    NewType(FieldUnnamed),
}

impl ContainerData {
    pub fn from_input(errors: &mut CombinedError, input: syn::DeriveInput) -> Result<Self> {
        let data = match input.data {
            syn::Data::Struct(data) => Data::Struct(DataStruct::new(errors, data)?),
            syn::Data::Enum(_) => Err(Error::new(Span::call_site(), "enums aren't supported"))?,
            syn::Data::Union(_) => Err(Error::new(Span::call_site(), "unions aren't supported"))?,
        };
        let attributes = parse_attributes::<ContainerAttributes>(errors, &input.attrs);

        Ok(Self {
            attributes,
            ident: input.ident,
            generics: input.generics,
            data,
        })
    }

    pub fn validate_attributes(&self, errors: &mut CombinedError) {
        match self.data {
            Data::Struct(ref data) => {
                r#struct::validate_attributes(errors, &self.attributes, data);
            }
        }
    }

    pub fn impl_into_arma(&self) -> TokenStream {
        match self.data {
            Data::Struct(ref data) => r#struct::impl_into_arma(&self.attributes, data),
        }
    }

    pub fn impl_from_arma(&self) -> TokenStream {
        match self.data {
            Data::Struct(ref data) => r#struct::impl_from_arma(&self.attributes, data),
        }
    }
}
