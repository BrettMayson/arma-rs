use arma_rs::{FromArma, IntoArma};

#[derive(FromArma, IntoArma)]
#[arma(transparent)]
struct TooManyFields {
    first: String,
    second: String,
}

fn main() {}
