pub mod dps_calc;
pub mod stat_calc;
mod ttk_calc;
pub mod reserve_calc;

use std::collections::HashMap;

use crate::d2_enums::{AmmoType, DamageType, StatHashes, WeaponSlot, WeaponType};
use crate::enemies::Enemy;
use crate::perks::{
    get_magazine_modifier, get_perk_stats, get_reserve_modifier, lib::CalculationInput, Perk,
};

use crate::types::rs_types::{
    AmmoFormula, DamageMods, HandlingFormula, RangeFormula, ReloadFormula, DpsResponse,
};

//JavaScript
#[cfg(feature = "wasm")]
use crate::types::js_types::{JsWeapon};
//Python
#[cfg(feature = "python")]
use crate::types::py_types::PyWeapon;

use self::dps_calc::complex_dps_calc;


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
}
impl From<i32> for Stat {
    fn from(_val: i32) -> Self {
        Stat {
            base_value: _val,
            part_value: 0,
            perk_value: 0,
        }
    }
}

#[derive(Debug, Clone, Default, Copy)]
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
    pub hash: u32,

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
    pub fn change_perk_val(&mut self, _perk_hash: u32, _val: u32) {
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
        self.hash = 0;
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
            &self.firing_data,
            &self.stats,
            &self.weapon_type,
            &self.ammo_type,
        )
    }

    pub fn sparse_calc_input(&self, _total_shots_fired: i32, _total_time: f64) -> CalculationInput {
        let tmp = CalculationInput::construct_pve_sparse(
            &self.firing_data,
            &self.stats,
            &self.weapon_type,
            &self.ammo_type,
            &self.damage_type,
            self.base_damage,
            self.base_crit_mult,
            self.calc_ammo_sizes(None).mag_size,
            _total_shots_fired,
            _total_time,
        )
        .clone();
        tmp
    }
    fn update_stats(&mut self) {
        let input = CalculationInput::construct_static(
            &self.firing_data,
            &self.stats,
            &self.weapon_type,
            &self.ammo_type,
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
    pub fn calc_dps(&self, _enemy: Enemy, _pl_dmg_mult: f64) -> DpsResponse {
        complex_dps_calc(self.clone(), _enemy, _pl_dmg_mult)
    }
}
impl Default for Weapon {
    fn default() -> Weapon {
        Weapon {
            is_pvp: false,

            perks: HashMap::new(),
            stats: HashMap::new(),
            hash: 0,
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
#[cfg(feature = "python")]
impl From<PyWeapon> for Weapon {
    fn from(_py_weapon: PyWeapon) -> Weapon {
        let mut weapon = Weapon::default();
        weapon.hash = _py_weapon.hash;
        weapon.damage_mods = _py_weapon.damage_mods.into();
        weapon.base_damage = _py_weapon.formulas.firing_data.damage;
        weapon.base_crit_mult = _py_weapon.formulas.firing_data.crit_mult;
        weapon.firing_data = _py_weapon.formulas.firing_data.into();
        weapon.range_formula = _py_weapon.formulas.range_data.into();
        weapon.ammo_formula = _py_weapon.formulas.ammo_data.into();
        weapon.handling_formula = _py_weapon.formulas.handling_data.into();
        weapon.reload_formula = _py_weapon.formulas.reload_data.into();
        weapon.weapon_type = WeaponType::from_u32(_py_weapon.weapon_type);
        weapon.weapon_slot = WeaponSlot::from_u32(_py_weapon.weapon_slot);
        weapon.damage_type = DamageType::from_u32(_py_weapon.damage_type);
        weapon.ammo_type = AmmoType::from_u32(_py_weapon.ammo_type);
        for stat in _py_weapon.stats {
            weapon.stats.insert(stat.0, Stat::from(stat.1));
        };
        for perk in _py_weapon.perks {
            weapon.perks.insert(perk.0, perk.1.into());
        };
        weapon.perks.insert(0, Perk::default());
        weapon
    }
}
