use crate::{IntoArma, Value};

impl IntoArma for serde_json::Value {
    fn to_arma(&self) -> Value {
        match self {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::Bool(b) => Value::Boolean(*b),
            serde_json::Value::Number(n) => Value::Number(if n.is_f64() {
                n.as_f64().unwrap()
            } else if n.is_i64() {
                n.as_i64().unwrap() as f64
            } else if n.is_u64() {
                n.as_u64().unwrap() as f64
            } else {
                unreachable!()
            }),
            serde_json::Value::String(s) => Value::String(s.to_owned()),
            serde_json::Value::Array(v) => {
                Value::Array(v.iter().map(|v| v.to_arma()).collect::<Vec<Value>>())
            }
            serde_json::Value::Object(o) => o
                .iter()
                .map(|(k, v)| vec![Value::String(k.to_owned()), v.to_arma()])
                .collect::<Vec<Vec<Value>>>()
                .to_arma(),
        }
    }
}
