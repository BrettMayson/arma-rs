#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use arma_rs::{rv, rv_handler};
#[allow(non_upper_case_globals)]
static hello_info: FunctionInfo = FunctionInfo {
    handler: hello_handler,
    name: "hello",
    thread: false,
};
unsafe fn hello_handler(
    output: *mut arma_rs_libc::c_char,
    size: usize,
    _: Option<*mut *mut i8>,
    _: Option<usize>,
) {
    write_cstr(hello().to_string(), output, size);
}
fn hello() -> &'static str {
    "Hello from Rust!"
}
#[allow(non_upper_case_globals)]
static is_arma3_info: FunctionInfo = FunctionInfo {
    handler: is_arma3_handler,
    name: "is_arma3",
    thread: false,
};
#[allow(clippy::transmute_ptr_to_ref)]
unsafe fn is_arma3_handler(
    output: *mut arma_rs_libc::c_char,
    size: usize,
    args: Option<*mut *mut i8>,
    count: Option<usize>,
) {
    let argv: &[*mut arma_rs_libc::c_char; 1usize] = std::mem::transmute(args.unwrap());
    let mut argv: Vec<String> = argv
        .to_vec()
        .into_iter()
        .map(|s| {
            std::ffi::CStr::from_ptr(s)
                .to_str()
                .unwrap()
                .trim_matches('\"')
                .to_owned()
        })
        .collect();
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["calling ", ": ", "\n"],
            &match (&"is_arma3", &argv) {
                (arg0, arg1) => [
                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                ],
            },
        ));
    };
    argv.reverse();
    let v = is_arma3(u8::from_str(&argv.pop().unwrap()).unwrap());
    write_cstr(v.to_string(), output, size);
}
fn is_arma3(version: u8) -> bool {
    version == 3
}
use std::str::FromStr;
use arma_rs::libc as arma_rs_libc;
pub struct FunctionInfo {
    name: &'static str,
    handler: unsafe fn(*mut arma_rs_libc::c_char, usize, Option<*mut *mut i8>, Option<usize>) -> (),
    thread: bool,
}
static arma_proxies: &[&FunctionInfo] = &[&hello_info];
static arma_proxies_arg: &[&FunctionInfo] = &[&is_arma3_info];
static mut did_init: bool = false;
static mut CALLBACK: Option<
    extern "stdcall" fn(
        *const arma_rs_libc::c_char,
        *const arma_rs_libc::c_char,
        *const arma_rs_libc::c_char,
    ) -> arma_rs_libc::c_int,
> = None;
#[no_mangle]
pub unsafe extern "stdcall" fn RvExtensionVersion(output: *mut arma_rs_libc::c_char, size: usize) {
    if !did_init {
        main();
        did_init = true;
    }
    write_cstr("0.2.0".to_string(), output, size);
}
#[no_mangle]
pub unsafe extern "stdcall" fn RVExtension(
    output: *mut arma_rs_libc::c_char,
    size: usize,
    function: *mut arma_rs_libc::c_char,
) {
    if !did_init {
        main();
        did_init = true;
    }
    let r_function = std::ffi::CStr::from_ptr(function).to_str().unwrap();
    for info in arma_proxies.iter() {
        if info.name == r_function {
            (info.handler)(output, size, None, None);
            return;
        }
    }
}
#[no_mangle]
pub unsafe extern "stdcall" fn RVExtensionArgs(
    output: *mut arma_rs_libc::c_char,
    size: usize,
    function: *mut arma_rs_libc::c_char,
    args: *mut *mut arma_rs_libc::c_char,
    arg_count: usize,
) {
    if !did_init {
        main();
        did_init = true;
    }
    let r_function = std::ffi::CStr::from_ptr(function).to_str().unwrap();
    for info in arma_proxies_arg.iter() {
        if info.name == r_function {
            (info.handler)(output, size, Some(args), Some(arg_count));
            return;
        }
    }
}
#[no_mangle]
pub unsafe extern "stdcall" fn RVExtensionRegisterCallback(
    callback: extern "stdcall" fn(
        *const arma_rs_libc::c_char,
        *const arma_rs_libc::c_char,
        *const arma_rs_libc::c_char,
    ) -> arma_rs_libc::c_int,
) {
    CALLBACK = Some(callback);
}
unsafe fn rv_send_callback(
    name: *const arma_rs_libc::c_char,
    function: *const arma_rs_libc::c_char,
    data: *const arma_rs_libc::c_char,
) {
    if let Some(c) = CALLBACK {
        c(name, function, data);
    }
}
unsafe fn write_cstr(
    string: String,
    ptr: *mut arma_rs_libc::c_char,
    buf_size: arma_rs_libc::size_t,
) -> Option<usize> {
    if !string.is_ascii() {
        return None;
    };
    let cstr = std::ffi::CString::new(string).ok()?;
    let cstr_bytes = cstr.as_bytes();
    let amount_to_copy = std::cmp::min(cstr_bytes.len(), buf_size - 1);
    if amount_to_copy > isize::MAX as usize {
        return None;
    }
    ptr.copy_from(cstr.as_ptr(), amount_to_copy);
    ptr.add(amount_to_copy).write(0x00);
    Some(amount_to_copy)
}
fn main() {}
