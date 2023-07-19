use arma_rs_proc::{FromArma, IntoArma};

#[derive(IntoArma, FromArma)]
struct Unit;

#[derive(IntoArma, FromArma)]
struct Empty {}

#[derive(IntoArma, FromArma)]
struct EmptyTuple();

fn main() {}
