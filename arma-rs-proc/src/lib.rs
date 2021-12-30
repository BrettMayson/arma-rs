use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn arma(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(item as ItemFn);
    let init = ast.sig.ident.clone();

    let extern_type = if cfg!(windows) { "stdcall" } else { "C" };

    let ext_init = quote! {
        if RV_EXTENSION.is_none() {
            RV_EXTENSION = Some(#init());
        }
    };

    TokenStream::from(quote! {

        use arma_rs::libc as arma_rs_libc;

        static mut RV_EXTENSION: Option<Extension> = None;

        #[no_mangle]
        pub unsafe extern #extern_type fn RVExtensionVersion(output: *mut arma_rs_libc::c_char, size: arma_rs_libc::c_int)-> arma_rs_libc::c_int {
            #ext_init
            if let Some(ext) = &RV_EXTENSION {
                arma_rs::write_cstr(ext.version().to_string(), output, size);
            }
            0
        }

        #[no_mangle]
        pub unsafe extern #extern_type fn RVExtension(output: *mut arma_rs_libc::c_char, size: arma_rs_libc::c_int, function: *mut arma_rs_libc::c_char) {
            #ext_init
            if let Some(ext) = &RV_EXTENSION {
                if ext.allow_no_args() {
                    ext.handle(function, output, size, None, None);
                }
            }
        }

        #[no_mangle]
        pub unsafe extern #extern_type fn RVExtensionArgs(output: *mut arma_rs_libc::c_char, size: arma_rs_libc::c_int, function: *mut arma_rs_libc::c_char, args: *mut *mut arma_rs_libc::c_char, arg_count: arma_rs_libc::c_int) -> arma_rs_libc::c_int {
            #ext_init
            if let Some(ext) = &RV_EXTENSION {
                ext.handle(function, output, size, Some(args), Some(arg_count))
            } else {
                0
            }
        }

        #[no_mangle]
        pub unsafe extern #extern_type fn RVExtensionRegisterCallback(callback: arma_rs::Callback) {
            #ext_init
            if let Some(ext) = &mut RV_EXTENSION {
                ext.register_callback(callback);
                ext.run_callbacks();
            }
        }

        #ast
    })
}
