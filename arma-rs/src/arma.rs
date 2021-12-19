use std::{fmt::Display, str::FromStr};

pub enum ArmaValue {
    Nil,
    Number(f32),
    Array(Vec<ArmaValue>),
    Boolean(bool),
    String(String),
}

impl Display for ArmaValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "null"),
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

impl<T: IntoArma> IntoArma for Option<T> {
    fn to_arma(&self) -> ArmaValue {
        match self {
            Some(v) => v.to_arma(),
            None => ArmaValue::Nil
        }
    }
}

impl ArmaValue {
    pub fn as_null(&self) -> Option<()> {
        match self {
            ArmaValue::Nil => Some(()),
            _ => None,
        }
    }

    pub fn is_nil(&self) -> bool {
        self.as_null().is_some()
    }

    pub fn as_f32(&self) -> Option<f32> {
        match *self {
            ArmaValue::Number(n) => Some(n),
            _ => None,
        }
    }

    pub fn is_number(&self) -> bool {
        self.as_f32().is_some()
    }

    pub fn as_vec(&self) -> Option<&Vec<ArmaValue>> {
        match *self {
            ArmaValue::Array(ref vec) => Some(vec),
            _ => None,
        }
    }

    pub fn is_array(&self) -> bool {
        self.as_vec().is_some()
    }

    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            ArmaValue::Boolean(bool) => Some(bool),
            _ => None,
        }
    }

    pub fn is_boolean(&self) -> bool {
        self.as_bool().is_some()
    }

    pub fn as_str(&self) -> Option<&str> {
        match *self {
            ArmaValue::String(ref string) => Some(string),
            _ => None,
        }
    }

    pub fn is_string(&self) -> bool {
        self.as_str().is_some()
    }

    pub fn is_empty(&self) -> bool {
        match self {
            ArmaValue::Nil => true,
            ArmaValue::Number(n) => (*n as f64) == 0.0,
            ArmaValue::Array(a) => a.is_empty(),
            ArmaValue::Boolean(b) => !*b,
            ArmaValue::String(s) => s.is_empty(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_nil() {
        assert!(ArmaValue::Nil.is_nil());
        assert!(!ArmaValue::Boolean(false).is_nil());
    }

    #[test]
    fn test_is_number() {
        assert!(ArmaValue::Number(54.0).is_number());
        assert!(!ArmaValue::Boolean(false).is_number());
    }

    #[test]
    fn test_is_array() {
        assert!(ArmaValue::Array(Vec::new()).is_array());
        assert!(!ArmaValue::Boolean(false).is_array());
    }

    #[test]
    fn test_is_boolean() {
        assert!(ArmaValue::Boolean(false).is_boolean());
        assert!(!ArmaValue::Number(54.0).is_boolean());
    }

    #[test]
    fn test_is_string() {
        assert!(ArmaValue::String(String::new()).is_string());
        assert!(!ArmaValue::Boolean(false).is_string());
    }

    #[test]
    fn test_as_nil() {
        match ArmaValue::Nil.as_null() {
            Some(_) => (),
            None => panic!("Failed to retrieve value")
        }
    }

    #[test]
    fn test_as_f32() {
        match ArmaValue::Number(54.0).as_f32() {
            Some(f) => assert!((54.0 - f) == 0.0),
            None => panic!("Failed to retrieve value")
        }
    }

    #[test]
    fn test_as_vec() {
        match ArmaValue::Array(vec![ArmaValue::String("hello".into())]).as_vec() {
            Some(v) => {
                let first_value = v.get(0).unwrap();
                
                assert!(first_value.is_string());
                assert_eq!(first_value.to_string(), String::from("\"hello\""));
            },
            None => panic!("Failed to retrieve value")
        }
    }

    #[test]
    fn test_as_bool() {
        match ArmaValue::Boolean(true).as_bool() {
            Some(b) => assert!(b),
            None => panic!("Failed to retrieve value")
        }
    }

    #[test]
    fn test_as_str() {
        match ArmaValue::String(String::from("hello world")).as_str() {
            Some(s) => assert_eq!(s, "hello world"),
            None => panic!("Failed to retrieve value")
        }
    }

    #[test]
    fn test_is_empty() {
        assert!(ArmaValue::String("".into()).is_empty());
        assert!(ArmaValue::Array(vec![]).is_empty());
        assert!(ArmaValue::Boolean(false).is_empty());
        assert!(ArmaValue::String(String::new()).is_empty());
        assert!(ArmaValue::Number(0.0).is_empty());

        assert!(!ArmaValue::String("test".into()).is_empty());
        assert!(!ArmaValue::Array(vec![ArmaValue::Boolean(false)]).is_empty());
        assert!(!ArmaValue::Boolean(true).is_empty());
        assert!(!ArmaValue::Number(55.0).is_empty());
    }
}
