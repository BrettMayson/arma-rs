use super::Value;

/// Convert a type to a value that can be sent into Arma
pub trait IntoArma {
    /// Convert a type to a value that can be sent into Arma
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
        Value::String((*self).to_string())
    }
}

impl IntoArma for bool {
    fn to_arma(&self) -> Value {
        Value::Boolean(*self)
    }
}

impl IntoArma for i8 {
    fn to_arma(&self) -> Value {
        Value::Number(f64::from(self.to_owned()))
    }
}

impl IntoArma for i16 {
    fn to_arma(&self) -> Value {
        Value::Number(f64::from(self.to_owned()))
    }
}

impl IntoArma for i32 {
    fn to_arma(&self) -> Value {
        Value::Number(f64::from(*self))
    }
}

impl IntoArma for f32 {
    fn to_arma(&self) -> Value {
        Value::Number(f64::from(*self))
    }
}

impl IntoArma for f64 {
    fn to_arma(&self) -> Value {
        Value::Number(*self)
    }
}

impl IntoArma for u8 {
    fn to_arma(&self) -> Value {
        Value::Number(f64::from(self.to_owned()))
    }
}

impl IntoArma for u16 {
    fn to_arma(&self) -> Value {
        Value::Number(f64::from(self.to_owned()))
    }
}

impl IntoArma for u32 {
    fn to_arma(&self) -> Value {
        Value::Number(f64::from(*self))
    }
}

impl<T: IntoArma> IntoArma for Option<T> {
    fn to_arma(&self) -> Value {
        match self {
            Some(v) => v.to_arma(),
            None => Value::Null,
        }
    }
}

impl Value {
    #[must_use]
    /// Returns an Option representing if the value is null
    pub const fn as_null(&self) -> Option<()> {
        match self {
            Self::Null => Some(()),
            _ => None,
        }
    }

    #[must_use]
    /// Checks if the value is a null variant
    pub const fn is_null(&self) -> bool {
        self.as_null().is_some()
    }

    #[must_use]
    /// Returns an Option representing if the value is a number
    pub const fn as_f64(&self) -> Option<f64> {
        match *self {
            Self::Number(n) => Some(n),
            _ => None,
        }
    }

    #[must_use]
    /// Checks if the value is a number
    pub const fn is_number(&self) -> bool {
        self.as_f64().is_some()
    }

    #[must_use]
    /// Returns an Option representing if the value is an array
    pub const fn as_vec(&self) -> Option<&Vec<Self>> {
        match *self {
            Self::Array(ref vec) => Some(vec),
            _ => None,
        }
    }

    #[must_use]
    /// Checks if the value is an array
    pub const fn is_array(&self) -> bool {
        self.as_vec().is_some()
    }

    #[must_use]
    /// Returns an Option representing if the value is a boolean
    pub const fn as_bool(&self) -> Option<bool> {
        match *self {
            Self::Boolean(bool) => Some(bool),
            _ => None,
        }
    }

    #[must_use]
    /// Checks if the value is a boolean
    pub const fn is_boolean(&self) -> bool {
        self.as_bool().is_some()
    }

    #[must_use]
    /// Returns an Option representing if the value is a string
    pub fn as_str(&self) -> Option<&str> {
        match *self {
            Self::String(ref string) => Some(string),
            _ => None,
        }
    }

    #[must_use]
    /// Checks if the value is a string
    pub fn is_string(&self) -> bool {
        self.as_str().is_some()
    }

    #[must_use]
    /// Checks if the value is empty
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Null => true,
            Self::Number(n) => (*n as f64) == 0.0,
            Self::Array(a) => a.is_empty(),
            Self::Boolean(b) => !*b,
            Self::String(s) => s.is_empty(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::FromArma;
    use super::*;

    #[test]
    fn is_null() {
        assert!(Value::Null.is_null());
        assert!(!Value::Boolean(false).is_null());
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
        match Value::Null.as_null() {
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
    fn as_boo() {
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
        );
    }
    #[test]
    fn parse_vec_tuple() {
        assert_eq!(
            (vec![(String::from("hello"), 123), (String::from("bye"), 321),]),
            <Vec<(String, i32)>>::from_arma(r#"[["hello", 123],["bye", 321]]"#.to_string())
                .unwrap()
        );
    }

    #[test]
    fn to_array() {
        let array = Value::Array(vec![]);

        assert_eq!(array.to_string(), r#"[]"#.to_string());
    }
}
