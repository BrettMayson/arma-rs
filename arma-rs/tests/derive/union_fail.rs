use arma_rs_proc::{FromArma, IntoArma};

#[derive(IntoArma, FromArma)]
pub union DeriveTest {
    a: u32,
    b: f32,
}

pub fn main() {}