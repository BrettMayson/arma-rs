mod derive;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{DeriveInput, Error, ItemFn};

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

/// Derive implementation of `IntoArma`, currently only supports structs.
/// - Named fields are converted to a hashmap.
/// - Multiple unnamed fields are converted to an array.
/// - Single unnamed field directly use's the fields `IntoArma` implementation.
#[proc_macro_derive(IntoArma)]
pub fn derive_into_arma(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as DeriveInput);
    derive::generate_into_arma(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Derive implementation of `FromArma`, currently only supports structs.
/// - Named fields are converted from a hashmap.
/// - Multiple unnamed fields are converted from an array.
/// - Single unnamed field directly use's the fields `FromArma` implementation.
#[proc_macro_derive(FromArma)]
pub fn derive_from_arma(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as DeriveInput);
    derive::generate_from_arma(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
