use arma_rs::{FromArma, IntoArma};

#[derive(FromArma, IntoArma)]
#[arma(transparent)]
struct DeriveTest(u32);

fn main() {}
