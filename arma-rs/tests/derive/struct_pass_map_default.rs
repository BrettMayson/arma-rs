use arma_rs::{FromArma, Value};
use arma_rs_proc::FromArma;

#[derive(FromArma, Debug, PartialEq)]
#[arma(default)]
pub struct DeriveTest {
    first: String,
    second: u32,
}

impl Default for DeriveTest {
    fn default() -> Self {
        Self {
            first: String::from("hello"),
            second: 42,
        }
    }
}

pub fn main() {
    let input = Value::Array(vec![]);
    assert_eq!(
        DeriveTest::from_arma(input.to_string()).unwrap(),
        DeriveTest::default()
    );

    let input = Value::Array(vec![Value::Array(vec![
        Value::String(String::from("first")),
        Value::String(String::from("bye")),
    ])]);
    assert_eq!(
        DeriveTest::from_arma(input.to_string()).unwrap(),
        DeriveTest {
            first: String::from("bye"),
            ..Default::default()
        }
    );
}
