#[cfg(all(feature = "extension", feature = "call-context"))]

#[cfg(feature = "extension")]
mod extension {
    use arma_rs::{Context, ContextState, Extension, Group};

    #[test]
    fn root_command() {
        let extension = Extension::build()
            .command("hello", || -> &'static str { "Hello" })
            .finish()
            .testing();
        let (result, _) = unsafe { extension.call("hello", None) };
        assert_eq!(result, "Hello");
    }

    #[test]
    fn root_command_with_args() {
        let extension = Extension::build()
            .command("hello", |name: String| -> String {
                format!("Hello {name}")
            })
            .finish()
            .testing();
        let (result, _) = unsafe { extension.call("hello", Some(vec![String::from("John")])) };
        assert_eq!(result, "Hello John");
    }

    #[test]
    fn root_command_no_return() {
        let extension = Extension::build().command("nop", || {}).finish().testing();
        let (result, code) = unsafe { extension.call("nop", None) };
        assert_eq!(code, 0);
        assert_eq!(result, "null");
    }

    #[test]
    fn root_command_with_args_no_return() {
        let extension = Extension::build()
            .command("nop", |_: i8| {})
            .finish()
            .testing();
        let (result, code) = unsafe { extension.call("nop", Some(vec![String::from("4")])) };
        assert_eq!(code, 0);
        assert_eq!(result, "null");
    }

    #[test]
    fn group_command() {
        let extension = Extension::build()
            .group(
                "english",
                Group::new().command("hello", || -> &'static str { "Hello" }),
            )
            .finish()
            .testing();
        let (result, _) = unsafe { extension.call("english:hello", None) };
        assert_eq!(result, "Hello");
    }

    #[test]
    fn group_command_with_args() {
        let extension = Extension::build()
            .group(
                "english",
                Group::new().command("hello", |name: String| -> String {
                    format!("Hello {name}")
                }),
            )
            .finish()
            .testing();
        let (result, _) =
            unsafe { extension.call("english:hello", Some(vec![String::from("John")])) };
        assert_eq!(result, "Hello John");
    }

    #[test]
    fn sub_group_command() {
        let extension = Extension::build()
            .group(
                "greeting",
                Group::new().group(
                    "english",
                    Group::new().command("hello", || -> &'static str { "Hello" }),
                ),
            )
            .finish()
            .testing();
        let (result, _) = unsafe { extension.call("greeting:english:hello", None) };
        assert_eq!(result, "Hello");
    }

    #[test]
    fn sub_group_command_with_args() {
        let extension = Extension::build()
            .group(
                "greeting",
                Group::new().group(
                    "english",
                    Group::new().command("hello", |name: String| -> String {
                        format!("Hello {name}")
                    }),
                ),
            )
            .finish()
            .testing();
        let (result, _) =
            unsafe { extension.call("greeting:english:hello", Some(vec![String::from("John")])) };
        assert_eq!(result, "Hello John");
    }

    #[test]
    fn result_ok() {
        let extension = Extension::build()
            .command("result", || -> Result<&str, &str> { Ok("Ok") })
            .finish()
            .testing();
        let (result, code) = unsafe { extension.call("result", None) };
        assert_eq!(code, 0);
        assert_eq!(result, "Ok");
    }

    #[test]
    fn result_err() {
        let extension = Extension::build()
            .command("result", || -> Result<&str, &str> { Err("Err") })
            .finish()
            .testing();
        let (result, code) = unsafe { extension.call("result", None) };
        assert_eq!(code, 9);
        assert_eq!(result, "Err");
    }

    #[test]
    fn result_unit_ok() {
        let extension = Extension::build()
            .command("result", || -> Result<(), &str> { Ok(()) })
            .finish()
            .testing();
        let (result, code) = unsafe { extension.call("result", None) };
        assert_eq!(code, 0);
        assert_eq!(result, "null");
    }

    #[test]
    fn result_unit_err() {
        let extension = Extension::build()
            .command("result", || -> Result<&str, ()> { Err(()) })
            .finish()
            .testing();
        let (result, code) = unsafe { extension.call("result", None) };
        assert_eq!(code, 9);
        assert_eq!(result, "null");
    }

    #[test]
    fn result_unit_both() {
        let extension = Extension::build()
            .command("result", || -> Result<(), ()> { Ok(()) })
            .finish()
            .testing();
        let (result, code) = unsafe { extension.call("result", None) };
        assert_eq!(code, 0);
        assert_eq!(result, "null");
    }

    #[test]
    fn not_found() {
        let extension = Extension::build().finish().testing();
        let (result, code) = unsafe { extension.call("hello", None) };
        assert_eq!(code, 1);
        assert_eq!(result, "");
    }

    #[test]
    fn invalid_arg_count() {
        let extension = Extension::build()
            .command("hello", || -> &'static str { "Hello" })
            .finish()
            .testing();
        let (result, code) = unsafe { extension.call("hello", Some(vec![String::from("John")])) };
        assert_eq!(code, 21);
        assert_eq!(result, "");
    }

    #[test]
    fn invalid_arg_type() {
        let extension = Extension::build()
            .command("hello", |_: i32| -> &'static str { "Hello" })
            .finish()
            .testing();
        let (result, code) = unsafe { extension.call("hello", Some(vec![String::from("John")])) };
        assert_eq!(code, 30);
        assert_eq!(result, "");
    }

    #[test]
    fn invalid_arg_type_position() {
        let extension = Extension::build()
            .command("hello", |_: String, _: i32| -> &'static str { "Hello" })
            .finish()
            .testing();
        let (result, code) = unsafe {
            extension.call(
                "hello",
                Some(vec![String::from("John"), String::from("John")]),
            )
        };
        assert_eq!(code, 31);
        assert_eq!(result, "");
    }

    #[test]
    fn filled_output() {
        let extension = Extension::build()
            .command("hello", |ctx: Context| -> String {
                "X".repeat(ctx.buffer_len())
            })
            .finish()
            .testing();
        let (result, _) = unsafe { extension.call("hello", None) };
        assert_eq!(result.len(), extension.context().buffer_len());
    }

    #[test]
    fn filled_output_with_args() {
        let extension = Extension::build()
            .command("hello", |ctx: Context, item: String| -> String {
                item.repeat(ctx.buffer_len())
            })
            .finish()
            .testing();
        let (result, _) = unsafe { extension.call("hello", Some(vec![String::from('X')])) };
        assert_eq!(result.len(), extension.context().buffer_len());
    }

    #[test]
    fn output_overflow() {
        let extension = Extension::build()
            .command("hello", |ctx: Context| -> String {
                "X".repeat(ctx.buffer_len() + 1)
            })
            .finish()
            .testing();
        let (_, code) = unsafe { extension.call("hello", None) };
        assert_eq!(code, 4);
    }

    #[test]
    fn output_overflow_with_args() {
        let extension = Extension::build()
            .command("hello", |ctx: Context, item: String| -> String {
                item.repeat(ctx.buffer_len() + 1)
            })
            .finish()
            .testing();
        let (_, code) = unsafe { extension.call("hello", Some(vec![String::from('X')])) };
        assert_eq!(code, 4);
    }

    #[test]
    fn application_error_ok() {
        let extension = Extension::build()
            .command("hello", || -> Result<&str, &str> { Ok("Ok") })
            .finish()
            .testing();
        let (_, code) = unsafe { extension.call("hello", None) };
        assert_eq!(code, 0);
    }

    #[test]
    fn application_error_err() {
        let extension = Extension::build()
            .command("hello", || -> Result<&str, &str> { Err("Error") })
            .finish()
            .testing();
        let (_, code) = unsafe { extension.call("hello", None) };
        assert_eq!(code, 9);
    }

    #[test]
    fn state_build() {
        let extension = Extension::build()
            .state(String::from("foobar"))
            .finish()
            .testing();
        let value = extension.state().try_get::<String>();
        assert_eq!(value, Some(&String::from("foobar")));
    }

    #[test]
    fn state_new() {
        let extension = Extension::build()
            .command("new", |ctx: Context, new: String| ctx.global().set(new))
            .finish()
            .testing();

        let (_, _) = unsafe { extension.call("new", Some(vec![String::from("foobar")])) };
        let value = extension.state().try_get::<String>();
        assert_eq!(value, Some(&String::from("foobar")));
    }

    #[test]
    fn state_freeze() {
        let extension = Extension::build()
            .command("new", |ctx: Context, new: String| ctx.global().set(new))
            .freeze_state()
            .finish()
            .testing();
        assert!(extension.state().is_frozen());

        let (_, _) = unsafe { extension.call("new", Some(vec![String::from("foobar")])) };
        let value = extension.state().try_get::<String>();
        assert_eq!(value, None);
    }

    #[test]
    fn state_change() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        let extension = Extension::build()
            .state(AtomicUsize::new(42))
            .command("set", |ctx: Context, new: usize| {
                ctx.global()
                    .get::<AtomicUsize>()
                    .expect("state not found")
                    .store(new, Ordering::Relaxed)
            })
            .finish()
            .testing();

        let (_, _) = unsafe { extension.call("set", Some(vec![String::from("21")])) };
        let value = extension
            .state()
            .get::<AtomicUsize>()
            .load(Ordering::Relaxed);
        assert_eq!(value, 21);
    }

    #[cfg(feature = "call-context")]
    mod call_context {
        use arma_rs::{Caller, Context, Extension, Mission, Server, Source};

        #[test]
        fn call() {
            let extension = Extension::build()
                .command("call_ctx", |ctx: Context| -> String {
                    format!(
                        "{:?},{:?},{:?},{:?}",
                        ctx.caller(),
                        ctx.source(),
                        ctx.mission(),
                        ctx.server()
                    )
                })
                .finish()
                .testing();
            let (result, _) = unsafe {
                extension.call_with_context(
                    "call_ctx",
                    None,
                    Caller::Steam(123),
                    Source::Pbo(String::from("pbo")),
                    Mission::Mission(String::from("mission")),
                    Server::Multiplayer(String::from("server")),
                )
            };
            assert_eq!(
                result,
                "Steam(123),Pbo(\"pbo\"),Mission(\"mission\"),Multiplayer(\"server\")"
            );
        }

        #[test]
        fn availability() {
            fn is_call_ctx_default(ctx: Context) -> bool {
                ctx.caller() == &Caller::default()
                    && ctx.source() == &Source::default()
                    && ctx.mission() == &Mission::default()
                    && ctx.server() == &Server::default()
            }

            let extension = Extension::build()
                .command("is_call_ctx_default", is_call_ctx_default)
                .finish()
                .testing();
            let (result, _) = unsafe {
                extension.call_with_context(
                    "is_call_ctx_default",
                    None,
                    Caller::Steam(123),
                    Source::Pbo(String::from("pbo")),
                    Mission::Mission(String::from("mission")),
                    Server::Multiplayer(String::from("server")),
                )
            };
            assert_eq!(result, "false");
            assert!(!is_call_ctx_default(extension.context()));

            let (result, _) = unsafe { extension.call("is_call_ctx_default", None) };
            assert_eq!(result, "true");
            assert!(is_call_ctx_default(extension.context()));
        }
    }
}
