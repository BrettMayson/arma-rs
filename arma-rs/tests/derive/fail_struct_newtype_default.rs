use arma_rs_proc::{FromArma, IntoArma};

#[derive(FromArma, IntoArma)]
#[arma(default)]
struct Container(u32);

#[derive(FromArma, IntoArma)]
struct Field(#[arma(default)] u32);

fn main() {}
