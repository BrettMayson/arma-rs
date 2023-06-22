use crate::{FromArma, FromArmaError, IntoArma, Value};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
/// Assigned items in the loadout
pub struct AssignedItems(String, String, String, String, String, String);
impl AssignedItems {
    /// The class name of the assigned map
    #[must_use]
    pub fn map(&self) -> &str {
        &self.0
    }

    /// Set the class name of the assigned map
    pub fn set_map(&mut self, map: String) {
        self.0 = map;
    }

    /// The class name of the assigned terminal
    #[must_use]
    pub fn terminal(&self) -> &str {
        &self.1
    }

    /// Set the class name of the assigned terminal
    pub fn set_terminal(&mut self, terminal: String) {
        self.1 = terminal;
    }

    /// The class name of the assigned radio
    #[must_use]
    pub fn radio(&self) -> &str {
        &self.2
    }

    /// Set the class name of the assigned radio
    pub fn set_radio(&mut self, radio: String) {
        self.2 = radio;
    }

    /// The class name of the assigned compass
    #[must_use]
    pub fn compass(&self) -> &str {
        &self.3
    }

    /// Set the class name of the assigned compass
    pub fn set_compass(&mut self, compass: String) {
        self.3 = compass;
    }

    /// The class name of the assigned watch
    #[must_use]
    pub fn watch(&self) -> &str {
        &self.4
    }

    /// Set the class name of the assigned watch
    pub fn set_watch(&mut self, watch: String) {
        self.4 = watch;
    }

    /// The class name of the assigned NVG
    #[must_use]
    pub fn nvg(&self) -> &str {
        &self.5
    }

    /// Set the class name of the assigned NVG
    pub fn set_nvg(&mut self, nvg: String) {
        self.5 = nvg;
    }

    /// Get all items
    #[must_use]
    pub fn classes(&self) -> [&str; 6] {
        [
            self.map(),
            self.terminal(),
            self.radio(),
            self.compass(),
            self.watch(),
            self.nvg(),
        ]
    }

    #[deprecated(note = "Use `classes` instead")]
    #[must_use]
    /// Get all items
    pub fn items(&self) -> [&str; 6] {
        self.classes()
    }
}
impl FromArma for AssignedItems {
    fn from_arma(s: String) -> Result<Self, FromArmaError> {
        <(String, String, String, String, String, String)>::from_arma(s).map(
            |(map, gps, radio, compass, watch, nvg)| Self(map, gps, radio, compass, watch, nvg),
        )
    }
}
impl IntoArma for AssignedItems {
    fn to_arma(&self) -> Value {
        Value::Array(vec![
            Value::String(self.map().to_owned()),
            Value::String(self.terminal().to_owned()),
            Value::String(self.radio().to_owned()),
            Value::String(self.compass().to_owned()),
            Value::String(self.watch().to_owned()),
            Value::String(self.nvg().to_owned()),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::AssignedItems;
    use crate::{FromArma, IntoArma, Value};

    #[test]
    fn map() {
        let mut assigned = AssignedItems::default();
        assert_eq!(assigned.map(), "");
        assigned.set_map("map".to_owned());
        assert_eq!(assigned.map(), "map");
    }

    #[test]
    fn terminal() {
        let mut assigned = AssignedItems::default();
        assert_eq!(assigned.terminal(), "");
        assigned.set_terminal("terminal".to_owned());
        assert_eq!(assigned.terminal(), "terminal");
    }

    #[test]
    fn radio() {
        let mut assigned = AssignedItems::default();
        assert_eq!(assigned.radio(), "");
        assigned.set_radio("radio".to_owned());
        assert_eq!(assigned.radio(), "radio");
    }

    #[test]
    fn compass() {
        let mut assigned = AssignedItems::default();
        assert_eq!(assigned.compass(), "");
        assigned.set_compass("compass".to_owned());
        assert_eq!(assigned.compass(), "compass");
    }

    #[test]
    fn watch() {
        let mut assigned = AssignedItems::default();
        assert_eq!(assigned.watch(), "");
        assigned.set_watch("watch".to_owned());
        assert_eq!(assigned.watch(), "watch");
    }

    #[test]
    fn nvg() {
        let mut assigned = AssignedItems::default();
        assert_eq!(assigned.nvg(), "");
        assigned.set_nvg("nvg".to_owned());
        assert_eq!(assigned.nvg(), "nvg");
    }

    #[test]
    fn from_arma() {
        let s = "[\"map\",\"terminal\",\"radio\",\"compass\",\"watch\",\"nvg\"]";
        let assigned = AssignedItems::from_arma(s.to_owned()).unwrap();
        assert_eq!(assigned.map(), "map");
        assert_eq!(assigned.terminal(), "terminal");
        assert_eq!(assigned.radio(), "radio");
        assert_eq!(assigned.compass(), "compass");
        assert_eq!(assigned.watch(), "watch");
        assert_eq!(assigned.nvg(), "nvg");
    }

    #[test]
    fn to_arma() {
        let mut assigned = AssignedItems::default();
        assigned.set_map("map".to_owned());
        assigned.set_terminal("terminal".to_owned());
        assigned.set_radio("radio".to_owned());
        assigned.set_compass("compass".to_owned());
        assigned.set_watch("watch".to_owned());
        assigned.set_nvg("nvg".to_owned());
        let s = assigned.to_arma();
        assert_eq!(
            s,
            Value::Array(vec![
                Value::String("map".to_owned()),
                Value::String("terminal".to_owned()),
                Value::String("radio".to_owned()),
                Value::String("compass".to_owned()),
                Value::String("watch".to_owned()),
                Value::String("nvg".to_owned()),
            ])
        );
    }
}
