use std::collections::HashMap;

use crate::d2_enums::{StatHashes, WeaponType};

use super::{
    clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExtraDamageResponse, FiringModifierResponse,
        HandlingModifierResponse, MagazineModifierResponse, RangeModifierResponse, RefundResponse,
        ReloadModifierResponse, ReloadOverrideResponse,
    },
};

pub(super) fn ror_veist_stinger(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadOverrideResponse {
    let data = _cached_data.get("veist_stinger");
    let last_proc;
    if data.is_none() {
        last_proc = 0.0;
    } else {
        last_proc = *data.unwrap();
    };
    let time_since_last_proc = _input.time_total - last_proc;
    if time_since_last_proc >= 4.0 && _value < 99 {
        println!("{:?}", _cached_data);
        _cached_data.insert("veist_stinger".to_string(), _input.time_total);
        ReloadOverrideResponse {
            count_as_reload: false,
            reload_time: 0.0,
            ammo_to_reload: (_input.base_mag / 4.0).ceil() as i32,
            priority: 9,
            uses_ammo: true,
            valid: true,
        }
    } else if _value >= 99 {
        ReloadOverrideResponse {
            count_as_reload: false,
            reload_time: 0.0,
            ammo_to_reload: _input.base_mag as i32,
            priority: 9,
            uses_ammo: true,
            valid: true,
        }
    } else {
        ReloadOverrideResponse::invalid()
    }
}

pub(super) fn dmr_hakke_breache(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let damage_mult = if _value > 0 { 0.3 } else { 0.0 };
    DamageModifierResponse {
        impact_dmg_scale: 1.0 + damage_mult,
        explosive_dmg_scale: 1.0 + damage_mult,
        crit_scale: 1.0,
    }
}

pub(super) fn rmr_alacrity(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> RangeModifierResponse {
    let range_add = if _value > 0 { 20 } else { 0 };
    RangeModifierResponse {
        range_stat_add: range_add,
        ..Default::default()
    }
}

pub(super) fn rsmr_alacrity(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    let reload_add = if _value > 0 { 50 } else { 0 };
    ReloadModifierResponse {
        reload_stat_add: reload_add,
        ..Default::default()
    }
}

pub(super) fn sbr_alacrity(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut map = HashMap::new();
    let range = if _value > 0 { 20 } else { 0 };
    let reload = if _value > 0 { 50 } else { 0 };
    let stability = if _value > 0 { 20 } else { 0 };
    let aim_assit = if _value > 0 { 10 } else { 0 };
    map.insert(StatHashes::RANGE.to_u32(), range);
    map.insert(StatHashes::RELOAD.to_u32(), reload);
    map.insert(StatHashes::STABILITY.to_u32(), stability);
    map.insert(StatHashes::AIM_ASSIST.to_u32(), aim_assit);
    map
}

pub(super) fn sbr_ambush(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut map = HashMap::new();
    let range = if _is_enhanced { 30 } else { 20 };
    let handling = if _is_enhanced { 40 } else { 20 };
    if _input.time_total < 2.0 && _value > 0 {
        map.insert(StatHashes::RANGE.to_u32(), range);
        map.insert(StatHashes::HANDLING.to_u32(), handling);
    }
    map
}

pub(super) fn dmr_ambush(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let damage_mult = if _value > 0 { 0.095 } else { 0.0 };
    DamageModifierResponse {
        impact_dmg_scale: 1.0 + damage_mult,
        explosive_dmg_scale: 1.0 + damage_mult,
        crit_scale: 1.0,
    }
}

pub(super) fn hmr_hot_swap(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HandlingModifierResponse {
    let handling_add = if _is_enhanced { 60 } else { 30 };
    if _value > 0 {
        HandlingModifierResponse {
            handling_stat_add: handling_add,
            ..Default::default()
        }
    } else {
        HandlingModifierResponse::default()
    }
}

pub(super) fn rsmr_fluid_dynamics(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    let reload_add = if _is_enhanced { 35 } else { 30 };
    if _input.shots_fired_this_mag <= _input.base_mag / 2.0 {
        ReloadModifierResponse {
            reload_stat_add: reload_add,
            reload_time_scale: 1.0,
        }
    } else {
        ReloadModifierResponse::default()
    }
}

pub(super) fn sbr_fluid_dynamics(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut map = HashMap::new();
    let reload = if _is_enhanced { 35 } else { 30 };
    let stability = if _is_enhanced { 25 } else { 20 };
    if _input.shots_fired_this_mag <= _input.base_mag / 2.0 && _value > 0 {
        map.insert(StatHashes::RELOAD.to_u32(), reload);
        map.insert(StatHashes::STABILITY.to_u32(), stability);
    }
    map
}

pub(super) fn rsmr_quiet_moment(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    if _value > 0 {
        ReloadModifierResponse {
            reload_stat_add: 40,
            reload_time_scale: 0.95,
        }
    } else {
        ReloadModifierResponse::default()
    }
}

pub(super) fn sbr_quiet_moment(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut map = HashMap::new();
    if _value > 0 {
        map.insert(StatHashes::RELOAD.to_u32(), 40);
    }
    map
}

pub(super) fn rsmr_bitter_spite(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    let val = clamp(_value, 0, 5) as i32;
    ReloadModifierResponse {
        reload_stat_add: val * 10,
        ..Default::default()
    }
}

pub(super) fn sbr_bitter_spite(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut map = HashMap::new();
    let val = clamp(_value, 0, 5) as i32;
    map.insert(StatHashes::RELOAD.to_u32(), val * 10);
    map
}

pub(super) fn rmr_right_hook(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> RangeModifierResponse {
    let range_add = if _is_enhanced { 20 } else { 10 };
    if _value > 0 {
        RangeModifierResponse {
            range_stat_add: range_add,
            ..Default::default()
        }
    } else {
        RangeModifierResponse::default()
    }
}

pub(super) fn sbr_right_hook(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut map = HashMap::new();
    let stat_bump = if _is_enhanced { 20 } else { 10 };
    if _value > 0 {
        map.insert(StatHashes::AIM_ASSIST.to_u32(), stat_bump);
        map.insert(StatHashes::RANGE.to_u32(), stat_bump);
    }
    map
}

pub(super) fn hmr_search_party(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HandlingModifierResponse {
    HandlingModifierResponse {
        handling_ads_scale: 0.85,
        ..Default::default()
    }
}

pub(super) fn mmr_runneth_over(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> MagazineModifierResponse {
    let val = clamp(_value, 0, 5) as f64;
    MagazineModifierResponse {
        magazine_scale: val * 0.1,
        ..Default::default()
    }
}
