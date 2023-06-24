use arma_rs::{FromArma, IntoArma, Value};
use arma_rs_proc::{FromArma, IntoArma};

#[derive(IntoArma, FromArma, Debug, PartialEq)]
struct DeriveTest(String, u32, f32);

fn main() {
    let serialized = DeriveTest(String::from("test"), 1, 2.0);
    let deserialized = Value::Array(vec![
        Value::String(String::from("test")),
        Value::Number(1.0),
        Value::Number(2.0),
    ]);
    assert_eq!(serialized.to_arma(), deserialized);
    assert_eq!(
        DeriveTest::from_arma(deserialized.to_string()).unwrap(),
        serialized
    );
}
