use std::collections::HashMap;

use crate::{D2Enums::{StatHashes, AmmoType}, D2Enemy::EnemyType};

use super::lib::{
    CalculationInput, 
    DamageModifierResponse,
    ExtraDamageResponse,
    FiringModifierResponse,
    ReloadModifierResponse,
    RangeModifierResponse,
    HandlingModifierResponse,
    RefundResponse,
    MagazineModifierResponse
};


pub fn mmr_clown_cartridge(_input: CalculationInput, _value: i32, _is_enhanced: bool, _pvp: bool) -> MagazineModifierResponse {
    MagazineModifierResponse {
        magazine_add: 0.0,
        magazine_scale: 1.5,
        magazine_stat_add: 0,
    }
}

pub fn sbr_elemental_capacitor(_input: CalculationInput, _value: i32, _is_enhanced: bool, _pvp: bool) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    if _value == 1{
        stats.insert(StatHashes::STABILITY.to_u32(), 20);
    } else if _value == 2 {
        stats.insert(StatHashes::RELOAD.to_u32(), 50);
    } else if _value == 3 {
        stats.insert(StatHashes::HANDLING.to_u32(), 50);
    } else if _value == 4 {
        stats.insert(StatHashes::RECOIL_DIR.to_u32(), 20);
    };
    stats
}

pub(super) fn hmr_elemental_capacitor(_input: CalculationInput, _value: i32, _is_enhanced: bool, _pvp: bool) -> HandlingModifierResponse {
    let mut handling = 0;
    if _value == 3 {
        handling = 50;
    };
    HandlingModifierResponse {
        handling_stat_add: handling,
        handling_ads_scale: 1.0,
        handling_swap_scale: 1.0,
    }
}

pub(super) fn rsmr_elemental_capacitor(_input: CalculationInput, _value: i32, _is_enhanced: bool, _pvp: bool) -> ReloadModifierResponse {
    let mut reload = 0;
    if _value == 2 {
        reload = 50;
    };
    ReloadModifierResponse {
        reload_stat_add: reload,
        reload_time_scale: 1.0,
    }
}

pub fn sbr_killng_wind(_input: CalculationInput, _value: i32, _is_enhanced: bool, _pvp: bool) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    if _value > 0 {
        stats.insert(StatHashes::HANDLING.to_u32(), 40);
        stats.insert(StatHashes::RANGE.to_u32(), 20);
    };
    stats
}

pub fn rmr_killing_wind(_input: CalculationInput, _value: i32, _is_enhanced: bool, _pvp: bool) -> RangeModifierResponse {
    if _value > 0 {
        RangeModifierResponse {
            range_stat_add: 20,
            range_all_scale: 1.05,
            range_zoom_scale: 1.0,
            range_hip_scale: 1.0,
        }
    } else {
        RangeModifierResponse {
            range_stat_add: 0,
            range_all_scale: 1.0,
            range_zoom_scale: 1.0,
            range_hip_scale: 1.0,
        }
    }
}

pub fn dmr_lasting_impressions(_input: CalculationInput, _value: i32, _is_enhanced: bool, _pvp: bool) -> DamageModifierResponse {
    DamageModifierResponse {
        damage_scale: 1.2,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_vorpal(_input: CalculationInput, _value: i32, _is_enhanced: bool, _pvp: bool) -> DamageModifierResponse {
    let mut buff = 1.0;
    if _input.enemy_type == EnemyType::BOSS ||
        _input.enemy_type == EnemyType::MINIBOSS ||
        _input.enemy_type == EnemyType::CHAMPION ||
        _input.enemy_type == EnemyType::VEHICLE {
        if _input.ammo_type == AmmoType::PRIMARY {
            buff == 1.2;
        } else if _input.ammo_type == AmmoType::SPECIAL {
            buff == 1.15;
        } else if _input.ammo_type == AmmoType::HEAVY {
            buff == 1.1;
        }
    }
    DamageModifierResponse {
        damage_scale: buff,
        crit_scale: 1.0,
    }
}

