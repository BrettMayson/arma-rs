use arma_rs::{FromArma, FromArmaError, Value};
use arma_rs_proc::FromArma;

#[derive(FromArma, Debug, PartialEq)]
pub struct DeriveTest(String, u32);

pub fn main() {
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
