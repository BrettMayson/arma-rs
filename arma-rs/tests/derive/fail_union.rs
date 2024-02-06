use arma_rs::{FromArma, IntoArma};

#[derive(FromArma, IntoArma)]
union DeriveTest {
    first: u32,
}

fn main() {}
