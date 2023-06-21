use arma_rs::{FromArma, Value};
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
    assert_eq!(
        DeriveTest::from_arma(input.to_string()),
        Err(String::from("DeriveTest: expected 1 fields, got 2"))
    );

    let input = Value::Array(vec![]);
    assert_eq!(
        DeriveTest::from_arma(input.to_string()),
        Err(String::from("DeriveTest: expected 1 fields, got 0"))
    );

    let input = Value::Array(vec![Value::Array(vec![
        Value::String(String::from("name")),
        Value::String(String::from("test")),
        Value::String(String::from("test")),
    ])]);
    assert_eq!(
        DeriveTest::from_arma(input.to_string()),
        Err(String::from("DeriveTest: too many values in tuple"))
    );

    let input = Value::Array(vec![Value::Array(vec![Value::String(String::from(
        "name",
    ))])]);
    assert_eq!(
        DeriveTest::from_arma(input.to_string()),
        Err(String::from("DeriveTest: missing value in tuple"))
    );
}
