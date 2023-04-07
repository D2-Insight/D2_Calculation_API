use std::collections::{btree_map::Range, HashMap};

use serde::de::value;

use crate::{
    d2_enums::{DamageType, StatHashes, WeaponType},
    enemies::EnemyType,
};

use super::{
    clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExtraDamageResponse, FiringModifierResponse,
        FlinchModifierResponse, HandlingModifierResponse, InventoryModifierResponse,
        MagazineModifierResponse, RangeModifierResponse, RefundResponse, ReloadModifierResponse,
        ReloadOverrideResponse,
    }, ModifierResponsInput,
};

pub(super) fn rsmr_alloy_mag(_input: ModifierResponsInput) -> ReloadModifierResponse {
    if _input.value > 0 {
        ReloadModifierResponse {
            reload_stat_add: 0,
            reload_time_scale: 0.85,
        }
    } else {
        ReloadModifierResponse::default()
    }
}

pub(super) fn rsmr_rapid_fire(_input: ModifierResponsInput) -> ReloadModifierResponse {
    if _input.value > 0 {
        ReloadModifierResponse {
            reload_stat_add: 0,
            reload_time_scale: 0.85,
        }
    } else {
        ReloadModifierResponse::default()
    }
}

pub(super) fn hmr_swap_mag(_input: ModifierResponsInput) -> HandlingModifierResponse {
    HandlingModifierResponse {
        draw_scale: 0.9,
        stow_scale: 0.9,
        ..Default::default()
    }
}

pub(super) fn hmr_freehand_grip(_input: ModifierResponsInput) -> HandlingModifierResponse {
    HandlingModifierResponse {
        draw_scale: 0.95,
        ..Default::default()
    }
}

pub(super) fn hmr_ophidian_aspects(_input: ModifierResponsInput) -> HandlingModifierResponse {
    HandlingModifierResponse {
        stat_add: 35,
        ..Default::default()
    }
}

pub(super) fn rsmr_ophidian_aspects(_input: ModifierResponsInput) -> ReloadModifierResponse {
    ReloadModifierResponse {
        reload_stat_add: 35,
        reload_time_scale: 1.0,
    }
}

pub(super) fn sbr_ophidian_aspects(_input: ModifierResponsInput) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    stats.insert(StatHashes::HANDLING.into(), 35);
    stats.insert(StatHashes::RELOAD.into(), 35);
    stats.insert(StatHashes::AIRBORNE.into(), 10);
    stats
}

pub(super) fn sbr_dragon_shadow(_input: ModifierResponsInput) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    if _input.value >= 1 {
        stats.insert(StatHashes::HANDLING.into(), 100);
        stats.insert(StatHashes::RELOAD.into(), 100);
    }
    stats
}

pub(super) fn hmr_dragon_shadow(_input: ModifierResponsInput) -> HandlingModifierResponse {
    if _input.value >= 1 {
        HandlingModifierResponse {
            stat_add: 100,
            draw_scale: 0.95,
            stow_scale: 0.95,
            ..Default::default()
        }
    } else {
        HandlingModifierResponse::default()
    }
}

pub(super) fn rsmr_dragon_shadow(_input: ModifierResponsInput) -> ReloadModifierResponse {
    if _input.value >= 1 {
        ReloadModifierResponse {
            reload_stat_add: 100,
            reload_time_scale: 1.0,
        }
    } else {
        ReloadModifierResponse::default()
    }
}

pub(super) fn sbr_amplified(_input: ModifierResponsInput) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    stats.insert(StatHashes::HANDLING.into(), 40);
    stats
}

pub(super) fn hmr_amplified(_input: ModifierResponsInput) -> HandlingModifierResponse {
    HandlingModifierResponse {
        stat_add: 40,
        draw_scale: 0.95,
        stow_scale: 0.95,
        ..Default::default()
    }
}

pub(super) fn rsmr_frequency(_input: ModifierResponsInput) -> ReloadModifierResponse {
    //far far too lazy to do this properly
    if _input.value > 0 {
        ReloadModifierResponse {
            reload_stat_add: 100,
            reload_time_scale: 0.8,
        }
    } else {
        ReloadModifierResponse::default()
    }
}

pub(super) fn rsmr_flow_state(_input: ModifierResponsInput) -> ReloadModifierResponse {
    //far far too lazy to do this properly
    ReloadModifierResponse {
        reload_stat_add: 55,
        reload_time_scale: 0.87,
    }
}

pub(super) fn sbr_tempering(_input: ModifierResponsInput) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    if _input.value > 0 {
        stats.insert(StatHashes::AIRBORNE.into(), 20);
    };
    stats
}

pub(super) fn sbr_on_your_mark(_input: ModifierResponsInput) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    let val = clamp(_input.value, 0, 3) as i32;
    if _input.value > 0 {
        stats.insert(StatHashes::HANDLING.into(), 20 * val);
        stats.insert(StatHashes::RELOAD.into(), 20 * val);
    };
    stats
}

pub(super) fn hmr_on_your_mark(_input: ModifierResponsInput) -> HandlingModifierResponse {
    let val = clamp(_input.value, 0, 3) as i32;
    HandlingModifierResponse {
        stat_add: 20 * val,
        ..Default::default()
    }
}

pub(super) fn rsmr_on_your_mark(_input: ModifierResponsInput) -> ReloadModifierResponse {
    let val = clamp(_input.value, 0, 3) as i32;
    ReloadModifierResponse {
        reload_stat_add: 20 * val,
        reload_time_scale: 0.93,
    }
}

pub(super) fn sbr_heat_rises(_input: ModifierResponsInput) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    let mut buff = 20;
    if _input.value > 0 {
        buff += 50;
    };
    stats.insert(StatHashes::AIRBORNE.into(), buff);
    stats
}

pub(super) fn sbr_hedrons(_input: ModifierResponsInput) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    if _input.value > 0 {
        stats.insert(StatHashes::AIRBORNE.into(), 20);
        stats.insert(StatHashes::AIM_ASSIST.into(), 15);
        stats.insert(StatHashes::STABILITY.into(), 30);
    };
    stats
}

pub(super) fn dmr_boss_spec(_input: ModifierResponsInput) -> DamageModifierResponse {
    let damage_mult = if *_input.calc_data.enemy_type == EnemyType::BOSS {
        1.077
    } else {
        1.0
    };
    DamageModifierResponse {
        impact_dmg_scale: damage_mult,
        explosive_dmg_scale: damage_mult,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_major_spec(_input: ModifierResponsInput) -> DamageModifierResponse {
    let damage_mult;
    if *_input.calc_data.enemy_type == EnemyType::MINIBOSS
        || *_input.calc_data.enemy_type == EnemyType::ELITE
        || *_input.calc_data.enemy_type == EnemyType::CHAMPION
    {
        damage_mult = 1.077;
    } else {
        damage_mult = 1.0;
    };
    DamageModifierResponse {
        impact_dmg_scale: damage_mult,
        explosive_dmg_scale: damage_mult,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_big_ones_spec(_input: ModifierResponsInput) -> DamageModifierResponse {
    let damage_mult;
    if *_input.calc_data.enemy_type == EnemyType::MINIBOSS
        || *_input.calc_data.enemy_type == EnemyType::ELITE
        || *_input.calc_data.enemy_type == EnemyType::CHAMPION
        || *_input.calc_data.enemy_type == EnemyType::BOSS
    {
        damage_mult = 1.077;
    } else {
        damage_mult = 1.0;
    };
    DamageModifierResponse {
        impact_dmg_scale: damage_mult,
        explosive_dmg_scale: damage_mult,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_minor_spec(_input: ModifierResponsInput) -> DamageModifierResponse {
    let damage_mult = if *_input.calc_data.enemy_type == EnemyType::MINOR {
        1.077
    } else {
        1.0
    };
    DamageModifierResponse {
        impact_dmg_scale: damage_mult,
        explosive_dmg_scale: damage_mult,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_taken_spec(_input: ModifierResponsInput) -> DamageModifierResponse {
    let damage_mult = if _input.value > 0 && !_input.pvp { 1.1 } else { 1.0 };
    DamageModifierResponse {
        impact_dmg_scale: damage_mult,
        explosive_dmg_scale: damage_mult,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_spike_grenades(_input: ModifierResponsInput) -> DamageModifierResponse {
    DamageModifierResponse {
        impact_dmg_scale: 1.5,
        explosive_dmg_scale: 1.0,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_disorienting_grenades(_input: ModifierResponsInput) -> DamageModifierResponse {
    DamageModifierResponse {
        impact_dmg_scale: 0.75,
        explosive_dmg_scale: 0.75,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_full_choke(_input: ModifierResponsInput) -> DamageModifierResponse {
    if _input.calc_data.weapon_type == &WeaponType::SHOTGUN
        && _input.calc_data.base_crit_mult < 1.15
    {
        DamageModifierResponse {
            impact_dmg_scale: 1.0,
            explosive_dmg_scale: 1.0,
            crit_scale: 0.92,
        }
    } else {
        DamageModifierResponse::default()
    }
}

pub(super) fn fmr_accelerated_coils(_input: ModifierResponsInput) -> FiringModifierResponse {
    if _input.calc_data.weapon_type == &WeaponType::LINEARFUSIONRIFLE {
        return FiringModifierResponse {
            burst_delay_add: -0.033,
            ..Default::default()
        };
    }
    FiringModifierResponse {
        burst_delay_add: -0.040,
        ..Default::default()
    }
}

pub(super) fn fmr_liquid_coils(_input: ModifierResponsInput) -> FiringModifierResponse {
    if _input.calc_data.weapon_type == &WeaponType::LINEARFUSIONRIFLE {
        return FiringModifierResponse {
            burst_delay_add: 0.033,
            ..Default::default()
        };
    }
    FiringModifierResponse {
        burst_delay_add: 0.040,
        ..Default::default()
    }
}

pub(super) fn dmr_liquid_coils(_input: ModifierResponsInput) -> DamageModifierResponse {
    DamageModifierResponse {
        impact_dmg_scale: 1.02,
        explosive_dmg_scale: 1.02,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_accelerated_coils(_input: ModifierResponsInput) -> DamageModifierResponse {
    DamageModifierResponse {
        impact_dmg_scale: 0.982,
        explosive_dmg_scale: 0.982,
        crit_scale: 1.0,
    }
}

pub(super) fn fmr_faster_string_t2(_input: ModifierResponsInput) -> FiringModifierResponse {
    FiringModifierResponse {
        burst_delay_add: -2.0 / 30.0,
        ..Default::default()
    }
}

pub(super) fn fmr_faster_string_t1(_input: ModifierResponsInput) -> FiringModifierResponse {
    FiringModifierResponse {
        burst_delay_add: -1.0 / 30.0,
        ..Default::default()
    }
}

pub(super) fn fmr_slower_string_t1(_input: ModifierResponsInput) -> FiringModifierResponse {
    FiringModifierResponse {
        burst_delay_add: 1.0 / 30.0,
        ..Default::default()
    }
}

pub(super) fn fmr_slower_string_t2(_input: ModifierResponsInput) -> FiringModifierResponse {
    FiringModifierResponse {
        burst_delay_add: 2.0 / 30.0,
        ..Default::default()
    }
}

pub(super) fn fmr_assault_mag(_input: ModifierResponsInput) -> FiringModifierResponse {
    let hash = _input.calc_data.intrinsic_hash;
    let tick_amount = if hash == 904 {
        3.0
    } else if hash == 906 {
        2.0
    } else {
        1.0
    };
    if _input.calc_data.weapon_type == &WeaponType::SHOTGUN {
        FiringModifierResponse {
            burst_delay_add: -(tick_amount / 30.0),
            ..Default::default()
        }
    } else {
        FiringModifierResponse::default()
    }
}
