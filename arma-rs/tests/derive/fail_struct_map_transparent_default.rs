use arma_rs_proc::FromArma;

#[derive(FromArma)]
#[arma(transparent, default)]
struct Container {
    first: String,
}

#[derive(FromArma)]
#[arma(transparent)]
struct Field {
    #[arma(default)]
    first: String,
}

fn main() {}
