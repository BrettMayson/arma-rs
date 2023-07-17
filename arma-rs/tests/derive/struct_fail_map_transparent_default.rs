use arma_rs_proc::FromArma;

#[derive(FromArma)]
#[arma(default, transparent)]
struct Container {
    name: String,
}

#[derive(FromArma)]
#[arma(transparent)]
struct Field {
    #[arma(default)]
    name: String,
}

fn main() {}
