use crate::{FromArma, IntoArma, Value};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
/// A magazine loaded into a weapon
pub struct Magazine(Option<(String, u32)>);
impl Magazine {
    /// Create a new magazine
    #[must_use]
    pub const fn new(class: String, count: u32) -> Self {
        Self(Some((class, count)))
    }

    /// The magazine exists
    #[must_use]
    pub const fn exists(&self) -> bool {
        self.0.is_some()
    }

    /// Arma class name of the magazine
    #[must_use]
    pub fn class(&self) -> Option<&str> {
        self.0.as_ref().map(|(c, _)| c.as_str())
    }

    /// Set the class name of the magazine
    pub fn set_class(&mut self, class: &str) {
        if let Some(magazine) = self.0.as_mut() {
            magazine.0 = class.to_string();
        } else {
            self.0 = Some((class.to_string(), 0));
        }
    }

    /// The remaining ammo in the magazine
    #[must_use]
    pub fn ammo(&self) -> Option<u32> {
        self.0.as_ref().map(|(_, a)| a.to_owned())
    }

    /// Set the remaining ammo in the magazine
    /// Returns true if the ammo was set, false if the magazine was not initialized
    pub fn set_ammo(&mut self, ammo: u32) -> bool {
        if let Some(magazine) = self.0.as_mut() {
            magazine.1 = ammo;
            true
        } else {
            false
        }
    }
}
impl FromArma for Magazine {
    fn from_arma(s: String) -> Result<Self, String> {
        if s == "[]" {
            return Ok(Self(None));
        }
        <(String, u32)>::from_arma(s).map(|(name, count)| Self(Some((name, count))))
    }
}
impl IntoArma for Magazine {
    fn to_arma(&self) -> Value {
        self.0.as_ref().map_or_else(
            || Value::Array(vec![]),
            |magazine| {
                Value::Array(vec![
                    Value::String(magazine.0.clone()),
                    Value::Number(f64::from(magazine.1)),
                ])
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Magazine;
    use crate::{FromArma, IntoArma, Value};

    #[test]
    fn test_magazine() {
        let mut magazine = Magazine::new("ACE_M84".to_string(), 10);
        assert!(magazine.exists());
        assert_eq!(magazine.class(), Some("ACE_M84"));
        assert_eq!(magazine.ammo(), Some(10));
        magazine.set_class("ACE_M84_HEDP");
        assert_eq!(magazine.class(), Some("ACE_M84_HEDP"));
        magazine.set_ammo(20);
        assert_eq!(magazine.ammo(), Some(20));
    }

    #[test]
    fn exists() {
        let magazine = Magazine::default();
        assert!(!magazine.exists());
        let magazine = Magazine::new("ACE_M84".to_string(), 10);
        assert!(magazine.exists());
    }

    #[test]
    fn class() {
        let mut magazine = Magazine::new("ACE_M84".to_string(), 10);
        assert_eq!(magazine.class(), Some("ACE_M84"));
        magazine.set_class("ACE_M84_HEDP");
        assert_eq!(magazine.class(), Some("ACE_M84_HEDP"));
    }

    #[test]
    fn ammo() {
        let mut magazine = Magazine::new("ACE_M84".to_string(), 10);
        assert_eq!(magazine.ammo(), Some(10));
        magazine.set_ammo(20);
        assert_eq!(magazine.ammo(), Some(20));
    }

    #[test]
    fn from_arma() {
        let magazine = Magazine::from_arma("[]".to_string()).unwrap();
        assert!(!magazine.exists());
        let magazine = Magazine::from_arma("[\"ACE_M84\", 10]".to_string()).unwrap();
        assert!(magazine.exists());
        assert_eq!(magazine.class(), Some("ACE_M84"));
        assert_eq!(magazine.ammo(), Some(10));
    }

    #[test]
    fn to_arma() {
        let magazine = Magazine::new("ACE_M84".to_string(), 10);
        assert_eq!(
            magazine.to_arma(),
            Value::Array(vec![
                Value::String("ACE_M84".to_string()),
                Value::Number(10.0),
            ])
        );
    }
}
