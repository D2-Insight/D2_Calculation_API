use std::collections::HashMap;

use crate::d2_enums::{StatHashes, WeaponType};

use super::{
    clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExtraDamageResponse, FiringModifierResponse,
        HandlingModifierResponse, MagazineModifierResponse, RangeModifierResponse, RefundResponse,
        ReloadModifierResponse, ReloadOverideResponse,
    },
};

pub fn sbr_air_assault(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    let ae_per_stack = if _is_enhanced { 35 } else { 20 };
    let ae = ae_per_stack * _value as i32;
    stats.insert(StatHashes::AIRBORNE.to_u32(), ae);
    stats
}

pub fn fmr_archers_tempo(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> FiringModifierResponse {
    FiringModifierResponse {
        burst_delay_scale: 0.75,
        burst_delay_add: 0.0,
        burst_duration_scale: 1.0,
        burst_size_add: 0.0,
    }
}

pub fn dmr_explosive_head(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> DamageModifierResponse {
    if _pvp {
        DamageModifierResponse {
            dmg_scale: 1.0,
            crit_scale: 1.0,
        }
    } else {
        let damage_mult = ((1.0 / _input.base_crit_mult) * 0.15) + 1.0;
        DamageModifierResponse {
            dmg_scale: damage_mult,
            crit_scale: 1.0,
        }
    }
}

pub fn rsmr_feeding_frenzy(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> ReloadModifierResponse {
    let val = clamp(_value, 0, 5);
    let duration = 3.5;
    let mut reload_mult = 1.0;
    let mut reload = 0;
    if val == 1 {
        reload = 10;
        reload_mult = 1.0;
    } else if val == 2 {
        reload = 45;
        reload_mult = 0.9;
    } else if val == 3 {
        reload = 55;
        reload_mult = 0.88;
    } else if val == 4 {
        reload = 70;
        reload_mult = 0.85;
    } else if val == 5 {
        reload = 100;
        reload_mult = 0.8;
    };
    if _input.time_total > duration {
        reload = 0;
        reload_mult = 1.0;
    };
    ReloadModifierResponse {
        reload_stat_add: reload,
        reload_time_scale: reload_mult,
    }
}

pub fn sbr_feeding_frenzy(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    let val = clamp(_value, 0, 5);
    let duration = 3.5;
    let mut reload = 0;
    if val == 1 {
        reload = 10;
    } else if val == 2 {
        reload = 45;
    } else if val == 3 {
        reload = 55;
    } else if val == 4 {
        reload = 70;
    } else if val == 5 {
        reload = 100;
    };
    if _input.time_total > duration {
        reload = 0;
    };
    stats.insert(StatHashes::RELOAD.to_u32(), reload);
    stats
}

pub fn dmr_firing_line(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> DamageModifierResponse {
    let mut crit_mult = 1.0;
    if _value > 1 {
        crit_mult = 1.2;
    }
    DamageModifierResponse {
        dmg_scale: 1.0,
        crit_scale: crit_mult,
    }
}

pub fn rr_fourth_times(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> RefundResponse {
    RefundResponse {
        crit: true,
        requirement: 4,
        refund_mag: 2,
        refund_reserves: 0,
    }
}

pub fn dmr_killing_tally(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> DamageModifierResponse {
    let mut damage_mult = 0.1 * _value as f64;
    if _pvp {
        damage_mult *= 0.5;
    };
    if _input.num_reloads > 0.0 {
        damage_mult = 0.0;
    };
    DamageModifierResponse {
        dmg_scale: 1.0 + damage_mult,
        crit_scale: 1.0,
    }
}

pub fn mmr_overflow(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> MagazineModifierResponse {
    let mut mag_scale = if _value > 0 { 2.0 } else { 1.0 };
    if _input.total_shots_fired == 0.0 {
        mag_scale = 1.0;
    };
    MagazineModifierResponse {
        magazine_stat_add: 0,
        magazine_scale: mag_scale,
        magazine_add: 0.0,
    }
}

pub fn rsmr_rapid_hit(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> ReloadModifierResponse {
    let reload_mult;
    let reload;
    let values = vec![
        (0, 1.0),
        (5, 0.99),
        (30, 0.97),
        (35, 0.96),
        (45, 0.94),
        (60, 0.93),
    ];
    if _input.shots_fired_this_mag > 5.0 {
        reload = values[5].0;
        reload_mult = values[5].1;
    } else {
        reload = values[_input.shots_fired_this_mag as usize].0;
        reload_mult = values[_input.shots_fired_this_mag as usize].1;
    };
    ReloadModifierResponse {
        reload_stat_add: reload,
        reload_time_scale: reload_mult,
    }
}

pub fn dmr_resevoir_burst(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> DamageModifierResponse {
    let mut damage_mult = 1.0;
    if _input.curr_mag >= _input.base_mag {
        damage_mult = 1.25;
    };
    DamageModifierResponse {
        dmg_scale: damage_mult,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_surrounded(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> DamageModifierResponse {
    let mut damage_mult = 1.0;
    if _value > 0 {
        damage_mult = if *_input.weapon_type == WeaponType::SWORD {
            1.35
        } else {
            1.4
        };
        if _is_enhanced {
            damage_mult *= 1.05;
        };
    };
    DamageModifierResponse {
        dmg_scale: damage_mult,
        crit_scale: 1.0,
    }
}

pub(super) fn ror_demolitionist(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> ReloadOverideResponse {
    //todo implement system for cooldown
    let grenade_throw_time = 0.8;
    if _value == 1 {
        return ReloadOverideResponse {
            valid: true,
            reload_time: _input.handling_data.ready_time + grenade_throw_time,
            ammo_to_reload: _input.base_mag,
            priority: 0,
            increments_reload_count: false,
            uses_ammo: true,
        };
    }
    ReloadOverideResponse::invalid()
}
