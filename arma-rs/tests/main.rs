use arma_rs::{Context, Extension, Group, State};

#[test]
fn root_command() {
    let mut extension = Extension::build()
        .command("hello", || -> &'static str { "Hello" })
        .finish()
        .testing();
    let (result, _) = unsafe { extension.call("hello", None) };
    assert_eq!(result, "Hello");
}

#[test]
fn root_command_with_args() {
    let mut extension = Extension::build()
        .command("hello", |name: String| -> String {
            format!("Hello {}", name)
        })
        .finish()
        .testing();
    let (result, _) = unsafe { extension.call("hello", Some(vec![String::from("John")])) };
    assert_eq!(result, "Hello John");
}

#[test]
fn root_command_no_return() {
    let mut extension = Extension::build().command("nop", || {}).finish().testing();
    let (result, code) = unsafe { extension.call("nop", None) };
    assert_eq!(code, 0);
    assert_eq!(result, "null");
}

#[test]
fn root_command_with_args_no_return() {
    let mut extension = Extension::build()
        .command("nop", |_: i8| {})
        .finish()
        .testing();
    let (result, code) = unsafe { extension.call("nop", Some(vec![String::from("4")])) };
    assert_eq!(code, 0);
    assert_eq!(result, "null");
}

#[test]
fn group_command() {
    let mut extension = Extension::build()
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
    let mut extension = Extension::build()
        .group(
            "english",
            Group::new().command("hello", |name: String| -> String {
                format!("Hello {}", name)
            }),
        )
        .finish()
        .testing();
    let (result, _) = unsafe { extension.call("english:hello", Some(vec![String::from("John")])) };
    assert_eq!(result, "Hello John");
}

#[test]
fn sub_group_command() {
    let mut extension = Extension::build()
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
    let mut extension = Extension::build()
        .group(
            "greeting",
            Group::new().group(
                "english",
                Group::new().command("hello", |name: String| -> String {
                    format!("Hello {}", name)
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
    let mut extension = Extension::build()
        .command("result", || -> Result<&str, &str> { Ok("Ok") })
        .finish()
        .testing();
    let (result, code) = unsafe { extension.call("result", None) };
    assert_eq!(code, 0);
    assert_eq!(result, "Ok");
}

#[test]
fn result_err() {
    let mut extension = Extension::build()
        .command("result", || -> Result<&str, &str> { Err("Err") })
        .finish()
        .testing();
    let (result, code) = unsafe { extension.call("result", None) };
    assert_eq!(code, 9);
    assert_eq!(result, "Err");
}

#[test]
fn result_unit_ok() {
    let mut extension = Extension::build()
        .command("result", || -> Result<(), &str> { Ok(()) })
        .finish()
        .testing();
    let (result, code) = unsafe { extension.call("result", None) };
    assert_eq!(code, 0);
    assert_eq!(result, "null");
}

#[test]
fn result_unit_err() {
    let mut extension = Extension::build()
        .command("result", || -> Result<&str, ()> { Err(()) })
        .finish()
        .testing();
    let (result, code) = unsafe { extension.call("result", None) };
    assert_eq!(code, 9);
    assert_eq!(result, "null");
}

#[test]
fn result_unit_both() {
    let mut extension = Extension::build()
        .command("result", || -> Result<(), ()> { Ok(()) })
        .finish()
        .testing();
    let (result, code) = unsafe { extension.call("result", None) };
    assert_eq!(code, 0);
    assert_eq!(result, "null");
}

#[test]
fn not_found() {
    let mut extension = Extension::build().finish().testing();
    let (result, code) = unsafe { extension.call("hello", None) };
    assert_eq!(code, 1);
    assert_eq!(result, "");
}

#[test]
fn invalid_arg_count() {
    let mut extension = Extension::build()
        .command("hello", || -> &'static str { "Hello" })
        .finish()
        .testing();
    let (result, code) = unsafe { extension.call("hello", Some(vec![String::from("John")])) };
    assert_eq!(code, 21);
    assert_eq!(result, "");
}

#[test]
fn invalid_arg_type() {
    let mut extension = Extension::build()
        .command("hello", |_: i32| -> &'static str { "Hello" })
        .finish()
        .testing();
    let (result, code) = unsafe { extension.call("hello", Some(vec![String::from("John")])) };
    assert_eq!(code, 30);
    assert_eq!(result, "");
}

#[test]
fn invalid_arg_type_position() {
    let mut extension = Extension::build()
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
    let mut extension = Extension::build()
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
    let mut extension = Extension::build()
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
    let mut extension = Extension::build()
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
    let mut extension = Extension::build()
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
    let mut extension = Extension::build()
        .command("hello", || -> Result<&str, &str> { Ok("Ok") })
        .finish()
        .testing();
    let (_, code) = unsafe { extension.call("hello", None) };
    assert_eq!(code, 0);
}

#[test]
fn application_error_err() {
    let mut extension = Extension::build()
        .command("hello", || -> Result<&str, &str> { Err("Error") })
        .finish()
        .testing();
    let (_, code) = unsafe { extension.call("hello", None) };
    assert_eq!(code, 9);
}

#[test]
fn state() {
    let mut extension = Extension::build_with_state(String::new())
        .command("get", |state: &mut State<String>| -> String {
            state.clone()
        })
        .command("set", |state: &mut State<String>, new: String| {
            **state = new
        })
        .finish()
        .testing();
    let (res, _) = unsafe { extension.call("get", None) };
    assert_eq!(res, "");
    unsafe { extension.call("set", Some(vec![String::from("foobar")])) };
    let (res, _) = unsafe { extension.call("get", None) };
    assert_eq!(res, "foobar");
}
