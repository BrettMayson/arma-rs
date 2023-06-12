//! For working with Arma's unit loadout array

use crate::{FromArma, IntoArma, Value};

mod assigned;
mod container;
mod extended;
mod inventory_item;
mod magazine;
mod weapon;

pub use assigned::AssignedItems;
pub use container::Container;
pub use extended::CBAExtended;
pub use inventory_item::InventoryItem;
pub use magazine::Magazine;
pub use weapon::Weapon;

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
    CBAExtended,
);

impl Loadout {
    /// Get the primary weapon
    #[must_use]
    pub const fn primary(&self) -> &Weapon {
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
    #[must_use]
    pub const fn secondary(&self) -> &Weapon {
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
    #[must_use]
    pub const fn handgun(&self) -> &Weapon {
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
    #[must_use]
    pub const fn uniform(&self) -> &Container {
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
    #[must_use]
    pub const fn vest(&self) -> &Container {
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
    #[must_use]
    pub const fn backpack(&self) -> &Container {
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
    #[must_use]
    pub fn headgear(&self) -> &str {
        &self.6
    }

    /// Set the class name of the current headgear
    pub fn set_headgear(&mut self, headgear: String) {
        self.6 = headgear;
    }

    /// The class name of the current goggles / facewear
    #[must_use]
    pub fn goggles(&self) -> &str {
        &self.7
    }

    /// Set the class name of the current goggles / facewear
    pub fn set_goggles(&mut self, goggles: String) {
        self.7 = goggles;
    }

    /// Get the binocular
    #[must_use]
    pub const fn binoculars(&self) -> &Weapon {
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
    #[must_use]
    pub const fn assigned_items(&self) -> &AssignedItems {
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

    /// Get the CBA Extended Loadout Array
    pub fn cba_extended(&self) -> &CBAExtended {
        &self.10
    }

    /// Get a map of all items in the loadout and their quantities
    pub fn classes(&self) -> std::collections::HashMap<String, u32> {
        let mut items = std::collections::HashMap::new();
        self.0.classes().iter().for_each(|c| {
            *items.entry(c.to_string()).or_insert(0) += 1;
        });
        self.1.classes().iter().for_each(|c| {
            *items.entry(c.to_string()).or_insert(0) += 1;
        });
        self.2.classes().iter().for_each(|c| {
            *items.entry(c.to_string()).or_insert(0) += 1;
        });
        self.3.classes().iter().for_each(|(c, q)| {
            *items.entry(c.clone()).or_insert(0) += q;
        });
        self.4.classes().iter().for_each(|(c, q)| {
            *items.entry(c.clone()).or_insert(0) += q;
        });
        self.5.classes().iter().for_each(|(c, q)| {
            *items.entry(c.clone()).or_insert(0) += q;
        });
        *items.entry(self.6.clone()).or_insert(0) += 1;
        *items.entry(self.7.clone()).or_insert(0) += 1;
        self.8.classes().iter().for_each(|c| {
            *items.entry(c.to_string()).or_insert(0) += 1;
        });
        self.9.classes().iter().for_each(|c| {
            *items.entry(c.to_string()).or_insert(0) += 1;
        });
        items.remove("");
        items
    }
}

impl FromArma for Loadout {
    fn from_arma(s: String) -> Result<Self, String> {
        let vanilla = <(
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
        )>::from_arma(s.clone())
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
                Self(
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
                    CBAExtended::default(),
                )
            },
        );
        if vanilla.is_err() {
            return <(
                (
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
                ),
                CBAExtended,
            )>::from_arma(s)
            .map(
                |(
                    (
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
                    ),
                    extended,
                )| {
                    Self(
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
                        extended,
                    )
                },
            );
        }
        vanilla
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

    #[test]
    fn extended_empty() {
        let loadout = r#"[[["arifle_XMS_Shot_lxWS","","tacgt_ANPEQ_15_Low_Light_Black","CUP_optic_Elcan_SpecterDR_black_PIP",["tacgt_30Rnd_556x45_Ball_Tracer_PMAG",30],["6Rnd_12Gauge_Pellets",6],""],["CUP_launch_M136_Loaded","","","",[],[],""],[],["tacs_Uniform_Floral_JP_RS_LP_BB",[["kat_guedel",1],["ACE_EntrenchingTool",1],["ACE_EarPlugs",1],["ACE_CableTie",2],["ACE_quikclot",1],["ACE_packingBandage",2],["ACE_elasticBandage",1],["SmokeShell",2,1],["Chemlight_yellow",1,1],["Chemlight_red",3,1],["ACE_Chemlight_IR",2,1],["ACE_Chemlight_HiBlue",1,1]]],["milgp_v_mmac_marksman_belt_CB",[["ACE_tourniquet",4],["ACE_splint",1],["tacgt_30Rnd_556x45_EPR_PMAG",15,30],["tacgt_30Rnd_556x45_AP_PMAG",2,30],["6rnd_Smoke_Mag_lxWS",1,6]]],["milgp_bp_Pointman_cb",[["ACE_SpraypaintGreen",1],["ACE_splint",1],["synixe_painkillers",2],["ACE_microDAGR",1],["ACE_MapTools",1],["ACE_bodyBag",1],["ACE_packingBandage",10],["ACE_elasticBandage",9],["ACE_quikclot",5],["ACE_EarPlugs",2],["ACRE_PRC152",1],["ACRE_PRC152",1],["6Rnd_12Gauge_Pellets",2,6],["6Rnd_12Gauge_Slug",1,6],["tacgt_30Rnd_556x45_Ball_Tracer_PMAG",1,30]]],"synixe_contractors_Hat_Beret_Black","",["ACE_VectorDay","","","",[],[],""],["ItemMap","ItemGPS","","ItemCompass","ItemWatch",""]],[]]"#;
        let loadout = Loadout::from_arma(loadout.to_string()).unwrap();
        assert!(loadout.cba_extended().is_empty());
    }

    #[test]
    fn extended_crash_1_9_0() {
        let loadout = r#"[[[],[],[],["synixe_contractors_Uniform_Contractor_Shirt",[]],[],[],"","",[],["","","","","",""]],[]]"#;
        let loadout = Loadout::from_arma(loadout.to_string()).unwrap();
        assert!(loadout.cba_extended().is_empty());
    }

    #[test]
    fn extended_items() {
        let loadout = r#"[[["CUP_arifle_M4A1_SOMMOD_Grip_tan","","","CUP_optic_Eotech553_Black",["tacgt_30Rnd_556x45_EPR_PMAG_Tan",30],[],""],[],["ACE_VMM3","","","",[],[],""],["casual_plaid_gray_khaki_uniform",[["ACE_packingBandage",10],["ACE_elasticBandage",10],["ACE_CableTie",2],["kat_guedel",1],["ACE_tourniquet",2],["ACE_splint",1],["synixe_painkillers",2]]],["milgp_v_mmac_assaulter_belt_AOR2",[["ACRE_PRC152",1],["SmokeShell",2,1],["HandGrenade",2,1],["tacgt_30Rnd_556x45_EPR_PMAG_Tan",10,30]]],["B_MU_TacticalPack_cbr",[["ACE_bodyBag",1],["ToolKit",1],["ACE_SpraypaintGreen",1],["synixe_axe",1],["ACE_wirecutter",1],["ACE_EntrenchingTool",1],["ACE_rope3",2],["DemoCharge_Remote_Mag",2,1]]],"synixe_contractors_Cap_Headphones_GreenLogo","CUP_G_Tan_Scarf_Shades",["Binocular","","","",[],[],""],["ItemMap","ItemGPS","","ItemCompass","ItemWatch",""]],[["grad_slingHelmet","CUP_H_OpsCore_Grey"]]]"#;
        let loadout = Loadout::from_arma(loadout.to_string()).unwrap();
        assert!(!loadout.cba_extended().is_empty());
        assert_eq!(
            loadout.primary().class(),
            Some("CUP_arifle_M4A1_SOMMOD_Grip_tan")
        );
        assert_eq!(
            loadout.cba_extended().get("grad_slingHelmet"),
            Some(&Value::String("CUP_H_OpsCore_Grey".to_string()))
        );
    }

    #[test]
    fn classes() {
        let loadout = r#"[[["CUP_arifle_M4A1_SOMMOD_Grip_tan","","","CUP_optic_Eotech553_Black",["tacgt_30Rnd_556x45_EPR_PMAG_Tan",30],[],""],[],["ACE_VMM3","","","",[],[],""],["casual_plaid_gray_khaki_uniform",[["ACE_packingBandage",10],["ACE_elasticBandage",10],["ACE_CableTie",2],["kat_guedel",1],["ACE_tourniquet",2],["ACE_splint",1],["synixe_painkillers",2]]],["milgp_v_mmac_assaulter_belt_AOR2",[["ACRE_PRC152",1],["SmokeShell",2,1],["HandGrenade",2,1],["tacgt_30Rnd_556x45_EPR_PMAG_Tan",10,30]]],["B_MU_TacticalPack_cbr",[["ACE_bodyBag",1],["ToolKit",1],["ACE_SpraypaintGreen",1],["synixe_axe",1],["ACE_wirecutter",1],["ACE_EntrenchingTool",1],["ACE_rope3",2],["DemoCharge_Remote_Mag",2,1]]],"synixe_contractors_Cap_Headphones_GreenLogo","CUP_G_Tan_Scarf_Shades",["Binocular","","","",[],[],""],["ItemMap","ItemGPS","","ItemCompass","ItemWatch",""]],[["grad_slingHelmet","CUP_H_OpsCore_Grey"]]]"#;
        let loadout = Loadout::from_arma(loadout.to_string()).unwrap();
        assert_eq!(loadout.classes(), {
            let mut classes = std::collections::HashMap::new();
            classes.insert("CUP_arifle_M4A1_SOMMOD_Grip_tan".to_string(), 1);
            classes.insert("CUP_optic_Eotech553_Black".to_string(), 1);
            classes.insert("tacgt_30Rnd_556x45_EPR_PMAG_Tan".to_string(), 11);
            classes.insert("ACE_VMM3".to_string(), 1);
            classes.insert("casual_plaid_gray_khaki_uniform".to_string(), 1);
            classes.insert("ACE_packingBandage".to_string(), 10);
            classes.insert("ACE_elasticBandage".to_string(), 10);
            classes.insert("ACE_CableTie".to_string(), 2);
            classes.insert("kat_guedel".to_string(), 1);
            classes.insert("ACE_tourniquet".to_string(), 2);
            classes.insert("ACE_splint".to_string(), 1);
            classes.insert("synixe_painkillers".to_string(), 2);
            classes.insert("milgp_v_mmac_assaulter_belt_AOR2".to_string(), 1);
            classes.insert("ACRE_PRC152".to_string(), 1);
            classes.insert("SmokeShell".to_string(), 2);
            classes.insert("HandGrenade".to_string(), 2);
            classes.insert("B_MU_TacticalPack_cbr".to_string(), 1);
            classes.insert("ACE_bodyBag".to_string(), 1);
            classes.insert("ToolKit".to_string(), 1);
            classes.insert("ACE_SpraypaintGreen".to_string(), 1);
            classes.insert("synixe_axe".to_string(), 1);
            classes.insert("ACE_wirecutter".to_string(), 1);
            classes.insert("ACE_EntrenchingTool".to_string(), 1);
            classes.insert("ACE_rope3".to_string(), 2);
            classes.insert("DemoCharge_Remote_Mag".to_string(), 2);
            classes.insert("synixe_contractors_Cap_Headphones_GreenLogo".to_string(), 1);
            classes.insert("CUP_G_Tan_Scarf_Shades".to_string(), 1);
            classes.insert("Binocular".to_string(), 1);
            classes.insert("ItemMap".to_string(), 1);
            classes.insert("ItemGPS".to_string(), 1);
            classes.insert("ItemCompass".to_string(), 1);
            classes.insert("ItemWatch".to_string(), 1);
            classes
        });
    }
}
