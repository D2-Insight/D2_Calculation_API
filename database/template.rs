use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

use crate::{types::rs_types::{StatQuadraticFormula, RangeFormula, HandlingFormula, ReloadFormula, DamageMods, AmmoFormula}, d2_enums::{DamageType, AmmoType, WeaponType}, perks::Perk};

use super::{Weapon, FiringConfig};

const HANDLING_DATA: [&str; {HANDLING_REPLACE_POINT_len}] = {HANDLING_REPLACE_POINT};
const RANGE_DATA:    [&str; {RANGE_REPLACE_POINT_len}] = {RANGE_REPLACE_POINT};
const RELOAD_DATA:   [&str; {RELOAD_REPLACE_POINT_len}] = {RELOAD_REPLACE_POINT};
const SCALAR_DATA:   [&str; {SCALAR_REPLACE_POINT_len}] = {SCALAR_REPLACE_POINT};
const FIRING_DATA:   [&str; {FIRING_REPLACE_POINT_len}] = {FIRING_REPLACE_POINT};
const AMMO_DATA:     [&str; {AMMO_REPLACE_POINT_len}] = {AMMO_REPLACE_POINT};

const WEAPON_PATHS: &str = r#"{PATH_REPLACE_POINT}"#;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct InterRangeFormula {
    pub vpp_start: f64,
    pub vpp_end: f64,
    pub offset_start: f64,
    pub offset_end: f64,
    pub floor_percent: f64,
    pub fusion: bool,
}
impl Into<RangeFormula> for InterRangeFormula {
    fn into(self) -> RangeFormula {
        RangeFormula {
            start: StatQuadraticFormula{evpp:0.0, vpp: self.vpp_start, offset: self.offset_start},
            end: StatQuadraticFormula{evpp: 0.0, vpp: self.vpp_end, offset: self.offset_end},
            floor_percent: self.floor_percent,
            is_fusion: self.fusion
        }
    }
}

impl Weapon {
    pub fn generate_weapon(_hash: u32, _weapon_type_id: u32, _intrinsic_hash: u32, _ammo_type_id: u32, _damage_type_id: u32) -> Result<Weapon, ()> {
        let jdata: Value = serde_json::from_str(WEAPON_PATHS).unwrap();
        
        let formula_map = jdata[_weapon_type_id.to_string()][_intrinsic_hash.to_string()].clone();
        if formula_map == Value::Null {
            return Err(())
        };

        let range_formula_id = formula_map["R"].as_i64().unwrap_or(0) as usize;
        let range_formula_str = RANGE_DATA.get(range_formula_id).unwrap();
        let range_formula: InterRangeFormula = serde_json::from_str(range_formula_str).unwrap();

        let handling_formula_id = formula_map["H"].as_i64().unwrap_or(0) as usize;
        let handling_formula_str = RANGE_DATA.get(handling_formula_id).unwrap();
        let handling_formula: HandlingFormula = serde_json::from_str(handling_formula_str).unwrap();

        let reload_formula_id = formula_map["RL"].as_i64().unwrap_or(0) as usize;
        let reload_formula_str = RANGE_DATA.get(reload_formula_id).unwrap();
        let reload_formula: ReloadFormula = serde_json::from_str(reload_formula_str).unwrap();

        let damage_mods_id = formula_map["S"].as_i64().unwrap_or(0) as usize;
        let damage_mods_str = RANGE_DATA.get(damage_mods_id).unwrap();
        let damage_mods: DamageMods = serde_json::from_str(damage_mods_str).unwrap();

        let firing_data_id = formula_map["S"].as_i64().unwrap_or(0) as usize;
        let firing_data_str = RANGE_DATA.get(firing_data_id).unwrap();
        let firing_data: FiringConfig = serde_json::from_str(firing_data_str).unwrap();

        let ammo_formula_id = formula_map["S"].as_i64().unwrap_or(0) as usize;
        let ammo_formula_str = RANGE_DATA.get(ammo_formula_id).unwrap();
        let ammo_formula: AmmoFormula = serde_json::from_str(ammo_formula_str).unwrap();

        let weapon_type = WeaponType::from_u32(_weapon_type_id);
        let ammo_type = AmmoType::from_u32(_ammo_type_id);
        let damage_type = DamageType::from_u32(_damage_type_id);

        Ok(Weapon {
            is_pvp: false,
            hash: _hash,
            perks: HashMap::from([(_intrinsic_hash, Perk{stat_buffs:HashMap::new(), enhanced: false, value: 0, hash:_intrinsic_hash})]),
            stats: HashMap::new(),
            damage_mods,
            ammo_formula,
            firing_data,
            handling_formula,
            reload_formula,
            range_formula: range_formula.into(),
            ammo_type,
            damage_type,
            weapon_type,
        })
    }
}