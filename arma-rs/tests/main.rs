use arma_rs::{ArmaContext, Context, Extension, Group};

include!(concat!(env!("OUT_DIR"), "/skeptic-tests.rs"));

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
    let (result, _) = unsafe { extension.call("english:hello", Some(vec![String::from("John")])) };
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
        .command("new", |ctx: Context, new: String| ctx.state().set(new))
        .finish()
        .testing();

    let (_, _) = unsafe { extension.call("new", Some(vec![String::from("foobar")])) };
    let value = extension.state().try_get::<String>();
    assert_eq!(value, Some(&String::from("foobar")));
}

#[test]
fn state_freeze() {
    let extension = Extension::build()
        .command("new", |ctx: Context, new: String| ctx.state().set(new))
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
            ctx.state()
                .get::<AtomicUsize>()
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

#[test]
fn arma_context_default() {
    let ctx = Extension::build().finish().testing().context();
    assert_eq!(ctx.steam_id(), None);
    assert_eq!(ctx.file_source(), None);
    assert_eq!(ctx.mission_name(), None);
    assert_eq!(ctx.server_name(), None);
}

#[test]
fn arma_context() {
    let mut extension = Extension::build()
        .command("steam_id", |ctx: Context| -> Option<String> {
            ctx.steam_id().map(String::from)
        })
        .command("file_source", |ctx: Context| -> Option<String> {
            ctx.file_source()
                .map(|p| p.to_str().unwrap())
                .map(String::from)
        })
        .command("mission_name", |ctx: Context| -> Option<String> {
            ctx.mission_name().map(String::from)
        })
        .command("server_name", |ctx: Context| -> Option<String> {
            ctx.server_name().map(String::from)
        })
        .finish()
        .testing();

    let (result, _) = unsafe {
        extension.call_with_context(
            "steam_id",
            None,
            ArmaContext::default().with_steam_id("steam_id"),
        )
    };
    assert_eq!(result, "steam_id");

    let (result, _) = unsafe {
        extension.call_with_context(
            "file_source",
            None,
            ArmaContext::default().with_file_source("file_source"),
        )
    };
    assert_eq!(result, "file_source");

    let (result, _) = unsafe {
        extension.call_with_context(
            "mission_name",
            None,
            ArmaContext::default().with_mission_name("mission_name"),
        )
    };
    assert_eq!(result, "mission_name");

    let (result, _) = unsafe {
        extension.call_with_context(
            "server_name",
            None,
            ArmaContext::default().with_server_name("server_name"),
        )
    };
    assert_eq!(result, "server_name");
}
