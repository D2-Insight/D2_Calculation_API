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
    }, ModifierResponsInput, Perks, add_dmr, add_hmr, add_rsmr, add_vmr, add_fmr, add_rmr, add_mmr, add_epr, add_sbr
};

pub fn other_perks() {
    add_rsmr(
        Perks::AlloyMag,
        Box::new(
            |_input: ModifierResponsInput| -> ReloadModifierResponse {
                if _input.value > 0 {
                    ReloadModifierResponse {
                        reload_stat_add: 0,
                        reload_time_scale: 0.85,
                    }
                } else {
                    ReloadModifierResponse::default()
                }
            }
        )
    );

    add_rsmr(
        Perks::RapidFireFrame,
        Box::new(
            |_input: ModifierResponsInput| -> ReloadModifierResponse {
                if _input.value > 0 {
                    ReloadModifierResponse {
                        reload_stat_add: 0,
                        reload_time_scale: 0.85,
                    }
                } else {
                    ReloadModifierResponse::default()
                }
            }
        )
    );

    add_hmr(
        Perks::SwapMag,
        Box::new(
            |_input: ModifierResponsInput| -> HandlingModifierResponse {
                HandlingModifierResponse {
                    draw_scale: 0.9,
                    stow_scale: 0.9,
                    ..Default::default()
                }
            }
        )
    );

    add_hmr_freehand_grip(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HandlingModifierResponse {
        HandlingModifierResponse {
            draw_scale: 0.95,
            ..Default::default()
        }
    }

    add_hmr_ophidian_aspects(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HandlingModifierResponse {
        HandlingModifierResponse {
            stat_add: 35,
            ..Default::default()
        }
    }

    add_rsmr_ophidian_aspects(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> ReloadModifierResponse {
        ReloadModifierResponse {
            reload_stat_add: 35,
            reload_time_scale: 1.0,
        }
    }

    add_sbr_ophidian_aspects(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut stats = HashMap::new();
        stats.insert(StatHashes::HANDLING.into(), 35);
        stats.insert(StatHashes::RELOAD.into(), 35);
        stats.insert(StatHashes::AIRBORNE.into(), 10);
        stats
    }

    add_sbr_dragon_shadow(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut stats = HashMap::new();
        if _input.value >= 1 {
            stats.insert(StatHashes::HANDLING.into(), 100);
            stats.insert(StatHashes::RELOAD.into(), 100);
        }
        stats
    }

    add_hmr_dragon_shadow(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HandlingModifierResponse {
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

    add_rsmr_dragon_shadow(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> ReloadModifierResponse {
        if _input.value >= 1 {
            ReloadModifierResponse {
                reload_stat_add: 100,
                reload_time_scale: 1.0,
            }
        } else {
            ReloadModifierResponse::default()
        }
    }

    add_sbr_amplified(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut stats = HashMap::new();
        stats.insert(StatHashes::HANDLING.into(), 40);
        stats
    }

    add_hmr_amplified(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HandlingModifierResponse {
        HandlingModifierResponse {
            stat_add: 40,
            draw_scale: 0.95,
            stow_scale: 0.95,
            ..Default::default()
        }
    }

    add_rsmr_frequency(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> ReloadModifierResponse {
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

    add_rsmr_flow_state(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> ReloadModifierResponse {
        //far far too lazy to do this properly
        ReloadModifierResponse {
            reload_stat_add: 55,
            reload_time_scale: 0.87,
        }
    }

    add_sbr_tempering(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut stats = HashMap::new();
        if _input.value > 0 {
            stats.insert(StatHashes::AIRBORNE.into(), 20);
        };
        stats
    }

    add_sbr_on_your_mark(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut stats = HashMap::new();
        let val = clamp(_input.value, 0, 3) as i32;
        if _input.value > 0 {
            stats.insert(StatHashes::HANDLING.into(), 20 * val);
            stats.insert(StatHashes::RELOAD.into(), 20 * val);
        };
        stats
    }

    add_hmr_on_your_mark(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HandlingModifierResponse {
        let val = clamp(_input.value, 0, 3) as i32;
        HandlingModifierResponse {
            stat_add: 20 * val,
            ..Default::default()
        }
    }

    add_rsmr_on_your_mark(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> ReloadModifierResponse {
        let val = clamp(_input.value, 0, 3) as i32;
        ReloadModifierResponse {
            reload_stat_add: 20 * val,
            reload_time_scale: 0.93,
        }
    }

    add_sbr_heat_rises(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut stats = HashMap::new();
        let mut buff = 20;
        if _input.value > 0 {
            buff += 50;
        };
        stats.insert(StatHashes::AIRBORNE.into(), buff);
        stats
    }

    add_sbr_hedrons(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut stats = HashMap::new();
        if _input.value > 0 {
            stats.insert(StatHashes::AIRBORNE.into(), 20);
            stats.insert(StatHashes::AIM_ASSIST.into(), 15);
            stats.insert(StatHashes::STABILITY.into(), 30);
        };
        stats
    }

    add_dmr_boss_spec(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
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

    add_dmr_major_spec(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
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

    add_dmr_big_ones_spec(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
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

    add_dmr_minor_spec(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
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

    add_dmr_taken_spec(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let damage_mult = if _input.value > 0 && !_input.pvp { 1.1 } else { 1.0 };
        DamageModifierResponse {
            impact_dmg_scale: damage_mult,
            explosive_dmg_scale: damage_mult,
            crit_scale: 1.0,
        }
    }

    add_dmr_spike_grenades(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        DamageModifierResponse {
            impact_dmg_scale: 1.5,
            explosive_dmg_scale: 1.0,
            crit_scale: 1.0,
        }
    }

    add_dmr_disorienting_grenades(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        DamageModifierResponse {
            impact_dmg_scale: 0.75,
            explosive_dmg_scale: 0.75,
            crit_scale: 1.0,
        }
    }

    add_dmr_full_choke(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
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

    add_fmr_accelerated_coils(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> FiringModifierResponse {
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

    add_fmr_liquid_coils(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> FiringModifierResponse {
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

    add_dmr_liquid_coils(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        DamageModifierResponse {
            impact_dmg_scale: 1.02,
            explosive_dmg_scale: 1.02,
            crit_scale: 1.0,
        }
    }

    add_dmr_accelerated_coils(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        DamageModifierResponse {
            impact_dmg_scale: 0.982,
            explosive_dmg_scale: 0.982,
            crit_scale: 1.0,
        }
    }

    add_fmr_faster_string_t2(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> FiringModifierResponse {
        FiringModifierResponse {
            burst_delay_add: -2.0 / 30.0,
            ..Default::default()
        }
    }

    add_fmr_faster_string_t1(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> FiringModifierResponse {
        FiringModifierResponse {
            burst_delay_add: -1.0 / 30.0,
            ..Default::default()
        }
    }

    add_fmr_slower_string_t1(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> FiringModifierResponse {
        FiringModifierResponse {
            burst_delay_add: 1.0 / 30.0,
            ..Default::default()
        }
    }

    add_fmr_slower_string_t2(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> FiringModifierResponse {
        FiringModifierResponse {
            burst_delay_add: 2.0 / 30.0,
            ..Default::default()
        }
    }

    add_fmr_assault_mag(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> FiringModifierResponse {
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
}