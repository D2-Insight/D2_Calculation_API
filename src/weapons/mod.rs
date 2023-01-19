pub mod stat_calc;
pub mod dps_calc;
mod ttk_calc;

use std::collections::HashMap;

use crate::d2_enums::{AmmoType, DamageType, StatHashes, WeaponSlot, WeaponType};
use crate::perks::{
    get_magazine_modifier, get_perk_stats, get_reserve_modifier, lib::CalculationInput, Perk,
};
use crate::types::rs_types::{DamageMods, RangeFormula, AmmoFormula, HandlingFormula, ReloadFormula};


//JavaScript
#[cfg(target_arch = "wasm32")]
use crate::types::js_types::{JsDamageModifiers, JsStat, JsWeaponFormula};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
//Python




#[derive(Debug, Clone)]
pub struct Stat {
    pub base_value: i32,
    pub part_value: i32,
    pub perk_value: i32,
}
impl Stat {
    pub fn new() -> Stat {
        Stat {
            base_value: 0,
            part_value: 0,
            perk_value: 0,
        }
    }
    pub fn val(&self) -> i32 {
        self.base_value + self.part_value
    }
    pub fn perk_val(&self) -> i32 {
        self.base_value + self.part_value + self.perk_value
    }
    #[cfg(target_arch = "wasm32")]
    pub fn to_js(&self, _hash: u32) -> JsStat {
        JsStat {
            stat_hash: _hash,
            base_value: self.base_value,
            part_value: self.part_value,
            perk_value: self.perk_value,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct FiringConfig {
    pub burst_delay: f64,
    pub burst_duration: f64,
    pub burst_size: i32,
    pub one_ammo_burst: bool,
    pub is_charge: bool,
    pub is_explosive: bool,
}

#[derive(Debug, Clone)]
pub struct Weapon {
    //ideally entirely interfaced with through funcs when acting mutably
    pub is_pvp: bool,

    pub perks: HashMap<u32, Perk>,
    pub stats: HashMap<u32, Stat>,
    pub damage_mods: DamageMods,
    pub firing_data: FiringConfig,
    pub id: u32,

    pub range_formula: RangeFormula,
    pub ammo_formula: AmmoFormula,
    pub handling_formula: HandlingFormula,
    pub reload_formula: ReloadFormula,

    pub base_damage: f64,
    pub base_crit_mult: f64,

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
    pub fn list_perks(&self) -> Vec<Perk> {
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
        self.perks = HashMap::new();
        self.stats = HashMap::new();
        self.id = 0;
        self.damage_mods = DamageMods::default();
        self.firing_data = FiringConfig::default();
        self.base_damage = 0.0;
        self.base_crit_mult = 0.0;
        self.range_formula = RangeFormula::default();
        self.ammo_formula = AmmoFormula::default();
        self.handling_formula = HandlingFormula::default();
        self.reload_formula = ReloadFormula::default();
    }

    pub fn static_calc_input(&self) -> CalculationInput {
        CalculationInput::construct_static(
            self.firing_data.clone(),
            self.stats.clone(),
            self.weapon_type.clone(),
            self.ammo_type.clone(),
        )
    }

    pub fn sparse_calc_input(&self, _total_shots_fired: &i32, _total_time: &f64) -> CalculationInput {
        CalculationInput::construct_pve_sparse(
            self.firing_data.clone(),
            self.stats.clone(),
            self.weapon_type.clone(),
            self.ammo_type.clone(),
            self.base_damage.clone(),
            self.base_crit_mult.clone(),
            self.calc_mag_size(None).mag_size,
            _total_shots_fired.clone(),
            _total_time.clone()
        )
    }
}
impl Default for Weapon {
    fn default() -> Weapon {
        Weapon {
            is_pvp: false,

            perks: HashMap::new(),
            stats: HashMap::new(),
            id: 0,
            damage_mods: DamageMods::default(),
            firing_data: FiringConfig::default(),

            base_damage: 0.0,
            base_crit_mult: 0.0,

            range_formula: RangeFormula::default(),
            ammo_formula: AmmoFormula::default(),
            handling_formula: HandlingFormula::default(),
            reload_formula: ReloadFormula::default(),

            weapon_type: WeaponType::UNKNOWN,
            weapon_slot: WeaponSlot::UNKNOWN,
            damage_type: DamageType::UNKNOWN,
            ammo_type: AmmoType::UNKNOWN,
        }
    }
}
impl Weapon {
    fn update_stats(&mut self) {
        let input = CalculationInput::construct_static(
            self.firing_data.clone(),
            self.stats.clone(),
            self.weapon_type,
            self.ammo_type,
        );
        let inter_var = get_perk_stats(self.list_perks(), input, false);
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
