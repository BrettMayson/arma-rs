use std::collections::HashMap;

use crate::{FromArma, Value};

#[derive(Debug, Default, Clone, PartialEq)]
/// CBA Extended Loadout
///
/// https://github.com/CBATeam/CBA_A3/pull/1503
pub struct CBAExtended(Option<HashMap<String, Value>>);

impl CBAExtended {
    /// Create a new CBA Extended Loadout Array
    pub fn new() -> Self {
        Self::default()
    }

    /// The map has no data or does not exist
    pub fn is_empty(&self) -> bool {
        self.0.is_none() || self.0.as_ref().unwrap().is_empty()
    }

    /// Get a value from the map
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.0.as_ref().and_then(|map| map.get(key))
    }

    /// Get a mutable value from the map
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.0.as_mut().and_then(|map| map.get_mut(key))
    }

    /// Insert a value into the map
    pub fn insert(&mut self, key: String, value: Value) -> Option<Value> {
        self.0.get_or_insert_with(HashMap::new).insert(key, value)
    }

    /// Remove a value from the map
    pub fn remove(&mut self, key: &str) -> Option<Value> {
        self.0.as_mut().and_then(|map| map.remove(key))
    }

    /// Iterate over the keys in the map
    pub fn values(&self) -> impl Iterator<Item = &Value> {
        self.0.iter().flat_map(|map| map.values())
    }
}

impl FromArma for CBAExtended {
    fn from_arma(s: String) -> Result<Self, String> {
        let value = <Vec<(String, Value)>>::from_arma(s);
        match value {
            Ok(value) => Ok(Self(Some(value.into_iter().collect()))),
            Err(_) => Ok(Self(None)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_arma() {
        let value = CBAExtended::from_arma("[]".to_string());
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), CBAExtended(Some(HashMap::new())));

        let value = CBAExtended::from_arma("[[\"cba_xeh_enabled\",true]]".to_string());
        assert!(value.is_ok());
        assert_eq!(
            value.unwrap(),
            CBAExtended(Some(
                vec![("cba_xeh_enabled".to_string(), Value::Boolean(true))]
                    .into_iter()
                    .collect()
            ))
        );
    }
}
