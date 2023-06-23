use arma_rs_proc::FromArma;

#[derive(FromArma)]
#[arma(default)]
pub struct DeriveTest(String, u32);

pub fn main() {}
