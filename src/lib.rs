#![recursion_limit="256"]

use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Mutex;

extern crate proc_macro;
#[macro_use]
extern crate lazy_static;

use proc_macro::TokenStream;
use quote::quote;
use regex::Regex;
use syn::ItemFn;

lazy_static! {
    static ref PROXIES:     Mutex<Vec<String>> = Mutex::new(Vec::new());
    static ref PROXIES_ARG: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

#[proc_macro_attribute]
pub fn rv(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(item as ItemFn);
    
    let mut thread = false;

    let sattr = attr.to_string();
    if !sattr.is_empty() {
        let re = Regex::new(r#"(?m)(?P<key>[^,]+?)(?:\s+)?=(?:\s+)?(?P<value>[^",]+|"(?:[^"\\]|\\.)*")"#).unwrap();
        for caps in re.captures_iter(&sattr) {
            if &caps["key"] == "thread" {
                thread = bool::from_str(&caps["value"]).unwrap();
            }
        }
    }

    let name = &ast.ident;
    let sname = ast.ident.to_string();
    let handler = syn::Ident::new(&format!("{}_handler", name), name.span());
    let info = syn::Ident::new(&format!("{}_info", name), name.span());

    let mut args: HashMap<syn::Ident, syn::Type> = HashMap::new();
    let mut argtypes: Vec<syn::Type> = Vec::new();

    let astargs = &ast.decl.inputs;
    astargs.pairs().for_each(|p| {
        let v = p.value();
        match v {
            syn::FnArg::Captured(cap) => {
                match &cap.pat {
                    syn::Pat::Ident(ident) => {
                        args.insert(ident.ident.clone(), cap.ty.clone());
                        argtypes.push(cap.ty.clone());
                    },
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
    });

    let argcount = args.len();

    let handlerfn = if args.is_empty() {
        match ast.decl.output {
            syn::ReturnType::Default => {
                if thread {
                    quote! { 
                        unsafe fn #handler(output: *mut libc::c_char, size: usize, _: Option<*mut *mut i8>, _: Option<usize>) {
                            std::thread::spawn(move || {
                                #name();
                            });
                        }
                    }
                } else {
                    quote! { 
                        unsafe fn #handler(output: *mut libc::c_char, size: usize, _: Option<*mut *mut i8>, _: Option<usize>) {
                            #name();
                        }
                    }
                }
            },
            _ => {
                if thread {
                    panic!("Threaded functions can not return a value");
                }
                quote! { 
                    unsafe fn #handler(output: *mut libc::c_char, size: usize, _: Option<*mut *mut i8>, _: Option<usize>) {
                        libc::strncpy(output, std::ffi::CString::new(#name().to_string()).unwrap().into_raw(), size);
                    }
                }
            }
        }
    } else {
        match ast.decl.output {
            syn::ReturnType::Default => {
                if thread {
                    quote! { 
                        unsafe fn #handler(output: *mut libc::c_char, size: usize, args: Option<*mut *mut i8>, count: Option<usize>) {
                            let argv: &[*mut libc::c_char; #argcount] = std::mem::transmute(args.unwrap());
                            let mut argv: Vec<String> = argv.to_vec().into_iter().map(|s| std::ffi::CStr::from_ptr(s).to_str().unwrap().replace("\"", "")).collect();
                            argv.reverse();
                            std::thread::spawn(move || {
                                #name(#(#argtypes::from_str(&argv.pop().unwrap()).unwrap(),)*);
                            });
                        }
                    }
                } else {
                    quote! { 
                        unsafe fn #handler(output: *mut libc::c_char, size: usize, args: Option<*mut *mut i8>, count: Option<usize>) {
                            let argv: &[*mut libc::c_char; #argcount] = std::mem::transmute(args.unwrap());
                            let mut argv: Vec<String> = argv.to_vec().into_iter().map(|s| std::ffi::CStr::from_ptr(s).to_str().unwrap().replace("\"", "")).collect();
                            argv.reverse();
                            #name(#(#argtypes::from_str(&argv.pop().unwrap()).unwrap(),)*);
                        }
                    }
                }
            },
            _ => {
                if thread {
                    panic!("Threaded functions can not return a value");
                }
                quote! { 
                    unsafe fn #handler(output: *mut libc::c_char, size: usize, args: Option<*mut *mut i8>, count: Option<usize>) {
                        let argv: &[*mut libc::c_char; #argcount] = std::mem::transmute(args.unwrap());
                        let mut argv: Vec<String> = argv.to_vec().into_iter().map(|s| std::ffi::CStr::from_ptr(s).to_str().unwrap().replace("\"", "")).collect();
                        argv.reverse();
                        let v = #name(#(#argtypes::from_str(&argv.pop().unwrap()).unwrap(),)*).to_string();
                        libc::strncpy(output, std::ffi::CString::new(v).unwrap().into_raw(), size);
                    }
                }
            }
        }
    };

    let expanded = quote! {
        #[allow(non_upper_case_globals)]
        static #info: FunctionInfo = FunctionInfo {
            handler: #handler,
            name: #sname,
            thread: #thread,
        };
        #handlerfn
        #ast
    };

    if args.is_empty() {
        PROXIES.lock().unwrap().push(name.to_string());
    } else {
        PROXIES_ARG.lock().unwrap().push(name.to_string());
    }

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn rv_handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(item as ItemFn);

    let proxies = (*PROXIES.lock().unwrap()).clone();
    let info: Vec<syn::Ident> = proxies.iter().map(|s| {
        syn::Ident::new(&format!("{}_info", s), proc_macro2::Span::call_site())
    }).collect();
    let proxies_arg = (*PROXIES_ARG.lock().unwrap()).clone();
    let infoarg: Vec<syn::Ident> = proxies_arg.iter().map(|s| {
        syn::Ident::new(&format!("{}_info", s), proc_macro2::Span::call_site())
    }).collect();

    let expanded = quote! {
        use std::str::FromStr;

        pub struct FunctionInfo {
            name: &'static str,
            handler: unsafe fn(*mut libc::c_char, usize, Option<*mut *mut i8>, Option<usize>) -> (),
            thread: bool,
        }

        static arma_proxies: &[&FunctionInfo] = &[#(&#info,)*];
        static arma_proxies_arg: &[&FunctionInfo] = &[#(&#infoarg,)*];
        static mut did_init: bool = false;

        extern crate libc;
        #[no_mangle]
        pub unsafe extern "stdcall" fn RvExtensionVersion(output: *mut libc::c_char, output_size: usize) {
            if !did_init {
                init();
                did_init = true;
            }
            let v = std::ffi::CString::new(env!("CARGO_PKG_VERSION")).unwrap().into_raw();
            libc::strncpy(output, v, output_size);
        }

        #[no_mangle]
        pub unsafe extern "stdcall" fn RVExtension(output: *mut libc::c_char, output_size: usize, function: *mut libc::c_char) {
            let size = output_size - 1;
            let r_function = std::ffi::CStr::from_ptr(function).to_str().unwrap();

            for info in arma_proxies.iter() {
                if info.name == r_function {
                    (info.handler)(output, size, None, None);
                    return;
                }
            }

            libc::strncpy(output, std::ffi::CString::new("").unwrap().into_raw(), size);
        }

        #[no_mangle]
        pub unsafe extern "stdcall" fn RVExtensionArgs(output: *mut libc::c_char, output_size: usize, function: *mut libc::c_char, args: *mut *mut libc::c_char, arg_count: usize) {
            let size = output_size - 1;
            let r_function = std::ffi::CStr::from_ptr(function).to_str().unwrap();
            for info in arma_proxies_arg.iter() {
                if info.name == r_function {
                    (info.handler)(output, size, Some(args), Some(arg_count));
                    return;
                }
            }
        }

        #ast
    };

    TokenStream::from(expanded)
}
