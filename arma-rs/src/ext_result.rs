use crate::value::{IntoArma, Value};

/// Convert a type to a successfully or failed extension result
pub trait IntoExtResult {
    /// Convert a type to a successfully or failed extension result
    fn to_ext_result(self) -> Result<Value, Value>;
}

impl IntoExtResult for Value {
    fn to_ext_result(self) -> Result<Value, Value> {
        Ok(self)
    }
}

impl<T> IntoExtResult for T
where
    T: IntoArma,
{
    fn to_ext_result(self) -> Result<Value, Value> {
        self.to_arma().to_ext_result()
    }
}

impl IntoExtResult for Result<Value, Value> {
    fn to_ext_result(self) -> Result<Value, Value> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        }
    }
}

impl<T, E> IntoExtResult for Result<T, E>
where
    T: IntoArma,
    E: IntoArma,
{
    fn to_ext_result(self) -> Result<Value, Value> {
        match self {
            Ok(v) => Ok(v.to_arma()),
            Err(e) => Err(e.to_arma()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value() {
        assert_eq!(
            Ok(Value::Boolean(true)),
            Value::Boolean(true).to_ext_result()
        );
    }

    #[test]
    fn option_none() {
        assert_eq!(Ok(Value::Null), None::<&str>.to_ext_result());
    }

    #[test]
    fn option_some() {
        assert_eq!(
            Ok(Value::String("Hello".into())),
            Some("Hello".to_string()).to_ext_result()
        );
    }

    #[test]
    fn ext_result_err() {
        assert_eq!(
            Ok(Value::Number(42.0)),
            Ok(Value::Number(42.0)).to_ext_result()
        );
    }

    #[test]
    fn ext_result_ok() {
        assert_eq!(
            Err(Value::String("Hello".into())),
            Err(Value::String("Hello".into())).to_ext_result()
        );
    }

    #[test]
    fn result_ok() {
        assert_eq!(
            Ok(Value::Number(42.0)),
            Ok::<f64, &str>(42.0).to_ext_result()
        );
    }

    #[test]
    fn result_err() {
        assert_eq!(
            Err(Value::String("Hello".into())),
            Err::<f64, &str>("Hello").to_ext_result()
        );
    }
}
