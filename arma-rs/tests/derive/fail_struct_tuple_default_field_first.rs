use arma_rs_proc::{FromArma, IntoArma};

#[derive(FromArma, IntoArma)]
struct DeriveTest(#[arma(default)] String, u32, bool);

fn main() {}
