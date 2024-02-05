use arma_rs::{FromArma, IntoArma};

#[derive(FromArma, IntoArma)]
#[arma]
struct NoList {
    test: u32,
}

#[derive(FromArma, IntoArma)]
#[arma("literal")]
struct Literal {
    test: u32,
}

#[derive(FromArma, IntoArma)]
#[arma(unknown)]
struct Unknown {
    test: u32,
}

#[derive(FromArma, IntoArma)]
#[arma(unknown::path)]
struct UnknownPath {
    test: u32,
}

#[derive(FromArma, IntoArma)]
#[arma(default, default)]
struct Duplicate {
    test: u32,
}

#[derive(FromArma, IntoArma)]
#[arma(default)]
#[arma(default)]
struct StackedDuplicate {
    test: u32,
}

#[derive(FromArma, IntoArma)]
struct FieldUnknown {
    #[arma(unknown)]
    test: u32,
}

#[derive(FromArma, IntoArma)]
struct FieldDuplicate {
    #[arma(default, default)]
    test: u32,
}

#[derive(FromArma, IntoArma)]
struct FieldStackedDuplicate {
    #[arma(default)]
    #[arma(default)]
    test: u32,
}

#[derive(FromArma, IntoArma)]
#[arma(unknown, default, default)]
struct MultipleErrors {
    test: u32,
}

fn main() {}
