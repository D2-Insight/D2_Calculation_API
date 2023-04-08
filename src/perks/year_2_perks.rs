use std::collections::HashMap;

use crate::d2_enums::{AmmoType, StatHashes, WeaponType};

use super::{
    clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExplosivePercentResponse, ExtraDamageResponse,
        FiringModifierResponse, FlinchModifierResponse, HandlingModifierResponse,
        MagazineModifierResponse, RangeModifierResponse, RefundResponse, ReloadModifierResponse,
        ReloadOverrideResponse,
    }, ModifierResponsInput, Perks, add_dmr, add_hmr, add_rsmr, add_vmr, add_fmr, add_rmr, add_mmr, add_epr
};

pub fn year_2_perks() {
    add_sbr(
        Perks::AirAssault,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
                let mut stats = HashMap::new();
                let ae_per_stack = if _input.is_enhanced { 35 } else { 20 };
                let ae = ae_per_stack * _input.value as i32;
                stats.insert(StatHashes::AIRBORNE.into(), ae);
                stats
            }
        )
    );


    add_fmr_archers_tempo(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> FiringModifierResponse {
        FiringModifierResponse {
            burst_delay_scale: if _input.value > 0 { 0.75 } else { 1.0 },
            burst_delay_add: 0.0,
            inner_burst_scale: 1.0,
            burst_size_add: 0.0,
        }
    }

    add_dmr_explosive_head(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        if _input.pvp {
            DamageModifierResponse::default()
        } else {
            DamageModifierResponse {
                impact_dmg_scale: 1.0,
                explosive_dmg_scale: 1.3,
                crit_scale: 1.0,
            }
        }
    }

    add_epr_explosive_head(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> ExplosivePercentResponse {
        ExplosivePercentResponse {
            percent: 0.5,
            delyed: if _input.pvp { 0.0 } else { 0.2 },
            retain_base_total: true,
        }
    }

    add_rsmr_feeding_frenzy(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> ReloadModifierResponse {
        let val = clamp(_input.value, 0, 5);
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
        if _input.calc_data.time_total > duration {
            reload = 0;
            reload_mult = 1.0;
        };
        ReloadModifierResponse {
            reload_stat_add: reload,
            reload_time_scale: reload_mult,
        }
    }

    add_sbr_feeding_frenzy(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut stats = HashMap::new();
        let val = clamp(_input.value, 0, 5);
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
        if _input.calc_data.time_total > duration {
            reload = 0;
        };
        stats.insert(StatHashes::RELOAD.into(), reload);
        stats
    }

    add_dmr_firing_line(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let mut crit_mult = 1.0;
        if _input.value > 0 {
            crit_mult = 1.2;
        }
        DamageModifierResponse {
            crit_scale: crit_mult,
            explosive_dmg_scale: 1.0,
            impact_dmg_scale: 1.0,
        }
    }

    add_rr_fourth_times(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> RefundResponse {
        RefundResponse {
            crit: true,
            requirement: 4,
            refund_mag: 2,
            refund_reserves: 0,
        }
    }

    add_dmr_killing_tally(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let val = clamp(_input.value, 0, 3);
        let mut damage_mult = 0.1 * val as f64;
        if _input.pvp {
            damage_mult *= 0.5;
        };
        if _input.calc_data.num_reloads > 0.0 {
            damage_mult = 0.0;
        };
        DamageModifierResponse {
            impact_dmg_scale: 1.0 + damage_mult,
            explosive_dmg_scale: 1.0 + damage_mult,
            crit_scale: 1.0,
        }
    }

    add_mmr_overflow(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> MagazineModifierResponse {
        let mut mag_scale = if _input.value > 0 { 2.0 } else { 1.0 };
        if _input.is_enhanced && _input.value > 0 {
            mag_scale *= 1.1;
        };
        if _input.calc_data.total_shots_fired > 0.0 {
            mag_scale = 1.0;
        };
        MagazineModifierResponse {
            magazine_stat_add: 0,
            magazine_scale: mag_scale,
            magazine_add: 0.0,
        }
    }

    add_rsmr_rapid_hit(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> ReloadModifierResponse {
        let values = vec![
            (0, 1.0),
            (5, 0.99),
            (30, 0.97),
            (35, 0.96),
            (45, 0.94),
            (60, 0.93),
        ];
        let entry_to_get = clamp(_input.value + _input.calc_data.shots_fired_this_mag as u32, 0, 5);
        ReloadModifierResponse {
            reload_stat_add: values[entry_to_get as usize].0,
            reload_time_scale: values[entry_to_get as usize].1,
        }
    }

    add_sbr_rapid_hit(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let rel_values = vec![0, 5, 30, 35, 45, 60];
        let stab_values = vec![0, 2, 12, 14, 18, 25];
        let entry_to_get = clamp(_input.value + _input.calc_data.shots_fired_this_mag as u32, 0, 5);
        let mut stats = HashMap::new();
        stats.insert(StatHashes::RELOAD.into(), rel_values[entry_to_get as usize]);
        stats.insert(
            StatHashes::STABILITY.into(),
            stab_values[entry_to_get as usize],
        );
        stats
    }

    add_dmr_resevoir_burst(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let mut damage_mult = 1.0;
        if _input.calc_data.curr_mag >= _input.calc_data.base_mag {
            damage_mult = 1.25;
        };
        DamageModifierResponse {
            impact_dmg_scale: damage_mult,
            explosive_dmg_scale: damage_mult,
            crit_scale: 1.0,
        }
    }

    add_dmr_surrounded(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let mut damage_mult = 1.0;
        if _input.value > 0 {
            damage_mult = if *_input.calc_data.weapon_type == WeaponType::SWORD {
                1.35
            } else {
                1.4
            };
            if _input.is_enhanced {
                damage_mult *= 1.05;
            };
        };
        DamageModifierResponse {
            impact_dmg_scale: damage_mult,
            explosive_dmg_scale: damage_mult,
            crit_scale: 1.0,
        }
    }

    add_ror_demolitionist(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> ReloadOverrideResponse {
        //todo implement system for cooldown
        let grenade_throw_time = 0.8;
        if _input.value == 1 {
            return ReloadOverrideResponse {
                valid: true,
                reload_time: _input.calc_data.handling_data.ready_time + grenade_throw_time,
                ammo_to_reload: _input.calc_data.base_mag as i32,
                priority: 0,
                count_as_reload: false,
                uses_ammo: true,
            };
        }
        ReloadOverrideResponse::invalid()
    }

    add_dmr_full_court(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let mut damage_mult = 1.0;
        if _input.value > 0 {
            damage_mult = 1.25;
        };
        DamageModifierResponse {
            impact_dmg_scale: 1.0,
            explosive_dmg_scale: damage_mult,
            crit_scale: 1.0,
        }
    }

    add_dmr_swash_buckler(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let val = clamp(_input.value, 0, 5);
        let duration = if _input.is_enhanced { 6.0 } else { 4.5 };
        let mut dmg_boost = 0.067 * val as f64;
        if _input.calc_data.time_total > duration {
            dmg_boost = 0.0;
        };
        DamageModifierResponse {
            impact_dmg_scale: 1.0 + dmg_boost,
            explosive_dmg_scale: 1.0 + dmg_boost,
            crit_scale: 1.0,
        }
    }

    add_dmr_multi_kill_clip(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let val = clamp(_input.value, 0, 5);
        let mut damage_mult = (1.0 / 6.0) * val as f64;
        if _input.calc_data.num_reloads > 0.0 {
            damage_mult = 0.0;
        };
        DamageModifierResponse {
            impact_dmg_scale: 1.0 + damage_mult,
            explosive_dmg_scale: 1.0 + damage_mult,
            crit_scale: 1.0,
        }
    }

    add_dmr_explosive_light(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let shots = if _input.is_enhanced { 7.0 } else { 6.0 };
        let shots_left = _input.value as f64 * shots - _input.calc_data.total_shots_fired;
        if shots_left <= 0.0 {
            return DamageModifierResponse::default();
        };
        if _input.calc_data.weapon_type == &WeaponType::GRENADELAUNCHER {
            let blast_radius_struct = _input.calc_data.stats.get(&StatHashes::BLAST_RADIUS.into());
            let blast_radius;
            if blast_radius_struct.is_none() {
                blast_radius = 0;
            } else {
                blast_radius = blast_radius_struct.unwrap().val();
            };
            if _input.calc_data.ammo_type == &AmmoType::HEAVY {
                let expl_percent = 0.7 + 0.00175 * blast_radius as f64;
                let impt_percent = 1.0 - expl_percent;
                let expl_mult = 0.875 / expl_percent * 1.6;
                let impt_mult = 0.125 / impt_percent;
                return DamageModifierResponse {
                    impact_dmg_scale: impt_mult,
                    explosive_dmg_scale: expl_mult,
                    crit_scale: 1.0,
                };
            }
            if _input.calc_data.ammo_type == &AmmoType::SPECIAL {
                let expl_percent = 0.5 + 0.0025 * blast_radius as f64;
                let impt_percent = 1.0 - expl_percent;
                let expl_mult = 0.75 / expl_percent * 1.6;
                let impt_mult = 0.25 / impt_percent;
                return DamageModifierResponse {
                    impact_dmg_scale: impt_mult,
                    explosive_dmg_scale: expl_mult,
                    crit_scale: 1.0,
                };
            }
        };
        DamageModifierResponse {
            explosive_dmg_scale: 1.25,
            impact_dmg_scale: 1.25,
            crit_scale: 1.0,
        }
    }

    add_sbr_explosive_light(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut out = HashMap::new();
        if _input.value > 0 {
            out.insert(StatHashes::BLAST_RADIUS.into(), 100);
        };
        out
    }

    add_sbr_eye_of_the_storm(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut out = HashMap::new();
        if _input.value > 0 {
            out.insert(StatHashes::HANDLING.into(), 30);
        };
        out
    }

    add_hmr_eye_of_the_storm(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HandlingModifierResponse {
        if _input.value > 0 {
            HandlingModifierResponse {
                stat_add: 30,
                ..Default::default()
            }
        } else {
            HandlingModifierResponse::default()
        }
    }

    add_flmr_no_distractions(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> FlinchModifierResponse {
        if _input.value > 0 {
            FlinchModifierResponse { flinch_scale: 0.65 }
        } else {
            FlinchModifierResponse::default()
        }
    }
}