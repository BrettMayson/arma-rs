#[test]
fn derive_compile() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/derive/*fail*.rs");
    tests.pass("tests/derive/*pass*.rs");
}

mod derive_errors {
    use arma_rs::{FromArma, FromArmaError, Value};
    use arma_rs_proc::FromArma;

    #[test]
    fn map_input_size() {
        #[derive(FromArma, Debug, PartialEq)]
        pub struct DeriveTest {
            name: String,
        }

        let input = Value::Array(vec![
            Value::Array(vec![
                Value::String(String::from("name")),
                Value::String(String::from("test")),
            ]),
            Value::Array(vec![
                Value::String(String::from("additional")),
                Value::String(String::from("should error")),
            ]),
        ]);
        let result = DeriveTest::from_arma(input.to_string());
        assert!(
            matches!(
                result,
                Err(FromArmaError::SizeMismatch {
                    expected: 1,
                    actual: 2
                })
            ),
            "Expected SizeMismatch error, got {:?}",
            result
        );

        let input = Value::Array(vec![]);
        let result = DeriveTest::from_arma(input.to_string());
        assert!(
            matches!(
                result,
                Err(FromArmaError::SizeMismatch {
                    expected: 1,
                    actual: 0
                })
            ),
            "Expected SizeMismatch error, got {:?}",
            result
        );
    }

    #[test]
    fn map_field_name() {
        #[derive(FromArma, Debug, PartialEq)]
        pub struct DeriveTest {
            name: String,
        }

        let deserialized = Value::Array(vec![Value::Array(vec![
            Value::String(String::from("wrong-name")),
            Value::String(String::from("test")),
        ])]);
        let result = DeriveTest::from_arma(deserialized.to_string());
        assert!(
            matches!(result, Err(FromArmaError::MapMissingField(_))),
            "Expected MapMissingField error, got {:?}",
            result
        );
    }

    #[test]
    fn tuple_input_size() {
        #[derive(FromArma, Debug, PartialEq)]
        pub struct DeriveTest(String, u32);

        let input = Value::Array(vec![
            Value::String(String::from("test")),
            Value::Number(1.0),
            Value::String(String::from("should error")),
        ]);
        let result = DeriveTest::from_arma(input.to_string());
        assert!(
            matches!(
                result,
                Err(FromArmaError::SizeMismatch {
                    expected: 2,
                    actual: 3
                })
            ),
            "Expected SizeMismatch error, got {:?}",
            result
        );

        let input = Value::Array(vec![Value::String(String::from("test"))]);
        let result = DeriveTest::from_arma(input.to_string());
        assert!(
            matches!(
                result,
                Err(FromArmaError::SizeMismatch {
                    expected: 2,
                    actual: 1
                })
            ),
            "Expected SizeMismatch error, got {:?}",
            result
        );
    }
}
