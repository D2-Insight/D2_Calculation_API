use std::collections::HashMap;

use crate::D2Enums::AmmoType;
use crate::D2Enums::DamageType;
use crate::D2Enums::WeaponSlot;
use crate::D2Enums::WeaponType;
use crate::js_types::JsWeaponFormula;
use crate::perks::Perk;
use crate::D2Enums::StatHashes;
use crate::D2Structs::BuffPackage;
use crate::D2Structs::FiringConfig;
use crate::perks::get_perk_stats;
use crate::perks::lib::CalculationInput;

use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct JS_Stat {
    base_value: i32,
    part_value: i32,
    perk_value: i32,
}
impl JS_Stat {
    pub fn from_stat(stat: &Stat) -> JS_Stat {
        JS_Stat {
            base_value: stat.base_value,
            part_value: stat.part_value,
            perk_value: stat.perk_value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Stat {
    pub base_value: i32,
    pub perk_value: i32,
    pub part_value: i32,
}
impl Stat {
    pub fn new() -> Stat {
        Stat {
            base_value: 0,
            part_value: 0,
            perk_value: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Weapon {
    //ideally entirely interfaced with through funcs
    pub perks: HashMap<u32, Perk>,
    pub stats: HashMap<u32, Stat>,
    pub damage_mods: BuffPackage,
    pub firing_data: FiringConfig,
    pub id: u32,
    pub formulas: Option<JsWeaponFormula>,

    pub weapon_type: WeaponType,
    pub weapon_slot: WeaponSlot,
    pub damage_type: DamageType,
    pub ammo_type: AmmoType,
}
impl Weapon {
    pub fn add_perk(&mut self, _perk: Perk) {
        self.perks.insert(_perk.hash, _perk);
    }
    pub fn remove_perk(&mut self, _perk: Perk) {
        self.perks.remove(&_perk.hash);
    }
    pub fn list_perk_ids(&self) -> Vec<u32> {
        let mut perk_list: Vec<u32> = Vec::new();
        for (key, _perk) in &self.perks {
            perk_list.push(*key);
        }
        perk_list
    }
    pub fn list_perk_values(&self) -> Vec<Perk> {
        let mut perk_list: Vec<Perk> = Vec::new();
        for (_key, perk) in &self.perks {
            perk_list.push(perk.clone());
        }
        perk_list
    }
    pub fn change_perk_val(&mut self, _perk_hash: u32, _val: i32) {
        let perk_opt = self.perks.get_mut(&_perk_hash);
        if perk_opt.is_some() {
            perk_opt.unwrap().value = _val;
        }
    }
    pub fn get_stats(&mut self) -> HashMap<u32, Stat> {
        self.update_stats();
        self.stats.clone()
    }
    pub fn reset(&mut self) {
        //TODO: make this a trait
        self.perks = HashMap::new();
        self.stats = HashMap::new();
        self.id = 0;
        self.damage_mods = BuffPackage::new();
        self.firing_data = FiringConfig::default();
        self.formulas = None;
    }
}
impl Default for Weapon {
    fn default() -> Weapon {
        Weapon {
            perks: HashMap::new(),
            stats: HashMap::new(),
            id: 0,
            damage_mods: BuffPackage::new(),
            firing_data: FiringConfig::default(),
            formulas: None,

            weapon_type: WeaponType::UNKNOWN,
            weapon_slot: WeaponSlot::UNKNOWN,
            damage_type: DamageType::UNKNOWN,
            ammo_type: AmmoType::UNKNOWN,
        }
    }
}
impl Weapon {
    fn update_stats(&mut self) {
        let input = CalculationInput::construct_static(self.firing_data.clone(), self.stats.clone(), self.weapon_type, self.ammo_type);
        let inter_var = get_perk_stats(self.list_perk_values(), input, false);
        let dynamic_stats = &inter_var[0];
        let static_stats = &inter_var[1];
        for (key, stat) in &mut self.stats {
            let a = static_stats.get(key);
            let b = dynamic_stats.get(key);
            if a.is_some() {
                stat.part_value = a.unwrap().clone();
            }
            if b.is_some() {
                stat.perk_value = b.unwrap().clone();
            }
        }
    }
}
