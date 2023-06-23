use arma_rs_proc::{FromArma, IntoArma};

pub struct MyType;

#[derive(IntoArma, FromArma)]
pub struct DeriveTest(MyType);

pub fn main() {}
