use std::collections::HashMap;

use crate::{types::rs_types::{StatQuadraticFormula, RangeFormula, HandlingFormula, ReloadFormula, DamageMods, AmmoFormula}, d2_enums::{DamageType, AmmoType, WeaponType}, perks::{Perk, enhanced_handler::enhanced_check}};

use super::{Weapon, FiringData};

const HANDLING_DATA: [HandlingFormula; {HANDLING_REPLACE_POINT_len}] = {HANDLING_REPLACE_POINT};
const RANGE_DATA:    [RangeFormula; {RANGE_REPLACE_POINT_len}] = {RANGE_REPLACE_POINT};
const RELOAD_DATA:   [ReloadFormula; {RELOAD_REPLACE_POINT_len}] = {RELOAD_REPLACE_POINT};
const SCALAR_DATA:   [DamageMods; {SCALAR_REPLACE_POINT_len}] = {SCALAR_REPLACE_POINT};
const FIRING_DATA:   [FiringData; {FIRING_REPLACE_POINT_len}] = {FIRING_REPLACE_POINT};
const AMMO_DATA:     [AmmoFormula; {AMMO_REPLACE_POINT_len}] = {AMMO_REPLACE_POINT};

const META_POINTERS: [(u8, usize); {META_REPLACE_POINT_len}] = {META_REPLACE_POINT};


#[derive(Debug, Clone)]
struct DataPointers {
    h: usize,
    r: usize,
    rl: usize,
    s: usize,
    f: usize,
    a: usize,
}

fn get_data_pointers(_weapon_type_id: u8, _intrinsic_hash: u32,) -> Result<DataPointers,()> {
    let weapon_paths:  [Vec<(u32, DataPointers)>; {META_REPLACE_POINT_len}] = {PATH_REPLACE_POINT};
    let meta_pointer_map = HashMap::from(META_POINTERS);
    let intrinsic_pointer = meta_pointer_map.get(&_weapon_type_id).clone();
    if intrinsic_pointer.is_none() {
        return Err(());
    };
    let intrinsic_vec = &weapon_paths[*intrinsic_pointer.unwrap()];
    let mut intrinsic_map = HashMap::new();
    for (hash, pointer) in intrinsic_vec {
        intrinsic_map.insert(hash, pointer);
    };
    let intrinsic_pointer = intrinsic_map.get(&_intrinsic_hash);
    if intrinsic_pointer.is_none() {
        return Err(());
    };
    let intrinsic_pointer = intrinsic_pointer.unwrap().clone();
    Ok(intrinsic_pointer.clone())
}


impl Weapon {
    pub fn generate_weapon(
        _hash: u32,
        _weapon_type_id: u8,
        _intrinsic_hash: u32,
        _ammo_type_id: u32,
        _damage_type_id: u32,
    ) -> Result<Weapon, ()> {
        let data_pointer_result = get_data_pointers(_weapon_type_id, _intrinsic_hash);
        if data_pointer_result.is_err() {
            return Err(());
        }
        let data_pointer = data_pointer_result.unwrap();

        let range_formula: RangeFormula = RANGE_DATA[data_pointer.r].clone();

        let handling_formula: HandlingFormula = HANDLING_DATA[data_pointer.h].clone();

        let reload_formula: ReloadFormula = RELOAD_DATA[data_pointer.rl].clone();

        let damage_mods: DamageMods = SCALAR_DATA[data_pointer.s].clone();

        let firing_data: FiringData = FIRING_DATA[data_pointer.f].clone();

        let ammo_formula: AmmoFormula = AMMO_DATA[data_pointer.a].clone();

        let weapon_type = WeaponType::from(_weapon_type_id as u32);
        let ammo_type = AmmoType::from(_ammo_type_id);
        let damage_type = DamageType::from(_damage_type_id);
        let intrinsic_alias = enhanced_check(_intrinsic_hash).0;
        Ok(Weapon {
            is_pvp: false,
            intrinsic_hash: _intrinsic_hash,
            hash: _hash,
            perks: HashMap::from([
                (
                    intrinsic_alias,
                    Perk {
                        stat_buffs: HashMap::new(),
                        enhanced: false,
                        value: 0,
                        hash: intrinsic_alias,
                    },
                ),
                (
                    0,
                    Perk {
                        stat_buffs: HashMap::new(),
                        enhanced: false,
                        value: 0,
                        hash: 0,
                    },
                ),
            ]),
            stats: HashMap::new(),
            perk_value_map: HashMap::from([
                (
                    intrinsic_alias,
                    0
                ),
                (
                    0,
                    0
                ),
            ]),
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