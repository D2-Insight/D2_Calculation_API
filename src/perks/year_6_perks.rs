use std::collections::HashMap;

use crate::d2_enums::{AmmoType, DamageType, StatHashes, WeaponType, StatBump, BungieHash};

use super::{
    clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExplosivePercentResponse, ExtraDamageResponse,
        FiringModifierResponse, HandlingModifierResponse, InventoryModifierResponse,
        MagazineModifierResponse, RangeModifierResponse, RefundResponse, ReloadModifierResponse,
        VelocityModifierResponse,
    }, add_sbr, Perks, ModifierResponsInput,
};

pub(super) fn sbr_keep_away(_input: ModifierResponsInput) -> HashMap<u32, i32> {
    let mut map = HashMap::new();
    let mut range_bonus = 0;
    let mut reload_bonus = 0;
    if _input.value > 0 {
        range_bonus = 10;
        reload_bonus = 30;
    };
    map.insert(StatHashes::RANGE.into(), range_bonus);
    map.insert(StatHashes::RELOAD.into(), reload_bonus);
    map
}

pub(super) fn rmr_keep_away(_input: ModifierResponsInput) -> RangeModifierResponse {
    let range_bonus = if _input.value > 0 { 10 } else { 0 };
    RangeModifierResponse {
        range_stat_add: range_bonus,
        ..Default::default()
    }
}

pub(super) fn rsmr_keep_away(_input: ModifierResponsInput) -> ReloadModifierResponse {
    let reload_bonus = if _input.value > 0 { 30 } else { 0 };
    ReloadModifierResponse {
        reload_stat_add: reload_bonus,
        ..Default::default()
    }
}

pub(super) fn sbr_field_tested(_input: ModifierResponsInput) -> HashMap<u32, i32> {
    let mut map = HashMap::new();
    let val = clamp(_input.value, 0, 5) as i32;
    map.insert(StatHashes::RANGE.into(), val * 5);
    map.insert(StatHashes::HANDLING.into(), val * 5);
    map.insert(StatHashes::RELOAD.into(), val * 5);
    map.insert(StatHashes::STABILITY.into(), val * 5);
    map
}

pub(super) fn hmr_field_tested(_input: ModifierResponsInput) -> HandlingModifierResponse {
    let val = clamp(_input.value, 0, 5) as i32;
    HandlingModifierResponse {
        stat_add: val * 5,
        ..Default::default()
    }
}

pub(super) fn rsmr_field_tested(_input: ModifierResponsInput) -> ReloadModifierResponse {
    let val = clamp(_input.value, 0, 5) as i32;
    ReloadModifierResponse {
        reload_stat_add: val * 5,
        ..Default::default()
    }
}

pub(super) fn rmr_field_tested(_input: ModifierResponsInput) -> RangeModifierResponse {
    let val = clamp(_input.value, 0, 5) as i32;
    RangeModifierResponse {
        range_stat_add: val * 5,
        ..Default::default()
    }
}

pub(super) fn dmr_paracausal_affinity(_input: ModifierResponsInput) -> DamageModifierResponse {
    if _input.value > 0 {
        DamageModifierResponse {
            explosive_dmg_scale: 1.2,
            impact_dmg_scale: 1.2,
            ..Default::default()
        }
    } else {
        DamageModifierResponse::default()
    }
}
