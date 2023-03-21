use std::collections::HashMap;

use crate::{d2_enums::{AmmoType, DamageType, StatHashes, WeaponType}, StatMap};

use super::{
    clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExplosivePercentResponse, ExtraDamageResponse,
        FiringModifierResponse, HandlingModifierResponse, InventoryModifierResponse,
        MagazineModifierResponse, RangeModifierResponse, RefundResponse, ReloadModifierResponse,
    },
};

pub(super) fn dmr_high_impact_reserves(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    fn lerp(a: f64, b: f64, t: f64) -> f64 {
        a + (b - a) * t
    }
    let mut out_dmg_scale = 1.0;
    let base_value = if _pvp { 0.03 } else { 0.121 };
    let max_value = if _pvp { 0.06 } else { 0.256 };
    let threshold_divisor = if _is_enhanced { 4.0 / 3.0 } else { 2.0 };
    if _input.curr_mag <= _input.curr_mag / threshold_divisor {
        let t = 1.0 - (_input.curr_mag - 1.0) / ((_input.base_mag / threshold_divisor) - 1.0);
        if t > 0.0 {
            out_dmg_scale = lerp(base_value, max_value, t);
        }
    };
    DamageModifierResponse {
        impact_dmg_scale: out_dmg_scale,
        explosive_dmg_scale: out_dmg_scale,
        crit_scale: 1.0,
    }
}

pub(super) fn hmr_threat_detector(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HandlingModifierResponse {
    let val = clamp(_value, 0, 2) as i32;
    let time_scale = 0.75_f64.powi(val);
    HandlingModifierResponse {
        handling_stat_add: 0,
        handling_swap_scale: time_scale,
        handling_ads_scale: time_scale,
    }
}

pub(super) fn rsmr_threat_detector(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    let mut reload = 0;
    if _value == 1 {
        reload = 15;
    } else if _value == 2 {
        reload = 55;
    };
    ReloadModifierResponse {
        reload_stat_add: reload,
        reload_time_scale: 1.0,
    }
}

pub(super) fn sbr_threat_detector(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> StatMap {
    let mut stability = 0;
    let mut reload = 0;
    if _value == 1 {
        stability = 15;
        reload = 15;
    } else if _value == 2 {
        stability = 40;
        reload = 55;
    };
    let mut out = HashMap::new();
    out.insert(StatHashes::STABILITY.into(), stability);
    out.insert(StatHashes::RELOAD.into(), reload);
    out
}

pub(super) fn mmr_ambitious_assassin(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> MagazineModifierResponse {
    let val = clamp(_value, 0, 15) as f64;
    if _input.total_shots_fired == 0.0 {
        let mut mag_mult = 1.0;
        if *_input.ammo_type == AmmoType::PRIMARY {
            mag_mult += 0.2 * val;
        } else {
            mag_mult += 0.1 * val;
        };
        return MagazineModifierResponse {
            magazine_stat_add: 0,
            magazine_scale: clamp(mag_mult, 1.0, 2.5),
            magazine_add: 0.0,
        };
    };
    MagazineModifierResponse {
        magazine_stat_add: 0,
        magazine_scale: 1.0,
        magazine_add: 0.0,
    }
}

pub(super) fn dmr_box_breathing(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    if _input.total_shots_fired == 0.0 && _value > 0 {
        let mut crit_mult = (_input.base_crit_mult + 1.0) / _input.base_crit_mult;
        if *_input.weapon_type == WeaponType::SCOUTRIFLE {
            crit_mult *= 0.95;
        }
        return DamageModifierResponse {
            impact_dmg_scale: 1.0,
            explosive_dmg_scale: 1.0,
            crit_scale: crit_mult,
        };
    };
    DamageModifierResponse::new()
}

pub(super) fn fmr_desperado(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> FiringModifierResponse {
    let mut delay_mult = 1.0;
    let duration = if _is_enhanced { 7.0 } else { 6.0 };
    if _input.time_total < duration && _value > 0 {
        delay_mult = 0.7;
    };
    FiringModifierResponse {
        burst_delay_scale: delay_mult,
        ..Default::default()
    }
}

pub(super) fn dmr_explosive_payload(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    if _pvp {
        DamageModifierResponse::new()
    } else {
        DamageModifierResponse {
            impact_dmg_scale: 1.0,
            explosive_dmg_scale: 1.3,
            crit_scale: 1.0,
        }
    }
}

pub(super) fn epr_explosive_payload(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ExplosivePercentResponse {
    ExplosivePercentResponse {
        percent: 0.5,
        delyed: 0.0,
        retain_base_total: true,
    }
}

pub(super) fn dmr_timed_payload(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    if _pvp {
        DamageModifierResponse::new()
    } else {
        // let damage_mult = ((1.0 / _input.base_crit_mult) * 0.15) + 1.0;
        DamageModifierResponse {
            impact_dmg_scale: 1.0,
            explosive_dmg_scale: 1.3,
            crit_scale: 1.0,
        }
    }
}

pub(super) fn epr_timed_payload(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ExplosivePercentResponse {
    ExplosivePercentResponse {
        percent: 0.5,
        delyed: 0.6,
        retain_base_total: true,
    }
}

pub(super) fn sbr_field_prep(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> StatMap {
    let mut out = HashMap::new();
    if _value > 0 {
        let reload = if _is_enhanced { 55 } else { 50 };
        out.insert(StatHashes::RELOAD.into(), reload);
    };
    let mut reserves = if _is_enhanced { 40 } else { 30 };
    if *_input.weapon_type == WeaponType::GRENADELAUNCHER {
        reserves -= 10;
    };
    out.insert(StatHashes::INVENTORY_SIZE.into(), reserves);
    out
}

pub(super) fn rsmr_field_prep(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    let mut reload = 0;
    let mut reload_mult = 1.0;
    if _value > 0 {
        reload = if _is_enhanced { 55 } else { 50 };
        reload_mult = if _is_enhanced { 0.77 } else { 0.8 };
    };
    ReloadModifierResponse {
        reload_stat_add: reload,
        reload_time_scale: reload_mult,
    }
}

pub(super) fn imr_field_prep(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> InventoryModifierResponse {
    InventoryModifierResponse {
        inv_stat_add: if _is_enhanced { 40 } else { 30 },
        ..Default::default()
    }
}

pub(super) fn sbr_firmly_planted(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> StatMap {
    let mut handling = if _is_enhanced { 35 } else { 30 };
    let mut stabiltiy = if _is_enhanced { 25 } else { 20 };
    if *_input.weapon_type == WeaponType::FUSIONRIFLE {
        handling = handling / 2;
        stabiltiy = stabiltiy / 2;
    };
    let mut out = HashMap::new();
    if _value > 0 {
        out.insert(StatHashes::HANDLING.into(), handling);
        out.insert(StatHashes::STABILITY.into(), stabiltiy);
    }
        out
}

pub(super) fn hmr_firmly_planted(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HandlingModifierResponse {
    let mut handling = if _is_enhanced { 35 } else { 30 };
    if *_input.weapon_type == WeaponType::FUSIONRIFLE {
        handling = handling / 2;
    };
    if _value > 0 {
        HandlingModifierResponse {
            handling_stat_add: handling,
            handling_ads_scale: 1.0,
            handling_swap_scale: 1.0,
        }
    } else {
        HandlingModifierResponse::default()
    }
}

pub(super) fn fmr_full_auto_trigger(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> FiringModifierResponse {
    let mut delay_mult = 1.0;
    if *_input.weapon_type == WeaponType::SHOTGUN {
        delay_mult = 0.91;
    };
    FiringModifierResponse {
        burst_delay_scale: delay_mult,
        burst_delay_add: 0.0,
        inner_burst_scale: 1.0,
        burst_size_add: 0.0,
    }
}

pub(super) fn rr_triple_tap(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> RefundResponse {
    RefundResponse {
        crit: true,
        requirement: 3,
        refund_mag: 1,
        refund_reserves: 0,
    }
}

pub(super) fn sbr_hip_fire_grip(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> StatMap {
    let mut out = HashMap::new();
    if _value > 0 {
        out.insert(StatHashes::AIM_ASSIST.into(), 15);
        out.insert(StatHashes::STABILITY.into(), 25);
    };
    out
}

pub(super) fn rmr_hip_fire_grip(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> RangeModifierResponse {
    let mut hf_range_scale = 1.2;
    if *_input.weapon_type == WeaponType::FUSIONRIFLE
        || *_input.weapon_type == WeaponType::SHOTGUN
        || _input.intrinsic_hash == 2770223582 //last word
    {
        hf_range_scale = 1.0;
    };
    RangeModifierResponse {
        range_stat_add: 0,
        range_all_scale: 1.0,
        range_hip_scale: hf_range_scale,
        range_zoom_scale: 1.0,
    }
}

pub(super) fn dmr_impact_casing(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    DamageModifierResponse {
        impact_dmg_scale: 1.1,
        explosive_dmg_scale: 1.0,
        crit_scale: 1.0,
    }
}

pub(super) fn sbr_moving_target(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> StatMap {
    let aim_assist = if _is_enhanced { 11 } else { 10 };
    let mut out = HashMap::new();
    out.insert(StatHashes::AIM_ASSIST.into(), aim_assist);
    out
}

pub(super) fn sbr_opening_shot(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> StatMap {
    let aim_assist = if _is_enhanced { 25 } else { 20 };
    let range = if _is_enhanced { 30 } else { 25 };
    let mut out = HashMap::new();
    if _value > 0 {
        out.insert(StatHashes::AIM_ASSIST.into(), aim_assist);
        out.insert(StatHashes::RANGE.into(), range);
    }
    out
}

pub(super) fn rmr_opening_shot(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> RangeModifierResponse {
    let mut range = if _is_enhanced { 30 } else { 25 };
    if _input.total_shots_fired != 0.0 || _value == 0 {
        range = 0;
    };
    RangeModifierResponse {
        range_stat_add: range,
        range_all_scale: 1.0,
        range_hip_scale: 1.0,
        range_zoom_scale: 1.0,
    }
}

pub(super) fn sbr_outlaw(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> StatMap {
    let mut out = HashMap::new();
    if _value > 0 {
        out.insert(StatHashes::RELOAD.into(), 70);
    }
    out
}

pub(super) fn rsmr_outlaw(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    let duration = if _is_enhanced { 7.0 } else { 6.0 };
    if _value > 0 && _input.time_total < duration {
        ReloadModifierResponse {
            reload_stat_add: 70,
            reload_time_scale: 0.9,
        }
    } else {
        ReloadModifierResponse::default()
    }
}

pub(super) fn rmr_range_finder(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> RangeModifierResponse {
    RangeModifierResponse {
        range_stat_add: 0,
        range_all_scale: 1.0,
        range_hip_scale: 1.0,
        range_zoom_scale: 1.1,
    }
}

pub(super) fn sbr_slide_shot(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> StatMap {
    let stability = if _is_enhanced { 35 } else { 30 };
    let range = if _is_enhanced { 25 } else { 20 };
    let mut out = HashMap::new();
    if _value > 0 {
        out.insert(StatHashes::STABILITY.into(), stability);
        out.insert(StatHashes::RANGE.into(), range);
    }
    out
}

pub(super) fn rmr_slide_shot(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> RangeModifierResponse {
    let range;
    if *_input.weapon_type == WeaponType::FUSIONRIFLE {
        range = 0; //only applies to first proj so like should do alot less
    } else if _value > 0 {
        range = if _is_enhanced { 25 } else { 20 }
    } else {
        range = 0;
    }
    RangeModifierResponse {
        range_stat_add: range,
        ..Default::default()
    }
}

pub(super) fn sbr_slide_ways(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> StatMap {
    let stability = if _is_enhanced { 25 } else { 20 };
    let handling = if _is_enhanced { 25 } else { 20 };
    let mut out = HashMap::new();
    if _value > 0 {
        out.insert(StatHashes::STABILITY.into(), stability);
        out.insert(StatHashes::HANDLING.into(), handling);
    }
    out
}

pub(super) fn hmr_slide_ways(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HandlingModifierResponse {
    let handling = if _value > 0 { 20 } else { 0 };
    HandlingModifierResponse {
        handling_stat_add: handling,
        ..Default::default()
    }
}

pub(super) fn hmr_snapshot(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HandlingModifierResponse {
    let mut ads_mult = 0.5;
    if *_input.ammo_type == AmmoType::SPECIAL {
        ads_mult = 0.8; //its 0.8 from my testing idk
    };
    HandlingModifierResponse {
        handling_ads_scale: ads_mult,
        ..Default::default()
    }
}

pub(super) fn sbr_tap_the_trigger(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> StatMap {
    let mut stability = if _is_enhanced { 44 } else { 40 };
    if *_input.weapon_type == WeaponType::FUSIONRIFLE {
        stability = stability / 4;
    }
    let mut out = HashMap::new();
    if _value > 0 {
        out.insert(StatHashes::STABILITY.into(), stability);
    }
    out
}

pub(super) fn dmr_rampage(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let val = clamp(_value, 0, 3);
    let mut damage_mult = 1.1_f64.powi(val as i32) - 1.0;
    let duration = if _is_enhanced { 5.0 } else { 4.0 };
    if _input.time_total > duration {
        damage_mult = 0.0;
    };
    if _input.perk_value_map.contains_key(&630329983) && !_pvp {
        //huckleberry
        damage_mult *= 2.0;
    }
    DamageModifierResponse {
        impact_dmg_scale: 1.0 + damage_mult,
        explosive_dmg_scale: 1.0 + damage_mult,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_kill_clip(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let mut damage_mult = if _value > 0 { 0.25 } else { 0.0 };
    let duration = if _is_enhanced { 5.0 } else { 4.0 };
    if _input.time_total > duration {
        damage_mult = 0.0;
    };
    DamageModifierResponse {
        impact_dmg_scale: 1.0 + damage_mult,
        explosive_dmg_scale: 1.0 + damage_mult,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_backup_plan(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let mut damage_mult = if _value > 0 { 0.2 } else { 0.0 };
    let duration = if _is_enhanced { 2.2 } else { 2.0 };
    if _input.time_total > duration {
        damage_mult = 0.0;
    };
    DamageModifierResponse {
        impact_dmg_scale: 1.0 - damage_mult,
        explosive_dmg_scale: 1.0 - damage_mult,
        crit_scale: 1.0,
    }
}

pub(super) fn fmr_backup_plan(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> FiringModifierResponse {
    let mut firing_mult = if _value > 0 { 0.7 } else { 1.0 };
    let duration = if _is_enhanced { 2.2 } else { 2.0 };
    if _input.time_total > duration {
        firing_mult = 0.0;
    };
    FiringModifierResponse {
        burst_delay_scale: firing_mult,
        ..Default::default()
    }
}

pub(super) fn hmr_backup_plan(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HandlingModifierResponse {
    let mut handling_add = if _value > 0 { 100 } else { 0 };
    let duration = if _is_enhanced { 2.2 } else { 2.0 };
    if _input.time_total > duration {
        handling_add = 0;
    };
    HandlingModifierResponse {
        handling_stat_add: handling_add,
        ..Default::default()
    }
}

pub(super) fn sbr_backup_plan(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> StatMap {
    let mut handling = if _value > 0 { 100 } else { 0 };
    let duration = if _is_enhanced { 2.2 } else { 2.0 };
    if _input.time_total > duration {
        handling = 0;
    };
    let mut out = HashMap::new();
    if _value > 0 {
        out.insert(StatHashes::HANDLING.into(), handling);
    }
    out
}

pub(super) fn edr_cluster_bomb(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ExtraDamageResponse {
    ExtraDamageResponse {
        additive_damage: 350.0 * 0.04,
        combatant_scale: true,
        crit_scale: false,
        increment_total_time: false,
        time_for_additive_damage: 0.8,
        times_to_hit: 6,
        weapon_scale: true,
        hit_at_same_time: true,
        is_dot: false,
    }
}

pub(super) fn dmr_disruption_break(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let mut damage_mult = if _value > 0 { 0.5 } else { 0.0 };
    let duration = if _is_enhanced { 5.0 } else { 4.0 };
    if _input.time_total > duration || *_input.damage_type != DamageType::KINETIC {
        damage_mult = 0.0;
    };
    DamageModifierResponse {
        impact_dmg_scale: 1.0 + damage_mult,
        explosive_dmg_scale: 1.0 + damage_mult,
        crit_scale: 1.0,
    }
}

pub(super) fn hmr_quickdraw(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HandlingModifierResponse {
    HandlingModifierResponse {
        handling_stat_add: 100,
        handling_swap_scale: 0.95,
        ..Default::default()
    }
}

pub(super) fn sbr_quickdraw(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> StatMap {
    let mut map = HashMap::new();
    map.insert(StatHashes::HANDLING.into(), 100);
    map
}

pub(super) fn hmr_pulse_monitor(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HandlingModifierResponse {
    if _value > 0 {
        HandlingModifierResponse {
            handling_stat_add: 50,
            handling_swap_scale: 0.95,
            ..Default::default()
        }
    } else {
        HandlingModifierResponse::default()
    }
}

pub(super) fn sbr_pulse_monitor(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) ->  StatMap {
    let mut map = HashMap::new();
    if _value > 0 {
        map.insert(StatHashes::HANDLING.into(), 50);
    }
    map
}

pub(super) fn sbr_underdog(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> StatMap {
    let mut map = HashMap::new();
    if _value > 0 {
        map.insert(StatHashes::RELOAD.into(), 100);
    }
    map
}

pub(super) fn rsmr_underdog(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    if _value > 0 {
        ReloadModifierResponse {
            reload_stat_add: 100,
            reload_time_scale: 0.9,
            ..Default::default()
        }
    } else {
        ReloadModifierResponse::default()
    }
}

pub(super) fn sbr_under_pressure(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> StatMap {
    let mut map = HashMap::new();
    let buff = if _is_enhanced { 35 } else { 30 };
    if _value > 0 {
        map.insert(StatHashes::STABILITY.into(), buff);
    }
    map
}