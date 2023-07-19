use arma_rs::{FromArma, FromArmaError, IntoArma, Value};
use arma_rs_proc::{FromArma, IntoArma};

#[derive(IntoArma)]
struct CustomFrom(u32);

impl FromArma for CustomFrom {
    fn from_arma(_: String) -> Result<Self, FromArmaError> {
        todo!()
    }
}

#[derive(FromArma)]
struct CustomInto(u32);

impl IntoArma for CustomInto {
    fn to_arma(&self) -> Value {
        todo!()
    }
}

fn main() {}
