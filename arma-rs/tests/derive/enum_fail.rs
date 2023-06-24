use arma_rs_proc::{FromArma, IntoArma};

#[derive(IntoArma, FromArma)]
enum DeriveTest {
    A,
    B,
}

fn main() {}
