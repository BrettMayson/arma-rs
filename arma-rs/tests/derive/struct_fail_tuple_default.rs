use arma_rs_proc::FromArma;

#[derive(FromArma)]
#[arma(default)]
struct DeriveTest(String, u32);

fn main() {}
