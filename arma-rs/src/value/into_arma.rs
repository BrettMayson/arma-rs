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

#[cfg(test)]
#[test]
fn test_vec() {
    assert_eq!(String::from("[1,2,3]"), vec![1, 2, 3].to_arma().to_string())
}

impl<T> IntoArma for &[T]
where
    T: IntoArma,
{
    fn to_arma(&self) -> Value {
        Value::Array(self.iter().map(IntoArma::to_arma).collect())
    }
}

#[cfg(test)]
#[test]
fn test_slice() {
    assert_eq!(
        String::from("[1,2,3]"),
        vec![1, 2, 3].as_slice().to_arma().to_string()
    )
}

impl IntoArma for String {
    fn to_arma(&self) -> Value {
        Value::String(self.to_string())
    }
}

#[cfg(test)]
#[test]
fn test_string() {
    assert_eq!(
        String::from("\"hello\""),
        String::from("hello").to_arma().to_string()
    )
}

impl IntoArma for &'static str {
    fn to_arma(&self) -> Value {
        Value::String((*self).to_string())
    }
}

#[cfg(test)]
#[test]
fn test_static_str() {
    assert_eq!(String::from("\"hello\""), "hello".to_arma().to_string())
}

impl IntoArma for bool {
    fn to_arma(&self) -> Value {
        Value::Boolean(*self)
    }
}

#[cfg(test)]
#[test]
fn test_bool() {
    assert_eq!(String::from("true"), true.to_arma().to_string())
}

impl IntoArma for i8 {
    fn to_arma(&self) -> Value {
        Value::Number(f64::from(self.to_owned()))
    }
}

#[cfg(test)]
#[test]
fn test_i8() {
    assert_eq!(String::from("1"), 1i8.to_arma().to_string())
}

impl IntoArma for i16 {
    fn to_arma(&self) -> Value {
        Value::Number(f64::from(self.to_owned()))
    }
}

#[cfg(test)]
#[test]
fn test_i16() {
    assert_eq!(String::from("1"), 1i16.to_arma().to_string())
}

impl IntoArma for i32 {
    fn to_arma(&self) -> Value {
        Value::Number(f64::from(*self))
    }
}

#[cfg(test)]
#[test]
fn test_i32() {
    assert_eq!(String::from("1"), 1i32.to_arma().to_string())
}

impl IntoArma for f32 {
    fn to_arma(&self) -> Value {
        Value::Number(f64::from(*self))
    }
}

#[cfg(test)]
#[test]
fn test_f32() {
    assert_eq!(String::from("1"), 1f32.to_arma().to_string());
    assert_eq!(String::from("1.5"), 1.5f32.to_arma().to_string());
}

impl IntoArma for f64 {
    fn to_arma(&self) -> Value {
        Value::Number(*self)
    }
}

#[cfg(test)]
#[test]
fn test_f64() {
    assert_eq!(String::from("1"), 1f64.to_arma().to_string());
    assert_eq!(String::from("1.5"), 1.5f64.to_arma().to_string());
}

impl IntoArma for u8 {
    fn to_arma(&self) -> Value {
        Value::Number(f64::from(self.to_owned()))
    }
}

#[cfg(test)]
#[test]
fn test_u8() {
    assert_eq!(String::from("1"), 1u8.to_arma().to_string())
}

impl IntoArma for u16 {
    fn to_arma(&self) -> Value {
        Value::Number(f64::from(self.to_owned()))
    }
}

#[cfg(test)]
#[test]
fn test_u16() {
    assert_eq!(String::from("1"), 1u16.to_arma().to_string())
}

impl IntoArma for u32 {
    fn to_arma(&self) -> Value {
        Value::Number(f64::from(*self))
    }
}

#[cfg(test)]
#[test]
fn test_u32() {
    assert_eq!(String::from("1"), 1u32.to_arma().to_string())
}

impl<T: IntoArma> IntoArma for Option<T> {
    fn to_arma(&self) -> Value {
        match self {
            Some(v) => v.to_arma(),
            None => Value::Null,
        }
    }
}

#[cfg(test)]
#[test]
fn test_option() {
    assert_eq!(String::from("null"), None::<i32>.to_arma().to_string());
    assert_eq!(String::from("1"), Some(1).to_arma().to_string());
}

impl<K: IntoArma, V: IntoArma, S: std::hash::BuildHasher> IntoArma
    for std::collections::HashMap<K, V, S>
{
    fn to_arma(&self) -> Value {
        self.iter()
            .map(|(k, v)| vec![k.to_arma(), v.to_arma()])
            .collect::<Vec<Vec<Value>>>()
            .to_arma()
    }
}

#[cfg(test)]
#[test]
fn test_hashmap() {
    use std::collections::HashMap;
    {
        let mut map = HashMap::new();
        map.insert("key".to_string(), "value".to_string());
        let map = map.to_arma();
        assert_eq!(map.to_string(), r#"[["key","value"]]"#.to_string());
    }
    {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), "value1".to_string());
        map.insert("key2".to_string(), "value2".to_string());
        let map = map.to_arma().to_string();
        assert!(
            map == r#"[["key1","value1"],["key2","value2"]]"#
                || map == r#"[["key2","value2"],["key1","value1"]]"#
        )
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
        assert!(Value::Null.as_null().is_some())
    }

    #[test]
    fn as_f32() {
        let v = Value::Number(54.0).as_f64().unwrap();
        assert!((54.0 - v) == 0.0)
    }

    #[test]
    fn as_vec() {
        let arr = Value::Array(vec![Value::String("hello".into())]);
        let v = arr.as_vec().unwrap();
        let first_value = v.get(0).unwrap();
        assert!(first_value.is_string());
        assert_eq!(first_value.to_string(), String::from("\"hello\""));
    }

    #[test]
    fn as_bool() {
        assert!(Value::Boolean(true).as_bool().unwrap());
    }

    #[test]
    fn as_str() {
        let v = Value::String("hello world".into());
        let s = v.as_str().unwrap();
        assert_eq!(s, "hello world");
    }

    #[test]
    fn is_empty() {
        assert!(Value::String("".into()).is_empty());
        assert!(Value::Array(vec![]).is_empty());
        assert!(Value::Boolean(false).is_empty());
        assert!(Value::String(String::new()).is_empty());
        assert!(Value::Number(0.0).is_empty());
        assert!(Value::Null.is_empty());

        assert!(!Value::String("test".into()).is_empty());
        assert!(!Value::Array(vec![Value::Boolean(false)]).is_empty());
        assert!(!Value::Boolean(true).is_empty());
        assert!(!Value::Number(55.0).is_empty());
    }

    #[test]
    fn to_array() {
        let array = Value::Array(vec![]);
        assert_eq!(array.to_string(), r#"[]"#.to_string());
    }
}
