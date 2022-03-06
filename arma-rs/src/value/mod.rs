use std::fmt::Display;

mod features;
mod from_arma;
mod into_arma;

pub use from_arma::FromArma;
pub use into_arma::IntoArma;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
/// A value that can be converted to and from Arma types.
pub enum Value {
    /// Arma's `nil` value.
    /// Represented as `null`
    Null,
    /// Arma's `number` value.
    Number(f64),
    /// Arma's `array` value.
    /// Represented as `[...]`
    Array(Vec<Value>),
    /// Arma's `boolean` value.
    /// Represented as `true` or `false`
    Boolean(bool),
    /// Arma's `string` value.
    /// Represented as `"..."`
    ///
    /// Note: Arma escapes quotes with two double quotes.
    /// This conversation will remove one step of escaping.
    /// Example: `"My name is ""John""."` will become `My name is "John".`
    String(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Number(n) => write!(f, "{}", n),
            Self::Array(a) => write!(
                f,
                "[{}]",
                a.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            Self::Boolean(b) => write!(f, "{}", b),
            Self::String(s) => write!(f, "\"{}\"", s.replace('\"', "\"\"")),
        }
    }
}
