use std::collections::HashMap;

use crate::d2_enums::{StatHashes, WeaponType};

use super::{
    clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExtraDamageResponse, FiringModifierResponse,
        HandlingModifierResponse, InventoryModifierResponse, MagazineModifierResponse,
        RangeModifierResponse, RefundResponse, ReloadModifierResponse, ReloadOverideResponse,
    },
};

pub(super) fn hmr_ophidian_aspects(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> HandlingModifierResponse {
    HandlingModifierResponse {
        handling_stat_add: 35,
        handling_ads_scale: 1.0,
        handling_swap_scale: 1.0,
    }
}

pub(super) fn rsmr_ophidian_aspects(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> ReloadModifierResponse {
    ReloadModifierResponse {
        reload_stat_add: 35,
        reload_time_scale: 1.0,
    }
}

pub(super) fn sbr_ophidian_aspects(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    stats.insert(StatHashes::HANDLING.to_u32(), 35);
    stats.insert(StatHashes::RELOAD.to_u32(), 35);
    stats.insert(StatHashes::AIRBORNE.to_u32(), 10);
    stats
}

pub(super) fn sbr_dragon_shadow(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    stats.insert(StatHashes::HANDLING.to_u32(), 100);
    stats.insert(StatHashes::RELOAD.to_u32(), 100);
    stats
}

pub(super) fn hmr_dragon_shadow(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> HandlingModifierResponse {
    HandlingModifierResponse {
        handling_stat_add: 100,
        handling_ads_scale: 1.0,
        handling_swap_scale: 0.95,
    }
}

pub(super) fn rsmr_dragon_shadow(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> ReloadModifierResponse {
    ReloadModifierResponse {
        reload_stat_add: 100,
        reload_time_scale: 1.0,
    }
}

pub(super) fn sbr_amplified(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    stats.insert(StatHashes::HANDLING.to_u32(), 40);
    stats
}

pub(super) fn hmr_amplified(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> HandlingModifierResponse {
    HandlingModifierResponse {
        handling_stat_add: 40,
        handling_ads_scale: 1.0,
        handling_swap_scale: 0.95,
    }
}

pub(super) fn rsmr_frequency(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> ReloadModifierResponse {
    //far far too lazy to do this properly
    ReloadModifierResponse {
        reload_stat_add: 100,
        reload_time_scale: 0.8,
    }
}

pub(super) fn rsmr_flow_state(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> ReloadModifierResponse {
    //far far too lazy to do this properly
    ReloadModifierResponse {
        reload_stat_add: 55,
        reload_time_scale: 0.87,
    }
}

pub(super) fn sbr_tempering(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    if _value > 0 {
        stats.insert(StatHashes::AIRBORNE.to_u32(), 20);
    };
    stats
}

pub(super) fn sbr_on_your_mark(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    let val = clamp(_value, 0, 3);
    if _value > 0 {
        stats.insert(StatHashes::HANDLING.to_u32(), 20 * val);
        stats.insert(StatHashes::RELOAD.to_u32(), 20 * val);
    };
    stats
}

pub(super) fn hmr_on_your_mark(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> HandlingModifierResponse {
    let val = clamp(_value, 0, 3);
    HandlingModifierResponse {
        handling_stat_add: 20 * val,
        handling_ads_scale: 1.0,
        handling_swap_scale: 1.0,
    }
}

pub(super) fn rsmr_on_your_mark(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> ReloadModifierResponse {
    let val = clamp(_value, 0, 3);
    ReloadModifierResponse {
        reload_stat_add: 20 * val,
        reload_time_scale: 0.93,
    }
}

pub(super) fn sbr_heat_rises(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    if _value > 0 {
        stats.insert(StatHashes::AIRBORNE.to_u32(), 70);
    };
    stats
}

pub(super) fn sbr_hedrons(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
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
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
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

pub(super) fn hmr_dexterity_mods(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> HandlingModifierResponse {
    HandlingModifierResponse {
        handling_stat_add: 0,
        handling_ads_scale: if _value > 0 { 0.8 } else { 1.0 },
        handling_swap_scale: 1.0,
    }
}

pub(super) fn hmr_targeting_mods(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> HandlingModifierResponse {
    HandlingModifierResponse {
        handling_stat_add: 0,
        handling_ads_scale: if _value > 0 { 0.75 } else { 1.0 },
        handling_swap_scale: 1.0,
    }
}

pub(super) fn sbr_targeting_mods(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    if _value == 1 {
        stats.insert(StatHashes::AIM_ASSIST.to_u32(), 10);
    } else if _value > 1 {
        stats.insert(StatHashes::AIM_ASSIST.to_u32(), 15);
    };
    stats
}

pub(super) fn imr_reserve_mods(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> InventoryModifierResponse {
    let mut inv_buff = 20;
    if *_input.weapon_type == WeaponType::MACHINEGUN {
        inv_buff = 10;
    } else if *_input.weapon_type == WeaponType::GLAIVE {
        inv_buff = 5;
    };
    if _value < 1 {
        inv_buff = 0;
    };
    InventoryModifierResponse {
        inv_stat_add: inv_buff,
        inv_scale: 1.0,
        inv_add: 0.0,
    }
}

pub(super) fn sbr_reserve_mods(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut inv_buff = 20;
    if *_input.weapon_type == WeaponType::MACHINEGUN {
        inv_buff = 10;
    } else if *_input.weapon_type == WeaponType::GLAIVE {
        inv_buff = 5;
    };
    if _value < 1 {
        inv_buff = 0;
    };
    let mut stats = HashMap::new();
    stats.insert(StatHashes::INVENTORY_SIZE.to_u32(), inv_buff);
    stats
}
