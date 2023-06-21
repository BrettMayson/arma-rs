use arma_rs_proc::{FromArma, IntoArma};

#[derive(IntoArma, FromArma)]
pub enum DeriveTest {
    A,
    B,
}

pub fn main() {}
