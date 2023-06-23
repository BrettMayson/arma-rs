use arma_rs_proc::{FromArma, IntoArma};

pub struct MyType;

#[derive(IntoArma, FromArma)]
pub struct DeriveTest(MyType);

#[derive(IntoArma, FromArma)]
pub struct Unit<T>(T);

pub fn main() {}
