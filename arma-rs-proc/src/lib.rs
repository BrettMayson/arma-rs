use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn arma(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(item as ItemFn);
    let init = ast.sig.ident.clone();

    let extern_type = if cfg!(windows) { "stdcall" } else { "C" };

    TokenStream::from(quote! {

        use arma_rs::libc as arma_rs_libc;

        static mut RV_EXTENSION: Option<Extension> = None;
        static mut RV_CALLBACK: Option<extern #extern_type fn(*const arma_rs_libc::c_char, *const arma_rs_libc::c_char, *const arma_rs_libc::c_char) -> usize> = None;

        #[no_mangle]
        pub unsafe extern #extern_type fn RVExtensionVersion(output: *mut arma_rs_libc::c_char, size: usize)-> i32 {
            if RV_EXTENSION.is_none() {
                RV_EXTENSION = Some(#init());
            }
            if let Some(ext) = &RV_EXTENSION {
                arma_rs::write_cstr(ext.version().to_string(), output, size);
            }
            0
        }

        #[no_mangle]
        pub unsafe extern #extern_type fn RVExtension(output: *mut arma_rs_libc::c_char, size: usize, function: *mut arma_rs_libc::c_char) {
            if RV_EXTENSION.is_none() {
                RV_EXTENSION = Some(#init());
            }
            if let Some(ext) = &RV_EXTENSION {
                if ext.allow_no_args() {
                    ext.handle(function, output, size, None, None);
                }
            }
        }

        #[no_mangle]
        pub unsafe extern #extern_type fn RVExtensionArgs(output: *mut arma_rs_libc::c_char, size: usize, function: *mut arma_rs_libc::c_char, args: *mut *mut arma_rs_libc::c_char, arg_count: usize) -> usize {
            if RV_EXTENSION.is_none() {
                RV_EXTENSION = Some(#init());
            }
            if let Some(ext) = &RV_EXTENSION {
                ext.handle(function, output, size, Some(args), Some(arg_count))
            } else {
                0
            }
        }

        #[no_mangle]
        pub unsafe extern #extern_type fn RVExtensionRegisterCallback(callback: extern #extern_type fn(*const arma_rs_libc::c_char, *const arma_rs_libc::c_char, *const arma_rs_libc::c_char) -> usize) {
            RV_CALLBACK = Some(callback);
        }

        unsafe fn RV_SEND_CALLBACK(name: *const arma_rs_libc::c_char, function: *const arma_rs_libc::c_char, data: *const arma_rs_libc::c_char) {
            if let Some(c) = RV_CALLBACK {
                loop {
                    if c(name, function, data) >= 0 {
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
            }
        }

        pub(crate) fn arma_callback<V: Into<arma_rs::ArmaValue>>(name: &str, func: &str, data: Option<V>) {
            unsafe {
                let name = std::ffi::CString::new(name).unwrap().into_raw();
                let func = std::ffi::CString::new(func).unwrap().into_raw();
                RV_SEND_CALLBACK(name, func, std::ffi::CString::new(match data {
                    Some(value) => match value.into() {
                        arma_rs::ArmaValue::String(s) => s,
                        v => v.to_string(),
                    },
                    None => String::new(),
                }).unwrap().into_raw());
            }
        }

        #ast
    })
}
