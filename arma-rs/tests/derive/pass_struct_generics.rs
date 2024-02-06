use arma_rs::{FromArma, IntoArma};

#[derive(FromArma, IntoArma)]
struct Newtype<T: IntoArma + FromArma>(T);

#[derive(FromArma, IntoArma)]
struct Tuple<A, B>(A, B)
where
    A: IntoArma + FromArma,
    B: IntoArma + FromArma;

#[derive(FromArma, IntoArma)]
struct Map<A, B>
where
    A: IntoArma + FromArma,
    B: IntoArma + FromArma,
{
    first: A,
    second: B,
}

fn main() {}
