use std::{
    collections::HashMap,
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

pub fn callback(ctx: Context, id: String) {
    ctx.callback("callback", "fired", Some(id));
}

#[test]
#[cfg(all(not(windows), feature = "extension"))]
fn c_interface_full() {
    use std::ffi::CString;

    extern "C" fn callback_handler(name: *const i8, func: *const i8, data: *const i8) -> i32 {
        let stack = get_callback_stack();
        unsafe {
            let name = if let Ok(cstring) = std::ffi::CStr::from_ptr(name).to_str() {
                cstring.to_string()
            } else {
                return 1;
            };
            let func = if let Ok(cstring) = std::ffi::CStr::from_ptr(func).to_str() {
                cstring.to_string()
            } else {
                return 1;
            };
            let data = if let Ok(cstring) = std::ffi::CStr::from_ptr(data).to_str() {
                cstring.to_string()
            } else {
                return 1;
            };
            let mut stack = stack.write().unwrap();
            stack
                .entry("c_interface_full".to_string())
                .or_insert(Vec::new())
                .push((name, func, data));
        }
        2
    }

    let mut extension = Extension::build()
        .command("hello", || -> &'static str { "Hello" })
        .command("welcome", |name: String| -> String {
            format!("Welcome {}", name)
        })
        .command("callback", callback)
        .finish();
    extension.register_callback(callback_handler);
    extension.run_callbacks();
    let stack = get_callback_stack();
    assert_eq!(stack.read().unwrap().get("c_interface_full"), None);
    unsafe {
        let mut output = [0i8; 1024];
        extension.handle(
            CString::new("callback").unwrap().into_raw(),
            output.as_mut_ptr(),
            1024,
            Some(vec![CString::new("hello").unwrap().into_raw()].as_mut_ptr()),
            Some(1),
        )
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
        let cstring = std::ffi::CStr::from_ptr(output.as_ptr()).to_str();
        assert_eq!(
            cstring,
            Ok("Hello")
        );
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
        let cstring = std::ffi::CStr::from_ptr(output.as_ptr()).to_str();
        assert_eq!(
            cstring,
            Ok("Welcome John")
        );
    }
    std::thread::sleep(std::time::Duration::from_millis(50));
    assert_eq!(stack.read().unwrap().get("c_interface_full").unwrap().len(), 1);
}
