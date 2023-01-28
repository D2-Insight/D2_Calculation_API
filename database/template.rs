use std::collections::HashMap;

use crate::{types::rs_types::{StatQuadraticFormula, RangeFormula, HandlingFormula, ReloadFormula, DamageMods, AmmoFormula}, d2_enums::{DamageType, AmmoType, WeaponType}, perks::Perk};

use super::{Weapon, FiringConfig};

const HANDLING_DATA: [HandlingFormula; {HANDLING_REPLACE_POINT_len}] = {HANDLING_REPLACE_POINT};
const RANGE_DATA:    [RangeFormula; {RANGE_REPLACE_POINT_len}] = {RANGE_REPLACE_POINT};
const RELOAD_DATA:   [ReloadFormula; {RELOAD_REPLACE_POINT_len}] = {RELOAD_REPLACE_POINT};
const SCALAR_DATA:   [DamageMods; {SCALAR_REPLACE_POINT_len}] = {SCALAR_REPLACE_POINT};
const FIRING_DATA:   [FiringConfig; {FIRING_REPLACE_POINT_len}] = {FIRING_REPLACE_POINT};
const AMMO_DATA:     [AmmoFormula; {AMMO_REPLACE_POINT_len}] = {AMMO_REPLACE_POINT};

const META_POINTERS: [(u8, usize); {META_REPLACE_POINT_len}] = {META_REPLACE_POINT};

fn get_weapon_paths() -> HashMap<u8, HashMap<&u32, &DataPointers>> {
    let weapon_paths:  [Vec<(u32, DataPointers)>; {PATH_REPLACE_POINT_len}] = {PATH_REPLACE_POINT};
    let mut meta_pointer_map = HashMap::new();
        let i = 0;
        while i < META_POINTERS.len() {
            let pair = META_POINTERS[i];
            let mut tmp_map = HashMap::new();
            for (k, v) in weapon_paths[pair.1].iter() {
                tmp_map.insert(*k, v.clone());
            }
            meta_pointer_map.insert(pair.0, tmp_map);
        }
    meta_pointer_map
}

#[derive(Debug, Clone)]
struct DataPointers {
    h: usize,
    r: usize,
    rl: usize,
    s: usize,
    f: usize,
    a: usize,
}

impl Weapon {
    pub fn generate_weapon(_hash: u32, _weapon_type_id: u8, _intrinsic_hash: u32, _ammo_type_id: u32, _damage_type_id: u32) -> Result<Weapon, ()> {
        
        let meta_pointer_map = get_weapon_paths();

        let intrinsic_map = meta_pointer_map.get(&_weapon_type_id);
        if intrinsic_map.is_none() {
            return Err(());
        };
        let intrinsic_map = intrinsic_map.unwrap();

        let data_pointer = intrinsic_map.get(&_intrinsic_hash);
        if data_pointer.is_none() {
            return Err(());
        };
        let data_pointer = data_pointer.unwrap().clone();


        let range_formula: RangeFormula = RANGE_DATA[data_pointer.r].clone();

        let handling_formula: HandlingFormula = HANDLING_DATA[data_pointer.h].clone();

        let reload_formula: ReloadFormula = RELOAD_DATA[data_pointer.rl].clone();

        let damage_mods: DamageMods = SCALAR_DATA[data_pointer.s].clone();

        let firing_data: FiringConfig = FIRING_DATA[data_pointer.f].clone();

        let ammo_formula: AmmoFormula = AMMO_DATA[data_pointer.a].clone();

        let weapon_type = WeaponType::from_u32(_weapon_type_id as u32);
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
            range_formula,
            ammo_type,
            damage_type,
            weapon_type,
        })
    }
}