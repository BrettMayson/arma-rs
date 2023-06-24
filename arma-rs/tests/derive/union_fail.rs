use arma_rs_proc::{FromArma, IntoArma};

#[derive(IntoArma, FromArma)]
union DeriveTest {
    a: u32,
    b: f32,
}

fn main() {}
