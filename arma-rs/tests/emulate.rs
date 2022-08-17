use std::{
    collections::HashMap,
    ffi::{CStr, CString},
    sync::{Arc, Once, RwLock},
};

use arma_rs::{Context, Extension};

type Stack = Arc<RwLock<HashMap<String, Vec<(String, String, String)>>>>;

static mut CALLBACK_STACK: Option<Stack> = None;
static CALLBACK_STACK_INIT: Once = Once::new();

fn get_callback_stack() -> Stack {
    unsafe {
        CALLBACK_STACK_INIT.call_once(|| {
            CALLBACK_STACK = Some(Arc::new(RwLock::new(HashMap::new())));
        });
        CALLBACK_STACK.as_ref().unwrap().clone()
    }
}

fn callback_handler(scope: String, name: *const i8, func: *const i8, data: *const i8) -> i32 {
    let stack = get_callback_stack();
    unsafe {
        let name = CStr::from_ptr(name).to_str().unwrap().to_string();
        let func = CStr::from_ptr(func).to_str().unwrap().to_string();
        let data = CStr::from_ptr(data).to_str().unwrap().to_string();
        let mut stack = stack.write().unwrap();
        stack
            .entry(scope)
            .or_insert(Vec::new())
            .push((name, func, data));
    }
    2
}

#[test]
#[cfg(all(not(windows), feature = "extension"))]
fn c_interface_full() {
    let mut extension = Extension::build()
        .command("hello", || -> &'static str { "Hello" })
        .command("welcome", |name: String| -> String {
            format!("Welcome {}", name)
        })
        .command("callback", |ctx: Context, id: String| {
            ctx.callback_data("callback", "fired", id);
        })
        .finish();
    extern "C" fn callback(name: *const i8, func: *const i8, data: *const i8) -> i32 {
        callback_handler("c_interface_full".to_string(), name, func, data)
    }
    extension.register_callback(callback);
    extension.run_callbacks();
    let stack = get_callback_stack();
    assert_eq!(stack.read().unwrap().get("c_interface_full"), None);
    unsafe {
        let mut output = [0i8; 1024];
        let code = extension.handle(
            CString::new("callback").unwrap().into_raw(),
            output.as_mut_ptr(),
            1024,
            Some(vec![CString::new("hello").unwrap().into_raw()].as_mut_ptr()),
            Some(1),
        );
        assert_eq!(code, 0);
    };
    unsafe {
        let mut output = [0i8; 1024];
        extension.handle(
            CString::new("hello").unwrap().into_raw(),
            output.as_mut_ptr(),
            1024,
            None,
            None,
        );
        let cstring = CStr::from_ptr(output.as_ptr()).to_str();
        assert_eq!(cstring, Ok("Hello"));
    }
    unsafe {
        let mut output = [0i8; 1024];
        extension.handle(
            CString::new("welcome").unwrap().into_raw(),
            output.as_mut_ptr(),
            1024,
            Some(vec![CString::new("John").unwrap().into_raw()].as_mut_ptr()),
            Some(1),
        );
        let cstring = CStr::from_ptr(output.as_ptr()).to_str();
        assert_eq!(cstring, Ok("Welcome John"));
    }
    std::thread::sleep(std::time::Duration::from_millis(50));
    assert_eq!(
        stack.read().unwrap().get("c_interface_full").unwrap().len(),
        1
    );
}

#[test]
fn c_interface_builder() {
    let extension = Extension::build().finish();
    assert_eq!(extension.version(), env!("CARGO_PKG_VERSION").to_string());
    assert!(!extension.allow_no_args());

    let extension = Extension::build().version("1.0.0".to_string()).finish();
    assert_eq!(extension.version(), "1.0.0".to_string());

    let extension = Extension::build().allow_no_args().finish();
    assert!(extension.allow_no_args());
}

#[test]
fn c_interface_invalid_calls() {
    let mut extension = Extension::build()
        .command("callback_invalid_name", |ctx: Context| {
            ctx.callback_null("call\0back", "fired");
        })
        .command("callback_invalid_func", |ctx: Context| {
            ctx.callback_null("callback", "fir\0ed");
        })
        .command("callback_invalid_data", |ctx: Context| {
            ctx.callback_data("callback", "fired", "dat\0a");
        })
        .command("callback_valid_null", |ctx: Context| {
            ctx.callback_null("callback", "fired");
        })
        .command("callback_valid_data", |ctx: Context| {
            ctx.callback_data("callback", "fired", "data");
        })
        .finish();
    extern "C" fn callback(name: *const i8, func: *const i8, data: *const i8) -> i32 {
        callback_handler("c_interface_invalid_calls".to_string(), name, func, data)
    }
    extension.register_callback(callback);
    extension.run_callbacks();
    unsafe {
        let mut output = [0i8; 1024];
        let code = extension.handle(
            CString::new("hello").unwrap().into_raw(),
            output.as_mut_ptr(),
            1024,
            None,
            None,
        );
        let cstring = CStr::from_ptr(output.as_ptr()).to_str();
        assert_eq!(cstring, Ok(""));
        assert_eq!(code, 1);
    }

    // Unknown function name
    unsafe {
        let mut output = [0i8; 1024];
        let code = extension.handle(
            CString::new("invalid").unwrap().into_raw(),
            output.as_mut_ptr(),
            1024,
            None,
            None,
        );
        let cstring = CStr::from_ptr(output.as_ptr()).to_str();
        assert_eq!(cstring, Ok(""));
        assert_eq!(code, 1);
    }

    // Invalid callback name
    unsafe {
        let mut output = [0i8; 1024];
        let code = extension.handle(
            CString::new("callback_invalid_name").unwrap().into_raw(),
            output.as_mut_ptr(),
            1024,
            None,
            None,
        );
        let cstring = CStr::from_ptr(output.as_ptr()).to_str();
        assert_eq!(cstring, Ok(""));
        assert_eq!(code, 0);
    }

    // Invalid callback func
    unsafe {
        let mut output = [0i8; 1024];
        let code = extension.handle(
            CString::new("callback_invalid_func").unwrap().into_raw(),
            output.as_mut_ptr(),
            1024,
            None,
            None,
        );
        let cstring = CStr::from_ptr(output.as_ptr()).to_str();
        assert_eq!(cstring, Ok(""));
        assert_eq!(code, 0);
    }

    // Invalid callback data
    unsafe {
        let mut output = [0i8; 1024];
        let code = extension.handle(
            CString::new("callback_invalid_data").unwrap().into_raw(),
            output.as_mut_ptr(),
            1024,
            None,
            None,
        );
        let cstring = CStr::from_ptr(output.as_ptr()).to_str();
        assert_eq!(cstring, Ok(""));
        assert_eq!(code, 0);
    }

    // Valid null callback
    unsafe {
        let mut output = [0i8; 1024];
        let code = extension.handle(
            CString::new("callback_valid_null").unwrap().into_raw(),
            output.as_mut_ptr(),
            1024,
            None,
            None,
        );
        let cstring = CStr::from_ptr(output.as_ptr()).to_str();
        assert_eq!(cstring, Ok(""));
        assert_eq!(code, 0);
    }

    // Valid data callback
    unsafe {
        let mut output = [0i8; 1024];
        let code = extension.handle(
            CString::new("callback_valid_data").unwrap().into_raw(),
            output.as_mut_ptr(),
            1024,
            None,
            None,
        );
        let cstring = CStr::from_ptr(output.as_ptr()).to_str();
        assert_eq!(cstring, Ok(""));
        assert_eq!(code, 0);
    }

    std::thread::sleep(std::time::Duration::from_millis(50));
    let stack = get_callback_stack();
    assert_eq!(
        stack
            .read()
            .unwrap()
            .get("c_interface_invalid_calls")
            .unwrap()
            .len(),
        2
    );
}
