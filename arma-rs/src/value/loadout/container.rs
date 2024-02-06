use crate::{FromArma, FromArmaError, IntoArma, Value};

use super::InventoryItem;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
/// A uniform, vest, or backpack
pub struct Container(Option<(String, Vec<InventoryItem>)>);
impl Container {
    /// Create a new container
    #[must_use]
    pub const fn new(class: String) -> Self {
        Self(Some((class, vec![])))
    }

    /// The container exists
    #[must_use]
    pub const fn exists(&self) -> bool {
        self.0.is_some()
    }

    /// The class name of the container
    #[must_use]
    pub fn class(&self) -> Option<&str> {
        self.0.as_ref().map(|(class, _)| class.as_str())
    }

    /// Set the class name of the container
    pub fn set_class(&mut self, class: String) {
        if let Some(container) = self.0.as_mut() {
            container.0 = class;
        } else {
            self.0 = Some((class, vec![]));
        }
    }

    /// The items in the container
    #[must_use]
    pub fn items(&self) -> Option<&Vec<InventoryItem>> {
        self.0.as_ref().map(|(_, items)| items)
    }

    /// The items in the container
    pub fn items_mut(&mut self) -> Option<&mut Vec<InventoryItem>> {
        self.0.as_mut().map(|(_, items)| items)
    }

    /// Get all classes and their quantities, including the container itself
    #[must_use]
    pub fn classes(&self) -> Vec<(String, u32)> {
        let mut classes = vec![];
        if let Some((class, items)) = &self.0 {
            classes.push((class.clone(), 1));
            for item in items {
                classes.push((item.class().to_string(), item.count()));
            }
        }
        classes
    }
}
impl FromArma for Container {
    fn from_arma(s: String) -> Result<Self, FromArmaError> {
        if s == "[]" {
            return Ok(Self(None));
        }
        <(String, Vec<InventoryItem>)>::from_arma(s).map(|(name, items)| Self(Some((name, items))))
    }
}
impl IntoArma for Container {
    fn to_arma(&self) -> Value {
        self.0.as_ref().map_or_else(
            || Value::Array(vec![]),
            |container| {
                Value::Array(vec![
                    Value::String(container.0.clone()),
                    Value::Array(container.1.iter().map(IntoArma::to_arma).collect()),
                ])
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{loadout::InventoryItem, FromArma, IntoArma, Value};

    use super::Container;

    #[test]
    fn exists() {
        let container = Container::default();
        assert!(!container.exists());
        let container = Container::new("container".to_string());
        assert!(container.exists());
    }

    #[test]
    fn class() {
        let container = Container::default();
        assert!(container.class().is_none());
        let mut container = Container::new("container".to_string());
        assert_eq!(container.class().unwrap(), "container");
        container.set_class("container2".to_string());
        assert_eq!(container.class().unwrap(), "container2");
    }

    #[test]
    fn items() {
        let container = Container::default();
        assert!(container.items().is_none());
        let mut container = Container::new("container".to_string());
        assert!(container.items().is_some());
        let items = vec![
            InventoryItem::new_item("item1".to_string(), 1),
            InventoryItem::new_item("item2".to_string(), 2),
        ];
        container.0.as_mut().unwrap().1 = items.clone();
        assert_eq!(container.items().unwrap(), &items);
    }

    #[test]
    fn from_arma() {
        let container = Container::from_arma("[]".to_string()).unwrap();
        assert!(!container.exists());
        let container =
            Container::from_arma("[\"container\",[[\"item1\",1],[\"item2\",2]]]".to_string())
                .unwrap();
        assert!(container.exists());
        assert_eq!(container.class().unwrap(), "container");
        assert_eq!(
            container.items().unwrap(),
            &vec![
                InventoryItem::new_item("item1".to_string(), 1),
                InventoryItem::new_item("item2".to_string(), 2),
            ]
        );
    }

    #[test]
    fn to_arma() {
        let container = Container::default();
        assert_eq!(container.to_arma(), Value::Array(vec![]));
        let mut container = Container::new("container".to_string());
        assert_eq!(
            container.to_arma(),
            Value::Array(vec![
                Value::String("container".to_string()),
                Value::Array(vec![]),
            ])
        );
        let items = vec![
            InventoryItem::new_item("item1".to_string(), 1),
            InventoryItem::new_item("item2".to_string(), 2),
        ];
        container.0.as_mut().unwrap().1 = items;
        assert_eq!(
            container.to_arma(),
            Value::Array(vec![
                Value::String("container".to_string()),
                Value::Array(vec![
                    Value::Array(vec![Value::String("item1".to_string()), Value::Number(1.0),]),
                    Value::Array(vec![Value::String("item2".to_string()), Value::Number(2.0),]),
                ]),
            ])
        );
    }

    #[test]
    fn classes() {
        let container = Container::from_arma("[]".to_string()).unwrap();
        assert!(!container.exists());
        let container =
            Container::from_arma("[\"container\",[[\"item1\",1],[\"item2\",2]]]".to_string())
                .unwrap();
        assert_eq!(
            container.classes(),
            vec![
                ("container".to_string(), 1),
                ("item1".to_string(), 1),
                ("item2".to_string(), 2)
            ]
        );
    }
}
