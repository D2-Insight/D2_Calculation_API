use std::collections::HashMap;

use crate::d2_enums::{StatHashes, WeaponType};

use super::{
    clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExtraDamageResponse, FiringModifierResponse,
        HandlingModifierResponse, MagazineModifierResponse, RangeModifierResponse, RefundResponse,
        ReloadModifierResponse, VelocityModifierResponse,
    },
    ModifierResponsInput, Perks, add_dmr, add_hmr, add_rsmr, add_vmr, add_fmr, add_rmr, add_mmr, add_epr
};

pub fn year_4_perks() {
    add_dmr_adagio(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let duration = if _input.is_enhanced { 8.0 } else { 7.0 };
        let mut dmg_boost = 0.3;
        if *_input.calc_data.weapon_type == WeaponType::BOW
            || *_input.calc_data.weapon_type == WeaponType::SHOTGUN
        {
            dmg_boost = 0.2;
        };
        if _input.calc_data.time_total > duration || _input.value == 0 {
            dmg_boost = 0.0;
        };
        DamageModifierResponse {
            impact_dmg_scale: 1.0 + dmg_boost,
            explosive_dmg_scale: 1.0 + dmg_boost,
            crit_scale: 1.0,
        }
    }

    add_fmr_adagio(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> FiringModifierResponse {
        let duration = if _input.is_enhanced { 8.0 } else { 7.0 };
        let mut firing_slow = 1.2;
        if _input.calc_data.time_total > duration || _input.value == 0 {
            firing_slow = 1.0;
        };
        FiringModifierResponse {
            burst_delay_scale: firing_slow,
            burst_delay_add: 0.0,
            inner_burst_scale: firing_slow,
            burst_size_add: 0.0,
        }
    }

    add_sbr_adagio(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut map = HashMap::new();
        let duration = if _input.is_enhanced { 8.0 } else { 7.0 };
        if _input.calc_data.time_total <= duration && _input.value > 0 {
            map.insert(StatHashes::RANGE.into(), 10);
        }
        map
    }

    add_rmr_adagio(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> RangeModifierResponse {
        let range_boost: i32;
        if _input.value > 0 {
            range_boost = 10;
        } else {
            range_boost = 0;
        };
        RangeModifierResponse {
            range_stat_add: range_boost,
            ..Default::default()
        }
    }

    add_dmr_adrenaline_junkie(
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

    add_sbr_adrenaline_junkie(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let duration = if _input.is_enhanced { 6.0 } else { 4.5 };
        let mut handling = 0;
        if _input.calc_data.time_total <= duration && _input.value > 0 {
            handling = 20;
        };
        let mut out = HashMap::new();
        out.insert(StatHashes::HANDLING.into(), handling);
        out
    }

    add_hmr_adrenaline_junkie(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HandlingModifierResponse {
        let handling = if _input.value > 0 { 20 } else { 0 };
        HandlingModifierResponse {
            stat_add: handling,
            ..Default::default()
        }
    }

    add_fmr_cornered(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> FiringModifierResponse {
        let mut delay_mult = 1.0;
        if _input.value > 0 {
            delay_mult = 0.85;
        };
        FiringModifierResponse {
            burst_delay_scale: delay_mult,
            burst_delay_add: 0.0,
            inner_burst_scale: 1.0,
            burst_size_add: 0.0,
        }
    }

    add_sbr_ensemble(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let handling = if _input.is_enhanced { 30 } else { 35 };
        let reload = if _input.is_enhanced { 40 } else { 45 };
        if _input.value > 0 {
            let mut out = HashMap::new();
            out.insert(StatHashes::HANDLING.into(), handling);
            out.insert(StatHashes::RELOAD.into(), reload);
            out
        } else {
            HashMap::new()
        }
    }

    add_hmr_ensemble(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HandlingModifierResponse {
        let handling = if _input.is_enhanced { 30 } else { 35 };
        if _input.value > 0 {
            HandlingModifierResponse {
                stat_add: handling,
                ..Default::default()
            }
        } else {
            HandlingModifierResponse::default()
        }
    }

    add_rsmr_ensemble(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> ReloadModifierResponse {
        let reload = if _input.is_enhanced { 40 } else { 45 };
        if _input.value > 0 {
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

    add_rsmr_frenzy(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> ReloadModifierResponse {
        let mut reload = 0;
        if _input.value > 0 {
            reload = 100;
        };
        if _input.calc_data.time_total > 12.0 {
            reload = 100;
        };
        ReloadModifierResponse {
            reload_stat_add: reload,
            reload_time_scale: 1.0,
        }
    }

    add_hmr_frenzy(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HandlingModifierResponse {
        let mut handling = 0;
        if _input.value > 0 {
            handling = 100;
        };
        if _input.calc_data.time_total > 12.0 {
            handling = 100;
        };
        HandlingModifierResponse {
            stat_add: handling,
            ..Default::default()
        }
    }

    add_dmr_frenzy(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let mut dmg = 0.0;
        if _input.value > 0 {
            dmg = 0.15;
        };
        if _input.calc_data.time_total > 12.0 {
            dmg = 0.15;
        };
        DamageModifierResponse {
            impact_dmg_scale: 1.0 + dmg,
            explosive_dmg_scale: 1.0 + dmg,
            crit_scale: 1.0,
        }
    }

    add_sbr_frenzy(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut handling = 0;
        let mut reload = 0;
        if _input.value > 0 {
            handling = 100;
            reload = 100;
        };
        if _input.calc_data.time_total > 12.0 {
            handling = 100;
            reload = 100;
        };
        let mut out = HashMap::new();
        out.insert(StatHashes::HANDLING.into(), handling);
        out.insert(StatHashes::RELOAD.into(), reload);
        out
    }

    add_rsmr_impulse_amplifier(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> ReloadModifierResponse {
        let reload = if _input.is_enhanced { 15 } else { 10 };
        let reload_mult = if _input.is_enhanced { 0.77 } else { 0.8 };
        ReloadModifierResponse {
            reload_stat_add: reload,
            reload_time_scale: reload_mult,
        }
    }

    add_sbr_impulse_amplifier(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let reload = if _input.is_enhanced { 15 } else { 10 };
        let mut out = HashMap::new();
        out.insert(StatHashes::RELOAD.into(), reload);
        out
    }

    add_vmr_impulse_amplifier(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> VelocityModifierResponse {
        VelocityModifierResponse {
            velocity_scaler: 1.35,
        }
    }

    add_sbr_perpetual_motion(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let val = clamp(_input.value, 0, 2);
        let mut stat_bump = 0;
        if val == 1 {
            stat_bump = 10;
        } else if val == 2 {
            stat_bump = 20;
        };
        let mut out = HashMap::new();
        out.insert(StatHashes::RELOAD.into(), stat_bump);
        out.insert(StatHashes::HANDLING.into(), stat_bump);
        out.insert(StatHashes::STABILITY.into(), stat_bump);
        out
    }

    add_hmr_perpetual_motion(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HandlingModifierResponse {
        let val = clamp(_input.value, 0, 2);
        let mut stat_bump = 0;
        if val == 1 {
            stat_bump = 10;
        } else if val == 2 {
            stat_bump = 20;
        };
        HandlingModifierResponse {
            stat_add: stat_bump,
            ..Default::default()
        }
    }

    add_rsmr_perpetual_motion(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> ReloadModifierResponse {
        let val = clamp(_input.value, 0, 2);
        let mut stat_bump = 0;
        if val == 1 {
            stat_bump = 10;
        } else if val == 2 {
            stat_bump = 20;
        };
        ReloadModifierResponse {
            reload_stat_add: stat_bump,
            reload_time_scale: 1.0,
        }
    }

    add_sbr_perfect_float(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut out = HashMap::new();
        if _input.value > 0 {
            out.insert(StatHashes::AIRBORNE.into(), 30);
        };
        out
    }

    add_sbr_pugilist(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut out = HashMap::new();
        if _input.value > 0 {
            out.insert(StatHashes::HANDLING.into(), 30);
        };
        out
    }

    add_hrm_pugilist(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HandlingModifierResponse {
        let mut handling = 0;
        if _input.value > 0 {
            handling = 30;
        };
        HandlingModifierResponse {
            stat_add: handling,
            ..Default::default()
        }
    }

    add_mmr_reconstruction(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> MagazineModifierResponse {
        let mag_scale = if _input.value > 0 { 2.0 } else { 1.0 };
        MagazineModifierResponse {
            magazine_stat_add: 0,
            magazine_scale: mag_scale,
            magazine_add: 0.0,
        }
    }

    add_sbr_danger_zone(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut out = HashMap::new();
        if _input.value > 0 {
            out.insert(StatHashes::BLAST_RADIUS.into(), 100);
        };
        out
    }

    add_dmr_one_for_all(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let mut dmg = 0.0;
        let duration = if _input.is_enhanced { 11.0 } else { 10.0 };
        if _input.value > 0 {
            dmg = 0.35;
        };
        if _input.calc_data.time_total > duration {
            dmg = 0.0;
        };
        DamageModifierResponse {
            impact_dmg_scale: 1.0 + dmg,
            explosive_dmg_scale: 1.0 + dmg,
            crit_scale: 1.0,
        }
    }

    add_rsmr_fire_fly(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> ReloadModifierResponse {
        let duration = if _input.is_enhanced { 7.0 } else { 6.0 };
        if _input.value > 0 && _input.calc_data.time_total < duration {
            ReloadModifierResponse {
                reload_stat_add: 50,
                reload_time_scale: 1.0,
            }
        } else {
            ReloadModifierResponse::default()
        }
    }

    add_dmr_golden_tricorn(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let val = clamp(_input.value, 0, 2);
        let mut duration = if val == 2 { 10.0 } else { 7.0 };
        if _input.is_enhanced && val == 1 {
            duration += 1.0;
        };
        let damage_mult = if val == 2 { 0.5 } else { 0.15 };
        if _input.value > 0 && _input.calc_data.time_total < duration {
            DamageModifierResponse {
                impact_dmg_scale: 1.0 + damage_mult,
                explosive_dmg_scale: 1.0 + damage_mult,
                crit_scale: 1.0,
            }
        } else {
            DamageModifierResponse::default()
        }
    }

    add_dmr_harmony(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let mut damage_mult = if _input.value > 0 { 0.20 } else { 0.0 };
        let duration = if _input.is_enhanced { 8.0 } else { 7.0 };
        if _input.calc_data.time_total > duration {
            damage_mult = 0.0;
        };
        DamageModifierResponse {
            impact_dmg_scale: 1.0 + damage_mult,
            explosive_dmg_scale: 1.0 + damage_mult,
            crit_scale: 1.0,
        }
    }

    add_hmr_harmony(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HandlingModifierResponse {
        let handling = if _input.value > 0 { 15 } else { 0 };
        HandlingModifierResponse {
            stat_add: handling,
            ..Default::default()
        }
    }

    add_sbr_harmony(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut out = HashMap::new();
        if _input.value > 0 {
            out.insert(StatHashes::HANDLING.into(), 15);
        }
        out
    }

    add_sbr_surplus(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut out = HashMap::new();
        if _input.value == 1 {
            out.insert(StatHashes::HANDLING.into(), 10);
            out.insert(StatHashes::RELOAD.into(), 5);
            out.insert(StatHashes::STABILITY.into(), 5);
        } else if _input.value == 2 {
            out.insert(StatHashes::HANDLING.into(), 25);
            out.insert(StatHashes::RELOAD.into(), 25);
            out.insert(StatHashes::STABILITY.into(), 15);
        } else if _input.value == 3 {
            out.insert(StatHashes::HANDLING.into(), 50);
            out.insert(StatHashes::RELOAD.into(), 50);
            out.insert(StatHashes::STABILITY.into(), 25);
        }
        out
    }

    add_hmr_surplus(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HandlingModifierResponse {
        let handling = if _input.value == 1 {
            10
        } else if _input.value == 2 {
            25
        } else if _input.value == 3 {
            50
        } else {
            0
        };
        HandlingModifierResponse {
            stat_add: handling,
            ..Default::default()
        }
    }

    add_rsmr_surplus(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> ReloadModifierResponse {
        let reload = if _input.value == 1 {
            5
        } else if _input.value == 2 {
            25
        } else if _input.value == 3 {
            50
        } else {
            0
        };
        ReloadModifierResponse {
            reload_stat_add: reload,
            reload_time_scale: 1.0,
        }
    }

    add_sbr_heating_up(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let val = clamp(_input.value, 0, 2) as i32;
        let mut out = HashMap::new();
        out.insert(StatHashes::RECOIL_DIR.into(), 20 * val);
        out.insert(StatHashes::STABILITY.into(), 15 * val);
        out
    }

    add_sbr_tunnel_vision(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut out = HashMap::new();
        if _input.value > 0 {
            out.insert(StatHashes::AIM_ASSIST.into(), 20);
        }
        out
    }

    add_hmr_tunnel_vision(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HandlingModifierResponse {
        if _input.value > 0 {
            HandlingModifierResponse {
                ads_scale: 0.85,
                ..Default::default()
            }
        } else {
            HandlingModifierResponse::default()
        }
    }

    add_dmr_kickstart(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let mut damage_mult = if _input.value > 0 { 0.20 } else { 0.0 };
        let duration = 1.0;
        if _input.calc_data.time_total > duration {
            damage_mult = 0.0;
        };
        DamageModifierResponse {
            impact_dmg_scale: 1.0 + damage_mult,
            explosive_dmg_scale: 1.0 + damage_mult,
            crit_scale: 1.0,
        }
    }

    add_fmr_kickstart(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> FiringModifierResponse {
        let mut fire_rate_mult = if _input.value > 0 { 0.20 } else { 0.0 };
        let duration = 1.0;
        if _input.calc_data.time_total > duration {
            fire_rate_mult = 0.0;
        };
        FiringModifierResponse {
            burst_delay_scale: 1.0 - fire_rate_mult,
            ..Default::default()
        }
    }
}
