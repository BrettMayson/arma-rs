use std::{fmt::Display, str::FromStr};

pub enum ArmaValue {
    Number(f32),
    Array(Vec<ArmaValue>),
    Boolean(bool),
    String(String),
}

impl Display for ArmaValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{}", n),
            Self::Array(a) => write!(
                f,
                "[{}]",
                a.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            Self::Boolean(b) => write!(f, "{}", b),
            Self::String(s) => write!(f, "\"{}\"", s.replace("\"", "\"\"")),
        }
    }
}

impl<T> From<T> for ArmaValue
where
    T: IntoArma,
{
    fn from(t: T) -> Self {
        t.to_arma()
    }
}

pub trait FromArma: Sized {
    fn from_arma(s: String) -> Result<Self, String>;
}

impl<T> FromArma for T
where
    T: FromStr,
    <T as FromStr>::Err: ToString,
{
    fn from_arma(s: String) -> Result<Self, String> {
        s.parse::<Self>().map_err(|e| e.to_string())
    }
}

pub trait IntoArma {
    fn to_arma(&self) -> ArmaValue;
}

impl<T> IntoArma for Vec<T>
where
    T: IntoArma,
{
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Array(self.iter().map(|x| x.to_arma()).collect())
    }
}

impl<T> IntoArma for &[T]
where
    T: IntoArma,
{
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Array(self.iter().map(|x| x.to_arma()).collect())
    }
}

impl IntoArma for String {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::String(self.to_string())
    }
}

impl IntoArma for &'static str {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::String(self.to_string())
    }
}

impl IntoArma for bool {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Boolean(*self)
    }
}

impl IntoArma for i8 {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Number(self.to_owned() as f32)
    }
}

impl IntoArma for i16 {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Number(self.to_owned() as f32)
    }
}

impl IntoArma for i32 {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Number(*self as f32)
    }
}

impl IntoArma for f32 {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Number(*self)
    }
}
