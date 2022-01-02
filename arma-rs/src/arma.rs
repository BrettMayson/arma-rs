use std::fmt::Display;

use regex::Regex;

/// A trait for converting a value from Arma to a Rust value.
pub trait FromArma: Sized {
    /// Converts a value from Arma to a Rust value.
    /// # Errors
    /// Will return an error if the value cannot be converted.
    fn from_arma(s: String) -> Result<Self, String>;
}

impl FromArma for String {
    fn from_arma(s: String) -> Result<Self, String> {
        Ok(s.trim_start_matches('"').trim_end_matches('"').to_string())
    }
}

macro_rules! impl_from_arma {
    ($($t:ty),*) => {
        $(
            impl FromArma for $t {
                fn from_arma(s: String) -> Result<Self, String> {
                    s.parse::<Self>().map_err(|e| e.to_string())
                }
            }
        )*
    };
}
impl_from_arma!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, bool);

macro_rules! impl_from_arma_tuple {
    ($($t:ident),*) => {
        impl<$($t),*> FromArma for ($($t),*)
        where
            $($t: FromArma),*
        {
            fn from_arma(s: String) -> Result<Self, String> {
                let source = s
                    .strip_prefix('[')
                    .ok_or_else(|| String::from("missing '[' at start of vec"))?
                    .strip_suffix(']')
                    .ok_or_else(|| String::from("missing ']' at end of vec"))?;
                lazy_static::lazy_static! {
                    static ref RE: Regex = Regex::new(r"(?m)(\[.+?\]|[^,]+),?").unwrap();
                }
                let mut iter = RE.captures_iter(source);
                Ok((
                    $(
                        {
                            let n = iter.next().unwrap().get(1).unwrap().as_str().to_string();
                            $t::from_arma(n.trim().to_string())?
                        }
                    ),*
                ))
            }
        }
    };
}

// impl_from_arma_tuple!(A);
impl_from_arma_tuple!(A, B);
impl_from_arma_tuple!(A, B, C);
impl_from_arma_tuple!(A, B, C, D);
impl_from_arma_tuple!(A, B, C, D, E);
impl_from_arma_tuple!(A, B, C, D, E, F);
impl_from_arma_tuple!(A, B, C, D, E, F, G);
impl_from_arma_tuple!(A, B, C, D, E, F, G, H);
impl_from_arma_tuple!(A, B, C, D, E, F, G, H, I);
impl_from_arma_tuple!(A, B, C, D, E, F, G, H, I, J);

impl<T> FromArma for Vec<T>
where
    T: FromArma,
{
    fn from_arma(s: String) -> Result<Self, String> {
        let source = s
            .strip_prefix('[')
            .ok_or_else(|| String::from("missing '[' at start of vec"))?
            .strip_suffix(']')
            .ok_or_else(|| String::from("missing ']' at end of vec"))?;
        lazy_static::lazy_static! {
            static ref RE: Regex = Regex::new(r"(?m)(\[.+?\]|[^,]+),?").unwrap();
        }
        let mut ret = Vec::new();
        let result = RE.captures_iter(source);
        for mat in result {
            ret.push(T::from_arma(mat.get(1).unwrap().as_str().to_string())?);
        }
        Ok(ret)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Nil,
    Number(f64),
    Array(Vec<Value>),
    Boolean(bool),
    String(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "null"),
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
            Self::String(s) => write!(f, "\"{}\"", s.replace("\"", "\"\"")),
        }
    }
}

pub trait IntoArma {
    fn to_arma(&self) -> Value;
}

impl<T> From<T> for Value
where
    T: IntoArma,
{
    fn from(t: T) -> Self {
        t.to_arma()
    }
}

impl IntoArma for Vec<Value> {
    fn to_arma(&self) -> Value {
        Value::Array(self.clone())
    }
}

impl<T> IntoArma for Vec<T>
where
    T: IntoArma,
{
    fn to_arma(&self) -> Value {
        Value::Array(self.iter().map(IntoArma::to_arma).collect())
    }
}

impl<T> IntoArma for &[T]
where
    T: IntoArma,
{
    fn to_arma(&self) -> Value {
        Value::Array(self.iter().map(IntoArma::to_arma).collect())
    }
}

impl IntoArma for String {
    fn to_arma(&self) -> Value {
        Value::String(self.to_string())
    }
}

impl IntoArma for &'static str {
    fn to_arma(&self) -> Value {
        Value::String(self.to_string())
    }
}

impl IntoArma for bool {
    fn to_arma(&self) -> Value {
        Value::Boolean(*self)
    }
}

impl IntoArma for i8 {
    fn to_arma(&self) -> Value {
        Value::Number(self.to_owned() as f64)
    }
}

impl IntoArma for i16 {
    fn to_arma(&self) -> Value {
        Value::Number(self.to_owned() as f64)
    }
}

impl IntoArma for i32 {
    fn to_arma(&self) -> Value {
        Value::Number(*self as f64)
    }
}

impl IntoArma for f32 {
    fn to_arma(&self) -> Value {
        Value::Number(*self as f64)
    }
}

impl IntoArma for f64 {
    fn to_arma(&self) -> Value {
        Value::Number(*self)
    }
}

impl<T: IntoArma> IntoArma for Option<T> {
    fn to_arma(&self) -> Value {
        match self {
            Some(v) => v.to_arma(),
            None => Value::Nil,
        }
    }
}

impl Value {
    #[must_use]
    pub const fn as_null(&self) -> Option<()> {
        match self {
            Self::Nil => Some(()),
            _ => None,
        }
    }

    #[must_use]
    pub const fn is_nil(&self) -> bool {
        self.as_null().is_some()
    }

    #[must_use]
    pub const fn as_f64(&self) -> Option<f64> {
        match *self {
            Self::Number(n) => Some(n),
            _ => None,
        }
    }

    #[must_use]
    pub const fn is_number(&self) -> bool {
        self.as_f64().is_some()
    }

    #[must_use]
    pub const fn as_vec(&self) -> Option<&Vec<Self>> {
        match *self {
            Self::Array(ref vec) => Some(vec),
            _ => None,
        }
    }

    #[must_use]
    pub const fn is_array(&self) -> bool {
        self.as_vec().is_some()
    }

    #[must_use]
    pub const fn as_bool(&self) -> Option<bool> {
        match *self {
            Self::Boolean(bool) => Some(bool),
            _ => None,
        }
    }

    #[must_use]
    pub const fn is_boolean(&self) -> bool {
        self.as_bool().is_some()
    }

    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match *self {
            Self::String(ref string) => Some(string),
            _ => None,
        }
    }

    #[must_use]
    pub fn is_string(&self) -> bool {
        self.as_str().is_some()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Nil => true,
            Self::Number(n) => (*n as f64) == 0.0,
            Self::Array(a) => a.is_empty(),
            Self::Boolean(b) => !*b,
            Self::String(s) => s.is_empty(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_nil() {
        assert!(Value::Nil.is_nil());
        assert!(!Value::Boolean(false).is_nil());
    }

    #[test]
    fn is_number() {
        assert!(Value::Number(54.0).is_number());
        assert!(!Value::Boolean(false).is_number());
    }

    #[test]
    fn is_array() {
        assert!(Value::Array(Vec::new()).is_array());
        assert!(!Value::Boolean(false).is_array());
    }

    #[test]
    fn is_boolean() {
        assert!(Value::Boolean(false).is_boolean());
        assert!(!Value::Number(54.0).is_boolean());
    }

    #[test]
    fn is_string() {
        assert!(Value::String(String::new()).is_string());
        assert!(!Value::Boolean(false).is_string());
    }

    #[test]
    fn as_nil() {
        match Value::Nil.as_null() {
            Some(_) => (),
            None => panic!("Failed to retrieve value"),
        }
    }

    #[test]
    fn as_f32() {
        match Value::Number(54.0).as_f64() {
            Some(f) => assert!((54.0 - f) == 0.0),
            None => panic!("Failed to retrieve value"),
        }
    }

    #[test]
    fn as_vec() {
        match Value::Array(vec![Value::String("hello".into())]).as_vec() {
            Some(v) => {
                let first_value = v.get(0).unwrap();

                assert!(first_value.is_string());
                assert_eq!(first_value.to_string(), String::from("\"hello\""));
            }
            None => panic!("Failed to retrieve value"),
        }
    }

    #[test]
    fn as_bool() {
        match Value::Boolean(true).as_bool() {
            Some(b) => assert!(b),
            None => panic!("Failed to retrieve value"),
        }
    }

    #[test]
    fn as_str() {
        match Value::String(String::from("hello world")).as_str() {
            Some(s) => assert_eq!(s, "hello world"),
            None => panic!("Failed to retrieve value"),
        }
    }

    #[test]
    fn is_empty() {
        assert!(Value::String("".into()).is_empty());
        assert!(Value::Array(vec![]).is_empty());
        assert!(Value::Boolean(false).is_empty());
        assert!(Value::String(String::new()).is_empty());
        assert!(Value::Number(0.0).is_empty());

        assert!(!Value::String("test".into()).is_empty());
        assert!(!Value::Array(vec![Value::Boolean(false)]).is_empty());
        assert!(!Value::Boolean(true).is_empty());
        assert!(!Value::Number(55.0).is_empty());
    }

    #[test]
    fn parse_tuple() {
        assert_eq!(
            (String::from("hello"), 123,),
            <(String, i32)>::from_arma(r#"["hello", 123]"#.to_string()).unwrap()
        )
    }
    #[test]
    fn parse_vec_tuple() {
        assert_eq!(
            (vec![(String::from("hello"), 123), (String::from("bye"), 321),]),
            <Vec<(String, i32)>>::from_arma(r#"[["hello", 123],["bye", 321]]"#.to_string())
                .unwrap()
        )
    }
}
