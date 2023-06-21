use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{DeriveInput, Error, Fields};

pub fn generate_into_arma(input: DeriveInput) -> syn::Result<TokenStream> {
    let ident = &input.ident;
    match input.data {
        syn::Data::Enum(_) => Err(Error::new(Span::call_site(), "Enums aren't supported")),
        syn::Data::Struct(data) => match &data.fields {
            Fields::Unit => Err(Error::new(
                Span::call_site(),
                "Unit structs aren't supported",
            )),
            Fields::Named(fields) => {
                let field_idents: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
                let field_names: Vec<_> = field_idents
                    .iter()
                    .map(|f| f.as_ref().unwrap().to_string())
                    .collect();
                Ok(quote! {
                #[automatically_derived]
                impl arma_rs::IntoArma for #ident {
                        fn to_arma(&self) -> arma_rs::Value {
                            std::collections::HashMap::<String, arma_rs::Value>::from([#(
                                (#field_names.to_string(), self.#field_idents.to_arma() ),
                            )*]).to_arma()
                        }
                    }
                })
            }
            Fields::Unnamed(fields) => {
                let count = fields.unnamed.len();
                let field_indices: Vec<_> = (0..count).map(syn::Index::from).collect();
                match count {
                    0 => Err(Error::new(
                        Span::call_site(),
                        "Unnamed structs with no fields aren't supported",
                    )),
                    1 => Ok(quote! {
                        #[automatically_derived]
                        impl arma_rs::IntoArma for #ident {
                            fn to_arma(&self) -> arma_rs::Value {
                                self.0.to_arma()
                            }
                        }
                    }),
                    _ => Ok(quote! {
                        #[automatically_derived]
                        impl arma_rs::IntoArma for #ident {
                            fn to_arma(&self) -> arma_rs::Value {
                                Vec::<arma_rs::Value>::from([#(
                                    self.#field_indices.to_arma(),
                                )*]).to_arma()
                            }
                        }
                    }),
                }
            }
        },
        syn::Data::Union(_) => Err(Error::new(Span::call_site(), "Unions aren't supported")),
    }
}

pub fn generate_from_arma(input: DeriveInput) -> syn::Result<TokenStream> {
    let ident = &input.ident;
    match input.data {
        syn::Data::Enum(_) => Err(Error::new(Span::call_site(), "Enums aren't supported")),
        syn::Data::Struct(data) => match &data.fields {
            Fields::Unit => Err(Error::new(
                Span::call_site(),
                "Unit structs aren't supported",
            )),
            Fields::Named(fields) => {
                let field_idents: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
                let field_names: Vec<_> = field_idents
                    .iter()
                    .map(|f| f.as_ref().unwrap().to_string())
                    .collect();
                Ok(quote! {
                    #[automatically_derived]
                    impl arma_rs::FromArma for #ident {
                        fn from_arma(source: String) -> Result<Self, String> {
                            let values: std::collections::HashMap<String, String> = arma_rs::FromArma::from_arma(source)?;
                            Ok(#ident {#(
                                #field_idents: arma_rs::FromArma::from_arma(values[#field_names].clone())?,
                            )*})
                        }
                    }
                })
            }
            Fields::Unnamed(fields) => {
                let count = fields.unnamed.len();
                let field_indices: Vec<_> = (0..count).map(syn::Index::from).collect();
                let field_types: Vec<_> = fields.unnamed.iter().map(|f| &f.ty).collect();
                match count {
                    0 => Err(Error::new(
                        Span::call_site(),
                        "Unnamed structs with no fields aren't supported",
                    )),
                    1 => Ok(quote! {
                        #[automatically_derived]
                        impl arma_rs::FromArma for #ident {
                            fn from_arma(source: String) -> Result<Self, String> {
                                Ok(#ident (<#(#field_types)*>::from_arma(source)?))
                            }
                        }
                    }),
                    _ => Ok(quote! {
                        #[automatically_derived]
                        impl arma_rs::FromArma for #ident {
                            fn from_arma(source: String) -> Result<Self, String> {
                                let values: (#(
                                    #field_types,
                                )*) = arma_rs::FromArma::from_arma(source)?;
                                Ok(#ident (#(
                                    values.#field_indices,
                                )*))
                            }
                        }
                    }),
                }
            }
        },
        syn::Data::Union(_) => Err(Error::new(Span::call_site(), "Unions aren't supported")),
    }
}
