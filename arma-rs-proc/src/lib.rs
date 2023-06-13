use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{ItemFn, ItemStruct};

#[proc_macro_attribute]
/// Used to generate the necessary boilerplate for an Arma extension.
/// It should be applied to a function that takes no arguments and returns an extension.
pub fn arma(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(item as ItemFn);
    let init = ast.sig.ident.clone();

    let extern_type = if cfg!(windows) { "stdcall" } else { "C" };

    let ext_init = quote! {
        if RV_EXTENSION.is_none() {
            RV_EXTENSION = Some(#init());
        }
    };

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    let prefix = "safe32_";

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    let prefix = "";

    macro_rules! fn_ident {
        ( $name:literal ) => {
            Ident::new(&format!("{prefix}{}", $name), Span::call_site())
        };
    }
    let versionfn = fn_ident!("RVExtensionVersion");
    let noargfn = fn_ident!("RVExtension");
    let argfn = fn_ident!("RVExtensionArgs");
    let callbackfn = fn_ident!("RVExtensionRegisterCallback");
    let contextfn = fn_ident!("RVExtensionContext");

    TokenStream::from(quote! {
        use arma_rs::libc as arma_rs_libc;

        static mut RV_EXTENSION: Option<Extension> = None;

        #[cfg(all(target_os="windows", target_arch="x86"))]
        arma_rs::link_args::windows! {
            unsafe {
                raw("/EXPORT:_RVExtensionVersion@8=_safe32_RVExtensionVersion@8");
                raw("/EXPORT:_RVExtension@12=_safe32_RVExtension@12");
                raw("/EXPORT:_RVExtensionArgs@20=_safe32_RVExtensionArgs@20");
                raw("/EXPORT:_RVExtensionRegisterCallback@4=_safe32_RVExtensionRegisterCallback@4");
                raw("/EXPORT:_RVExtensionContext@8=_safe32_RVExtensionContext@8");
            }
        }

        #[no_mangle]
        pub unsafe extern #extern_type fn #versionfn(output: *mut arma_rs_libc::c_char, size: arma_rs_libc::size_t) -> arma_rs_libc::c_int {
            #ext_init
            if let Some(ext) = &RV_EXTENSION {
                arma_rs::write_cstr(ext.version().to_string(), output, size);
            }
            0
        }

        #[no_mangle]
        pub unsafe extern #extern_type fn #noargfn(output: *mut arma_rs_libc::c_char, size: arma_rs_libc::size_t, function: *mut arma_rs_libc::c_char) {
            #ext_init
            if let Some(ext) = &RV_EXTENSION {
                if ext.allow_no_args() {
                    ext.handle_call(function, output, size, None, None);
                }
            }
        }

        #[no_mangle]
        pub unsafe extern #extern_type fn #argfn(output: *mut arma_rs_libc::c_char, size: arma_rs_libc::size_t, function: *mut arma_rs_libc::c_char, args: *mut *mut arma_rs_libc::c_char, arg_count: arma_rs_libc::c_int) -> arma_rs_libc::c_int {
            #ext_init
            if let Some(ext) = &RV_EXTENSION {
                ext.handle_call(function, output, size, Some(args), Some(arg_count))
            } else {
                0
            }
        }

        #[no_mangle]
        pub unsafe extern #extern_type fn #callbackfn(callback: arma_rs::Callback) {
            #ext_init
            if let Some(ext) = &mut RV_EXTENSION {
                ext.register_callback(callback);
                ext.run_callbacks();
            }
        }

        #[no_mangle]
        pub unsafe extern #extern_type fn #contextfn(args: *mut *mut arma_rs_libc::c_char, arg_count: arma_rs_libc::c_int) {
            #ext_init
            if let Some(ext) = &mut RV_EXTENSION {
                ext.handle_call_context(args, arg_count);
            }
        }

        #ast
    })
}

#[proc_macro_derive(Arma)]
/// Derive macro generating implementations of `IntoArma` and `FromArma` for a struct.
/// - Structs with named fields are converted to a hashmap.
/// - Structs with multiple unnamed fields are converted to an array.
/// - Structs with a single unnamed field directly use the field's `IntoArma` and `FromArma` implementations.
/// - Unit structs and structs with no fields are not supported.
pub fn derive_arma(item: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(item as ItemStruct);
    let ident = &ast.ident;

    let tokens = match &ast.fields {
        syn::Fields::Named(fields) => {
            let field_idents: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
            let field_names: Vec<_> = field_idents
                .iter()
                .map(|f| f.as_ref().unwrap().to_string())
                .collect();
            quote! {
                impl arma_rs::IntoArma for #ident {
                    fn to_arma(&self) -> arma_rs::Value {
                        std::collections::HashMap::from([#(
                            (#field_names.to_string(), self.#field_idents.to_arma()),
                        )*]).to_arma()
                    }
                }
                impl arma_rs::FromArma for #ident {
                    fn from_arma(source: String) -> Result<Self, String> {
                        let values: std::collections::HashMap<String, String> = arma_rs::FromArma::from_arma(source)?;
                        Ok(#ident {#(
                            #field_idents: arma_rs::FromArma::from_arma(values[#field_names].clone())?,
                        )*})
                    }
                }
            }
        }
        syn::Fields::Unnamed(fields) => {
            let count = fields.unnamed.len();
            let field_indices: Vec<_> = (0..count).map(syn::Index::from).collect();
            let field_types: Vec<_> = fields.unnamed.iter().map(|f| &f.ty).collect();
            match count {
                0 => syn::Error::new(Span::call_site(), "Structs with no fields aren't supported")
                    .to_compile_error(),
                1 => quote! {
                    impl arma_rs::IntoArma for #ident {
                        fn to_arma(&self) -> arma_rs::Value {
                            self.0.to_arma()
                        }
                    }
                    impl arma_rs::FromArma for #ident {
                        fn from_arma(source: String) -> Result<Self, String> {
                            Ok(#ident (<#(#field_types)*>::from_arma(source)?))
                        }
                    }
                },
                _ => quote! {
                    impl arma_rs::IntoArma for #ident {
                        fn to_arma(&self) -> arma_rs::Value {
                            vec![#(
                                self.#field_indices.to_arma(),
                            )*].to_arma()
                        }
                    }
                    impl arma_rs::FromArma for #ident {
                        fn from_arma(source: String) -> Result<Self, String> {
                            let values: (#(
                                #field_types,
                            )*) = arma_rs::FromArma::from_arma(source)?;
                            Ok(#ident {#(
                                #field_indices: values.#field_indices,
                            )*})
                        }
                    }
                },
            }
        }
        syn::Fields::Unit => {
            syn::Error::new(Span::call_site(), "Unit structs aren't supported").to_compile_error()
        }
    };
    TokenStream::from(tokens)
}
