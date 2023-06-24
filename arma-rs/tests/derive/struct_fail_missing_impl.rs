use arma_rs_proc::{FromArma, IntoArma};

struct MyType;

#[derive(IntoArma, FromArma)]
struct DeriveTest(MyType);

#[derive(IntoArma, FromArma)]
struct Unit<T>(T);

fn main() {}
