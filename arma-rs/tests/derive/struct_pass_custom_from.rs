use arma_rs::{FromArma, FromArmaError};
use arma_rs_proc::IntoArma;

#[derive(IntoArma)]
pub struct DeriveTest(u32);

impl FromArma for DeriveTest {
    fn from_arma(_: String) -> Result<Self, FromArmaError> {
        todo!()
    }
}

pub fn main() {}
