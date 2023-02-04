use crate::{FromArma, IntoArma, Value};

impl IntoArma for uuid::Uuid {
    fn to_arma(&self) -> Value {
        self.to_string().to_arma()
    }
}

impl FromArma for uuid::Uuid {
    fn from_arma(s: String) -> Result<Self, String> {
        uuid::Uuid::parse_str(&s).map_err(|e| e.to_string())
    }
}
