use arma_rs::{Extension, Group};

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
