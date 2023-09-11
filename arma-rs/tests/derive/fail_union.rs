use arma_rs::{FromArma, IntoArma};

#[derive(IntoArma, FromArma)]
union DeriveTest {
    first: u32,
}

fn main() {}
