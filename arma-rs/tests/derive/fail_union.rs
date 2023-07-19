use arma_rs_proc::{FromArma, IntoArma};

#[derive(IntoArma, FromArma)]
union DeriveTest {
    first: u32,
}

fn main() {}
