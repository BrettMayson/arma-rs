use std::{
    collections::HashMap,
    ffi::{CStr, CString},
    sync::{Arc, Once, RwLock},
};

use arma_rs::{Caller, Context, Extension, Mission, Server, Source};

macro_rules! platform_extern {
    ($($func_body:tt)*) => {
        #[cfg(windows)]
        extern "stdcall" $($func_body)*
        #[cfg(not(windows))]
        extern "C" $($func_body)*
    };
}

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
        stack.entry(scope).or_default().push((name, func, data));
    }
    2
}

#[test]
fn c_interface_full() {
    let mut extension = Extension::build()
        .command("hello", || -> &'static str { "Hello" })
        .command("welcome", |name: String| -> String {
            format!("Welcome {name}")
        })
        .command("callback", |ctx: Context, id: String| {
            ctx.callback_data("callback", "fired", id);
        })
        .command("arma_call_context", |ctx: Context| -> String {
            format!(
                "{:?},{:?},{:?},{:?}",
                ctx.caller(),
                ctx.source(),
                ctx.mission(),
                ctx.server()
            )
        })
        .finish();
    platform_extern!(
        fn callback(name: *const i8, func: *const i8, data: *const i8) -> i32 {
            callback_handler("c_interface_full".to_string(), name, func, data)
        }
    );
    extension.register_callback(callback);
    extension.run_callbacks();
    let stack = get_callback_stack();
    assert_eq!(stack.read().unwrap().get("c_interface_full"), None);
    unsafe {
        let mut output = [0i8; 1024];
        let code = extension.handle_call(
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
        extension.handle_call(
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
        extension.handle_call(
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
    unsafe {
        let mut output = [0i8; 1024];
        extension.handle_arma_call_context(
            vec![
                CString::new("123").unwrap().into_raw(),     // steam ID
                CString::new("pbo").unwrap().into_raw(),     // file source
                CString::new("mission").unwrap().into_raw(), // mission name
                CString::new("server").unwrap().into_raw(),  // server name
            ]
            .as_mut_ptr(),
            4,
        );
        extension.handle_call(
            CString::new("arma_call_context").unwrap().into_raw(),
            output.as_mut_ptr(),
            1024,
            None,
            None,
        );
        let cstring = CStr::from_ptr(output.as_ptr()).to_str();
        assert_eq!(
            cstring,
            Ok("Steam(123),Pbo(\"pbo\"),Mission(\"mission\"),Multiplayer(\"server\")")
        );
    }
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
    platform_extern!(
        fn callback(name: *const i8, func: *const i8, data: *const i8) -> i32 {
            callback_handler("c_interface_invalid_calls".to_string(), name, func, data)
        }
    );
    extension.register_callback(callback);
    extension.run_callbacks();
    unsafe {
        let mut output = [0i8; 1024];
        let code = extension.handle_call(
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
        let code = extension.handle_call(
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
        let code = extension.handle_call(
            CString::new("callback_invalid_name").unwrap().into_raw(),
            output.as_mut_ptr(),
            1024,
            None,
            None,
        );
        let cstring = CStr::from_ptr(output.as_ptr()).to_str();
        assert_eq!(cstring, Ok("null"));
        assert_eq!(code, 0);
    }

    // Invalid callback func
    unsafe {
        let mut output = [0i8; 1024];
        let code = extension.handle_call(
            CString::new("callback_invalid_func").unwrap().into_raw(),
            output.as_mut_ptr(),
            1024,
            None,
            None,
        );
        let cstring = CStr::from_ptr(output.as_ptr()).to_str();
        assert_eq!(cstring, Ok("null"));
        assert_eq!(code, 0);
    }

    // Invalid callback data
    unsafe {
        let mut output = [0i8; 1024];
        let code = extension.handle_call(
            CString::new("callback_invalid_data").unwrap().into_raw(),
            output.as_mut_ptr(),
            1024,
            None,
            None,
        );
        let cstring = CStr::from_ptr(output.as_ptr()).to_str();
        assert_eq!(cstring, Ok("null"));
        assert_eq!(code, 0);
    }

    // Valid null callback
    unsafe {
        let mut output = [0i8; 1024];
        let code = extension.handle_call(
            CString::new("callback_valid_null").unwrap().into_raw(),
            output.as_mut_ptr(),
            1024,
            None,
            None,
        );
        let cstring = CStr::from_ptr(output.as_ptr()).to_str();
        assert_eq!(cstring, Ok("null"));
        assert_eq!(code, 0);
    }

    // Valid data callback
    unsafe {
        let mut output = [0i8; 1024];
        let code = extension.handle_call(
            CString::new("callback_valid_data").unwrap().into_raw(),
            output.as_mut_ptr(),
            1024,
            None,
            None,
        );
        let cstring = CStr::from_ptr(output.as_ptr()).to_str();
        assert_eq!(cstring, Ok("null"));
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

    // Valid Arma call context
    // Note: Ordering of these arma call context tests matter, used to confirm that the test correctly set arma call context
    fn is_call_ctx_default(ctx: Context) -> bool {
        ctx.caller() == &Caller::default()
            && ctx.source() == &Source::default()
            && ctx.mission() == &Mission::default()
            && ctx.server() == &Server::default()
    }

    unsafe {
        // Confirm expected status
        assert!(is_call_ctx_default(extension.context()));
        extension.handle_arma_call_context(
            vec![
                CString::new("123").unwrap().into_raw(),     // steam ID
                CString::new("pbo").unwrap().into_raw(),     // file source
                CString::new("mission").unwrap().into_raw(), // mission name
                CString::new("server").unwrap().into_raw(),  // server name
            ]
            .as_mut_ptr(),
            4,
        );
        assert!(!is_call_ctx_default(extension.context()));
    }

    // Arma call context not enough args
    unsafe {
        // Confirm expected status
        assert!(!is_call_ctx_default(extension.context()));
        extension.handle_arma_call_context(vec![].as_mut_ptr(), 0);
        assert!(is_call_ctx_default(extension.context()));
    }

    // Arma call context too many args
    unsafe {
        // Confirm expected status
        assert!(is_call_ctx_default(extension.context()));
        extension.handle_arma_call_context(
            vec![
                CString::new("123").unwrap().into_raw(),     // steam ID
                CString::new("pbo").unwrap().into_raw(),     // file source
                CString::new("mission").unwrap().into_raw(), // mission name
                CString::new("server").unwrap().into_raw(),  // server name
                CString::new("").unwrap().into_raw(),
                CString::new("").unwrap().into_raw(),
            ]
            .as_mut_ptr(),
            6,
        );
        assert!(!is_call_ctx_default(extension.context()));
    }
}

#[test]
fn c_interface_errors() {
    let extension = Extension::build()
        .command("add_no_context", |a: i32, b: i32| {
            let _ = a + b;
        })
        .command("add_no_context_return", |a: i32, b: i32| a + b)
        .command("add_context", |_ctx: Context, a: i32, b: i32| {
            let _ = a + b;
        })
        .command("add_context_return", |_ctx: Context, a: i32, b: i32| a + b)
        .command("overflow", |ctx: Context| "X".repeat(ctx.buffer_len() + 1))
        .command("result", |error: bool| -> Result<String, String> {
            if error {
                Err(String::from("told to error"))
            } else {
                Ok(String::from("told to succeed"))
            }
        })
        .finish();

    // Valid
    unsafe {
        for (func, result) in [
            ("add_no_context", "null"),
            ("add_no_context_return", "3"),
            ("add_context", "null"),
            ("add_context_return", "3"),
        ] {
            let mut output = [0i8; 1024];
            let code = extension.handle_call(
                CString::new(func).unwrap().into_raw(),
                output.as_mut_ptr(),
                1024,
                Some(
                    vec![
                        CString::new("1").unwrap().into_raw(),
                        CString::new("2").unwrap().into_raw(),
                    ]
                    .as_mut_ptr(),
                ),
                Some(2),
            );
            let cstring = CStr::from_ptr(output.as_ptr()).to_str();
            assert_eq!(cstring, Ok(result));
            assert_eq!(code, 0);
        }
    }

    // Invalid too many arguments
    unsafe {
        for func in [
            "add_no_context",
            "add_no_context_return",
            "add_context",
            "add_context_return",
        ] {
            let mut output = [0i8; 1024];
            let code = extension.handle_call(
                CString::new(func).unwrap().into_raw(),
                output.as_mut_ptr(),
                1024,
                Some(
                    vec![
                        CString::new("1").unwrap().into_raw(),
                        CString::new("2").unwrap().into_raw(),
                        CString::new("3").unwrap().into_raw(),
                    ]
                    .as_mut_ptr(),
                ),
                Some(3),
            );
            let cstring = CStr::from_ptr(output.as_ptr()).to_str();
            assert_eq!(cstring, Ok(""));
            assert_eq!(code, 23);
        }
    }

    // Invalid too few arguments
    unsafe {
        for func in [
            "add_no_context",
            "add_no_context_return",
            "add_context",
            "add_context_return",
        ] {
            let mut output = [0i8; 1024];
            let code = extension.handle_call(
                CString::new(func).unwrap().into_raw(),
                output.as_mut_ptr(),
                1024,
                Some(vec![CString::new("1").unwrap().into_raw()].as_mut_ptr()),
                Some(1),
            );
            let cstring = CStr::from_ptr(output.as_ptr()).to_str();
            assert_eq!(cstring, Ok(""));
            assert_eq!(code, 21);
        }
    }

    // Valid type conversion
    unsafe {
        for (func, result) in [
            ("add_no_context", "null"),
            ("add_no_context_return", "3"),
            ("add_context", "null"),
            ("add_context_return", "3"),
        ] {
            let mut output = [0i8; 1024];
            let code = extension.handle_call(
                CString::new(func).unwrap().into_raw(),
                output.as_mut_ptr(),
                1024,
                Some(
                    vec![
                        CString::new("1").unwrap().into_raw(),
                        CString::new("\"2\"").unwrap().into_raw(),
                    ]
                    .as_mut_ptr(),
                ),
                Some(2),
            );
            let cstring = CStr::from_ptr(output.as_ptr()).to_str();
            assert_eq!(cstring, Ok(result));
            assert_eq!(code, 0);
        }
    }

    // Invalid type
    unsafe {
        for func in [
            "add_no_context",
            "add_no_context_return",
            "add_context",
            "add_context_return",
        ] {
            let mut output = [0i8; 1024];
            let code = extension.handle_call(
                CString::new(func).unwrap().into_raw(),
                output.as_mut_ptr(),
                1024,
                Some(
                    vec![
                        CString::new("1").unwrap().into_raw(),
                        CString::new("\"two\"").unwrap().into_raw(),
                    ]
                    .as_mut_ptr(),
                ),
                Some(2),
            );
            let cstring = CStr::from_ptr(output.as_ptr()).to_str();
            assert_eq!(cstring, Ok(""));
            assert_eq!(code, 31);
        }
    }

    // Overflow
    unsafe {
        let mut output = [0i8; 1024];
        let code = extension.handle_call(
            CString::new("overflow").unwrap().into_raw(),
            output.as_mut_ptr(),
            1024,
            None,
            None,
        );
        let cstring = CStr::from_ptr(output.as_ptr()).to_str();
        assert_eq!(cstring, Ok(""));
        assert_eq!(code, 4);
    }

    // Result - true
    unsafe {
        let mut output = [0i8; 1024];
        let code = extension.handle_call(
            CString::new("result").unwrap().into_raw(),
            output.as_mut_ptr(),
            1024,
            Some(vec![CString::new("true").unwrap().into_raw()].as_mut_ptr()),
            Some(1),
        );
        let cstring = CStr::from_ptr(output.as_ptr()).to_str();
        assert_eq!(cstring, Ok("told to error"));
        assert_eq!(code, 9);
    }

    // Result - false
    unsafe {
        let mut output = [0i8; 1024];
        let code = extension.handle_call(
            CString::new("result").unwrap().into_raw(),
            output.as_mut_ptr(),
            1024,
            Some(vec![CString::new("false").unwrap().into_raw()].as_mut_ptr()),
            Some(1),
        );
        let cstring = CStr::from_ptr(output.as_ptr()).to_str();
        assert_eq!(cstring, Ok("told to succeed"));
        assert_eq!(code, 0);
    }
}
