#[cfg(feature = "extension")]
mod extension {
    use std::{
        collections::HashMap,
        ffi::{CStr, CString},
        sync::{Arc, Once, RwLock},
    };

    use arma_rs::{CallbackError, Context, Extension};

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
            .command(
                "callback",
                |ctx: Context, id: String| -> Result<(), CallbackError> {
                    ctx.callback_data("callback", "fired", id)
                },
            )
            .command("arma_context", |ctx: Context| -> String {
                let arma = ctx.arma().unwrap();
                format!(
                    "{:?},{:?},{:?},{:?}",
                    arma.caller(),
                    arma.source(),
                    arma.mission(),
                    arma.server()
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
            let ptr = CString::new("callback").unwrap().into_raw();
            let ptr_hello = CString::new("hello").unwrap().into_raw();
            let code = extension.handle_call(
                ptr,
                output.as_mut_ptr(),
                1024,
                Some(vec![ptr_hello].as_mut_ptr()),
                Some(1),
            );
            assert_eq!(code, 0);
            let _ = CString::from_raw(ptr);
            let _ = CString::from_raw(ptr_hello);
        };
        unsafe {
            let mut output = [0i8; 1024];
            let ptr = CString::new("hello").unwrap().into_raw();
            extension.handle_call(ptr, output.as_mut_ptr(), 1024, None, None);
            let cstring = CStr::from_ptr(output.as_ptr()).to_str();
            assert_eq!(cstring, Ok("Hello"));
            let _ = CString::from_raw(ptr);
        }
        unsafe {
            let mut output = [0i8; 1024];
            let ptr = CString::new("welcome").unwrap().into_raw();
            let ptr_john = CString::new("John").unwrap().into_raw();
            extension.handle_call(
                ptr,
                output.as_mut_ptr(),
                1024,
                Some(vec![ptr_john].as_mut_ptr()),
                Some(1),
            );
            let cstring = CStr::from_ptr(output.as_ptr()).to_str();
            assert_eq!(cstring, Ok("Welcome John"));
            let _ = CString::from_raw(ptr);
            let _ = CString::from_raw(ptr_john);
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
        assert_eq!(
            stack.read().unwrap().get("c_interface_full").unwrap().len(),
            1
        );
        unsafe {
            let mut output = [0i8; 1024];
            let ptr1 = CString::new("123").unwrap().into_raw();
            let ptr2 = CString::new("pbo").unwrap().into_raw();
            let ptr3 = CString::new("mission").unwrap().into_raw();
            let ptr4 = CString::new("server").unwrap().into_raw();
            extension.handle_arma_context(
                vec![
                    ptr1, // steam ID
                    ptr2, // file source
                    ptr3, // mission name
                    ptr4, // server name
                ]
                .as_mut_ptr(),
                4,
            );
            let ptr = CString::new("arma_context").unwrap().into_raw();
            extension.handle_call(ptr, output.as_mut_ptr(), 1024, None, None);
            let cstring = CStr::from_ptr(output.as_ptr()).to_str();
            assert_eq!(
                cstring,
                Ok("Steam(123),Pbo(\"pbo\"),Mission(\"mission\"),Multiplayer(\"server\")")
            );
            let _ = CString::from_raw(ptr);
            let _ = CString::from_raw(ptr1);
            let _ = CString::from_raw(ptr2);
            let _ = CString::from_raw(ptr3);
            let _ = CString::from_raw(ptr4);
        }
    }

    #[test]
    fn c_interface_builder() {
        let extension = Extension::build().finish();
        assert_eq!(extension.version(), "0.0.0");
        assert!(!extension.allow_no_args());

        let extension = Extension::build().version("1.0.0".to_string()).finish();
        assert_eq!(extension.version(), "1.0.0".to_string());

        let extension = Extension::build().allow_no_args().finish();
        assert!(extension.allow_no_args());
    }

    #[test]
    fn c_interface_invalid_calls() {
        let mut extension = Extension::build()
            .command(
                "callback_invalid_name",
                |ctx: Context| -> Result<(), CallbackError> {
                    ctx.callback_null("call\0back", "fired")
                },
            )
            .command(
                "callback_invalid_func",
                |ctx: Context| -> Result<(), CallbackError> {
                    ctx.callback_null("callback", "fir\0ed")
                },
            )
            .command(
                "callback_invalid_data",
                |ctx: Context| -> Result<(), CallbackError> {
                    ctx.callback_data("callback", "fired", "dat\0a")
                },
            )
            .command(
                "callback_valid_null",
                |ctx: Context| -> Result<(), CallbackError> {
                    ctx.callback_null("callback", "fired")
                },
            )
            .command(
                "callback_valid_data",
                |ctx: Context| -> Result<(), CallbackError> {
                    ctx.callback_data("callback", "fired", "data")
                },
            )
            .finish();
        platform_extern!(
            fn callback(name: *const i8, func: *const i8, data: *const i8) -> i32 {
                callback_handler("c_interface_invalid_calls".to_string(), name, func, data)
            }
        );
        extension.register_callback(callback);
        extension.run_callbacks();
        let ptr = CString::new("hello").unwrap().into_raw();
        unsafe {
            let mut output = [0i8; 1024];
            let code = extension.handle_call(ptr, output.as_mut_ptr(), 1024, None, None);
            let cstring = CStr::from_ptr(output.as_ptr()).to_str();
            assert_eq!(cstring, Ok(""));
            assert_eq!(code, 1);
            let _ = CString::from_raw(ptr);
        }

        // Unknown function name
        unsafe {
            let mut output = [0i8; 1024];
            let ptr = CString::new("invalid").unwrap().into_raw();
            let code = extension.handle_call(ptr, output.as_mut_ptr(), 1024, None, None);
            let cstring = CStr::from_ptr(output.as_ptr()).to_str();
            assert_eq!(cstring, Ok(""));
            assert_eq!(code, 1);
            let _ = CString::from_raw(ptr);
        }

        // Invalid callback name
        unsafe {
            let mut output = [0i8; 1024];
            let ptr = CString::new("callback_invalid_name").unwrap().into_raw();
            let code = extension.handle_call(ptr, output.as_mut_ptr(), 1024, None, None);
            let cstring = CStr::from_ptr(output.as_ptr()).to_str();
            assert_eq!(cstring, Ok("null"));
            assert_eq!(code, 0);
            let _ = CString::from_raw(ptr);
        }

        // Invalid callback func
        unsafe {
            let mut output = [0i8; 1024];
            let ptr = CString::new("callback_invalid_func").unwrap().into_raw();
            let code = extension.handle_call(ptr, output.as_mut_ptr(), 1024, None, None);
            let cstring = CStr::from_ptr(output.as_ptr()).to_str();
            assert_eq!(cstring, Ok("null"));
            assert_eq!(code, 0);
            let _ = CString::from_raw(ptr);
        }

        // Invalid callback data
        unsafe {
            let mut output = [0i8; 1024];
            let ptr = CString::new("callback_invalid_data").unwrap().into_raw();
            let code = extension.handle_call(ptr, output.as_mut_ptr(), 1024, None, None);
            let cstring = CStr::from_ptr(output.as_ptr()).to_str();
            assert_eq!(cstring, Ok("null"));
            assert_eq!(code, 0);
            let _ = CString::from_raw(ptr);
        }

        // Valid null callback
        unsafe {
            let mut output = [0i8; 1024];
            let ptr = CString::new("callback_valid_null").unwrap().into_raw();
            let code = extension.handle_call(ptr, output.as_mut_ptr(), 1024, None, None);
            let cstring = CStr::from_ptr(output.as_ptr()).to_str();
            assert_eq!(cstring, Ok("null"));
            assert_eq!(code, 0);
            let _ = CString::from_raw(ptr);
        }

        // Valid data callback
        unsafe {
            let mut output = [0i8; 1024];
            let ptr = CString::new("callback_valid_data").unwrap().into_raw();
            let code = extension.handle_call(ptr, output.as_mut_ptr(), 1024, None, None);
            let cstring = CStr::from_ptr(output.as_ptr()).to_str();
            assert_eq!(cstring, Ok("null"));
            assert_eq!(code, 0);
            let _ = CString::from_raw(ptr);
        }

        std::thread::sleep(std::time::Duration::from_millis(500));
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

        // Valid Arma context
        // Note: Ordering of these arma context tests matter, used to confirm that the test correctly set arma context
        unsafe {
            assert!(extension.context().arma().is_none()); // Confirm expected status
            let ptr = CString::new("").unwrap().into_raw();
            extension.handle_arma_context(vec![ptr, ptr, ptr, ptr].as_mut_ptr(), 4);
            assert!(extension.context().arma().is_some());
            let _ = CString::from_raw(ptr);
        }

        // Arma context not enough args
        unsafe {
            assert!(extension.context().arma().is_some()); // Confirm expected status
            extension.handle_arma_context(vec![].as_mut_ptr(), 0);
            assert!(extension.context().arma().is_none());
        }

        // Arma context too many args
        unsafe {
            assert!(extension.context().arma().is_none()); // Confirm expected status
            let ptr = CString::new("").unwrap().into_raw();
            extension.handle_arma_context(vec![ptr, ptr, ptr, ptr, ptr, ptr].as_mut_ptr(), 6);
            assert!(extension.context().arma().is_some());
            let _ = CString::from_raw(ptr);
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
            let ptr1 = CString::new("1").unwrap().into_raw();
            let ptr2 = CString::new("2").unwrap().into_raw();
            for (func, result) in [
                ("add_no_context", "null"),
                ("add_no_context_return", "3"),
                ("add_context", "null"),
                ("add_context_return", "3"),
            ] {
                let mut output = [0i8; 1024];
                let ptr = CString::new(func).unwrap().into_raw();
                let code = extension.handle_call(
                    ptr,
                    output.as_mut_ptr(),
                    1024,
                    Some(vec![ptr1, ptr2].as_mut_ptr()),
                    Some(2),
                );
                let cstring = CStr::from_ptr(output.as_ptr()).to_str();
                assert_eq!(cstring, Ok(result));
                assert_eq!(code, 0);
                let _ = CString::from_raw(ptr);
            }
            let _ = CString::from_raw(ptr1);
            let _ = CString::from_raw(ptr2);
        }

        // Invalid too many arguments
        unsafe {
            let ptr1 = CString::new("1").unwrap().into_raw();
            let ptr2 = CString::new("2").unwrap().into_raw();
            let ptr3 = CString::new("3").unwrap().into_raw();
            for func in [
                "add_no_context",
                "add_no_context_return",
                "add_context",
                "add_context_return",
            ] {
                let mut output = [0i8; 1024];
                let ptr = CString::new(func).unwrap().into_raw();
                let code = extension.handle_call(
                    ptr,
                    output.as_mut_ptr(),
                    1024,
                    Some(vec![ptr1, ptr2, ptr3].as_mut_ptr()),
                    Some(3),
                );
                let cstring = CStr::from_ptr(output.as_ptr()).to_str();
                assert_eq!(cstring, Ok(""));
                assert_eq!(code, 23);
                let _ = CString::from_raw(ptr);
            }
            let _ = CString::from_raw(ptr1);
            let _ = CString::from_raw(ptr2);
            let _ = CString::from_raw(ptr3);
        }

        // Invalid too few arguments
        unsafe {
            let ptr1 = CString::new("1").unwrap().into_raw();
            for func in [
                "add_no_context",
                "add_no_context_return",
                "add_context",
                "add_context_return",
            ] {
                let mut output = [0i8; 1024];
                let ptr = CString::new(func).unwrap().into_raw();
                let code = extension.handle_call(
                    ptr,
                    output.as_mut_ptr(),
                    1024,
                    Some(vec![ptr1].as_mut_ptr()),
                    Some(1),
                );
                let cstring = CStr::from_ptr(output.as_ptr()).to_str();
                assert_eq!(cstring, Ok(""));
                assert_eq!(code, 21);
                let _ = CString::from_raw(ptr);
            }
            let _ = CString::from_raw(ptr1);
        }

        // Valid type conversion
        unsafe {
            let ptr1 = CString::new("1").unwrap().into_raw();
            let ptr2 = CString::new("\"2\"").unwrap().into_raw();
            for (func, result) in [
                ("add_no_context", "null"),
                ("add_no_context_return", "3"),
                ("add_context", "null"),
                ("add_context_return", "3"),
            ] {
                let mut output = [0i8; 1024];
                let ptr = CString::new(func).unwrap().into_raw();
                let code = extension.handle_call(
                    ptr,
                    output.as_mut_ptr(),
                    1024,
                    Some(vec![ptr1, ptr2].as_mut_ptr()),
                    Some(2),
                );
                let cstring = CStr::from_ptr(output.as_ptr()).to_str();
                assert_eq!(cstring, Ok(result));
                assert_eq!(code, 0);
                let _ = CString::from_raw(ptr);
            }
            let _ = CString::from_raw(ptr1);
            let _ = CString::from_raw(ptr2);
        }

        // Invalid type
        unsafe {
            let ptr1 = CString::new("1").unwrap().into_raw();
            let ptr2 = CString::new("\"two\"").unwrap().into_raw();
            for func in [
                "add_no_context",
                "add_no_context_return",
                "add_context",
                "add_context_return",
            ] {
                let mut output = [0i8; 1024];
                let ptr = CString::new(func).unwrap().into_raw();
                let code = extension.handle_call(
                    ptr,
                    output.as_mut_ptr(),
                    1024,
                    Some(vec![ptr1, ptr2].as_mut_ptr()),
                    Some(2),
                );
                let cstring = CStr::from_ptr(output.as_ptr()).to_str();
                assert_eq!(cstring, Ok(""));
                assert_eq!(code, 31);
                let _ = CString::from_raw(ptr);
            }
            let _ = CString::from_raw(ptr1);
            let _ = CString::from_raw(ptr2);
        }

        // Overflow
        unsafe {
            let ptr = CString::new("overflow").unwrap().into_raw();
            let mut output = [0i8; 1024];
            let code = extension.handle_call(ptr, output.as_mut_ptr(), 1024, None, None);
            let cstring = CStr::from_ptr(output.as_ptr()).to_str();
            assert_eq!(cstring, Ok(""));
            assert_eq!(code, 4);
            let _ = CString::from_raw(ptr);
        }

        // Result - true
        unsafe {
            let mut output = [0i8; 1024];
            let ptr = CString::new("result").unwrap().into_raw();
            let ptr_true = CString::new("true").unwrap().into_raw();
            let code = extension.handle_call(
                ptr,
                output.as_mut_ptr(),
                1024,
                Some(vec![ptr_true].as_mut_ptr()),
                Some(1),
            );
            let cstring = CStr::from_ptr(output.as_ptr()).to_str();
            assert_eq!(cstring, Ok("told to error"));
            assert_eq!(code, 9);
            let _ = CString::from_raw(ptr);
            let _ = CString::from_raw(ptr_true);
        }

        // Result - false
        unsafe {
            let ptr = CString::new("result").unwrap().into_raw();
            let ptr_false = CString::new("false").unwrap().into_raw();
            let mut output = [0i8; 1024];
            let code = extension.handle_call(
                ptr,
                output.as_mut_ptr(),
                1024,
                Some(vec![ptr_false].as_mut_ptr()),
                Some(1),
            );
            let cstring = CStr::from_ptr(output.as_ptr()).to_str();
            assert_eq!(cstring, Ok("told to succeed"));
            assert_eq!(code, 0);
            let _ = CString::from_raw(ptr);
            let _ = CString::from_raw(ptr_false);
        }
    }
}
