use arma_rs::{quote, simple_array};

#[test]
fn test_macros() {
    assert_eq!(r#"["my_data"]"#, quote!(simple_array!("my_data")));
    assert_eq!(
        r#"["my_player","[""items"",""in"",""quotes""]"]"#,
        quote!(simple_array!("my_player", "[\"items\",\"in\",\"quotes\"]"))
    );
    assert_eq!(r#"["my_data",10]"#, quote!(simple_array!("my_data", 10)));
    assert_eq!(
        r#"[["existing data"],10]"#,
        quote!(simple_array!(vec!["existing data"], 10))
    );
    assert_eq!(
        r#"[["existing data"],10,["my_data",10]]"#,
        quote!(simple_array!(
            vec!["existing data"],
            10,
            simple_array!("my_data", 10)
        ))
    );
}
