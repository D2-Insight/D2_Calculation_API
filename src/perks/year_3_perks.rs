use std::collections::HashMap;

use crate::{
    d2_enums::{AmmoType, StatHashes},
    enemies::EnemyType,
};

use super::{
    lib::{
        CalculationInput, DamageModifierResponse, ExtraDamageResponse, FiringModifierResponse,
        HandlingModifierResponse, MagazineModifierResponse, RangeModifierResponse, RefundResponse,
        ReloadModifierResponse,
    },
    ModifierResponsInput,
};

pub(super) fn mmr_clown_cartridge(_input: ModifierResponsInput) -> MagazineModifierResponse {
    MagazineModifierResponse {
        magazine_add: 0.0,
        magazine_scale: 1.5,
        magazine_stat_add: 0,
    }
}

pub(super) fn sbr_elemental_capacitor(_input: ModifierResponsInput) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    let ev = if _input.is_enhanced { 5 } else { 0 };
    if _input.value == 1 {
        stats.insert(StatHashes::STABILITY.into(), 20 + ev);
    } else if _input.value == 2 {
        stats.insert(StatHashes::RELOAD.into(), 50 + ev);
    } else if _input.value == 3 {
        stats.insert(StatHashes::HANDLING.into(), 50 + ev);
    } else if _input.value == 4 {
        stats.insert(StatHashes::RECOIL_DIR.into(), 20 + ev);
    } else if _input.value == 5 {
        stats.insert(StatHashes::AIRBORNE.into(), 20 + ev);
    };
    stats
}

pub(super) fn hmr_elemental_capacitor(_input: ModifierResponsInput) -> HandlingModifierResponse {
    let mut handling = 0;
    if _input.value == 3 {
        handling = if _input.is_enhanced { 55 } else { 50 };
    };
    HandlingModifierResponse {
        stat_add: handling,
        ..Default::default()
    }
}

pub(super) fn rsmr_elemental_capacitor(_input: ModifierResponsInput) -> ReloadModifierResponse {
    let mut reload = 0;
    if _input.value == 2 {
        reload = if _input.is_enhanced { 55 } else { 50 };
    };
    ReloadModifierResponse {
        reload_stat_add: reload,
        ..Default::default()
    }
}

pub(super) fn sbr_killing_wind(_input: ModifierResponsInput) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    if _input.value > 0 {
        stats.insert(StatHashes::HANDLING.into(), 40);
        stats.insert(StatHashes::RANGE.into(), 20);
    };
    stats
}

pub(super) fn rmr_killing_wind(_input: ModifierResponsInput) -> RangeModifierResponse {
    if _input.value > 0 {
        RangeModifierResponse {
            range_stat_add: 20,
            range_all_scale: 1.05,
            range_zoom_scale: 1.0,
            range_hip_scale: 1.0,
        }
    } else {
        RangeModifierResponse {
            range_stat_add: 0,
            range_all_scale: 1.0,
            range_zoom_scale: 1.0,
            range_hip_scale: 1.0,
        }
    }
}

pub(super) fn hmr_killing_wind(_input: ModifierResponsInput) -> HandlingModifierResponse {
    if _input.value > 0 {
        HandlingModifierResponse {
            stat_add: 40,
            ..Default::default()
        }
    } else {
        HandlingModifierResponse::default()
    }
}

pub(super) fn dmr_lasting_impressions(_input: ModifierResponsInput) -> DamageModifierResponse {
    DamageModifierResponse {
        impact_dmg_scale: 1.0,
        explosive_dmg_scale: 1.25,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_vorpal(_input: ModifierResponsInput) -> DamageModifierResponse {
    let mut buff = 1.0;
    if *_input.calc_data.enemy_type == EnemyType::BOSS
        || *_input.calc_data.enemy_type == EnemyType::MINIBOSS
        || *_input.calc_data.enemy_type == EnemyType::CHAMPION
        || *_input.calc_data.enemy_type == EnemyType::VEHICLE
    {
        if *_input.calc_data.ammo_type == AmmoType::PRIMARY {
            buff = 1.2;
        } else if *_input.calc_data.ammo_type == AmmoType::SPECIAL {
            buff = 1.15;
        } else if *_input.calc_data.ammo_type == AmmoType::HEAVY {
            buff = 1.1;
        }
    }
    DamageModifierResponse {
        impact_dmg_scale: buff,
        explosive_dmg_scale: buff,
        crit_scale: 1.0,
    }
}
