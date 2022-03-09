use crate::value::{IntoArma, Value};

pub type ExtResult = Result<Value, Value>;
pub trait IntoExtResult {
    fn to_ext_result(&self) -> ExtResult;
}

impl IntoExtResult for Value {
    fn to_ext_result(&self) -> ExtResult {
        Ok(self.to_owned())
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
    fn to_ext_result(&self) -> ExtResult {
        match self {
            Ok(v) => Ok(v.to_arma()),
            Err(e) => Err(e.to_arma()),
        }
    }
}
