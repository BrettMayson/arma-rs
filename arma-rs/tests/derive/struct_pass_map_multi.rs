use arma_rs::{FromArma, IntoArma, Value};
use arma_rs_proc::{FromArma, IntoArma};

#[derive(IntoArma, FromArma, Debug, PartialEq)]
pub struct DeriveTest {
    first: String,
    second: String,
}

pub fn main() {
    let serialized = DeriveTest {
        first: String::from("hello"),
        second: String::from("world"),
    };
    let deserialized_values = (
        Value::Array(vec![
            Value::String(String::from("first")),
            Value::String(String::from("hello")),
        ]),
        Value::Array(vec![
            Value::String(String::from("second")),
            Value::String(String::from("world")),
        ]),
    );
    let deserialized_permutations = vec![
        Value::Array(vec![
            deserialized_values.0.clone(),
            deserialized_values.1.clone(),
        ]),
        Value::Array(vec![
            deserialized_values.1.clone(),
            deserialized_values.0.clone(),
        ]),
    ];

    assert!(deserialized_permutations.contains(&serialized.to_arma()));
    for permutation in deserialized_permutations.iter() {
        assert_eq!(
            DeriveTest::from_arma(permutation.to_string()).unwrap(),
            serialized
        );
    }
}
