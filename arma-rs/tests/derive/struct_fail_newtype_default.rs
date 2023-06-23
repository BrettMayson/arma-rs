use arma_rs_proc::FromArma;

#[derive(FromArma)]
#[arma(default)]
pub struct DeriveTest(u32);

pub fn main() {}
