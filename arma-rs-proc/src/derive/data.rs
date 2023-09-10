use proc_macro2::Span;
use syn::{Error, Result};

use super::{ContainerAttributes, FieldAttributes};

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

impl ContainerData {
    pub fn from_input(input: syn::DeriveInput) -> Result<Self> {
        let attributes = ContainerAttributes::from_attrs(&input.attrs)?;
        let data = match input.data {
            syn::Data::Struct(data) => Data::Struct(DataStruct::new(data)?),
            syn::Data::Enum(_) => Err(Error::new(Span::call_site(), "enums aren't supported"))?,
            syn::Data::Union(_) => Err(Error::new(Span::call_site(), "unions aren't supported"))?,
        };

        Ok(Self {
            attributes,
            ident: input.ident,
            data,
            generics: input.generics,
        })
    }
}

impl DataStruct {
    fn new(data: syn::DataStruct) -> Result<Self> {
        match data.fields {
            syn::Fields::Unit => Err(Error::new(
                Span::call_site(),
                "unit-like structs aren't supported",
            )),
            syn::Fields::Named(fields) => {
                if fields.named.is_empty() {
                    return Err(Error::new(
                        Span::call_site(),
                        "unit-like structs aren't supported",
                    ));
                }

                let fields = fields
                    .named
                    .into_iter()
                    .map(FieldNamed::new)
                    .collect::<Result<_>>()?;
                Ok(Self::Map(fields))
            }
            syn::Fields::Unnamed(fields) => {
                if fields.unnamed.is_empty() {
                    return Err(Error::new(
                        Span::call_site(),
                        "unit-like structs aren't supported",
                    ));
                }

                if fields.unnamed.len() == 1 {
                    let field = FieldUnnamed::new(fields.unnamed[0].clone(), 0)?;
                    Ok(Self::NewType(field))
                } else {
                    let fields = fields
                        .unnamed
                        .into_iter()
                        .enumerate()
                        .map(|(i, f)| FieldUnnamed::new(f, i))
                        .collect::<Result<_>>()?;
                    Ok(Self::Tuple(fields))
                }
            }
        }
    }
}

impl FieldNamed {
    fn new(field: syn::Field) -> Result<Self> {
        let ident = field.ident.unwrap();
        let name = ident.to_string();
        Ok(Self {
            attributes: FieldAttributes::from_attrs(&field.attrs)?,
            ident,
            name,
            ty: field.ty,
        })
    }
}

impl FieldUnnamed {
    fn new(field: syn::Field, index: usize) -> Result<Self> {
        Ok(Self {
            attributes: FieldAttributes::from_attrs(&field.attrs)?,
            index: syn::Index::from(index),
            ty: field.ty,
        })
    }
}
