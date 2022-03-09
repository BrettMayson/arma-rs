use crate::value::{IntoArma, Value};

pub trait IntoExtResult {
    fn to_ext_result(&self) -> Result<Value, Value>;
}

impl IntoExtResult for Value {
    fn to_ext_result(&self) -> Result<Value, Value> {
        Ok(self.to_owned())
    }
}

impl<T> IntoExtResult for T
where
    T: IntoArma,
{
    fn to_ext_result(&self) -> Result<Value, Value> {
        self.to_arma().to_ext_result()
    }
}

impl IntoExtResult for Result<Value, Value> {
    fn to_ext_result(&self) -> Result<Value, Value> {
        match self {
            Ok(v) => Ok(v.to_owned()),
            Err(e) => Err(e.to_owned()),
        }
    }
}

impl<T, E> IntoExtResult for Result<T, E>
where
    T: IntoArma,
    E: IntoArma,
{
    fn to_ext_result(&self) -> Result<Value, Value> {
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
    fn number() {
        assert_eq!(Ok(Value::Number(42.0)), 42.0.to_ext_result());
    }

    #[test]
    fn boolean() {
        assert_eq!(Ok(Value::Boolean(true)), true.to_ext_result());
    }

    #[test]
    fn string() {
        assert_eq!(Ok(Value::String("Hello".into())), "Hello".to_ext_result());
    }

    #[test]
    fn array() {
        assert_eq!(
            Ok(Value::Array(vec![Value::Number(42.0)])),
            vec![Value::Number(42.0)].to_ext_result()
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
