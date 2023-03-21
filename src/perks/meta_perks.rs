use std::collections::HashMap;

use crate::{
    d2_enums::{AmmoType, DamageType, StatHashes, WeaponType},
    weapons::Stat,
};

use super::{
    clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExplosivePercentResponse, ExtraDamageResponse,
        FiringModifierResponse, HandlingModifierResponse, InventoryModifierResponse,
        MagazineModifierResponse, RangeModifierResponse, RefundResponse, ReloadModifierResponse, FlinchModifierResponse,
    }, other_perks::{fmr_accelerated_coils, fmr_faster_string_t1},
};

pub(super) fn dmr_builtin(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let mut crit_scale = 1.0;
    let mut dmg_scale = 1.0;
    if *_input.weapon_type == WeaponType::LINEARFUSIONRIFLE && !_pvp {
        crit_scale *= 1.15;
    };
    if *_input.damage_type == DamageType::KINETIC && !_pvp {
        if _input.ammo_type == &AmmoType::PRIMARY {
            dmg_scale *= 1.1;
        } else if _input.ammo_type == &AmmoType::SPECIAL {
            dmg_scale *= 1.15;
        };
    };
    if *_input.perk_value_map.get(&_input.intrinsic_hash).unwrap_or(&0) > 1 && _input.intrinsic_hash < 1000 {
        let stat_bump_id: StatHashes = _input.perk_value_map.get(&_input.intrinsic_hash).unwrap().to_owned().into();
        if stat_bump_id == StatHashes::CHARGE_TIME && _input.weapon_type == &WeaponType::FUSIONRIFLE {
            dmg_scale *= dmr_chargetime_mw(_input, _value, _is_enhanced, _pvp, _cached_data).impact_dmg_scale;
        }
    }
    DamageModifierResponse {
        crit_scale,
        impact_dmg_scale: dmg_scale,
        explosive_dmg_scale: dmg_scale,
    }
}

pub(super) fn fmr_builtin(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> FiringModifierResponse {
    let mut delay_add = 0.0;
    if *_input.perk_value_map.get(&_input.intrinsic_hash).unwrap_or(&0) > 1 && _input.intrinsic_hash < 1000 {
        let stat_bump_id: StatHashes = _input.perk_value_map.get(&_input.intrinsic_hash).unwrap().to_owned().into();
        if stat_bump_id == StatHashes::CHARGE_TIME {
            delay_add += fmr_accelerated_coils(_input, _value, _is_enhanced, _pvp, _cached_data).burst_delay_add;
        }
        if stat_bump_id == StatHashes::DRAW_TIME && _input.weapon_type == &WeaponType::BOW {
            delay_add += fmr_faster_string_t1(_input, _value, _is_enhanced, _pvp, _cached_data).burst_delay_add;
        }
    }
    FiringModifierResponse {
        burst_delay_add: delay_add,
        ..Default::default()
    }
}


pub(super) fn epr_builtin(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ExplosivePercentResponse {
    if *_input.weapon_type == WeaponType::GRENADELAUNCHER {
        let blast_radius_struct = _input.stats.get(&StatHashes::BLAST_RADIUS.into());
        let blast_radius;
        if blast_radius_struct.is_none() {
            blast_radius = 0;
        } else {
            blast_radius = blast_radius_struct.unwrap().val();
        };
        if _input.ammo_type == &AmmoType::SPECIAL {
            return ExplosivePercentResponse {
                percent: 0.5 + 0.0025 * blast_radius as f64,
                delyed: 0.0,
                retain_base_total: true,
            };
        } else if _input.ammo_type == &AmmoType::HEAVY {
            return ExplosivePercentResponse {
                percent: 0.7 + 0.00175 * blast_radius as f64,
                delyed: 0.0,
                retain_base_total: true,
            };
        };
    }
    if *_input.weapon_type == WeaponType::ROCKET && _input.intrinsic_hash < 1000
    //ensures not exotic
    {
        return ExplosivePercentResponse {
            percent: 0.28,
            delyed: 0.0,
            retain_base_total: true,
        };
    }
    ExplosivePercentResponse {
        percent: 0.0,
        delyed: 0.0,
        retain_base_total: true,
    }
}

pub(super) fn hmr_dexterity_mods(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HandlingModifierResponse {
    HandlingModifierResponse {
        handling_stat_add: 0,
        handling_ads_scale: if _value > 0 { 0.85 - clamp(_value, 1, 3) as f64 * 0.05 } else { 1.0 },
        handling_swap_scale: 1.0,
    }
}

pub(super) fn hmr_targeting_mods(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HandlingModifierResponse {
    HandlingModifierResponse {
        handling_stat_add: 0,
        handling_ads_scale: if _value > 0 { 0.75 } else { 1.0 },
        handling_swap_scale: 1.0,
    }
}

pub(super) fn sbr_targeting_mods(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    if _value == 1 {
        stats.insert(StatHashes::AIM_ASSIST.into(), 10);
    } else if _value > 1 {
        stats.insert(StatHashes::AIM_ASSIST.into(), 15);
    };
    stats
}

pub(super) fn imr_reserve_mods(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> InventoryModifierResponse {
    let mut inv_buff = if _value > 0 { 20 } else { 0 };
    if _value == 2 {
        inv_buff += 15;
    }
    if _value > 3 {
        inv_buff += 20;
    }
    InventoryModifierResponse {
        inv_stat_add: inv_buff,
        inv_scale: 1.0,
        inv_add: 0.0,
    }
}

pub(super) fn sbr_reserve_mods(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut inv_buff = if _value > 0 { 20 } else { 0 };
    if _value == 2 {
        inv_buff += 15;
    }
    if _value > 3 {
        inv_buff += 20;
    }
    let mut stats = HashMap::new();
    stats.insert(StatHashes::INVENTORY_SIZE.into(), inv_buff);
    stats
}

pub(super) fn rsmr_loader_mods(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    if _value > 0 {
        let mut reload_stat_buff = 10;
        if _value > 1 {
            reload_stat_buff += 5;
        };
        if _value > 2 {
            reload_stat_buff += 5;
        };
        return ReloadModifierResponse {
            reload_stat_add: reload_stat_buff,
            reload_time_scale: 0.85,
        };
    } else {
        return ReloadModifierResponse::default();
    };
}

pub(super) fn sbr_loader_mods(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    if _value > 0 {
        let mut reload_stat_buff = 10;
        if _value > 1 {
            reload_stat_buff += 5;
        };
        if _value > 2 {
            reload_stat_buff += 5;
        };
        stats.insert(StatHashes::RELOAD.into(), reload_stat_buff);
    };
    stats
}

pub(super) fn flmr_unflinching_mod(
_input: &CalculationInput,
_value: u32,
_is_enhanced: bool,
_pvp: bool,
_cached_data: &mut HashMap<String, f64>,
) -> FlinchModifierResponse {
    if _value > 1 {
        FlinchModifierResponse {   
            flinch_scale: 0.7
        }
    } else if _value > 0 {
        FlinchModifierResponse {
            flinch_scale: 0.75
        }
    } else {
        FlinchModifierResponse::default()
    }
}

pub(super) fn sbr_rally_barricade(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    if _value > 0 {
        stats.insert(StatHashes::STABILITY.into(), 30);
        stats.insert(StatHashes::RELOAD.into(), 100);
    }
    stats
}

pub(super) fn flmr_rally_barricade(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
    ) -> FlinchModifierResponse {
        if _value > 0 {
            FlinchModifierResponse {
                flinch_scale: 0.5
            }
         } else {
            FlinchModifierResponse::default()
        }
    }

pub(super) fn rsmr_rally_barricade(
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
        }
    } else {
        ReloadModifierResponse::default()
    }
}


pub(super) fn rmr_rally_barricade(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> RangeModifierResponse {
    if _value > 0 {
        RangeModifierResponse {
            range_stat_add: 0,
            range_all_scale: 1.1,
            range_hip_scale: 1.0,
            range_zoom_scale: 1.0,
        } 
    } else {
        RangeModifierResponse::default()
    }
}

pub(super) fn dmr_chargetime_mw(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    fn down5(x: i32) -> f64 {
        (x as f64 -5.0) / x as f64
    }
    let damage_mod = match _input.intrinsic_hash {
        901 => down5(330), //high impact
        906 => down5(280),
        903 => down5(270),
        902 => down5(245), //rapid fire
        _ => 1.0
    };
    DamageModifierResponse {
        explosive_dmg_scale: damage_mod,
        impact_dmg_scale: damage_mod,
        ..Default::default()
    }
}