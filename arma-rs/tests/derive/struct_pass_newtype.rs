use arma_rs::{FromArma, IntoArma, Value};
use arma_rs_proc::{FromArma, IntoArma};

#[derive(IntoArma, FromArma, Debug, PartialEq)]
pub struct DeriveTest(u32);

pub fn main() {
    let serialized = DeriveTest(1);
    let deserialized = Value::Number(1.0);
    assert_eq!(serialized.to_arma(), deserialized);
    assert_eq!(
        DeriveTest::from_arma(deserialized.to_string()).unwrap(),
        serialized
    );
}
