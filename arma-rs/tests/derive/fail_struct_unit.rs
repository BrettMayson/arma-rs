use arma_rs::{FromArma, IntoArma};

#[derive(FromArma, IntoArma)]
struct Unit;

#[derive(FromArma, IntoArma)]
struct EmptyMap {}

#[derive(FromArma, IntoArma)]
struct EmptyTuple();

fn main() {}
