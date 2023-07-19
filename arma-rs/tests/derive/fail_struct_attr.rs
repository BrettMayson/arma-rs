use arma_rs_proc::{FromArma, IntoArma};

#[derive(IntoArma, FromArma)]
#[arma]
struct NoList;

#[derive(IntoArma, FromArma)]
#[arma("literal")]
struct Literal;

#[derive(IntoArma, FromArma)]
#[arma(unknown)]
struct Unknown;

#[derive(IntoArma, FromArma)]
#[arma(unknown::path)]
struct UnknownPath;

#[derive(IntoArma, FromArma)]
#[arma(duplicate, duplicate)]
struct Duplicate;

#[derive(IntoArma, FromArma)]
struct FieldUnknown {
    #[arma(unknown)]
    test: u32,
}

#[derive(IntoArma, FromArma)]
struct FieldDuplicate {
    #[arma(duplicate, duplicate)]
    test: u32,
}

fn main() {}
