fn split_array(s: &str) -> Vec<String> {
    let mut nest = 0;
    let mut parts = Vec::new();
    let mut part = String::new();
    for c in s.chars() {
        if c == '[' {
            part.push(c);
            nest += 1;
        } else if c == ']' {
            nest -= 1;
            part.push(c);
        } else if c == ',' && nest == 0 {
            parts.push(part.trim().to_string());
            part = String::new();
        } else {
            part.push(c);
        }
    }
    let part = part.trim().to_string();
    if !part.is_empty() {
        parts.push(part);
    }
    parts
}

/// Error type for [`FromArma`]
#[derive(Debug)]
pub enum FromArmaError {
    /// Invalid [`crate::Value`]
    ValueInvalid(String),

    /// Invalid primitive value
    PrimitiveParseError(String),
    /// Missing base in exponential notation
    NumberMissingBase,
    /// Missing exponent in exponential notation
    NumberMissingExponent,

    /// Missing array/tuple bracket
    ArrayMissingBracket(bool),
    /// Missing field in map
    MapMissingField(String),
    /// Unknown field in map
    MapUnknownField(String),
    /// Collection size mismatch
    SizeMismatch {
        /// Expected size
        expected: usize,
        /// Actual size
        actual: usize,
    },

    /// Custom error message
    Custom(String),
}

impl std::fmt::Display for FromArmaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ValueInvalid(s) => write!(f, "invalid value: {s}"),
            Self::PrimitiveParseError(s) => write!(f, "error parsing primitive: {s}"),
            Self::NumberMissingBase => write!(f, "missing base in exponential notation"),
            Self::NumberMissingExponent => write!(f, "missing exponent in exponential notation"),
            Self::ArrayMissingBracket(start) => match *start {
                true => write!(f, "missing '[' at start of array"),
                false => write!(f, "missing ']' at end of array"),
            },
            Self::SizeMismatch { expected, actual } => {
                write!(f, "expected {expected} elements, got {actual}")
            }
            Self::MapMissingField(s) => write!(f, "missing field: {s}"),
            Self::MapUnknownField(s) => write!(f, "unknown field: {s}"),
            Self::Custom(s) => f.write_str(s),
        }
    }
}

impl FromArmaError {
    /// Creates a new [`FromArmaError::Custom`]
    pub fn custom(s: impl AsRef<str>) -> Self {
        Self::Custom(s.as_ref().to_string())
    }
}

/// A trait for converting a value from Arma to a Rust value.
pub trait FromArma: Sized {
    /// Converts a value from Arma to a Rust value.
    /// # Errors
    /// Will return an error if the value cannot be converted.
    fn from_arma(s: String) -> Result<Self, FromArmaError>;
}

impl FromArma for String {
    fn from_arma(s: String) -> Result<Self, FromArmaError> {
        Ok(s.trim_start_matches('"').trim_end_matches('"').to_string())
    }
}

macro_rules! impl_from_arma {
    ($($t:ty),*) => {
        $(
            impl FromArma for $t {
                fn from_arma(s: String) -> Result<Self, FromArmaError> {
                    s.parse::<Self>().map_err(|e| FromArmaError::PrimitiveParseError(e.to_string()))
                }
            }
        )*
    };
}
impl_from_arma!(f32, f64, bool, char);

macro_rules! impl_from_arma_number {
    ($($t:ty),*) => {
        $(
            impl FromArma for $t {
                fn from_arma(s: String) -> Result<Self, FromArmaError> {
                    fn string_to_option(s: &str) -> Option<&str> {
                        match s.is_empty() {
                            true => None,
                            _ => Some(s),
                        }
                    }

                    if s.contains("e") {
                        // parse exponential notation
                        let mut parts = s.split('e');
                        let base = match string_to_option(parts.next().unwrap()) {
                            Some(s) => s.parse::<f64>().map_err(|e| FromArmaError::PrimitiveParseError(e.to_string()))?,
                            None => return Err(FromArmaError::NumberMissingBase),
                        };
                        let exp = match string_to_option(parts.next().unwrap()) {
                            Some(s) => s.parse::<i32>().map_err(|e| FromArmaError::PrimitiveParseError(e.to_string()))?,
                            None => return Err(FromArmaError::NumberMissingExponent),
                        };
                        return Ok((base * 10.0_f64.powi(exp)) as $t);
                    }
                    s.parse::<Self>().map_err(|e| FromArmaError::PrimitiveParseError(e.to_string()))
                }
            }
        )*
    };
}
impl_from_arma_number!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

macro_rules! impl_from_arma_tuple {
    { $c: expr, $($t:ident)* } => {
        impl<$($t),*> FromArma for ($($t),*)
        where
            $($t: FromArma),*
        {
            fn from_arma(s: String) -> Result<Self, FromArmaError> {
                let source = s
                    .strip_prefix('[')
                    .ok_or(FromArmaError::ArrayMissingBracket(true))?
                    .strip_suffix(']')
                    .ok_or(FromArmaError::ArrayMissingBracket(false))?;
                let parts = split_array(source);
                let len = parts.len();
                if len != $c {
                    return Err(FromArmaError::SizeMismatch {
                        expected: $c,
                        actual: len,
                    });
                }
                let mut parts_iter = parts.iter();
                Ok(($(
                    $t::from_arma(parts_iter.next().unwrap().to_string())?
                ),*))
            }
        }
    };
}

impl_from_arma_tuple! { 2, A B }
impl_from_arma_tuple! { 3, A B C }
impl_from_arma_tuple! { 4, A B C D }
impl_from_arma_tuple! { 5, A B C D E }
impl_from_arma_tuple! { 6, A B C D E F }
impl_from_arma_tuple! { 7, A B C D E F G }
impl_from_arma_tuple! { 8, A B C D E F G H }
impl_from_arma_tuple! { 9, A B C D E F G H I }
impl_from_arma_tuple! { 10, A B C D E F G H I J }
impl_from_arma_tuple! { 11, A B C D E F G H I J K }
impl_from_arma_tuple! { 12, A B C D E F G H I J K L }
impl_from_arma_tuple! { 13, A B C D E F G H I J K L M }
impl_from_arma_tuple! { 14, A B C D E F G H I J K L M N }
impl_from_arma_tuple! { 15, A B C D E F G H I J K L M N O }
impl_from_arma_tuple! { 16, A B C D E F G H I J K L M N O P }
impl_from_arma_tuple! { 17, A B C D E F G H I J K L M N O P Q }
impl_from_arma_tuple! { 18, A B C D E F G H I J K L M N O P Q R }
impl_from_arma_tuple! { 19, A B C D E F G H I J K L M N O P Q R S }
impl_from_arma_tuple! { 20, A B C D E F G H I J K L M N O P Q R S T }
impl_from_arma_tuple! { 21, A B C D E F G H I J K L M N O P Q R S T U }
impl_from_arma_tuple! { 22, A B C D E F G H I J K L M N O P Q R S T U V }
impl_from_arma_tuple! { 23, A B C D E F G H I J K L M N O P Q R S T U V W }
impl_from_arma_tuple! { 24, A B C D E F G H I J K L M N O P Q R S T U V W X }
impl_from_arma_tuple! { 25, A B C D E F G H I J K L M N O P Q R S T U V W X Y }
impl_from_arma_tuple! { 26, A B C D E F G H I J K L M N O P Q R S T U V W X Y Z }

impl<T> FromArma for Vec<T>
where
    T: FromArma,
{
    fn from_arma(s: String) -> Result<Self, FromArmaError> {
        let source = s
            .strip_prefix('[')
            .ok_or(FromArmaError::ArrayMissingBracket(true))?
            .strip_suffix(']')
            .ok_or(FromArmaError::ArrayMissingBracket(false))?;
        let parts = split_array(source);
        parts.iter().try_fold(Self::new(), |mut acc, p| {
            acc.push(T::from_arma(p.to_string())?);
            Ok(acc)
        })
    }
}

impl<T, const N: usize> FromArma for [T; N]
where
    T: FromArma,
{
    fn from_arma(s: String) -> Result<Self, FromArmaError> {
        let v: Vec<T> = FromArma::from_arma(s)?;
        let len = v.len();
        v.try_into().map_err(|_| FromArmaError::SizeMismatch {
            expected: N,
            actual: len,
        })
    }
}

impl<K, V, S> FromArma for std::collections::HashMap<K, V, S>
where
    K: FromArma + Eq + std::hash::Hash,
    V: FromArma,
    S: std::hash::BuildHasher + Default,
{
    fn from_arma(s: String) -> Result<Self, FromArmaError> {
        let data: Vec<(K, V)> = FromArma::from_arma(s)?;
        let mut ret = Self::default();
        for (k, v) in data {
            ret.insert(k, v);
        }
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Value;

    #[test]
    fn parse_tuple_varying_types() {
        assert_eq!(
            (String::from("hello"), 123),
            <(String, i32)>::from_arma(r#"["hello", 123]"#.to_string()).unwrap()
        );
        assert_eq!(
            (String::from("hello"), String::from("world")),
            <(String, String)>::from_arma(r#"[hello, "world"]"#.to_string()).unwrap()
        );
    }

    #[test]
    fn parse_tuple_size_errors() {
        assert!(matches!(
            <(String, i32)>::from_arma(r#"[]"#.to_string()),
            Err(FromArmaError::SizeMismatch {
                expected: 2,
                actual: 0
            })
        ));
        assert!(matches!(
            <(String, i32)>::from_arma(r#"["hello"]"#.to_string()),
            Err(FromArmaError::SizeMismatch {
                expected: 2,
                actual: 1
            })
        ));
        assert!(matches!(
            <(String, i32)>::from_arma(r#"["hello", 123, 456]"#.to_string()),
            Err(FromArmaError::SizeMismatch {
                expected: 2,
                actual: 3
            })
        ));
    }

    #[test]
    fn parse_tuple_bracket_errors() {
        assert!(matches!(
            <(String, i32)>::from_arma(r#"["hello", 123"#.to_string()),
            Err(FromArmaError::ArrayMissingBracket(false))
        ));
        assert!(matches!(
            <(String, i32)>::from_arma(r#""hello", 123"#.to_string()),
            Err(FromArmaError::ArrayMissingBracket(true))
        ));
    }

    #[test]
    fn test_tuple_2() {
        assert_eq!(
            (0, 1),
            <(u8, u8)>::from_arma(r#"[0, 1]"#.to_string()).unwrap()
        );
    }

    #[test]
    fn test_tuple_3() {
        assert_eq!(
            (0, 1, 2),
            <(u8, u8, u8)>::from_arma(r#"[0, 1, 2]"#.to_string()).unwrap()
        );
    }

    #[test]
    fn test_tuple_4() {
        assert_eq!(
            (0, 1, 2, 3),
            <(u8, u8, u8, u8)>::from_arma(r#"[0, 1, 2, 3]"#.to_string()).unwrap()
        );
    }

    #[test]
    fn test_tuple_5() {
        assert_eq!(
            (0, 1, 2, 3, 4),
            <(u8, u8, u8, u8, u8)>::from_arma(r#"[0, 1, 2, 3, 4]"#.to_string()).unwrap()
        );
    }

    #[test]
    fn test_tuple_6() {
        assert_eq!(
            (0, 1, 2, 3, 4, 5),
            <(u8, u8, u8, u8, u8, u8)>::from_arma(r#"[0, 1, 2, 3, 4, 5]"#.to_string()).unwrap()
        );
    }

    #[test]
    fn test_tuple_7() {
        assert_eq!(
            (0, 1, 2, 3, 4, 5, 6),
            <(u8, u8, u8, u8, u8, u8, u8)>::from_arma(r#"[0, 1, 2, 3, 4, 5, 6]"#.to_string())
                .unwrap()
        );
    }

    #[test]
    fn test_tuple_8() {
        assert_eq!(
            (0, 1, 2, 3, 4, 5, 6, 7),
            <(u8, u8, u8, u8, u8, u8, u8, u8)>::from_arma(
                r#"[0, 1, 2, 3, 4, 5, 6, 7]"#.to_string()
            )
            .unwrap()
        );
    }

    #[test]
    fn test_tuple_9() {
        assert_eq!(
            (0, 1, 2, 3, 4, 5, 6, 7, 8),
            <(u8, u8, u8, u8, u8, u8, u8, u8, u8)>::from_arma(
                r#"[0, 1, 2, 3, 4, 5, 6, 7, 8]"#.to_string()
            )
            .unwrap()
        );
    }

    #[test]
    fn test_tuple_10() {
        assert_eq!(
            (0, 1, 2, 3, 4, 5, 6, 7, 8, 9),
            <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)>::from_arma(
                r#"[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]"#.to_string()
            )
            .unwrap()
        );
    }

    #[test]
    fn parse_string() {
        assert_eq!(
            String::from("hello"),
            <String>::from_arma("hello".to_string()).unwrap()
        );
        assert_eq!(
            String::from("hello"),
            <String>::from_arma(r#""hello""#.to_string()).unwrap()
        );
    }

    #[test]
    fn parse_vec() {
        assert_eq!(
            vec![String::from("hello"), String::from("bye"),],
            <Vec<String>>::from_arma(r#"["hello","bye"]"#.to_string()).unwrap()
        );
        assert_eq!(
            vec![String::from("hello"), String::from("world")],
            <Vec<String>>::from_arma(r#"[hello, "world"]"#.to_string()).unwrap()
        );
    }

    #[test]
    fn parse_vec_bracket_errors() {
        assert!(matches!(
            <Vec<String>>::from_arma(r#"["hello","bye""#.to_string()),
            Err(FromArmaError::ArrayMissingBracket(false))
        ));
        assert!(matches!(
            <Vec<String>>::from_arma(r#""hello","bye"]"#.to_string()),
            Err(FromArmaError::ArrayMissingBracket(true))
        ));
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
    fn parse_slice() {
        assert_eq!(
            vec![String::from("hello"), String::from("bye"),],
            <[String; 2]>::from_arma(r#"["hello","bye"]"#.to_string()).unwrap()
        );
    }

    #[test]
    fn parse_slice_size_errors() {
        assert!(matches!(
            <[String; 2]>::from_arma(r#"[]"#.to_string()),
            Err(FromArmaError::SizeMismatch {
                expected: 2,
                actual: 0
            })
        ));
        assert!(matches!(
            <[String; 2]>::from_arma(r#"["hello"]"#.to_string()),
            Err(FromArmaError::SizeMismatch {
                expected: 2,
                actual: 1
            })
        ));
        assert!(matches!(
            <[String; 2]>::from_arma(r#"["hello","bye","world"]"#.to_string()),
            Err(FromArmaError::SizeMismatch {
                expected: 2,
                actual: 3
            })
        ));
    }

    #[test]
    fn parse_hashmap() {
        assert_eq!(
            std::collections::HashMap::from_iter(
                vec![(String::from("hello"), 123), (String::from("bye"), 321),].into_iter()
            ),
            <std::collections::HashMap<String, i32>>::from_arma(
                r#"[["hello", 123],["bye",321]]"#.to_string()
            )
            .unwrap()
        );
    }

    #[test]
    fn parse_exponential() {
        assert_eq!(1.0e-10, <f64>::from_arma(r#"1.0e-10"#.to_string()).unwrap());
        assert_eq!(
            1_227_700,
            <u32>::from_arma(r#"1.2277e+006"#.to_string()).unwrap()
        );
    }

    #[test]
    fn parse_exponential_errors() {
        assert!(matches!(
            <u32>::from_arma(r#"e-10"#.to_string()),
            Err(FromArmaError::NumberMissingBase)
        ));
        assert!(matches!(
            <u32>::from_arma(r#"1.0e"#.to_string()),
            Err(FromArmaError::NumberMissingExponent)
        ));
    }

    #[test]
    fn parse_value_tuple() {
        assert_eq!(
            (
                Value::String(String::from("hello")),
                Value::String(String::from("world"))
            ),
            <(Value, Value)>::from_arma(r#"["hello", "world"]"#.to_string()).unwrap()
        );
    }

    #[test]
    fn parse_value_vec() {
        assert_eq!(
            vec![
                Value::String(String::from("hello")),
                Value::String(String::from("world"))
            ],
            <Vec<Value>>::from_arma(r#"["hello", "world"]"#.to_string()).unwrap()
        );
    }
}
