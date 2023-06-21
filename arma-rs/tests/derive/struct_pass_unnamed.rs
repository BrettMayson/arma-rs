use arma_rs::{FromArma, IntoArma, Value};
use arma_rs_proc::{FromArma, IntoArma};

#[derive(IntoArma, FromArma, Debug, PartialEq)]
pub struct DeriveTest(u32);

pub fn main() {
    let unnamed = DeriveTest(1);
    let expected_value = Value::Number(1.0);
    assert_eq!(unnamed.to_arma(), expected_value);
    assert_eq!(
        DeriveTest::from_arma(expected_value.to_string()).unwrap(),
        unnamed
    );
}
