pub mod stat_calc;

use std::collections::HashMap;

use crate::js_types::{JsWeaponFormula, JsStat, JsDamageModifiers};
use crate::perks::{Perk, get_magazine_modifier, get_perk_stats, get_reserve_modifier, lib::CalculationInput};
use crate::d2_enums::{WeaponType, AmmoType, DamageType, StatHashes, WeaponSlot};

use wasm_bindgen::prelude::wasm_bindgen;

use self::stat_calc::calc_ammo;

#[derive(Debug, Clone)]
pub struct BuffPackage {
    pub pve: f64,
    pub minor: f64,
    pub elite: f64,
    pub miniboss: f64,
    pub boss: f64,
    pub vehicle: f64,
}
impl BuffPackage {
    pub fn new() -> BuffPackage {
        BuffPackage {
            pve: 1.0,
            minor: 1.0,
            elite: 1.0,
            miniboss: 1.0,
            boss: 1.0,
            vehicle: 1.0,
        }
    }
    pub fn from_js(_js_equiv: &JsDamageModifiers) -> BuffPackage {
        BuffPackage {
            pve: _js_equiv.global,
            minor: _js_equiv.minor,
            elite: _js_equiv.elite,
            miniboss: _js_equiv.miniboss,
            boss: _js_equiv.boss,
            vehicle: _js_equiv.vehicle,
        }
    }
}

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
    //ideally entirely interfaced with through funcs
    pub perks: HashMap<u32, Perk>,
    pub stats: HashMap<u32, Stat>,
    pub damage_mods: BuffPackage,
    pub firing_data: FiringConfig,
    pub id: u32,
    pub formulas: JsWeaponFormula,

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
        //TODO: make this a trait
        self.perks = HashMap::new();
        self.stats = HashMap::new();
        self.id = 0;
        self.damage_mods = BuffPackage::new();
        self.firing_data = FiringConfig::default();
        self.formulas = JsWeaponFormula::new();
    }

    pub fn static_calc_input(&self) -> CalculationInput {
        CalculationInput::construct_static(
            self.firing_data.clone(),
            self.stats.clone(),
            self.weapon_type,
            self.ammo_type,
        )
    }

    pub fn sparse_calc_input(&self, _total_shots_fired: i32) -> CalculationInput {
        CalculationInput::construct_pve_sparse(
            self.firing_data.clone(),
            self.stats.clone(),
            self.weapon_type.clone(),
            self.ammo_type.clone(),
            self.formulas.firing_data.damage.clone(),
            self.formulas.firing_data.crit_mult.clone(),
            self.magsize(false, _total_shots_fired),
            _total_shots_fired,
        )
    }

    //we need the total shots fired for dps calcs, easiest way around it.
    pub fn magsize(&self, use_perks: bool, _total_shots_fired: i32) -> i32 {
        let magazine_stat = self.stats.get(&StatHashes::MAGAZINE.to_u32());
        if magazine_stat.is_none() {
            return 0;
        }
        if use_perks {
            let input = self.sparse_calc_input(_total_shots_fired);
            let res_mod_details =
                get_magazine_modifier(self.list_perks(), input, false);
            let final_mag_size = magazine_stat.unwrap().val() + res_mod_details.magazine_stat_add;
            let mut val = calc_ammo(
                final_mag_size,
                50,
                self.formulas.ammo_data.clone()
            ).mag_size as f64;
            val *= res_mod_details.magazine_scale;
            val += res_mod_details.magazine_add;
            return val.ceil() as i32;
        } else {
            return calc_ammo(
                magazine_stat.unwrap().val(), 
                50, 
                self.formulas.ammo_data.clone()
            ).mag_size;
        }
    }

    pub fn reserves(&self, use_perks: bool) -> i32 {
        //TODO
        let res_mod_details =
            get_reserve_modifier(self.list_perks(), self.static_calc_input(), false);
        let mut val = 23.0;
        val *= res_mod_details.ammo_scale;
        val += res_mod_details.ammo_add;
        if val < 0.0 {
            val = 0.5;
        }
        val.ceil() as i32
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
            formulas: JsWeaponFormula::new(),

            base_damage: 0.0,
            base_crit_mult: 0.0,

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
