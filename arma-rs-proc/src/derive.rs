use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Error, Fields, Ident, Index, Result};

pub fn generate_into_arma(input: DeriveInput) -> Result<TokenStream> {
    let ident = &input.ident;
    match input.data {
        Data::Enum(_) => Err(Error::new(Span::call_site(), "Enums aren't supported")),
        Data::Struct(data) => {
            let body = struct_into_arma(&data.fields)?;
            Ok(quote! {
                #[automatically_derived]
                impl arma_rs::IntoArma for #ident {
                    fn to_arma(&self) -> arma_rs::Value {
                        #body
                    }
                }
            })
        }
        Data::Union(_) => Err(Error::new(Span::call_site(), "Unions aren't supported")),
    }
}

fn struct_into_arma(fields: &Fields) -> Result<TokenStream> {
    match fields {
        Fields::Unit => Err(Error::new(
            Span::call_site(),
            "Unit struct's aren't supported",
        )),
        Fields::Named(fields) => {
            let count = fields.named.len();
            if count == 0 {
                return Err(Error::new(
                    Span::call_site(),
                    "Unit struct's aren't supported",
                ));
            }

            let field_idents: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
            let field_names: Vec<_> = field_idents
                .iter()
                .map(|f| f.as_ref().unwrap().to_string())
                .collect();
            Ok(quote! {
                std::collections::HashMap::<String, arma_rs::Value>::from([#(
                    (#field_names.to_string(), self.#field_idents.to_arma() ),
                )*]).to_arma()
            })
        }
        Fields::Unnamed(fields) => {
            let count = fields.unnamed.len();
            if count == 0 {
                return Err(Error::new(
                    Span::call_site(),
                    "Unit struct's aren't supported",
                ));
            }

            let field_indices: Vec<_> = (0..count).map(Index::from).collect();
            if count == 1 {
                Ok(quote! {
                    self.0.to_arma()
                })
            } else {
                Ok(quote! {
                    Vec::<arma_rs::Value>::from([#(
                        self.#field_indices.to_arma(),
                    )*]).to_arma()
                })
            }
        }
    }
}

pub fn generate_from_arma(input: DeriveInput) -> Result<TokenStream> {
    let ident = &input.ident;
    match input.data {
        Data::Enum(_) => Err(Error::new(Span::call_site(), "Enums aren't supported")),
        Data::Struct(data) => {
            let body = struct_from_arma_body(ident, &data.fields)?;
            Ok(quote! {
                #[automatically_derived]
                impl arma_rs::FromArma for #ident {
                    fn from_arma(source: String) -> Result<Self, arma_rs::FromArmaError> {
                        #body
                    }
                }
            })
        }
        Data::Union(_) => Err(Error::new(Span::call_site(), "Unions aren't supported")),
    }
}

fn struct_from_arma_body(ident: &Ident, fields: &Fields) -> Result<TokenStream> {
    match fields {
        Fields::Unit => Err(Error::new(
            Span::call_site(),
            "Unit struct's aren't supported",
        )),
        Fields::Named(fields) => {
            let count = fields.named.len();
            if count == 0 {
                return Err(Error::new(
                    Span::call_site(),
                    "Unit struct's aren't supported",
                ));
            }

            let field_idents: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
            let field_names: Vec<_> = field_idents
                .iter()
                .map(|f| f.as_ref().unwrap().to_string())
                .collect();
            Ok(quote! {
                let values: std::collections::HashMap<String, String> = arma_rs::FromArma::from_arma(source)?;
                if values.len() != #count {
                    Err(arma_rs::FromArmaError::SizeMismatch {
                        expected: #count,
                        actual: values.len(),
                    })
                } else {
                    Ok(#ident {#(
                        #field_idents: arma_rs::FromArma::from_arma(values[#field_names].clone())?,
                    )*})
                }
            })
        }
        Fields::Unnamed(fields) => {
            let count = fields.unnamed.len();
            if count == 0 {
                return Err(Error::new(
                    Span::call_site(),
                    "Unit struct's aren't supported",
                ));
            }

            let field_indices: Vec<_> = (0..count).map(Index::from).collect();
            let field_types: Vec<_> = fields.unnamed.iter().map(|f| &f.ty).collect();
            if count == 1 {
                Ok(quote! {
                    Ok(#ident (<#(#field_types)*>::from_arma(source)?))
                })
            } else {
                Ok(quote! {
                    let values: (#(
                        #field_types,
                    )*) = arma_rs::FromArma::from_arma(source)?;
                    Ok(#ident (#(
                        values.#field_indices,
                    )*))
                })
            }
        }
    }
}
