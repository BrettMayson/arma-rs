use arma_rs_proc::{FromArma, IntoArma};

#[derive(FromArma, IntoArma)]
#[arma(transparent)]
struct DeriveTest(String, u32);

fn main() {}
