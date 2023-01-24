use std::collections::HashMap;

use crate::d2_enums::{StatHashes, WeaponType};

use super::{
    clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExtraDamageResponse, FiringModifierResponse,
        HandlingModifierResponse, MagazineModifierResponse, RangeModifierResponse, RefundResponse,
        ReloadModifierResponse,
    },
};

pub fn dmr_adagio(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> DamageModifierResponse {
    let mut duration = 0.0;
    if _value > 1 {
        duration = if _is_enhanced { 8.0 } else { 7.0 };
    }
    let mut dmg_boost = 0.3;
    if *_input.weapon_type == WeaponType::BOW || *_input.weapon_type == WeaponType::SHOTGUN {
        dmg_boost = 0.2;
    };
    if _input.time_total >= duration && _pvp == false {
        dmg_boost = 0.0;
    };
    DamageModifierResponse {
        dmg_scale: 1.0 + dmg_boost,
        crit_scale: 1.0,
    }
}

pub fn fmr_adagio(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> FiringModifierResponse {
    let duration = if _is_enhanced { 8.0 } else { 7.0 };
    let mut firing_slow = 1.2;
    if _input.time_total >= duration && _pvp == false {
        firing_slow = 1.0;
    };
    FiringModifierResponse {
        burst_delay_scale: firing_slow,
        burst_delay_add: 0.0,
        burst_duration_scale: 1.0,
        burst_size_add: 0.0,
    }
}

pub fn dmr_adrenaline_junkie(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> DamageModifierResponse {
    let val = clamp(_value, 0, 5);
    let duration = if _is_enhanced { 6.0 } else { 4.5 };
    let mut dmg_boost = 0.067 * val as f64;
    if _input.time_total > duration {
        dmg_boost = 0.0;
    };
    DamageModifierResponse {
        dmg_scale: 1.0 + dmg_boost,
        crit_scale: 1.0,
    }
}

pub fn sbr_adrenaline_junkie(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let duration = if _is_enhanced { 6.0 } else { 4.5 };
    let mut handling = 0;
    if _input.time_total <= duration {
        handling = 20;
    };
    let mut out = HashMap::new();
    out.insert(StatHashes::HANDLING.to_u32(), handling);
    out
}

pub fn hmr_adrenaline_junkie(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> HandlingModifierResponse {
    let handling = if _value > 0 { 20 } else { 0 };
    HandlingModifierResponse {
        handling_stat_add: handling,
        handling_swap_scale: 1.0,
        handling_ads_scale: 1.0,
    }
}

pub fn fmr_cornered(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> FiringModifierResponse {
    let mut delay_mult = 1.0;
    if _value > 0 {
        delay_mult = 0.85;
    };
    FiringModifierResponse {
        burst_delay_scale: delay_mult,
        burst_delay_add: 0.0,
        burst_duration_scale: 1.0,
        burst_size_add: 0.0,
    }
}

pub fn sbr_ensemble(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let handling = if _is_enhanced { 30 } else { 35 };
    let reload = if _is_enhanced { 40 } else { 45 };
    if _value > 0 {
        let mut out = HashMap::new();
        out.insert(StatHashes::HANDLING.to_u32(), handling);
        out.insert(StatHashes::RELOAD.to_u32(), reload);
        out
    } else {
        HashMap::new()
    }
}

pub fn hmr_ensemble(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> HandlingModifierResponse {
    let handling = if _is_enhanced { 30 } else { 35 };
    if _value > 0 {
        HandlingModifierResponse {
            handling_stat_add: handling,
            handling_swap_scale: 1.0,
            handling_ads_scale: 1.0,
        }
    } else {
        HandlingModifierResponse {
            handling_stat_add: 0,
            handling_swap_scale: 1.0,
            handling_ads_scale: 1.0,
        }
    }
}

pub fn rsmr_ensemble(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> ReloadModifierResponse {
    let reload = if _is_enhanced { 40 } else { 45 };
    if _value > 0 {
        ReloadModifierResponse {
            reload_stat_add: reload,
            reload_time_scale: 1.0,
        }
    } else {
        ReloadModifierResponse {
            reload_stat_add: 0,
            reload_time_scale: 1.0,
        }
    }
}

pub fn rsmr_frenzy(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> ReloadModifierResponse {
    let mut reload = 0;
    if _value > 0 {
        reload = 50;
    };
    if _input.time_total > 12.0 {
        reload = 50;
    };
    ReloadModifierResponse {
        reload_stat_add: reload,
        reload_time_scale: 1.0,
    }
}

pub fn hmr_frenzy(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> HandlingModifierResponse {
    let mut handling = 0;
    if _value > 0 {
        handling = 50;
    };
    if _input.time_total > 12.0 {
        handling = 50;
    };
    HandlingModifierResponse {
        handling_stat_add: handling,
        handling_swap_scale: 1.0,
        handling_ads_scale: 1.0,
    }
}

pub fn dmr_frenzy(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> DamageModifierResponse {
    let mut dmg = 0.0;
    if _value > 0 {
        dmg = 0.15;
    };
    if _input.time_total > 12.0 {
        dmg = 0.15;
    };
    DamageModifierResponse {
        dmg_scale: 1.0 + dmg,
        crit_scale: 1.0,
    }
}

pub fn sbr_frenzy(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut handling = 0;
    let mut reload = 0;
    if _value > 0 {
        handling = 50;
        reload = 50;
    };
    if _input.time_total > 12.0 {
        handling = 50;
        reload = 50;
    };
    let mut out = HashMap::new();
    out.insert(StatHashes::HANDLING.to_u32(), handling);
    out.insert(StatHashes::RELOAD.to_u32(), reload);
    out
}

pub fn rsmr_impulse_amplifier(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> ReloadModifierResponse {
    let reload = if _is_enhanced { 15 } else { 10 };
    let reload_mult = if _is_enhanced { 0.77 } else { 0.8 };
    ReloadModifierResponse {
        reload_stat_add: reload,
        reload_time_scale: reload_mult,
    }
}

pub fn sbr_perpetual_motion(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let val = clamp(_value, 0, 2);
    let mut stat_bump = 0;
    if val == 1 {
        stat_bump = 5;
    } else if val == 2 {
        stat_bump = 15;
    };
    let mut out = HashMap::new();
    out.insert(StatHashes::RELOAD.to_u32(), stat_bump);
    out.insert(StatHashes::HANDLING.to_u32(), stat_bump);
    out.insert(StatHashes::STABILITY.to_u32(), stat_bump);
    out
}

pub fn hmr_perpetual_motion(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> HandlingModifierResponse {
    let val = clamp(_value, 0, 2);
    let mut stat_bump = 0;
    if val == 1 {
        stat_bump = 5;
    } else if val == 2 {
        stat_bump = 15;
    };
    HandlingModifierResponse {
        handling_stat_add: stat_bump,
        handling_swap_scale: 1.0,
        handling_ads_scale: 1.0,
    }
}

pub fn rsmr_perpetual_motion(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> ReloadModifierResponse {
    let val = clamp(_value, 0, 2);
    let mut stat_bump = 0;
    if val == 1 {
        stat_bump = 5;
    } else if val == 2 {
        stat_bump = 15;
    };
    ReloadModifierResponse {
        reload_stat_add: stat_bump,
        reload_time_scale: 1.0,
    }
}

pub fn sbr_perfect_float(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut out = HashMap::new();
    if _value > 0 {
        out.insert(StatHashes::AIRBORNE.to_u32(), 30);
    };
    out
}

pub fn sbr_pugilist(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut out = HashMap::new();
    if _value > 0 {
        out.insert(StatHashes::HANDLING.to_u32(), 30);
    };
    out
}

pub fn hrm_pugilist(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> HandlingModifierResponse {
    let mut handling = 0;
    if _value > 0 {
        handling = 30;
    };
    HandlingModifierResponse {
        handling_stat_add: handling,
        handling_swap_scale: 1.0,
        handling_ads_scale: 1.0,
    }
}

pub fn mmr_reconstruction(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> MagazineModifierResponse {
    let mag_scale = if _value > 0 { 2.0 } else { 1.0 };
    MagazineModifierResponse {
        magazine_stat_add: 0,
        magazine_scale: mag_scale,
        magazine_add: 0.0,
    }
}
