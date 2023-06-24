use syn::Result;

use super::ContainerAttributes;

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
    pub ident: syn::Ident,
    pub name: String,
    pub ty: syn::Type,
}

pub struct FieldUnnamed {
    pub index: syn::Index,
    pub ty: syn::Type,
}

impl ContainerData {
    pub fn from_input(input: &syn::DeriveInput) -> Result<Self> {
        let data = match input.data.clone() {
            syn::Data::Struct(data) => Data::Struct(DataStruct::new(data)),
            syn::Data::Enum(_) => Data::Enum,
            syn::Data::Union(_) => Data::Union,
        };
        Ok(Self {
            attributes: ContainerAttributes::from_attrs(&input.attrs)?,
            ident: input.ident.clone(),
            data,
            generics: input.generics.clone(),
        })
    }
}

impl DataStruct {
    fn new(data: syn::DataStruct) -> Self {
        match data.fields {
            syn::Fields::Unit => Self::Unit,
            syn::Fields::Named(fields) => {
                if fields.named.is_empty() {
                    return Self::Unit;
                }

                Self::Map(Fields(
                    fields.named.into_iter().map(FieldNamed::new).collect(),
                ))
            }
            syn::Fields::Unnamed(fields) => {
                match fields.unnamed.len() {
                    0 => return Self::Unit,
                    1 => return Self::NewType(FieldUnnamed::new(fields.unnamed[0].clone(), 0)),
                    _ => (),
                }

                Self::Tuple(Fields(
                    fields
                        .unnamed
                        .into_iter()
                        .enumerate()
                        .map(|(i, f)| FieldUnnamed::new(f, i))
                        .collect(),
                ))
            }
        }
    }
}

impl FieldNamed {
    fn new(field: syn::Field) -> Self {
        let ident = field.ident.unwrap();
        let name = ident.to_string();
        let ty = field.ty;
        Self { ident, name, ty }
    }
}

impl FieldUnnamed {
    fn new(field: syn::Field, index: usize) -> Self {
        let index = syn::Index::from(index);
        let ty = field.ty;
        Self { index, ty }
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
