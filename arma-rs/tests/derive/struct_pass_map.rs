use arma_rs::{FromArma, IntoArma, Value};
use arma_rs_proc::{FromArma, IntoArma};

#[derive(IntoArma, FromArma, Debug, PartialEq)]
struct DeriveTest {
    name: String,
}

fn main() {
    let serialized = DeriveTest {
        name: String::from("test"),
    };
    let deserialized = Value::Array(vec![Value::Array(vec![
        Value::String(String::from("name")),
        Value::String(String::from("test")),
    ])]);
    assert_eq!(serialized.to_arma(), deserialized);
    assert_eq!(
        DeriveTest::from_arma(deserialized.to_string()).unwrap(),
        serialized
    );
}
