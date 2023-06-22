use arma_rs::{FromArma, FromArmaError, Value};
use arma_rs_proc::FromArma;

#[derive(FromArma, Debug, PartialEq)]
pub struct DeriveTest {
    name: String,
}

pub fn main() {
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
