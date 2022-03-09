use crate::value::{IntoArma, Value};

pub type ExtResult = Result<Value, Value>;
pub trait IntoExtResult {
    fn to_ext_result(&self) -> ExtResult;
}

impl IntoExtResult for Value {
    fn to_ext_result(&self) -> ExtResult {
        ExtResult::Ok(self.to_owned())
    }
}

impl<T> IntoExtResult for T
where
    T: IntoArma,
{
    fn to_ext_result(&self) -> ExtResult {
        self.to_arma().to_ext_result()
    }
}

impl IntoExtResult for Result<Value, Value> {
    fn to_ext_result(&self) -> ExtResult {
        match self {
            Ok(v) => ExtResult::Ok(v.to_owned()),
            Err(e) => ExtResult::Err(e.to_owned()),
        }
    }
}

impl<T, E> IntoExtResult for Result<T, E>
where
    T: IntoArma,
    E: IntoArma,
{
    fn to_ext_result(&self) -> ExtResult {
        match self {
            Ok(v) => ExtResult::Ok(v.to_arma()),
            Err(e) => ExtResult::Err(e.to_arma()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value() {
        assert_eq!(
            ExtResult::Ok(Value::Boolean(true)),
            Value::Boolean(true).to_ext_result()
        );
    }

    #[test]
    fn option_none() {
        assert_eq!(ExtResult::Ok(Value::Null), None::<&str>.to_ext_result());
    }

    #[test]
    fn option_some() {
        assert_eq!(
            ExtResult::Ok(Value::String("Hello".into())),
            Some("Hello".to_string()).to_ext_result()
        );
    }

    #[test]
    fn number() {
        assert_eq!(ExtResult::Ok(Value::Number(42.0)), 42.0.to_ext_result());
    }

    #[test]
    fn boolean() {
        assert_eq!(ExtResult::Ok(Value::Boolean(true)), true.to_ext_result());
    }

    #[test]
    fn string() {
        assert_eq!(
            ExtResult::Ok(Value::String("Hello".into())),
            "Hello".to_ext_result()
        );
    }

    #[test]
    fn array() {
        assert_eq!(
            ExtResult::Ok(Value::Array(vec![Value::Number(42.0)])),
            vec![Value::Number(42.0)].to_ext_result()
        );
    }

    #[test]
    fn ext_result_err() {
        assert_eq!(
            ExtResult::Ok(Value::Number(42.0)),
            Ok(Value::Number(42.0)).to_ext_result()
        );
    }

    #[test]
    fn ext_result_ok() {
        assert_eq!(
            ExtResult::Err(Value::String("Hello".into())),
            Err(Value::String("Hello".into())).to_ext_result()
        );
    }

    #[test]
    fn result_ok() {
        assert_eq!(
            ExtResult::Ok(Value::Number(42.0)),
            Ok::<f64, &str>(42.0).to_ext_result()
        );
    }

    #[test]
    fn result_err() {
        assert_eq!(
            ExtResult::Err(Value::String("Hello".into())),
            Err::<f64, &str>("Hello").to_ext_result()
        );
    }
}
