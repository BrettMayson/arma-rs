use arma_rs::{FromArma, IntoArma};

#[derive(IntoArma, FromArma)]
#[arma(transparent)]
struct TooManyFields {
    first: String,
    second: String,
}

fn main() {}
