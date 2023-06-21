use arma_rs::{FromArma, IntoArma, Value};
use arma_rs_proc::{FromArma, IntoArma};

#[derive(IntoArma, FromArma, Debug, PartialEq)]
pub struct DeriveTest {
    name: String,
}

pub fn main() {
    let named = DeriveTest {
        name: String::from("test"),
    };
    let expected_value = Value::Array(vec![Value::Array(vec![
        Value::String(String::from("name")),
        Value::String(String::from("test")),
    ])]);
    assert_eq!(named.to_arma(), expected_value);
    assert_eq!(
        DeriveTest::from_arma(expected_value.to_string()).unwrap(),
        named
    );
}
