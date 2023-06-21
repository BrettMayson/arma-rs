use arma_rs::{FromArma, IntoArma, Value};
use arma_rs_proc::{FromArma, IntoArma};

#[derive(IntoArma, FromArma, Debug, PartialEq)]
pub struct DeriveTest(String, u32, f32);

pub fn main() {
    let multi_unnamed = DeriveTest(String::from("test"), 1, 2.0);
    let expected_value = Value::Array(vec![
        Value::String(String::from("test")),
        Value::Number(1.0),
        Value::Number(2.0),
    ]);
    assert_eq!(multi_unnamed.to_arma(), expected_value);
    assert_eq!(
        DeriveTest::from_arma(expected_value.to_string()).unwrap(),
        multi_unnamed
    );
}
