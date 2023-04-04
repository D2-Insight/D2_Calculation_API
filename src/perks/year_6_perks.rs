use std::collections::HashMap;

use crate::d2_enums::{StatHashes, WeaponType};

use super::{
    clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExtraDamageResponse, FiringModifierResponse,
        HandlingModifierResponse, RangeModifierResponse, RefundResponse, ReloadModifierResponse,
        ReloadOverrideResponse,
    },
};

pub(super) fn sbr_keep_away(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut map = HashMap::new();
    let mut range_bonus = 0;
    let mut reload_bonus = 0;
    if _value > 0 {
        range_bonus = 10;
        reload_bonus = 30;
    };
    map.insert(StatHashes::RANGE.into(), range_bonus);
    map.insert(StatHashes::RELOAD.into(), reload_bonus);
    map
}

pub(super) fn rmr_keep_away(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> RangeModifierResponse {
    let range_bonus = if _value > 0 { 10 } else { 0 };
    RangeModifierResponse {
        range_stat_add: range_bonus,
        ..Default::default()
    }
}

pub(super) fn rsmr_keep_away(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    let reload_bonus = if _value > 0 { 30 } else { 0 };
    ReloadModifierResponse {
        reload_stat_add: reload_bonus,
        ..Default::default()
    }
}

pub(super) fn sbr_field_tested(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut map = HashMap::new();
    let val = clamp(_value, 0, 5) as i32;
    map.insert(StatHashes::RANGE.into(), val * 5);
    map.insert(StatHashes::HANDLING.into(), val * 5);
    map.insert(StatHashes::RELOAD.into(), val * 5);
    map.insert(StatHashes::STABILITY.into(), val * 5);
    map
}

pub(super) fn hmr_field_tested(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HandlingModifierResponse {
    let val = clamp(_value, 0, 5) as i32;
    HandlingModifierResponse {
        stat_add: val * 5,
        ..Default::default()
    }
}

pub(super) fn rsmr_field_tested(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    let val = clamp(_value, 0, 5) as i32;
    ReloadModifierResponse {
        reload_stat_add: val * 5,
        ..Default::default()
    }
}

pub(super) fn rmr_field_tested(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> RangeModifierResponse {
    let val = clamp(_value, 0, 5) as i32;
    RangeModifierResponse {
        range_stat_add: val * 5,
        ..Default::default()
    }
}
