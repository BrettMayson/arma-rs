use arma_rs::{FromArma, IntoArma};

#[derive(FromArma, IntoArma)]
#[arma(transparent, default)]
struct Container {
    first: String,
}

#[derive(FromArma, IntoArma)]
#[arma(transparent)]
struct Field {
    #[arma(default)]
    first: String,
}

fn main() {}
