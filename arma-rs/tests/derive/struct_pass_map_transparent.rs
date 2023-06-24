use arma_rs::{FromArma, IntoArma, Value};
use arma_rs_proc::{FromArma, IntoArma};

#[derive(IntoArma, FromArma, Debug, PartialEq)]
#[arma(transparent)]
struct DeriveTest {
    name: String,
}

fn main() {
    let serialized = DeriveTest {
        name: String::from("test"),
    };
    let deserialized = Value::String(String::from("test"));
    assert_eq!(serialized.to_arma(), deserialized);
    assert_eq!(
        DeriveTest::from_arma(deserialized.to_string()).unwrap(),
        serialized
    );
}
