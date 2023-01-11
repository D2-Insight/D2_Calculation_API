use std::collections::HashMap;

use crate::D2Enums::{StatHashes, WeaponType};

use super::{
    clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExtraDamageResponse, FiringModifierResponse,
        HandlingModifierResponse, RangeModifierResponse, RefundResponse, ReloadModifierResponse,
    },
};

pub fn fmr_cascade_point(
    _input: CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> FiringModifierResponse {
    let duration = if _is_enhanced { 3.0 } else { 2.5 };
    let mut delay_mult = 1.0;
    if _input.time_total < duration {
        if _input.weapon_type == WeaponType::MACHINEGUN
            || _input.weapon_type == WeaponType::SUBMACHINEGUN
        {
            delay_mult = 0.7;
        } else {
            delay_mult = 0.6;
        }
    }
    FiringModifierResponse {
        burst_delay_scale: delay_mult,
        burst_duration_scale: 1.0,
        burst_size_add: 0.0,
    }
}

pub fn sbr_encore(
    _input: CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> HashMap<u32, i32> {
    let mut map = HashMap::new();
    let val = clamp(_value, 0, 4);
    let stability_boost = 8 * val;
    let range_boost = 5 * val;
    map.insert(StatHashes::RANGE.to_u32(), range_boost);
    map.insert(StatHashes::STABILITY.to_u32(), stability_boost);
    map
}

pub fn dmr_focused_fury(
    _input: CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> DamageModifierResponse {
    let mut dmg_boost = 1.0;
    let mut shots_needed = 0.0;
    if _input.curr_firing_data.one_ammo_burst == false || _input.curr_firing_data.burst_size == 1 {
        shots_needed = _input.base_mag / 2.0;
    } else {
        shots_needed = (_input.base_mag * (_input.curr_firing_data.burst_size as f64)) / 2.0;
    }
    if _input.total_shots_hit >= shots_needed {
        dmg_boost = 1.2;
    }
    DamageModifierResponse {
        damage_scale: dmg_boost,
        crit_scale: 1.0,
    }
}

pub fn rmr_fragile_focus(
    _input: CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> RangeModifierResponse {
    let mut range_bonus = 0;
    if _value > 0 {
        range_bonus = 20;
    };
    RangeModifierResponse {
        range_stat_add: range_bonus,
        range_all_scale: 1.0,
        range_hip_scale: 1.0,
        range_zoom_scale: 1.0,
    }
}

pub fn sbr_fragile_focus(
    _input: CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> HashMap<u32, i32> {
    let mut map = HashMap::new();
    let mut range_bonus = 0;
    if _value > 0 {
        range_bonus = 20;
    };
    map.insert(StatHashes::RANGE.to_u32(), range_bonus);
    map
}

pub fn dmr_gutshot_straight(
    _input: CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> DamageModifierResponse {
    let mut dmg_boost = 1.0;
    let mut crit_mult = 1.0;
    let high_weapons = [
        WeaponType::AUTORIFLE,
        WeaponType::HANDCANNON,
        WeaponType::BOW,
    ];
    if high_weapons.contains(&_input.weapon_type) {
        dmg_boost = 1.2;
        crit_mult = _input.base_crit_mult * (1.0 / 1.2);
    } else {
        dmg_boost = 1.1;
        crit_mult = _input.base_crit_mult * (1.0 / 1.1);
    }
    DamageModifierResponse {
        damage_scale: dmg_boost,
        crit_scale: crit_mult,
    }
}

pub fn sbr_offhand_strike(
    _input: CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> HashMap<u32, i32> {
    let mut map = HashMap::new();
    let mut stability_boost = 0;
    if _value > 0 {
        stability_boost = 30;
    };
    map.insert(StatHashes::STABILITY.to_u32(), stability_boost);
    map
}

pub fn rmr_offhand_strike(
    _input: CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> RangeModifierResponse {
    let mut range_hip_mult = 0.0;
    if _value > 0 {
        range_hip_mult = 1.45;
    };
    RangeModifierResponse {
        range_stat_add: 0,
        range_all_scale: 1.0,
        range_hip_scale: range_hip_mult,
        range_zoom_scale: 1.0,
    }
}

pub fn hmr_slickdraw(
    _input: CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> HandlingModifierResponse {
    HandlingModifierResponse {
        handling_stat_add: 100,
        handling_ads_scale: 1.0,
        handling_swap_scale: 0.9,
    }
}

pub(super) fn sbr_stats_for_all(
    _input: CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> HashMap<u32, i32> {
    let mut out = HashMap::new();
    let mut stability_boost = 0;
    let mut range_boost = 0;
    let mut reload_boost = 0;
    let mut handling_boost = 0;
    if _value > 0 {
        stability_boost = 10;
        range_boost = 10;
        reload_boost = 35;
        handling_boost = 35;
    };
    out.insert(StatHashes::STABILITY.to_u32(), stability_boost);
    out.insert(StatHashes::RANGE.to_u32(), range_boost);
    out.insert(StatHashes::RELOAD.to_u32(), reload_boost);
    out.insert(StatHashes::HANDLING.to_u32(), handling_boost);
    out
}

pub(super) fn hmr_stats_for_all(
    _input: CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> HandlingModifierResponse {
    let mut handling_boost = 0;
    let duration = if _is_enhanced { 11.0 } else { 10.0 };
    if _value > 0 && _input.time_total < duration {
        handling_boost = 35;
    };
    HandlingModifierResponse {
        handling_stat_add: handling_boost,
        handling_ads_scale: 1.0,
        handling_swap_scale: 1.0,
    }
}

pub(super) fn rmr_stats_for_all(
    _input: CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> RangeModifierResponse {
    let mut range = 0;
    let mut range_mult = 1.0;
    if _value > 0 {
        range = 10;
        range_mult = 1.05;
    };
    RangeModifierResponse {
        range_stat_add: range,
        range_all_scale: range_mult,
        range_hip_scale: 1.0,
        range_zoom_scale: 1.0,
    }
}

pub(super) fn rsmr_stats_for_all(
    _input: CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> ReloadModifierResponse {
    let mut reload = 0;
    let mut reload_mult = 1.0;
    let duration = if _is_enhanced { 11.0 } else { 10.0 };
    if _value > 0 && _input.time_total < duration {
        reload = 35;
        reload_mult = 0.95;
    };
    ReloadModifierResponse {
        reload_stat_add: reload,
        reload_time_scale: reload_mult,
    }
}

pub(super) fn sbr_steady_hands(
    _input: CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> HashMap<u32, i32> {
    let mut map = HashMap::new();
    let mut handling = 0;
    if _value > 0 {
        handling = 100;
    };
    map.insert(StatHashes::HANDLING.to_u32(), handling);
    map
}

pub(super) fn hmr_steady_hands(
    _input: CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> HandlingModifierResponse {
    let mut handling_mult = 1.0;
    let mut handling = 0;
    let duration = if _is_enhanced { 9.0 } else { 8.5 };
    if _value > 0 && _input.time_total < duration {
        handling_mult = 0.825;
        handling = 100;
    };
    HandlingModifierResponse {
        handling_stat_add: handling,
        handling_ads_scale: 1.0,
        handling_swap_scale: handling_mult,
    }
}

pub(super) fn dmr_target_lock(
    _input: CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> DamageModifierResponse {
    let lerp_table = vec![
        (0.15, 0.166),
        (0.37, 0.23),
        (0.55, 0.28),
        (0.75, 0.34),
        (1.05, 0.4),
    ];
    let percent_through_mag = _input.shots_hit_this_mag as f64 / _input.base_mag as f64;
    let mut buff = 0.0_f64;
    if percent_through_mag > 1.05 {
        buff = 0.4;
    } else if percent_through_mag < 0.15 {
        buff = 0.0;
    } else {
        for i in 0..lerp_table.len() {
            if percent_through_mag < lerp_table[i].0 {
                buff = lerp_table[i - 1].1
                    + ((lerp_table[i].1 - lerp_table[i - 1].1)
                        * (percent_through_mag - lerp_table[i - 1].0)
                        / (lerp_table[i].0 - lerp_table[i - 1].0));
                break;
            }
        }
    }
    if _is_enhanced {
        buff *= 1.125;
    }
    DamageModifierResponse {
        damage_scale: buff + 1.0,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_over_under(
    _input: CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> DamageModifierResponse {
    let mut buff = 0.0_f64;
    if _input.has_overshield {
        buff = 0.2;
    }
    if _is_enhanced {
        buff *= 1.05;
    }
    DamageModifierResponse {
        damage_scale: buff + 1.0,
        crit_scale: 1.0,
    }
}

pub(super) fn sbr_well_rounded(
    _input: CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> HashMap<u32, i32> {
    let val = clamp(_value, 0, 2);
    let mut map = HashMap::new();
    let stat_base = if _is_enhanced { 12 } else { 10 };
    let stat_bump = stat_base * val;
    map.insert(StatHashes::STABILITY.to_u32(), stat_bump);
    map.insert(StatHashes::RANGE.to_u32(), stat_bump);
    map.insert(StatHashes::HANDLING.to_u32(), stat_bump);
    map
}

pub(super) fn hmr_well_rounded(
    _input: CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> HandlingModifierResponse {
    let val = clamp(_value, 0, 2);
    //due to ease of activation and upkeep will assume its always active
    // let mut duration = if _is_enhanced {9.0} else {8.5};
    let stat_base = if _is_enhanced { 12 } else { 10 };
    let handling = stat_base * val;
    HandlingModifierResponse {
        handling_stat_add: handling,
        handling_ads_scale: 1.0,
        handling_swap_scale: 1.0,
    }
}

pub(super) fn rmr_well_rounded(
    _input: CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> RangeModifierResponse {
    let val = clamp(_value, 0, 2);
    let stat_base = if _is_enhanced { 12 } else { 10 };
    let range = stat_base * val;
    RangeModifierResponse {
        range_stat_add: range,
        range_all_scale: 1.0,
        range_hip_scale: 1.0,
        range_zoom_scale: 1.0,
    }
}
