use arma_rs::{arma_rs_proc, FromArma, IntoArma};

#[derive(arma_rs_proc::IntoArma, arma_rs_proc::FromArma)]
struct MyUnnamedStruct(i32, String);

#[test]
fn test_unnamed_struct_into() {
    let my_unnamed_struct = MyUnnamedStruct(1, "hello".to_string());
    assert_eq!(
        my_unnamed_struct.to_arma().to_string(),
        r#"[1,"hello"]"#.to_string()
    );
}

#[test]
fn test_unnamed_struct_from() {
    let my_unnamed_struct = MyUnnamedStruct::from_arma(r#"[1, "hello"]"#.to_string()).unwrap();
    assert_eq!(my_unnamed_struct.0, 1);
    assert_eq!(my_unnamed_struct.1, "hello".to_string());
}

#[derive(arma_rs_proc::IntoArma, arma_rs_proc::FromArma)]
struct MyNamedStruct {
    id: i32,
    message: String,
}

#[test]
fn test_named_struct_into() {
    let my_named_struct = MyNamedStruct {
        id: 1,
        message: "hello".to_string(),
    };
    let result = my_named_struct.to_arma().to_string();
    assert!(
        result == r#"[["id",1],["message","hello"]]"#
            || result == r#"[["message","hello"],["id",1]]"#
    );
}

#[test]
fn test_named_struct_from() {
    let my_named_struct =
        MyNamedStruct::from_arma(r#"[["id",1],["message","hello"]]"#.to_string()).unwrap();
    assert_eq!(my_named_struct.id, 1);
    assert_eq!(my_named_struct.message, "hello".to_string());
}
