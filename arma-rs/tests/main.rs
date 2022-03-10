use arma_rs::{Context, Extension, Group};

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
            format!("Hello {}", name)
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
    assert_eq!(result, "");
}

#[test]
fn root_command_with_args_no_return() {
    let extension = Extension::build()
        .command("nop", |_: i8| {})
        .finish()
        .testing();
    let (result, code) = unsafe { extension.call("nop", Some(vec![String::from("4")])) };
    assert_eq!(code, 0);
    assert_eq!(result, "");
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
    let extension = Extension::build()
        .command("hello", || -> Result<&str, &str> { Ok("Hello") })
        .finish()
        .testing();
    let (result, _) = unsafe { extension.call("hello", None) };
    assert_eq!(result, "Hello");
}

#[test]
fn result_err() {
    let extension = Extension::build()
        .command("hello", || -> Result<&str, &str> { Err("Error") })
        .finish()
        .testing();
    let (result, _) = unsafe { extension.call("hello", None) };
    assert_eq!(result, "Error");
}

#[test]
fn not_found() {
    let extension = Extension::build().finish().testing();
    let (_, code) = unsafe { extension.call("hello", None) };
    assert_eq!(code, 1);
}

#[test]
fn invalid_arg_count() {
    let extension = Extension::build()
        .command("hello", || -> &'static str { "Hello" })
        .finish()
        .testing();
    let (_, code) = unsafe { extension.call("hello", Some(vec![String::from("John")])) };
    assert_eq!(code, 21);
}

#[test]
fn invalid_arg_type() {
    let extension = Extension::build()
        .command("hello", |_: i32| -> &'static str { "Hello" })
        .finish()
        .testing();
    let (_, code) = unsafe { extension.call("hello", Some(vec![String::from("John")])) };
    assert_eq!(code, 30);
}

#[test]
fn invalid_arg_type_position() {
    let extension = Extension::build()
        .command("hello", |_: String, _: i32| -> &'static str { "Hello" })
        .finish()
        .testing();
    let (_, code) = unsafe {
        extension.call(
            "hello",
            Some(vec![String::from("John"), String::from("John")]),
        )
    };
    assert_eq!(code, 31);
}

#[test]
fn output_overflow() {
    let extension = Extension::build()
        .command("hello", |ctx: Context| -> String {
            "X".repeat((ctx.buffer_len() / 8) + 1)
        })
        .finish()
        .testing();
    let (_, code) = unsafe { extension.call("hello", None) };
    assert_eq!(code, 4);
}

#[test]
fn output_overflow_with_args() {
    let extension = Extension::build()
        .command("hello", |ctx: Context, item: char| -> String {
            item.to_string().repeat((ctx.buffer_len() / 8) + 1)
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
