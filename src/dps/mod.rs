pub mod instruction_set;
pub mod dps_scheduler;
pub mod dps_simulation;
#[cfg(feature = "wasm")]
pub mod js_dps_interface;

use std::collections::HashMap;

use crate::HashId;
use crate::weapons::Weapon;
use crate::abilities::Ability;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedAbilityDpsData {
    pub untyped: HashMap<String, f64>,
    pub available: bool,
}
impl Default for CachedAbilityDpsData {
    fn default() -> Self {
        CachedAbilityDpsData {
            untyped: HashMap::new(),
            available: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedWeaponDpsData {
    pub untyped: HashMap<String, f64>,
    pub magazine: f64,
    pub base_magazine: f64,
}
impl Default for CachedWeaponDpsData {
    fn default() -> Self {
        CachedWeaponDpsData {
            untyped: HashMap::new(),
            magazine: 0.0,
            base_magazine: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadoutDpsData {
    pub kinetic: Option<CachedWeaponDpsData>,
    pub energy: Option<CachedWeaponDpsData>,
    pub power: Option<CachedWeaponDpsData>,
    pub melee: Option<CachedAbilityDpsData>,
    pub grenade: Option<CachedAbilityDpsData>,
    pub _super: Option<CachedAbilityDpsData>,
    pub class: Option<CachedAbilityDpsData>,
}
impl LoadoutDpsData {
    pub fn new() -> LoadoutDpsData {
        LoadoutDpsData {
            kinetic: Some(CachedWeaponDpsData::default()),
            energy: Some(CachedWeaponDpsData::default()),
            power: Some(CachedWeaponDpsData::default()),
            melee: Some(CachedAbilityDpsData::default()),
            grenade: Some(CachedAbilityDpsData::default()),
            _super: Some(CachedAbilityDpsData::default()),
            class: Some(CachedAbilityDpsData::default()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Loadout {
    pub kinetic_weapon: Option<Weapon>,
    pub energy_weapon: Option<Weapon>,
    pub power_weapon: Option<Weapon>,
    pub melee_ability: Option<Ability>,
    pub grenade_ability: Option<Ability>,
    pub super_ability: Option<Ability>,
    pub class_ability: Option<Ability>,
    pub armor_effects: Vec<(HashId, u32)>
}
impl Loadout {
    // easier field population checks
    pub fn has_kinetic_weapon(&self) -> bool {
        self.kinetic_weapon.is_some()
    }
    pub fn has_energy_weapon(&self) -> bool {
        self.energy_weapon.is_some()
    }
    pub fn has_power_weapon(&self) -> bool {
        self.power_weapon.is_some()
    }
    pub fn has_melee_ability(&self) -> bool {
        self.melee_ability.is_some()
    }
    pub fn has_grenade_ability(&self) -> bool {
        self.grenade_ability.is_some()
    }
    pub fn has_super_ability(&self) -> bool {
        self.super_ability.is_some()
    }
    pub fn has_class_ability(&self) -> bool {
        self.class_ability.is_some()
    }
}

