use crate::{FromArma, FromArmaError, IntoArma, Value};

#[derive(Debug, Clone, PartialEq, Eq)]
/// An item stored in a uniform, vest, or backpack
pub enum InventoryItem {
    /// An item that is not a magazine
    Item(String, u32),
    /// A magazine
    Magazine(String, u32, u32),
}
impl InventoryItem {
    /// Create a new item
    #[must_use]
    pub const fn new_item(class: String, count: u32) -> Self {
        Self::Item(class, count)
    }

    /// Create a new magazine
    #[must_use]
    pub const fn new_magazine(class: String, count: u32, ammo: u32) -> Self {
        Self::Magazine(class, count, ammo)
    }

    /// The item is a magazine
    #[must_use]
    pub const fn is_magazine(&self) -> bool {
        matches!(self, Self::Magazine(_, _, _))
    }

    /// The class name of the item
    #[must_use]
    pub fn class(&self) -> &str {
        match self {
            Self::Item(c, _) | Self::Magazine(c, _, _) => c.as_str(),
        }
    }

    /// Set the class name of the item
    pub fn set_class(&mut self, class: String) {
        match self {
            Self::Item(c, _) | Self::Magazine(c, _, _) => *c = class,
        }
    }

    /// The amount of the item
    #[must_use]
    pub fn count(&self) -> u32 {
        match self {
            Self::Item(_, c) | Self::Magazine(_, c, _) => c.to_owned(),
        }
    }

    /// Set the amount of the item
    pub fn set_count(&mut self, count: u32) {
        match self {
            Self::Item(_, c) | Self::Magazine(_, c, _) => *c = count,
        }
    }

    /// The amount of ammo in the magazine
    #[must_use]
    pub fn ammo(&self) -> Option<u32> {
        match self {
            Self::Item(..) => None,
            Self::Magazine(_, _, a) => Some(a.to_owned()),
        }
    }

    /// Set the amount of ammo in the magazine
    /// Returns true if the ammo was set, false if the item is not a magazine
    pub fn set_ammo(&mut self, ammo: u32) -> bool {
        match self {
            Self::Magazine(_, _, a) => {
                *a = ammo;
                true
            }
            Self::Item(..) => false,
        }
    }
}
impl FromArma for InventoryItem {
    fn from_arma(s: String) -> Result<Self, FromArmaError> {
        let commas = s.matches(',').count();
        match commas {
            1 => <(String, u32)>::from_arma(s).map(|(name, count)| Self::Item(name, count)),
            2 => <(String, u32, u32)>::from_arma(s)
                .map(|(name, count, ammo)| Self::Magazine(name, count, ammo)),
            _ => Err(FromArmaError::custom(format!(
                "Invalid inventory item: {s}"
            ))),
        }
    }
}
impl IntoArma for InventoryItem {
    fn to_arma(&self) -> Value {
        match self {
            Self::Item(name, count) => Value::Array(vec![
                Value::String(name.clone()),
                Value::Number(f64::from(*count)),
            ]),
            Self::Magazine(name, count, ammo) => Value::Array(vec![
                Value::String(name.clone()),
                Value::Number(f64::from(*count)),
                Value::Number(f64::from(*ammo)),
            ]),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{FromArma, IntoArma, Value};

    use super::InventoryItem;

    #[test]
    fn is() {
        let item = InventoryItem::new_item("test".to_owned(), 1);
        assert_eq!(item.class(), "test");
        assert_eq!(item.count(), 1);
        assert_eq!(item.ammo(), None);
        assert!(!item.is_magazine());
        let item = InventoryItem::new_magazine("test".to_owned(), 1, 1);
        assert_eq!(item.class(), "test");
        assert_eq!(item.count(), 1);
        assert_eq!(item.ammo(), Some(1));
        assert!(item.is_magazine());
    }

    #[test]
    fn class() {
        let mut item = InventoryItem::new_item("test".to_owned(), 1);
        assert_eq!(item.class(), "test");
        item.set_class("test2".to_owned());
        assert_eq!(item.class(), "test2");
    }

    #[test]
    fn count() {
        let mut item = InventoryItem::new_item("test".to_owned(), 1);
        assert_eq!(item.count(), 1);
        item.set_count(2);
        assert_eq!(item.count(), 2);
    }

    #[test]
    fn ammo() {
        let item = InventoryItem::new_magazine("test".to_owned(), 1, 1);
        assert_eq!(item.ammo(), Some(1));
        assert!(item.is_magazine());
        let item = InventoryItem::new_item("test".to_owned(), 1);
        assert_eq!(item.ammo(), None);
        assert!(!item.is_magazine());
    }

    #[test]
    fn from_arma() {
        let item = InventoryItem::from_arma("[\"test\",1]".to_owned()).unwrap();
        assert_eq!(item.class(), "test");
        assert_eq!(item.count(), 1);
        assert_eq!(item.ammo(), None);
        assert!(!item.is_magazine());
        let item = InventoryItem::from_arma("[\"test\",1,1]".to_owned()).unwrap();
        assert_eq!(item.class(), "test");
        assert_eq!(item.count(), 1);
        assert_eq!(item.ammo(), Some(1));
        assert!(item.is_magazine());
    }

    #[test]
    fn to_arma() {
        let item = InventoryItem::new_item("test".to_owned(), 1);
        assert_eq!(
            item.to_arma(),
            Value::Array(vec![Value::String("test".to_owned()), Value::Number(1.0),])
        );
        let item = InventoryItem::new_magazine("test".to_owned(), 1, 1);
        assert_eq!(
            item.to_arma(),
            Value::Array(vec![
                Value::String("test".to_owned()),
                Value::Number(1.0),
                Value::Number(1.0),
            ])
        );
    }
}
