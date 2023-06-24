use arma_rs_proc::{FromArma, IntoArma};

#[derive(IntoArma, FromArma)]
#[arma(transparent)]
struct DeriveTest {
    first: String,
    second: String,
}

fn main() {}
