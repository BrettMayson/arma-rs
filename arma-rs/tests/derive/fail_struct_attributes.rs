use arma_rs_proc::{FromArma, IntoArma};

#[derive(IntoArma, FromArma)]
#[arma]
struct NoList {
    test: u32,
}

#[derive(IntoArma, FromArma)]
#[arma("literal")]
struct Literal {
    test: u32,
}

#[derive(IntoArma, FromArma)]
#[arma(unknown)]
struct Unknown {
    test: u32,
}

#[derive(IntoArma, FromArma)]
#[arma(unknown::path)]
struct UnknownPath {
    test: u32,
}

#[derive(IntoArma, FromArma)]
#[arma(default, default)]
struct Duplicate {
    test: u32,
}

#[derive(IntoArma, FromArma)]
#[arma(default)]
#[arma(default)]
struct StackedDuplicate {
    test: u32,
}

#[derive(IntoArma, FromArma)]
struct FieldUnknown {
    #[arma(unknown)]
    test: u32,
}

#[derive(IntoArma, FromArma)]
struct FieldDuplicate {
    #[arma(default, default)]
    test: u32,
}

#[derive(IntoArma, FromArma)]
struct FieldStackedDuplicate {
    #[arma(default)]
    #[arma(default)]
    test: u32,
}

fn main() {}
