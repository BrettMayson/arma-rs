use syn::Result;

use super::{parse_attributes, ContainerAttributes, FieldAttributes};

pub struct ContainerData {
    pub attributes: ContainerAttributes,
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub data: Data,
}

pub enum Data {
    Enum, // Not supported
    Struct(DataStruct),
    Union, // Not supported
}

pub enum DataStruct {
    Map(Fields<FieldNamed>),
    Tuple(Fields<FieldUnnamed>),
    NewType(FieldUnnamed),
    Unit,
}

pub struct Fields<T>(Vec<T>);

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
    pub fn from_input(input: &syn::DeriveInput) -> Result<Self> {
        let data = match input.data.clone() {
            syn::Data::Struct(data) => Data::Struct(DataStruct::new(data)?),
            syn::Data::Enum(_) => Data::Enum,
            syn::Data::Union(_) => Data::Union,
        };
        Ok(Self {
            attributes: parse_attributes(&input.attrs)?,
            ident: input.ident.clone(),
            data,
            generics: input.generics.clone(),
        })
    }
}

impl DataStruct {
    fn new(data: syn::DataStruct) -> Result<Self> {
        match data.fields {
            syn::Fields::Unit => Ok(Self::Unit),
            syn::Fields::Named(fields) => {
                if fields.named.is_empty() {
                    return Ok(Self::Unit);
                }

                let fields = fields
                    .named
                    .into_iter()
                    .map(FieldNamed::new)
                    .collect::<Result<_>>()?;
                Ok(Self::Map(Fields(fields)))
            }
            syn::Fields::Unnamed(fields) => {
                if fields.unnamed.is_empty() {
                    return Ok(Self::Unit);
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
                    Ok(Self::Tuple(Fields(fields)))
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
            attributes: parse_attributes(&field.attrs)?,
            ident,
            name,
            ty: field.ty,
        })
    }
}

impl FieldUnnamed {
    fn new(field: syn::Field, index: usize) -> Result<Self> {
        Ok(Self {
            attributes: parse_attributes(&field.attrs)?,
            index: syn::Index::from(index),
            ty: field.ty,
        })
    }
}

impl<T> Fields<T> {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Fields<FieldNamed> {
    pub fn idents(&self) -> Vec<&syn::Ident> {
        self.0.iter().map(|f| &f.ident).collect()
    }

    pub fn names(&self) -> Vec<&str> {
        self.0.iter().map(|f| &f.name[..]).collect()
    }

    // pub fn types(&self) -> Vec<&syn::Type> {
    //     self.0.iter().map(|f| &f.ty).collect()
    // }
}

impl Fields<FieldUnnamed> {
    pub fn indexes(&self) -> Vec<&syn::Index> {
        self.0.iter().map(|f| &f.index).collect()
    }

    pub fn types(&self) -> Vec<&syn::Type> {
        self.0.iter().map(|f| &f.ty).collect()
    }
}
