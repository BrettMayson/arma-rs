pub struct ContainerData {
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
    Map(Vec<FieldNamed>),
    Tuple(Vec<FieldUnnamed>),
    NewType(FieldUnnamed),
    Unit,
}

pub struct FieldNamed {
    pub ident: syn::Ident,
    pub name: String,
    pub ty: syn::Type,
}

pub struct FieldUnnamed {
    pub index: syn::Index,
    pub ty: syn::Type,
}

impl From<syn::DeriveInput> for ContainerData {
    fn from(input: syn::DeriveInput) -> Self {
        let ident = input.ident;
        let generics = input.generics;
        let data = match input.data {
            syn::Data::Struct(data) => Data::Struct(DataStruct::new(data)),
            syn::Data::Enum(_) => Data::Enum,
            syn::Data::Union(_) => Data::Union,
        };
        Self {
            ident,
            data,
            generics,
        }
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

                Self::Map(fields.named.into_iter().map(FieldNamed::new).collect())
            }
            syn::Fields::Unnamed(fields) => {
                match fields.unnamed.len() {
                    0 => return Self::Unit,
                    1 => return Self::NewType(FieldUnnamed::new(fields.unnamed[0].clone(), 0)),
                    _ => (),
                }

                Self::Tuple(
                    fields
                        .unnamed
                        .into_iter()
                        .enumerate()
                        .map(|(i, f)| FieldUnnamed::new(f, i))
                        .collect(),
                )
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

pub trait FieldNamedUtil {
    fn idents(&self) -> Vec<&syn::Ident>;
    fn names(&self) -> Vec<&str>;
    fn types(&self) -> Vec<&syn::Type>;
}

impl FieldNamedUtil for Vec<FieldNamed> {
    fn idents(&self) -> Vec<&syn::Ident> {
        self.iter().map(|f| &f.ident).collect()
    }

    fn names(&self) -> Vec<&str> {
        self.iter().map(|f| &f.name[..]).collect()
    }

    fn types(&self) -> Vec<&syn::Type> {
        self.iter().map(|f| &f.ty).collect()
    }
}

pub trait FieldUnnamedUtil {
    fn indexes(&self) -> Vec<&syn::Index>;
    fn types(&self) -> Vec<&syn::Type>;
}

impl FieldUnnamedUtil for Vec<FieldUnnamed> {
    fn indexes(&self) -> Vec<&syn::Index> {
        self.iter().map(|f| &f.index).collect()
    }

    fn types(&self) -> Vec<&syn::Type> {
        self.iter().map(|f| &f.ty).collect()
    }
}
