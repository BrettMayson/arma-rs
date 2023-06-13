use arma_rs_proc::Arma;

#[derive(Arma)]
pub union DeriveTest {
    a: u32,
    b: f32,
}

pub fn main() {}
