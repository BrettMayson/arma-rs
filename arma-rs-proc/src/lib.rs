use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
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

    #[cfg(all(target_arch = "x86", target_os = "windows"))]
    let prefix = "safe32_";

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    let prefix = "";

    let versionfn = Ident::new(&format!("{}RVExtensionVersion", prefix), Span::call_site());
    let noargfn = Ident::new(&format!("{}RVExtension", prefix), Span::call_site());
    let argfn = Ident::new(&format!("{}RVExtensionArgs", prefix), Span::call_site());
    let callbackfn = Ident::new(
        &format!("{}RVExtensionRegisterCallback", prefix),
        Span::call_site(),
    );

    TokenStream::from(quote! {

        use arma_rs::libc as arma_rs_libc;

        static mut RV_EXTENSION: Option<Extension> = None;

        #[cfg(all(target_os="windows", target_arch="x86"))]
        arma_rs::link_args::windows::raw! {
            unsafe "/EXPORT:_RVExtensionVersion@8=_safe32_RVExtensionVersion@8"
        }
        #[cfg(all(target_os="windows", target_arch="x86"))]
        arma_rs::link_args::windows::raw! {
            unsafe "/EXPORT:_RVExtension@12=_safe32_RVExtension@12"
        }
        #[cfg(all(target_os="windows", target_arch="x86"))]
        arma_rs::link_args::windows::raw! {
            unsafe "/EXPORT:_RVExtensionArgs@20=_safe32_RVExtensionArgs@20"
        }
        #[cfg(all(target_os="windows", target_arch="x86"))]
        arma_rs::link_args::windows::raw! {
            unsafe "/EXPORT:_RVExtensionRegisterCallback@4=_safe32_RVExtensionRegisterCallback@4"
        }

        #[no_mangle]
        pub unsafe extern #extern_type fn #versionfn(output: *mut arma_rs_libc::c_char, size: arma_rs_libc::size_t)-> arma_rs_libc::c_int {
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
                    ext.handle(function, output, size, None, None);
                }
            }
        }

        #[no_mangle]
        pub unsafe extern #extern_type fn #argfn(output: *mut arma_rs_libc::c_char, size: arma_rs_libc::size_t, function: *mut arma_rs_libc::c_char, args: *mut *mut arma_rs_libc::c_char, arg_count: arma_rs_libc::c_int) -> arma_rs_libc::c_int {
            #ext_init
            if let Some(ext) = &RV_EXTENSION {
                ext.handle(function, output, size, Some(args), Some(arg_count))
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

        #ast
    })
}
