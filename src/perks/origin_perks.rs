use std::collections::HashMap;

use crate::d2_enums::{StatHashes, WeaponType};

use super::{
    clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExtraDamageResponse, FiringModifierResponse,
        FlinchModifierResponse, HandlingModifierResponse, MagazineModifierResponse,
        RangeModifierResponse, RefundResponse, ReloadModifierResponse, ReloadOverrideResponse,
    }, ModifierResponsInput,
};

pub(super) fn rr_veist_stinger(_input: ModifierResponsInput) -> RefundResponse {
    let data = _input.cached_data.get("veist_stinger");
    let last_proc;
    if data.is_none() {
        last_proc = 0.0;
    } else {
        last_proc = *data.unwrap();
    };
    let time_since_last_proc = _input.calc_data.time_total - last_proc;
    if time_since_last_proc >= 4.0 && _input.value > 0 {
        let max_refund = _input.calc_data.base_mag - _input.calc_data.curr_mag;
        let refund_amount = (_input.calc_data.base_mag / 4.0).ceil() as i32;
        if max_refund > 0.0 {
            _input.cached_data.insert("veist_stinger".to_string(), _input.calc_data.time_total);
            let final_refund_ammount = clamp(refund_amount, 0, max_refund as i32);
            return RefundResponse {
                requirement: 1,
                crit: false,
                refund_mag: refund_amount,
                refund_reserves: -final_refund_ammount,
            };
        } else {
            RefundResponse::default()
        }
    } else {
        RefundResponse::default()
    }
}

pub(super) fn dmr_hakke_breache(_input: ModifierResponsInput) -> DamageModifierResponse {
    let damage_mult = if _input.value > 0 { 0.3 } else { 0.0 };
    DamageModifierResponse {
        impact_dmg_scale: 1.0 + damage_mult,
        explosive_dmg_scale: 1.0 + damage_mult,
        crit_scale: 1.0,
    }
}

pub(super) fn rmr_alacrity(_input: ModifierResponsInput) -> RangeModifierResponse {
    let range_add = if _input.value > 0 { 20 } else { 0 };
    RangeModifierResponse {
        range_stat_add: range_add,
        ..Default::default()
    }
}

pub(super) fn rsmr_alacrity(_input: ModifierResponsInput) -> ReloadModifierResponse {
    let reload_add = if _input.value > 0 { 50 } else { 0 };
    ReloadModifierResponse {
        reload_stat_add: reload_add,
        ..Default::default()
    }
}

pub(super) fn sbr_alacrity(_input: ModifierResponsInput) -> HashMap<u32, i32> {
    let mut map = HashMap::new();
    let range = if _input.value > 0 { 20 } else { 0 };
    let reload = if _input.value > 0 { 50 } else { 0 };
    let stability = if _input.value > 0 { 20 } else { 0 };
    let aim_assit = if _input.value > 0 { 10 } else { 0 };
    map.insert(StatHashes::RANGE.into(), range);
    map.insert(StatHashes::RELOAD.into(), reload);
    map.insert(StatHashes::STABILITY.into(), stability);
    map.insert(StatHashes::AIM_ASSIST.into(), aim_assit);
    map
}

pub(super) fn sbr_ambush(_input: ModifierResponsInput) -> HashMap<u32, i32> {
    let mut map = HashMap::new();
    let range = if _input.is_enhanced { 30 } else { 20 };
    let handling = if _input.is_enhanced { 40 } else { 20 };
    if _input.calc_data.time_total < 2.0 && _input.value > 0 {
        map.insert(StatHashes::RANGE.into(), range);
        map.insert(StatHashes::HANDLING.into(), handling);
    }
    map
}

pub(super) fn dmr_ambush(_input: ModifierResponsInput) -> DamageModifierResponse {
    let damage_mult = if _input.value > 0 { 0.095 } else { 0.0 };
    DamageModifierResponse {
        impact_dmg_scale: 1.0 + damage_mult,
        explosive_dmg_scale: 1.0 + damage_mult,
        crit_scale: 1.0,
    }
}

pub(super) fn hmr_hot_swap(_input: ModifierResponsInput) -> HandlingModifierResponse {
    let handling_add = if _input.is_enhanced { 60 } else { 30 };
    if _input.value > 0 {
        HandlingModifierResponse {
            stat_add: handling_add,
            ..Default::default()
        }
    } else {
        HandlingModifierResponse::default()
    }
}

pub(super) fn rsmr_fluid_dynamics(_input: ModifierResponsInput) -> ReloadModifierResponse {
    let reload_add = if _input.is_enhanced { 35 } else { 30 };
    if _input.calc_data.shots_fired_this_mag <= _input.calc_data.base_mag / 2.0 {
        ReloadModifierResponse {
            reload_stat_add: reload_add,
            reload_time_scale: 1.0,
        }
    } else {
        ReloadModifierResponse::default()
    }
}

pub(super) fn sbr_fluid_dynamics(_input: ModifierResponsInput) -> HashMap<u32, i32> {
    let mut map = HashMap::new();
    let reload = if _input.is_enhanced { 35 } else { 30 };
    let stability = if _input.is_enhanced { 25 } else { 20 };
    if _input.calc_data.shots_fired_this_mag <= _input.calc_data.base_mag / 2.0 && _input.value > 0 {
        map.insert(StatHashes::RELOAD.into(), reload);
        map.insert(StatHashes::STABILITY.into(), stability);
    }
    map
}

pub(super) fn rsmr_quiet_moment(_input: ModifierResponsInput) -> ReloadModifierResponse {
    if _input.value > 0 {
        ReloadModifierResponse {
            reload_stat_add: 40,
            reload_time_scale: 0.95,
        }
    } else {
        ReloadModifierResponse::default()
    }
}

pub(super) fn sbr_quiet_moment(_input: ModifierResponsInput) -> HashMap<u32, i32> {
    let mut map = HashMap::new();
    if _input.value > 0 {
        map.insert(StatHashes::RELOAD.into(), 40);
    }
    map
}

pub(super) fn rsmr_bitter_spite(_input: ModifierResponsInput) -> ReloadModifierResponse {
    let val = clamp(_input.value, 0, 5) as i32;
    let mult = match val {
        0 => 1.0,
        1 => 0.97,
        2 => 0.96,
        3 => 0.95,
        4 => 0.92,
        5 => 0.90,
        _ => 0.90,
    };
    ReloadModifierResponse {
        reload_stat_add: val * 10,
        reload_time_scale: mult,
    }
}

pub(super) fn sbr_bitter_spite(_input: ModifierResponsInput) -> HashMap<u32, i32> {
    let mut map = HashMap::new();
    let val = clamp(_input.value, 0, 5) as i32;
    map.insert(StatHashes::RELOAD.into(), val * 10);
    map
}

pub(super) fn rmr_right_hook(_input: ModifierResponsInput) -> RangeModifierResponse {
    let range_add = if _input.is_enhanced { 20 } else { 10 };
    if _input.value > 0 {
        RangeModifierResponse {
            range_stat_add: range_add,
            ..Default::default()
        }
    } else {
        RangeModifierResponse::default()
    }
}

pub(super) fn sbr_right_hook(_input: ModifierResponsInput) -> HashMap<u32, i32> {
    let mut map = HashMap::new();
    let stat_bump = if _input.is_enhanced { 20 } else { 10 };
    if _input.value > 0 {
        map.insert(StatHashes::AIM_ASSIST.into(), stat_bump);
        map.insert(StatHashes::RANGE.into(), stat_bump);
    }
    map
}

pub(super) fn hmr_search_party(_input: ModifierResponsInput) -> HandlingModifierResponse {
    HandlingModifierResponse {
        ads_scale: 0.85,
        ..Default::default()
    }
}

pub(super) fn mmr_runneth_over(_input: ModifierResponsInput) -> MagazineModifierResponse {
    let val = clamp(_input.value, 0, 5) as f64;
    MagazineModifierResponse {
        magazine_scale: 1.0 + val * 0.1,
        ..Default::default()
    }
}

pub(super) fn sbr_tex_balanced_stock(_input: ModifierResponsInput) -> HashMap<u32, i32> {
    let mut map = HashMap::new();
    if _input.value > 0 {
        map.insert(StatHashes::HANDLING.into(), 20);
        map.insert(StatHashes::RELOAD.into(), 20);
    }
    map
}

pub(super) fn hmr_tex_balanced_stock(_input: ModifierResponsInput) -> HandlingModifierResponse {
    if _input.value > 0 {
        HandlingModifierResponse {
            stat_add: 50,
            ..Default::default()
        }
    } else {
        HandlingModifierResponse::default()
    }
}

pub(super) fn rsmr_tex_balanced_stock(_input: ModifierResponsInput) -> ReloadModifierResponse {
    if _input.value > 0 {
        ReloadModifierResponse {
            reload_stat_add: 20,
            reload_time_scale: 0.9,
            ..Default::default()
        }
    } else {
        ReloadModifierResponse::default()
    }
}

pub(super) fn sbr_suros_synergy(_input: ModifierResponsInput) -> HashMap<u32, i32> {
    let mut out = HashMap::new();
    if _input.value > 0 {
        out.insert(StatHashes::HANDLING.into(), 40);
    }
    out
}

pub(super) fn hmr_suros_synergy(_input: ModifierResponsInput) -> HandlingModifierResponse {
    if _input.value > 0 {
        HandlingModifierResponse {
            stat_add: 40,
            ..Default::default()
        }
    } else {
        HandlingModifierResponse::default()
    }
}

pub(super) fn flmr_suros_synergy(_input: ModifierResponsInput) -> FlinchModifierResponse {
    if _input.value > 0 {
        FlinchModifierResponse { flinch_scale: 0.80 }
    } else {
        FlinchModifierResponse::default()
    }
}
