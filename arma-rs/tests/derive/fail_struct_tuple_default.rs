use arma_rs_proc::FromArma;

#[derive(FromArma)]
#[arma(default)]
struct Container(String, u32);

#[derive(FromArma)]
struct Field(String, #[arma(default)] u32);

fn main() {}
