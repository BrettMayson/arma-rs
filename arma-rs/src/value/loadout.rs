//! For working with Arma's unit loadout array

use crate::{FromArma, IntoArma, Value};

#[derive(Debug, Default, Clone, PartialEq)]
/// A magazine loaded into a weapon
pub struct Magazine(Option<(String, u32)>);
impl Magazine {
    /// Create a new magazine
    pub fn new(class: String, count: u32) -> Self {
        Self(Some((class, count)))
    }

    /// The magazine exists
    pub fn exists(&self) -> bool {
        self.0.is_some()
    }

    /// Arma class name of the magazine
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
        <(String, u32)>::from_arma(s).map(|(name, count)| Magazine(Some((name, count))))
    }
}
impl IntoArma for Magazine {
    fn to_arma(&self) -> Value {
        if let Some(magazine) = self.0.as_ref() {
            Value::Array(vec![
                Value::String(magazine.0.to_owned()),
                Value::Number(magazine.1 as f64),
            ])
        } else {
            Value::Array(vec![])
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
/// An item stored in a uniform, vest, or backpack
pub enum InventoryItem {
    /// An item that is not a magazine
    Item(String, u32),
    /// A magazine
    Magazine(String, u32, u32),
}
impl InventoryItem {
    /// Create a new item
    pub fn new_item(class: String, count: u32) -> Self {
        Self::Item(class, count)
    }

    /// Create a new magazine
    pub fn new_magazine(class: String, count: u32, ammo: u32) -> Self {
        Self::Magazine(class, count, ammo)
    }

    /// The item is a magazine
    pub fn is_magazine(&self) -> bool {
        matches!(self, InventoryItem::Magazine(_, _, _))
    }

    /// The class name of the item
    pub fn class(&self) -> &str {
        match self {
            InventoryItem::Item(c, _) => c.as_str(),
            InventoryItem::Magazine(c, _, _) => c.as_str(),
        }
    }

    /// Set the class name of the item
    pub fn set_class(&mut self, class: String) {
        match self {
            InventoryItem::Item(c, _) => *c = class,
            InventoryItem::Magazine(c, _, _) => *c = class,
        }
    }

    /// The amount of the item
    pub fn count(&self) -> u32 {
        match self {
            InventoryItem::Item(_, c) => c.to_owned(),
            InventoryItem::Magazine(_, c, _) => c.to_owned(),
        }
    }

    /// Set the amount of the item
    pub fn set_count(&mut self, count: u32) {
        match self {
            InventoryItem::Item(_, c) => *c = count,
            InventoryItem::Magazine(_, c, _) => *c = count,
        }
    }

    /// The amount of ammo in the magazine
    pub fn ammo(&self) -> Option<u32> {
        match self {
            InventoryItem::Magazine(_, _, a) => Some(a.to_owned()),
            _ => None,
        }
    }

    /// Set the amount of ammo in the magazine
    /// Returns true if the ammo was set, false if the item is not a magazine
    pub fn set_ammo(&mut self, ammo: u32) -> bool {
        match self {
            InventoryItem::Magazine(_, _, a) => {
                *a = ammo;
                true
            }
            _ => false,
        }
    }
}
impl FromArma for InventoryItem {
    fn from_arma(s: String) -> Result<Self, String> {
        let commas = s.matches(',').count();
        match commas {
            1 => {
                <(String, u32)>::from_arma(s).map(|(name, count)| InventoryItem::Item(name, count))
            }
            2 => <(String, u32, u32)>::from_arma(s)
                .map(|(name, count, ammo)| InventoryItem::Magazine(name, count, ammo)),
            _ => Err(format!("Invalid inventory item: {}", s)),
        }
    }
}
impl IntoArma for InventoryItem {
    fn to_arma(&self) -> Value {
        match self {
            InventoryItem::Item(name, count) => Value::Array(vec![
                Value::String(name.to_owned()),
                Value::Number(*count as f64),
            ]),
            InventoryItem::Magazine(name, count, ammo) => Value::Array(vec![
                Value::String(name.to_owned()),
                Value::Number(*count as f64),
                Value::Number(*ammo as f64),
            ]),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
/// A primary, secondary, or handgun weapon
pub struct Weapon(Option<(String, String, String, String, Magazine, Magazine, String)>);
impl Weapon {
    /// Create a new weapon
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
    pub fn exists(&self) -> bool {
        self.0.is_some()
    }

    /// The class name of the weapon
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
                Magazine(None),
                Magazine(None),
                "".to_owned(),
            ));
        }
    }

    /// The class name of the attached suppressor
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
                Weapon(Some((
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
        if let Some(weapon) = self.0.as_ref() {
            Value::Array(vec![
                Value::String(weapon.0.to_owned()),
                Value::String(weapon.1.to_owned()),
                Value::String(weapon.2.to_owned()),
                Value::String(weapon.3.to_owned()),
                weapon.4.to_arma(),
                weapon.5.to_arma(),
                Value::String(weapon.6.to_owned()),
            ])
        } else {
            Value::Array(vec![])
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
/// A uniform, vest, or backpack
pub struct Container(Option<(String, Vec<InventoryItem>)>);
impl Container {
    /// Create a new container
    pub fn new(class: String) -> Self {
        Container(Some((class, vec![])))
    }

    /// The container exists
    pub fn exists(&self) -> bool {
        self.0.is_some()
    }

    /// The class name of the container
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
    pub fn items(&self) -> Option<&Vec<InventoryItem>> {
        self.0.as_ref().map(|(_, items)| items)
    }

    /// The items in the container
    pub fn items_mut(&mut self) -> Option<&mut Vec<InventoryItem>> {
        self.0.as_mut().map(|(_, items)| items)
    }
}
impl FromArma for Container {
    fn from_arma(s: String) -> Result<Self, String> {
        if s == "[]" {
            return Ok(Self(None));
        }
        <(String, Vec<InventoryItem>)>::from_arma(s)
            .map(|(name, items)| Container(Some((name, items))))
    }
}
impl IntoArma for Container {
    fn to_arma(&self) -> Value {
        if let Some(container) = self.0.as_ref() {
            Value::Array(vec![
                Value::String(container.0.to_owned()),
                Value::Array(container.1.iter().map(|i| i.to_arma()).collect()),
            ])
        } else {
            Value::Array(vec![])
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
/// Assigned items in the loadout
pub struct AssignedItems(String, String, String, String, String, String);
impl AssignedItems {
    /// The class name of the assigned map
    pub fn map(&self) -> &str {
        &self.0
    }

    /// Set the class name of the assigned map
    pub fn set_map(&mut self, map: String) {
        self.0 = map;
    }

    /// The class name of the assigned terminal
    pub fn terminal(&self) -> &str {
        &self.1
    }

    /// Set the class name of the assigned terminal
    pub fn set_terminal(&mut self, terminal: String) {
        self.1 = terminal;
    }

    /// The class name of the assigned radio
    pub fn radio(&self) -> &str {
        &self.2
    }

    /// Set the class name of the assigned radio
    pub fn set_radio(&mut self, radio: String) {
        self.2 = radio;
    }

    /// The class name of the assigned compass
    pub fn compass(&self) -> &str {
        &self.3
    }

    /// Set the class name of the assigned compass
    pub fn set_compass(&mut self, compass: String) {
        self.3 = compass;
    }

    /// The class name of the assigned watch
    pub fn watch(&self) -> &str {
        &self.4
    }

    /// Set the class name of the assigned watch
    pub fn set_watch(&mut self, watch: String) {
        self.4 = watch;
    }

    /// The class name of the assigned NVG
    pub fn nvg(&self) -> &str {
        &self.5
    }

    /// Set the class name of the assigned NVG
    pub fn set_nvg(&mut self, nvg: String) {
        self.5 = nvg;
    }

    /// Get all items
    pub fn items(&self) -> [&str; 6] {
        [
            self.map(),
            self.terminal(),
            self.radio(),
            self.compass(),
            self.watch(),
            self.nvg(),
        ]
    }
}
impl FromArma for AssignedItems {
    fn from_arma(s: String) -> Result<Self, String> {
        <(String, String, String, String, String, String)>::from_arma(s).map(
            |(map, gps, radio, compass, watch, nvg)| {
                AssignedItems(map, gps, radio, compass, watch, nvg)
            },
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

#[derive(Debug, Default, Clone, PartialEq)]
/// Arma Unit Loadout Array
pub struct Loadout(
    Weapon,
    Weapon,
    Weapon,
    Container,
    Container,
    Container,
    String,
    String,
    Weapon,
    AssignedItems,
);
impl Loadout {
    /// Get the primary weapon
    pub fn primary(&self) -> &Weapon {
        &self.0
    }

    /// Get the primary weapon mutably
    pub fn primary_mut(&mut self) -> &mut Weapon {
        &mut self.0
    }

    /// Set the primary weapon
    pub fn set_primary(&mut self, primary: Weapon) {
        self.0 = primary;
    }

    /// Get the secondary weapon (launcher)
    pub fn secondary(&self) -> &Weapon {
        &self.1
    }

    /// Get the secondary weapon (launcher) mutably
    pub fn secondary_mut(&mut self) -> &mut Weapon {
        &mut self.1
    }

    /// Set the secondary weapon (launcher)
    pub fn set_secondary(&mut self, secondary: Weapon) {
        self.1 = secondary;
    }

    /// Get the handgun weapon
    pub fn handgun(&self) -> &Weapon {
        &self.2
    }

    /// Get the handgun weapon mutably
    pub fn handgun_mut(&mut self) -> &mut Weapon {
        &mut self.2
    }

    /// Set the handgun weapon
    pub fn set_handgun(&mut self, handgun: Weapon) {
        self.2 = handgun;
    }

    /// Get the uniform
    pub fn uniform(&self) -> &Container {
        &self.3
    }

    /// Get the uniform mutably
    pub fn uniform_mut(&mut self) -> &mut Container {
        &mut self.3
    }

    /// Set the uniform
    pub fn set_uniform(&mut self, uniform: Container) {
        self.3 = uniform;
    }

    /// Get the vest
    pub fn vest(&self) -> &Container {
        &self.4
    }

    /// Get the vest mutably
    pub fn vest_mut(&mut self) -> &mut Container {
        &mut self.4
    }

    /// Set the vest
    pub fn set_vest(&mut self, vest: Container) {
        self.4 = vest;
    }

    /// Get the backpack
    pub fn backpack(&self) -> &Container {
        &self.5
    }

    /// Get the backpack mutably
    pub fn backpack_mut(&mut self) -> &mut Container {
        &mut self.5
    }

    /// Set the backpack
    pub fn set_backpack(&mut self, backpack: Container) {
        self.5 = backpack;
    }

    /// The class name of the current headgear
    pub fn headgear(&self) -> &str {
        &self.6
    }

    /// Set the class name of the current headgear
    pub fn set_headgear(&mut self, headgear: String) {
        self.6 = headgear;
    }

    /// The class name of the current goggles / facewear
    pub fn goggles(&self) -> &str {
        &self.7
    }

    /// Set the class name of the current goggles / facewear
    pub fn set_goggles(&mut self, goggles: String) {
        self.7 = goggles;
    }

    /// Get the binocular
    pub fn binoculars(&self) -> &Weapon {
        &self.8
    }

    /// Get the binocular mutably
    pub fn binoculars_mut(&mut self) -> &mut Weapon {
        &mut self.8
    }

    /// Set the binocular
    pub fn set_binoculars(&mut self, binoculars: Weapon) {
        self.8 = binoculars;
    }

    /// Get the assigned items
    pub fn assigned_items(&self) -> &AssignedItems {
        &self.9
    }

    /// Get the assigned items mutably
    pub fn assigned_items_mut(&mut self) -> &mut AssignedItems {
        &mut self.9
    }

    /// Set the assigned items
    pub fn set_assigned_items(&mut self, assigned_items: AssignedItems) {
        self.9 = assigned_items;
    }
}
impl FromArma for Loadout {
    fn from_arma(s: String) -> Result<Self, String> {
        <(
            Weapon,
            Weapon,
            Weapon,
            Container,
            Container,
            Container,
            String,
            String,
            Weapon,
            AssignedItems,
        )>::from_arma(s)
        .map(
            |(
                primary,
                secondary,
                handgun,
                uniform,
                vest,
                backpack,
                headgear,
                goggles,
                binoculars,
                linked_items,
            )| {
                Loadout(
                    primary,
                    secondary,
                    handgun,
                    uniform,
                    vest,
                    backpack,
                    headgear,
                    goggles,
                    binoculars,
                    linked_items,
                )
            },
        )
    }
}
impl IntoArma for Loadout {
    fn to_arma(&self) -> Value {
        Value::Array(vec![
            self.primary().to_arma(),
            self.secondary().to_arma(),
            self.handgun().to_arma(),
            self.uniform().to_arma(),
            self.vest().to_arma(),
            self.backpack().to_arma(),
            Value::String(self.headgear().to_owned()),
            Value::String(self.goggles().to_owned()),
            self.binoculars().to_arma(),
            self.assigned_items().to_arma(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn b_soldier_at_f() {
        let loadout = r#"[["arifle_MXC_Holo_pointer_F", "", "acc_pointer_IR", "optic_Holosight", ["30Rnd_65x39_caseless_mag", 30], [], ""],
        ["launch_B_Titan_short_F", "", "", "", ["Titan_AT", 1], [], ""],
        ["hgun_P07_F", "", "", "", ["16Rnd_9x21_Mag", 16], [], ""],
        ["U_B_CombatUniform_mcam", [["FirstAidKit", 1], ["30Rnd_65x39_caseless_mag", 2, 30], ["Chemlight_green", 1, 1]]],
        ["V_PlateCarrier1_rgr", [["30Rnd_65x39_caseless_mag", 3, 30], ["16Rnd_9x21_Mag", 2, 16], ["SmokeShell", 1 ,1], ["SmokeShellGreen", 1, 1], ["Chemlight_green", 1, 1]]],
        ["B_AssaultPack_mcamo_AT",[["Titan_AT", 2, 1]]],
        "H_HelmetB_light_desert", "G_Bandanna_tan",[],
        ["ItemMap", "", "ItemRadio", "ItemCompass", "ItemWatch", "NVGoggles"]]"#;
        Loadout::from_arma(loadout.to_string()).unwrap();
        let loadout = r#"[["arifle_SPAR_02_blk_F","","","optic_Holosight_blk_F",["30Rnd_556x45_Stanag",30],[],""],[],["hgun_ACPC2_F","","","",["9Rnd_45ACP_Mag",8],[],""],["tacs_Uniform_Polo_TP_LS_TP_TB_NoLogo",[]],["V_PlateCarrier1_rgr_noflag_F",[]],[],"H_Cap_headphones","G_Shades_Black",[],["ItemMap","ItemGPS","ItemRadio","ItemCompass","ItemWatch",""]]"#;
        let mut loadout = Loadout::from_arma(loadout.to_string()).unwrap();
        loadout.set_secondary({
            let mut weapon = Weapon::new("launch_B_Titan_short_F".to_string());
            weapon.set_primary_magazine(Magazine::new("Titan_AT".to_string(), 1));
            weapon
        });
        Loadout::from_arma(loadout.to_arma().to_string()).unwrap();
    }

    #[test]
    fn marshal() {
        let loadout = r#"[[],[],[],["U_Marshal",[]],[],[],"H_Cap_headphones","G_Aviator",[],["ItemMap","ItemGPS","","ItemCompass","ItemWatch",""]]"#;
        let mut loadout = Loadout::from_arma(loadout.to_string()).unwrap();
        loadout.set_secondary({
            let mut weapon = Weapon::new("launch_B_Titan_short_F".to_string());
            weapon.set_primary_magazine(Magazine::new("Titan_AT".to_string(), 1));
            weapon
        });
        loadout.set_primary({
            let mut weapon = Weapon::new("arifle_MXC_F".to_string());
            weapon.set_optic("optic_Holosight".to_string());
            weapon
        });
        let uniform = loadout.uniform_mut();
        uniform.set_class("U_B_CombatUniform_mcam".to_string());
        let uniform_items = uniform.items_mut().unwrap();
        uniform_items.push(InventoryItem::new_item("FirstAidKit".to_string(), 3));
        uniform_items.push(InventoryItem::new_magazine(
            "30Rnd_65x39_caseless_mag".to_string(),
            5,
            30,
        ));
    }
}
