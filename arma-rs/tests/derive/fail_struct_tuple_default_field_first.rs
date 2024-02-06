use arma_rs::{FromArma, IntoArma};

#[derive(FromArma, IntoArma)]
struct DeriveTest(#[arma(default)] String, u32, bool);

fn main() {}
