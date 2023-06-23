use arma_rs_proc::FromArma;

#[derive(FromArma)]
#[arma(default, transparent)]
pub struct DeriveTest {
    name: String,
}

pub fn main() {}
