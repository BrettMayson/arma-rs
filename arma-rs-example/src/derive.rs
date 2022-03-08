use arma_rs::Arma;

#[derive(Arma)]
struct MyUnnamedStruct(i32, String);

#[test]
fn test_unnamed_struct() {
    let my_unnamed_struct = MyUnnamedStruct::from_arma(r#"[1, "hello"]"#.to_string()).unwrap();
    assert_eq!(my_unnamed_struct.0, 1);
    assert_eq!(my_unnamed_struct.1, "hello".to_string());
    assert_eq!(
        my_unnamed_struct.to_arma().to_string(),
        r#"[1,"hello"]"#.to_string()
    );
}

#[derive(Arma)]
struct MyNamedStruct {
    id: i32,
    message: String,
}

#[test]
fn test_named_struct() {
    let my_named_struct =
        MyNamedStruct::from_arma(r#"[["id",1], ["message","hello"]]"#.to_string()).unwrap();
    assert_eq!(my_named_struct.id, 1);
    assert_eq!(my_named_struct.message, "hello".to_string());
    assert_eq!(
        my_named_struct.to_arma().to_string(),
        r#"[["id",1],["message","hello"]]"#.to_string()
    );
}
