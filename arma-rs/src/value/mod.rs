use std::fmt::Display;

mod features;
mod from_arma;
mod into_arma;
pub mod loadout;

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
            Self::Number(n) => write!(f, "{n}"),
            Self::Array(a) => write!(
                f,
                "[{}]",
                a.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            Self::Boolean(b) => write!(f, "{b}"),
            Self::String(s) => write!(f, "\"{}\"", s.replace('\"', "\"\"")),
        }
    }
}

impl FromArma for Value {
    fn from_arma(s: String) -> Result<Self, String> {
        match s.chars().next().unwrap() {
            'n' => Ok(Self::Null),
            't' | 'f' => Ok(Value::Boolean(<bool>::from_arma(s)?)),
            '0'..='9' | '-' => Ok(Value::Number(<f64>::from_arma(s)?)),
            '[' => Ok(Value::Array(<Vec<Value>>::from_arma(s)?)),
            '"' => Ok(Value::String(<String>::from_arma(s)?)),
            _ => Err(format!("Invalid value: {s}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_display() {
        assert_eq!(Value::Null.to_string(), "null");
        assert_eq!(Value::Number(1.0).to_string(), "1");
        assert_eq!(Value::Number(1.5).to_string(), "1.5");
        assert_eq!(Value::Number(-1.5).to_string(), "-1.5");
        assert_eq!(Value::Boolean(true).to_string(), "true");
        assert_eq!(Value::Boolean(false).to_string(), "false");
        assert_eq!(Value::String("".to_string()).to_string(), "\"\"");
        assert_eq!(Value::String(" ".to_string()).to_string(), "\" \"");
        assert_eq!(Value::String("Hello".to_string()).to_string(), "\"Hello\"");
        assert_eq!(
            Value::String("Hello \"World\"".to_string()).to_string(),
            "\"Hello \"\"World\"\"\""
        );
        assert_eq!(
            Value::Array(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0)
            ])
            .to_string(),
            "[1,2,3]"
        );
        assert_eq!(
            Value::Array(vec![
                Value::String("Hello".to_string()),
                Value::String("World".to_string())
            ])
            .to_string(),
            "[\"Hello\",\"World\"]"
        );
    }
}
