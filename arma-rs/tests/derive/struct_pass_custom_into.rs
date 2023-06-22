use arma_rs::{IntoArma, Value};
use arma_rs_proc::FromArma;

#[derive(FromArma)]
pub struct DeriveTest(u32);

impl IntoArma for DeriveTest {
    fn to_arma(&self) -> Value {
        todo!()
    }
}

pub fn main() {}
