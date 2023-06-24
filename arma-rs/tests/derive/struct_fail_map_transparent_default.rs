use arma_rs_proc::FromArma;

#[derive(FromArma)]
#[arma(default, transparent)]
struct DeriveTest {
    name: String,
}

fn main() {}
