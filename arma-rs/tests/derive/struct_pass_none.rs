use arma_rs::{FromArma, IntoArma, Value};
use arma_rs_proc::{FromArma, IntoArma};

#[derive(IntoArma, FromArma, Debug, PartialEq)]
pub struct DeriveTest {}

pub fn main() {
    let named = DeriveTest {};
    let expected_value = Value::Array(vec![]);
    assert_eq!(named.to_arma(), expected_value);
    assert_eq!(
        DeriveTest::from_arma(expected_value.to_string()).unwrap(),
        named
    );
}
