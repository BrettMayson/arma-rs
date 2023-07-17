use arma_rs::{FromArma, Value};
use arma_rs_proc::FromArma;

#[derive(FromArma, Debug, PartialEq)]
#[arma(default)]
struct Container {
    first: String,
    second: u32,
}

impl Default for Container {
    fn default() -> Self {
        Self {
            first: String::from("hello"),
            second: 42,
        }
    }
}

#[derive(FromArma, Debug, PartialEq)]
struct Field {
    first: String,
    #[arma(default)]
    second: u32,
}

fn main() {
    let input = Value::Array(vec![]);
    assert_eq!(
        Container::from_arma(input.to_string()).unwrap(),
        Container::default()
    );

    let input = Value::Array(vec![Value::Array(vec![
        Value::String(String::from("first")),
        Value::String(String::from("bye")),
    ])]);
    assert_eq!(
        Container::from_arma(input.to_string()).unwrap(),
        Container {
            first: String::from("bye"),
            ..Default::default()
        }
    );

    let input = Value::Array(vec![Value::Array(vec![
        Value::String(String::from("first")),
        Value::String(String::from("bye")),
    ])]);
    assert_eq!(
        Field::from_arma(input.to_string()).unwrap(),
        Field {
            first: String::from("bye"),
            second: Default::default(),
        }
    );
}
