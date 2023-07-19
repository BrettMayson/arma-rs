use arma_rs_proc::FromArma;

#[derive(FromArma)]
#[arma(default)]
struct Container(u32);

#[derive(FromArma)]
struct Field(#[arma(default)] u32);

fn main() {}
