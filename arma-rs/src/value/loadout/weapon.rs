use crate::{FromArma, IntoArma, Value};

use super::Magazine;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
/// A primary, secondary, or handgun weapon
pub struct Weapon(Option<(String, String, String, String, Magazine, Magazine, String)>);
impl Weapon {
    /// Create a new weapon
    #[must_use]
    pub fn new(class: String) -> Self {
        Self(Some((
            class,
            "".to_string(),
            "".to_string(),
            "".to_string(),
            Magazine::default(),
            Magazine::default(),
            "".to_string(),
        )))
    }

    /// The weapon slot is occupied
    #[must_use]
    pub const fn exists(&self) -> bool {
        self.0.is_some()
    }

    /// The class name of the weapon
    #[must_use]
    pub fn class(&self) -> Option<&str> {
        self.0
            .as_ref()
            .map(|(class, _, _, _, _, _, _)| class.as_str())
    }

    /// Set the class name of the weapon
    pub fn set_class(&mut self, class: String) {
        if let Some(weapon) = self.0.as_mut() {
            weapon.0 = class;
        } else {
            self.0 = Some((
                class,
                "".to_owned(),
                "".to_owned(),
                "".to_owned(),
                Magazine::default(),
                Magazine::default(),
                "".to_owned(),
            ));
        }
    }

    /// The class name of the attached suppressor
    #[must_use]
    pub fn suppressor(&self) -> Option<&str> {
        self.0
            .as_ref()
            .map(|(_, suppressor, _, _, _, _, _)| suppressor.as_str())
    }

    /// Set the class name of the attached suppressor
    /// Returns true if the suppressor was set, false if the weapon was not initialized
    pub fn set_suppressor(&mut self, suppressor: String) -> bool {
        if let Some(weapon) = self.0.as_mut() {
            weapon.1 = suppressor;
            true
        } else {
            false
        }
    }

    /// The class name of the attached pointer
    #[must_use]
    pub fn pointer(&self) -> Option<&str> {
        self.0
            .as_ref()
            .map(|(_, _, pointer, _, _, _, _)| pointer.as_str())
    }

    /// Set the class name of the attached pointer
    /// Returns true if the pointer was set, false if the weapon was not initialized
    pub fn set_pointer(&mut self, pointer: String) -> bool {
        if let Some(weapon) = self.0.as_mut() {
            weapon.2 = pointer;
            true
        } else {
            false
        }
    }

    /// The class name of the attached optic
    #[must_use]
    pub fn optic(&self) -> Option<&str> {
        self.0
            .as_ref()
            .map(|(_, _, _, optic, _, _, _)| optic.as_str())
    }

    /// Set the class name of the attached optic
    /// Returns true if the optic was set, false if the weapon was not initialized
    pub fn set_optic(&mut self, optic: String) -> bool {
        if let Some(weapon) = self.0.as_mut() {
            weapon.3 = optic;
            true
        } else {
            false
        }
    }

    /// Get the inserted primary magazine
    #[must_use]
    pub fn primary_magazine(&self) -> Option<&Magazine> {
        self.0.as_ref().map(|(_, _, _, _, primary, _, _)| primary)
    }

    /// Get the inserted primary magazine mutably
    pub fn primary_magazine_mut(&mut self) -> Option<&mut Magazine> {
        self.0.as_mut().map(|(_, _, _, _, primary, _, _)| primary)
    }

    /// Set the inserted primary magazine
    /// Returns true if the primary magazine was set, false if the weapon was not initialized
    pub fn set_primary_magazine(&mut self, primary: Magazine) -> bool {
        if let Some(weapon) = self.0.as_mut() {
            weapon.4 = primary;
            true
        } else {
            false
        }
    }

    /// Get the inserted secondary magazine
    #[must_use]
    pub fn secondary_magazine(&self) -> Option<&Magazine> {
        self.0
            .as_ref()
            .map(|(_, _, _, _, _, secondary, _)| secondary)
    }

    /// Get the inserted secondary magazine mutably
    pub fn secondary_magazine_mut(&mut self) -> Option<&mut Magazine> {
        self.0
            .as_mut()
            .map(|(_, _, _, _, _, secondary, _)| secondary)
    }

    /// Set the inserted secondary magazine
    /// Returns true if the secondary magazine was set, false if the weapon was not initialized
    pub fn set_secondary_magazine(&mut self, secondary: Magazine) -> bool {
        if let Some(weapon) = self.0.as_mut() {
            weapon.5 = secondary;
            true
        } else {
            false
        }
    }

    /// The class name of the attached bipod
    #[must_use]
    pub fn bipod(&self) -> Option<&str> {
        self.0
            .as_ref()
            .map(|(_, _, _, _, _, _, bipod)| bipod.as_str())
    }

    /// Set the class name of the attached bipod
    /// Returns true if the bipod was set, false if the weapon was not initialized
    pub fn set_bipod(&mut self, bipod: String) -> bool {
        if let Some(weapon) = self.0.as_mut() {
            weapon.6 = bipod;
            true
        } else {
            false
        }
    }
}
impl FromArma for Weapon {
    fn from_arma(s: String) -> Result<Self, String> {
        if s == "[]" {
            return Ok(Self(None));
        }
        <(String, String, String, String, Magazine, Magazine, String)>::from_arma(s).map(
            |(weapon, suppressor, pointer, optic, primary_mag, secondary_mag, bipod)| {
                Self(Some((
                    weapon,
                    suppressor,
                    pointer,
                    optic,
                    primary_mag,
                    secondary_mag,
                    bipod,
                )))
            },
        )
    }
}
impl IntoArma for Weapon {
    fn to_arma(&self) -> Value {
        self.0.as_ref().map_or_else(
            || Value::Array(vec![]),
            |weaon| {
                Value::Array(vec![
                    Value::String(weaon.0.clone()),
                    Value::String(weaon.1.clone()),
                    Value::String(weaon.2.clone()),
                    Value::String(weaon.3.clone()),
                    weaon.4.to_arma(),
                    weaon.5.to_arma(),
                    Value::String(weaon.6.clone()),
                ])
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Weapon;
    use crate::{loadout::Magazine, FromArma, IntoArma, Value};

    #[test]
    fn exists() {
        let weapon = Weapon::default();
        assert!(!weapon.exists());
        let weapon = Weapon::new("arifle_Mk20_GL_F".to_owned());
        assert!(weapon.exists());
    }

    #[test]
    fn class() {
        let weapon = Weapon::default();
        assert_eq!(weapon.class(), None);
        let mut weapon = Weapon::new("arifle_Mk20_GL_F".to_owned());
        assert_eq!(weapon.class(), Some("arifle_Mk20_GL_F"));
        weapon.set_class("arifle_Mk20_F".to_owned());
        assert_eq!(weapon.class(), Some("arifle_Mk20_F"));
    }

    #[test]
    fn suppressor() {
        let weapon = Weapon::default();
        assert_eq!(weapon.suppressor(), None);
        let mut weapon = Weapon::new("arifle_Mk20_GL_F".to_owned());
        assert_eq!(weapon.suppressor(), Some(""));
        weapon.set_suppressor("muzzle_snds_H".to_owned());
        assert_eq!(weapon.suppressor(), Some("muzzle_snds_H"));
    }

    #[test]
    fn pointer() {
        let weapon = Weapon::default();
        assert_eq!(weapon.pointer(), None);
        let mut weapon = Weapon::new("arifle_Mk20_GL_F".to_owned());
        assert_eq!(weapon.pointer(), Some(""));
        weapon.set_pointer("acc_pointer_IR".to_owned());
        assert_eq!(weapon.pointer(), Some("acc_pointer_IR"));
    }

    #[test]
    fn optic() {
        let weapon = Weapon::default();
        assert_eq!(weapon.optic(), None);
        let mut weapon = Weapon::new("arifle_Mk20_GL_F".to_owned());
        assert_eq!(weapon.optic(), Some(""));
        weapon.set_optic("optic_Hamr".to_owned());
        assert_eq!(weapon.optic(), Some("optic_Hamr"));
    }

    #[test]
    fn primary_magazine() {
        let weapon = Weapon::default();
        assert_eq!(weapon.primary_magazine(), None);
        let mut weapon = Weapon::new("arifle_Mk20_GL_F".to_owned());
        assert_eq!(weapon.primary_magazine(), Some(&Magazine::default()));
        weapon.set_primary_magazine(Magazine::new("30Rnd_556x45_Stanag".to_string(), 30));
        assert_eq!(
            weapon.primary_magazine(),
            Some(&Magazine::new("30Rnd_556x45_Stanag".to_string(), 30))
        );
    }

    #[test]
    fn secondary_magazine() {
        let weapon = Weapon::default();
        assert_eq!(weapon.secondary_magazine(), None);
        let mut weapon = Weapon::new("arifle_Mk20_GL_F".to_owned());
        assert_eq!(weapon.secondary_magazine(), Some(&Magazine::default()));
        weapon.set_secondary_magazine(Magazine::new("1Rnd_HE_Grenade_shell".to_string(), 1));
        assert_eq!(
            weapon.secondary_magazine(),
            Some(&Magazine::new("1Rnd_HE_Grenade_shell".to_string(), 1))
        );
    }

    #[test]
    fn bipod() {
        let weapon = Weapon::default();
        assert_eq!(weapon.bipod(), None);
        let mut weapon = Weapon::new("arifle_Mk20_GL_F".to_owned());
        assert_eq!(weapon.bipod(), Some(""));
        weapon.set_bipod("bipod_01_F".to_owned());
        assert_eq!(weapon.bipod(), Some("bipod_01_F"));
    }

    #[test]
    fn from_arma() {
        let weapon = Weapon::from_arma("[]".to_owned()).unwrap();
        assert_eq!(weapon, Weapon::default());
        let weapon = Weapon::from_arma(
            "[\"arifle_Mk20_GL_F\",\"\",\"\",\"\",[\"30Rnd_556x45_Stanag\",30],[\"1Rnd_HE_Grenade_shell\",1],\"\"]".to_owned(),
        )
        .unwrap();
        assert_eq!(weapon, {
            let mut weapon = Weapon::new("arifle_Mk20_GL_F".to_owned());
            weapon.set_primary_magazine(Magazine::new("30Rnd_556x45_Stanag".to_string(), 30));
            weapon.set_secondary_magazine(Magazine::new("1Rnd_HE_Grenade_shell".to_string(), 1));
            weapon
        });
    }

    #[test]
    fn to_arma() {
        let weapon = Weapon::default();
        assert_eq!(weapon.to_arma(), Value::Array(vec![]));
        let weapon = Weapon::new("arifle_Mk20_GL_F".to_owned());
        assert_eq!(
            weapon.to_arma(),
            Value::Array(vec![
                Value::String("arifle_Mk20_GL_F".to_owned()),
                Value::String("".to_owned()),
                Value::String("".to_owned()),
                Value::String("".to_owned()),
                Value::Array(vec![]),
                Value::Array(vec![]),
                Value::String("".to_owned()),
            ])
        );
    }
}
