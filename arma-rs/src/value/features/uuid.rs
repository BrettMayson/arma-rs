use crate::{IntoArma, Value};

impl IntoArma for uuid::Uuid {
    fn to_arma(&self) -> Value {
        self.to_string().to_arma()
    }
}
