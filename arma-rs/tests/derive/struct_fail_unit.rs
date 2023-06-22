use arma_rs_proc::{FromArma, IntoArma};

#[derive(IntoArma, FromArma)]
pub struct Unit;

#[derive(IntoArma, FromArma)]
pub struct Empty {}

#[derive(IntoArma, FromArma)]
pub struct EmptyTuple();

pub fn main() {}
