use arma_rs_proc::{FromArma, IntoArma};

#[derive(IntoArma, FromArma)]
#[arma]
pub struct NoList;

#[derive(IntoArma, FromArma)]
#[arma("literal")]
pub struct Literal;

#[derive(IntoArma, FromArma)]
#[arma(unknown)]
pub struct Unknown;

#[derive(IntoArma, FromArma)]
#[arma(unknown::path)]
pub struct UnknownPath;

#[derive(IntoArma, FromArma)]
#[arma(duplicate, duplicate)]
pub struct Duplicate;

#[derive(IntoArma, FromArma)]
#[arma(duplicate::path)]
#[arma(duplicate::path)]
pub struct DuplicatePath;

pub fn main() {}
