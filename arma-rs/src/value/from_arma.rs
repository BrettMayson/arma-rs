fn split_array(s: &str) -> Option<Vec<String>> {
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
    parts.push(part.trim().to_string());
    Some(parts)
}

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
impl_from_arma!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, bool, char);

macro_rules! impl_from_arma_tuple {
    ($($t:ident),*) => {
        impl<$($t),*> FromArma for ($($t),*)
        where
            $($t: FromArma),*
        {
            #[allow(unused_assignments)]
            #[allow(clippy::eval_order_dependence)]
            fn from_arma(s: String) -> Result<Self, String> {
                let source = s
                    .strip_prefix('[')
                    .ok_or_else(|| String::from("missing '[' at start of vec"))?
                    .strip_suffix(']')
                    .ok_or_else(|| String::from("missing ']' at end of vec"))?;
                let parts = split_array(&source).ok_or_else(|| "Could not split array".to_string())?;
                let mut parts_iter = parts.iter();
                Ok((
                    $(
                        {
                            let n = parts_iter.next().unwrap().to_string();
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
        let parts = split_array(source).ok_or_else(|| "Could not split array".to_string())?;
        if parts.len() == 1 && parts[0].is_empty() {
            return Ok(Vec::new());
        }
        let parts_iter = parts.iter();
        let mut ret = Self::new();
        for n in parts_iter {
            ret.push(T::from_arma(n.trim().to_string())?);
        }
        Ok(ret)
    }
}

impl<T, const N: usize> FromArma for [T; N]
where
    T: FromArma,
{
    fn from_arma(s: String) -> Result<Self, String> {
        let v: Vec<T> = FromArma::from_arma(s)?;
        let len = v.len();
        v.try_into()
            .map_err(|_| format!("expected {} elements, got {}", N, len))
    }
}

impl<K, V, S> FromArma for std::collections::HashMap<K, V, S>
where
    K: FromArma + Eq + std::hash::Hash,
    V: FromArma,
    S: std::hash::BuildHasher + Default,
{
    fn from_arma(s: String) -> Result<Self, String> {
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

    #[test]
    fn parse_tuple_varying_types() {
        assert_eq!(
            (String::from("hello"), 123,),
            <(String, i32)>::from_arma(r#"["hello", 123]"#.to_string()).unwrap()
        );
        assert!(<(String, i32)>::from_arma(r#"["hello", 123"#.to_string()).is_err());
        assert!(<(String, i32)>::from_arma(r#""hello", 123"#.to_string()).is_err());
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
    fn parse_vec_tuple() {
        assert_eq!(
            (vec![(String::from("hello"), 123), (String::from("bye"), 321),]),
            <Vec<(String, i32)>>::from_arma(r#"[["hello", 123],["bye", 321]]"#.to_string())
                .unwrap()
        );
    }

    #[test]
    fn parse_vec() {
        assert_eq!(
            vec![String::from("hello"), String::from("bye"),],
            <Vec<String>>::from_arma(r#"["hello","bye"]"#.to_string()).unwrap()
        );
        assert!(<Vec<String>>::from_arma(r#""hello","bye"]"#.to_string()).is_err());
        assert!(<Vec<String>>::from_arma(r#"["hello","bye""#.to_string()).is_err());
    }

    #[test]
    fn parse_slice() {
        assert_eq!(
            vec![String::from("hello"), String::from("bye"),],
            <[String; 2]>::from_arma(r#"["hello","bye"]"#.to_string()).unwrap()
        );
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
}
