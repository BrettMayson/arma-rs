use crate::{FromArma, FromArmaError, IntoArma, Value};

impl IntoArma for uuid::Uuid {
    fn to_arma(&self) -> Value {
        self.to_string().to_arma()
    }
}

impl FromArma for uuid::Uuid {
    fn from_arma(s: String) -> Result<Self, FromArmaError> {
        let s = s
            .strip_prefix('"')
            .and_then(|s| s.strip_suffix('"'))
            .unwrap_or(&s);
        uuid::Uuid::parse_str(s).map_err(FromArmaError::custom)
    }
}
