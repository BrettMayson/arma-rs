#[test]
#[ignore]
fn derive_compile() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/derive/*fail*.rs");
    tests.pass("tests/derive/*pass*.rs");
}

mod derive_errors {
    use arma_rs::{FromArma, FromArmaError, Value};
    use arma_rs_proc::FromArma;

    #[test]
    fn map_missing_field() {
        #[derive(FromArma, Debug, PartialEq)]
        pub struct DeriveTest {
            pub test: String,
        }

        let input = Value::Array(vec![]);
        let result = DeriveTest::from_arma(input.to_string());
        assert!(
            matches!(
                result,
                Err(FromArmaError::MapMissingField(ref field)) if field == "test"
            ),
            "Expected MapMissingField error, got {:?}",
            result
        );
    }

    #[test]
    fn map_unknown_field() {
        #[derive(FromArma, Debug, PartialEq)]
        pub struct DeriveTest {
            pub test: String,
        }

        let input = Value::Array(vec![Value::Array(vec![
            Value::String(String::from("unknown")),
            Value::String(String::from("blabla")),
        ])]);
        let result = DeriveTest::from_arma(input.to_string());
        assert!(
            matches!(
                result,
                Err(FromArmaError::MapUnknownField(ref field)) if field == "unknown"
            ),
            "Expected MapUnknownField error, got {:?}",
            result
        );
    }

    #[test]
    fn map_default_unknown_field() {
        #[derive(FromArma, Default, Debug, PartialEq)]
        pub struct DeriveTest {
            pub test: String,
        }

        let input = Value::Array(vec![
            Value::Array(vec![
                Value::String(String::from("test")),
                Value::String(String::from("expected")),
            ]),
            Value::Array(vec![
                Value::String(String::from("unknown")),
                Value::String(String::from("blabla")),
            ]),
        ]);
        let result = DeriveTest::from_arma(input.to_string());
        assert!(
            matches!(
                result,
                Err(FromArmaError::MapUnknownField(ref field)) if field == "unknown"
            ),
            "Expected MapUnknownField error, got {:?}",
            result
        );
    }
}
