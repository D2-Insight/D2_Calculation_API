use std::collections::HashMap;

use crate::{
    d2_enums::{DamageType, StatHashes, WeaponType},
    enemies::EnemyType,
};

use super::{
    clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExtraDamageResponse, FiringModifierResponse,
        HandlingModifierResponse, InventoryModifierResponse, MagazineModifierResponse,
        RangeModifierResponse, RefundResponse, ReloadModifierResponse, ReloadOverrideResponse,
    },
};

pub(super) fn hmr_ophidian_aspects(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HandlingModifierResponse {
    HandlingModifierResponse {
        handling_stat_add: 35,
        handling_ads_scale: 1.0,
        handling_swap_scale: 1.0,
    }
}

pub(super) fn rsmr_ophidian_aspects(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    ReloadModifierResponse {
        reload_stat_add: 35,
        reload_time_scale: 1.0,
    }
}

pub(super) fn sbr_ophidian_aspects(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    stats.insert(StatHashes::HANDLING.to_u32(), 35);
    stats.insert(StatHashes::RELOAD.to_u32(), 35);
    stats.insert(StatHashes::AIRBORNE.to_u32(), 10);
    stats
}

pub(super) fn sbr_dragon_shadow(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    stats.insert(StatHashes::HANDLING.to_u32(), 100);
    stats.insert(StatHashes::RELOAD.to_u32(), 100);
    stats
}

pub(super) fn hmr_dragon_shadow(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HandlingModifierResponse {
    HandlingModifierResponse {
        handling_stat_add: 100,
        handling_ads_scale: 1.0,
        handling_swap_scale: 0.95,
    }
}

pub(super) fn rsmr_dragon_shadow(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    ReloadModifierResponse {
        reload_stat_add: 100,
        reload_time_scale: 1.0,
    }
}

pub(super) fn sbr_amplified(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    stats.insert(StatHashes::HANDLING.to_u32(), 40);
    stats
}

pub(super) fn hmr_amplified(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HandlingModifierResponse {
    HandlingModifierResponse {
        handling_stat_add: 40,
        handling_ads_scale: 1.0,
        handling_swap_scale: 0.95,
    }
}

pub(super) fn rsmr_frequency(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    //far far too lazy to do this properly
    ReloadModifierResponse {
        reload_stat_add: 100,
        reload_time_scale: 0.8,
    }
}

pub(super) fn rsmr_flow_state(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    //far far too lazy to do this properly
    ReloadModifierResponse {
        reload_stat_add: 55,
        reload_time_scale: 0.87,
    }
}

pub(super) fn sbr_tempering(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    if _value > 0 {
        stats.insert(StatHashes::AIRBORNE.to_u32(), 20);
    };
    stats
}

pub(super) fn sbr_on_your_mark(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    let val = clamp(_value, 0, 3) as i32;
    if _value > 0 {
        stats.insert(StatHashes::HANDLING.to_u32(), 20 * val);
        stats.insert(StatHashes::RELOAD.to_u32(), 20 * val);
    };
    stats
}

pub(super) fn hmr_on_your_mark(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HandlingModifierResponse {
    let val = clamp(_value, 0, 3) as i32;
    HandlingModifierResponse {
        handling_stat_add: 20 * val,
        handling_ads_scale: 1.0,
        handling_swap_scale: 1.0,
    }
}

pub(super) fn rsmr_on_your_mark(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    let val = clamp(_value, 0, 3) as i32;
    ReloadModifierResponse {
        reload_stat_add: 20 * val,
        reload_time_scale: 0.93,
    }
}

pub(super) fn sbr_heat_rises(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    if _value > 0 {
        stats.insert(StatHashes::AIRBORNE.to_u32(), 70);
    };
    stats
}

pub(super) fn sbr_hedrons(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    if _value > 0 {
        stats.insert(StatHashes::AIRBORNE.to_u32(), 20);
        stats.insert(StatHashes::AIM_ASSIST.to_u32(), 15);
        stats.insert(StatHashes::STABILITY.to_u32(), 30);
    };
    stats
}

pub(super) fn sbr_quick_charge(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    if *_input.weapon_type == WeaponType::FUSIONRIFLE
        || *_input.weapon_type == WeaponType::SHOTGUN
        || *_input.weapon_type == WeaponType::SIDEARM
    {
        stats.insert(StatHashes::HANDLING.to_u32(), 25);
    };
    stats
}

pub(super) fn dmr_boss_spec(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let damage_mult = if *_input.enemy_type == EnemyType::BOSS {
        1.077
    } else {
        1.0
    };
    DamageModifierResponse {
        dmg_scale: damage_mult,
        ..Default::default()
    }
}

pub(super) fn dmr_major_spec(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let damage_mult;
    if *_input.enemy_type == EnemyType::MINIBOSS
        || *_input.enemy_type == EnemyType::ELITE
        || *_input.enemy_type == EnemyType::CHAMPION
    {
        damage_mult = 1.077;
    } else {
        damage_mult = 1.0;
    };
    DamageModifierResponse {
        dmg_scale: damage_mult,
        ..Default::default()
    }
}

pub(super) fn dmr_big_ones_spec(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let damage_mult;
    if *_input.enemy_type == EnemyType::MINIBOSS
        || *_input.enemy_type == EnemyType::ELITE
        || *_input.enemy_type == EnemyType::CHAMPION
        || *_input.enemy_type == EnemyType::BOSS
    {
        damage_mult = 1.077;
    } else {
        damage_mult = 1.0;
    };
    DamageModifierResponse {
        dmg_scale: damage_mult,
        ..Default::default()
    }
}

pub(super) fn dmr_minor_spec(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let damage_mult = if *_input.enemy_type == EnemyType::MINOR {
        1.077
    } else {
        1.0
    };
    DamageModifierResponse {
        dmg_scale: damage_mult,
        ..Default::default()
    }
}

pub(super) fn dmr_taken_spec(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let damage_mult = if _value > 0 { 1.1 } else { 1.0 };
    DamageModifierResponse {
        dmg_scale: damage_mult,
        ..Default::default()
    }
}

pub(super) fn fmr_accelerated_coils(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> FiringModifierResponse {
    FiringModifierResponse {
        burst_delay_add: -0.040,
        ..Default::default()
    }
}

pub(super) fn fmr_liquid_coils(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> FiringModifierResponse {
    FiringModifierResponse {
        burst_delay_add: 0.040,
        ..Default::default()
    }
}

pub(super) fn dmr_liquid_coils(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    DamageModifierResponse {
        dmg_scale: 1.02,
        ..Default::default()
    }
}

pub(super) fn dmr_accelerated_coils(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    DamageModifierResponse {
        dmg_scale: 0.982,
        ..Default::default()
    }
}

pub(super) fn fmr_faster_string_t2(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> FiringModifierResponse {
    FiringModifierResponse {
        burst_delay_add: -0.072,
        ..Default::default()
    }
}

pub(super) fn fmr_faster_string_t1(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> FiringModifierResponse {
    FiringModifierResponse {
        burst_delay_add: -0.036,
        ..Default::default()
    }
}

pub(super) fn fmr_slower_string_t1(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> FiringModifierResponse {
    FiringModifierResponse {
        burst_delay_add: 0.036,
        ..Default::default()
    }
}

pub(super) fn fmr_slower_string_t2(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> FiringModifierResponse {
    FiringModifierResponse {
        burst_delay_add: 0.072,
        ..Default::default()
    }
}
