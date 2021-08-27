#[derive(Clone)]
pub enum ArmaValue {
    Number(f32),
    Array(Vec<ArmaValue>),
    Boolean(bool),
    String(String),
}

impl std::fmt::Display for ArmaValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{}", n.to_string()),
            Self::Array(a) => write!(
                f,
                "[{}]",
                a.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            Self::Boolean(b) => write!(f, "{}", b.to_string()),
            Self::String(s) => write!(f, "\"{}\"", s.to_string().replace("\"", "\"\"")),
        }
    }
}

pub trait ToArma {
    fn to_arma(&self) -> ArmaValue;
}

impl ToArma for ArmaValue {
    fn to_arma(&self) -> ArmaValue {
        self.to_owned()
    }
}

impl<T: ToArma> ToArma for Vec<T> {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Array(self.iter().map(|x| x.to_arma()).collect::<Vec<ArmaValue>>())
    }
}

impl ToArma for String {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::String(self.to_string())
    }
}
impl ToArma for &'static str {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::String(self.to_string())
    }
}

impl ToArma for u8 {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Number(self.to_owned() as f32)
    }
}
impl ToArma for u16 {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Number(self.to_owned() as f32)
    }
}
impl ToArma for u32 {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Number(self.to_owned() as f32)
    }
}
impl ToArma for u64 {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Number(self.to_owned() as f32)
    }
}
impl ToArma for u128 {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Number(self.to_owned() as f32)
    }
}

impl ToArma for i8 {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Number(self.to_owned() as f32)
    }
}
impl ToArma for i16 {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Number(self.to_owned() as f32)
    }
}
impl ToArma for i32 {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Number(self.to_owned() as f32)
    }
}
impl ToArma for i64 {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Number(self.to_owned() as f32)
    }
}
impl ToArma for i128 {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Number(self.to_owned() as f32)
    }
}

impl ToArma for f32 {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Number(self.to_owned() as f32)
    }
}
impl ToArma for f64 {
    fn to_arma(&self) -> ArmaValue {
        ArmaValue::Number(self.to_owned() as f32)
    }
}
