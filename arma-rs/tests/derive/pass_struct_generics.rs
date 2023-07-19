use arma_rs::{FromArma, IntoArma};
use arma_rs_proc::{FromArma, IntoArma};

#[derive(IntoArma, FromArma)]
struct Newtype<T: IntoArma + FromArma>(T);

#[derive(IntoArma, FromArma)]
struct Tuple<A, B>(A, B)
where
    A: IntoArma + FromArma,
    B: IntoArma + FromArma;

#[derive(IntoArma, FromArma)]
struct Map<A, B>
where
    A: IntoArma + FromArma,
    B: IntoArma + FromArma,
{
    first: A,
    second: B,
}

fn main() {}
