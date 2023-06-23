use arma_rs_proc::{FromArma, IntoArma};

#[derive(IntoArma, FromArma)]
#[arma(transparent)]
pub struct DeriveTest {
    first: String,
    second: String,
}

pub fn main() {}
