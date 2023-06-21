use arma_rs::{FromArma, Value};
use arma_rs_proc::FromArma;

#[derive(FromArma, Debug, PartialEq)]
pub struct DeriveTest(String, u32);

pub fn main() {
    let input = Value::Array(vec![
        Value::String(String::from("test")),
        Value::Number(1.0),
        Value::String(String::from("should error")),
    ]);
    assert_eq!(
        DeriveTest::from_arma(input.to_string()),
        Err(String::from("DeriveTest: too many values in tuple"))
    );

    let input = Value::Array(vec![Value::String(String::from("test"))]);
    assert_eq!(
        DeriveTest::from_arma(input.to_string()),
        Err(String::from("DeriveTest: missing value in tuple"))
    );
}
