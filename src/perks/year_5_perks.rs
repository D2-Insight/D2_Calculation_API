use std::collections::HashMap;

use crate::d2_enums::{StatHashes, WeaponType};

use super::{
    clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExtraDamageResponse, FiringModifierResponse,
        HandlingModifierResponse, RangeModifierResponse, RefundResponse, ReloadModifierResponse,
        ReloadOverrideResponse,
    }, ModifierResponsInput, Perks, add_dmr, add_hmr, add_rsmr, add_vmr, add_fmr, add_rmr, add_mmr, add_epr
};

pub fn year_5_perks() {
    add_fmr_cascade_point(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> FiringModifierResponse {
        let duration = if _input.is_enhanced { 3.0 } else { 2.5 };
        let mut delay_mult = 1.0;
        if _input.calc_data.time_total < duration && _input.value > 0 {
            if *_input.calc_data.weapon_type == WeaponType::MACHINEGUN
                || *_input.calc_data.weapon_type == WeaponType::SUBMACHINEGUN
            {
                delay_mult = 0.7;
            } else {
                delay_mult = 0.6;
            }
        }
        FiringModifierResponse {
            burst_delay_scale: delay_mult,
            burst_delay_add: 0.0,
            inner_burst_scale: 1.0,
            burst_size_add: 0.0,
        }
    }

    add_sbr_encore(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut map = HashMap::new();
        let val = clamp(_input.value, 0, 4) as i32;
        let stability_boost = 8 * val;
        let range_boost = 5 * val;
        map.insert(StatHashes::RANGE.into(), range_boost);
        map.insert(StatHashes::STABILITY.into(), stability_boost);
        map
    }

    add_rmr_encore(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> RangeModifierResponse {
        let val = clamp(_input.value, 0, 4) as i32;
        let range_boost = 5 * val;
        RangeModifierResponse {
            range_stat_add: range_boost,
            ..Default::default()
        }
    }

    add_dmr_focused_fury(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let mut dmg_boost = 1.0;
        let shots_needed;
        if _input.calc_data.curr_firing_data.one_ammo == false
            || _input.calc_data.curr_firing_data.burst_size == 1
        {
            shots_needed = _input.calc_data.base_mag / 2.0;
        } else {
            shots_needed = (_input.calc_data.base_mag * (_input.calc_data.curr_firing_data.burst_size as f64)) / 2.0;
        }
        if _input.calc_data.total_shots_fired >= shots_needed || _input.value > 0 {
            dmg_boost = 1.2;
        }
        DamageModifierResponse {
            impact_dmg_scale: dmg_boost,
            explosive_dmg_scale: dmg_boost,
            crit_scale: 1.0,
        }
    }

    add_rmr_fragile_focus(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> RangeModifierResponse {
        let range_bonus = if _input.value > 0 { 20 } else { 0 };
        RangeModifierResponse {
            range_stat_add: range_bonus,
            range_all_scale: 1.0,
            range_hip_scale: 1.0,
            range_zoom_scale: 1.0,
        }
    }

    add_sbr_fragile_focus(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut map = HashMap::new();
        let mut range_bonus = 0;
        if _input.value > 0 {
            range_bonus = 20;
        };
        map.insert(StatHashes::RANGE.into(), range_bonus);
        map
    }

    add_dmr_gutshot_straight(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let high_weapons = [
            WeaponType::AUTORIFLE,
            WeaponType::HANDCANNON,
            WeaponType::BOW,
        ];
        let dmg_scale: f64;
        let crit_scale: f64;
        if high_weapons.contains(&_input.calc_data.weapon_type) {
            dmg_scale = 1.2;
            crit_scale = 1.0 / 1.2;
        } else {
            dmg_scale = 1.1;
            crit_scale = 1.0 / 1.1;
        };
        // if  _input.calc_data.base_crit_mult <= 1.0 {
        //     crit_scale = 1.0;
        // }
        DamageModifierResponse {
            impact_dmg_scale: dmg_scale,
            explosive_dmg_scale: dmg_scale,
            crit_scale,
        }
    }

    add_sbr_offhand_strike(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut map = HashMap::new();
        let mut stability_boost = 0;
        if _input.value > 0 {
            stability_boost = 30;
        };
        map.insert(StatHashes::STABILITY.into(), stability_boost);
        map
    }

    add_rmr_offhand_strike(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> RangeModifierResponse {
        let mut range_hip_mult = 1.0;
        if _input.value > 0 {
            range_hip_mult = 1.45;
        };
        RangeModifierResponse {
            range_stat_add: 0,
            range_all_scale: 1.0,
            range_hip_scale: range_hip_mult,
            range_zoom_scale: 1.0,
        }
    }

    add_hmr_slickdraw(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HandlingModifierResponse {
        HandlingModifierResponse {
            stat_add: 100,
            stow_scale: 0.9,
            draw_scale: 0.9,
            ..Default::default()
        }
    }

    add_sbr_slickdraw(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut map = HashMap::new();
        map.insert(StatHashes::HANDLING.into(), 100);
        map
    }

    add_sbr_stats_for_all(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut out = HashMap::new();
        let mut stability_boost = 0;
        let mut range_boost = 0;
        let mut reload_boost = 0;
        let mut handling_boost = 0;
        if _input.value > 0 {
            stability_boost = 10;
            range_boost = 10;
            reload_boost = 35;
            handling_boost = 35;
        };
        out.insert(StatHashes::STABILITY.into(), stability_boost);
        out.insert(StatHashes::RANGE.into(), range_boost);
        out.insert(StatHashes::RELOAD.into(), reload_boost);
        out.insert(StatHashes::HANDLING.into(), handling_boost);
        out
    }

    add_hmr_stats_for_all(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HandlingModifierResponse {
        let mut handling_boost = 0;
        let duration = if _input.is_enhanced { 11.0 } else { 10.0 };
        if _input.value > 0 && _input.calc_data.time_total < duration {
            handling_boost = 35;
        };
        HandlingModifierResponse {
            stat_add: handling_boost,
            ..Default::default()
        }
    }

    add_rmr_stats_for_all(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> RangeModifierResponse {
        let mut range = 0;
        let mut range_mult = 1.0;
        if _input.value > 0 {
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

    add_rsmr_stats_for_all(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> ReloadModifierResponse {
        let mut reload = 0;
        let mut reload_mult = 1.0;
        let duration = if _input.is_enhanced { 11.0 } else { 10.0 };
        if _input.value > 0 && _input.calc_data.time_total < duration {
            reload = 35;
            reload_mult = 0.95;
        };
        ReloadModifierResponse {
            reload_stat_add: reload,
            reload_time_scale: reload_mult,
        }
    }

    add_sbr_steady_hands(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut map = HashMap::new();
        let mut handling = 0;
        if _input.value > 0 {
            handling = 100;
        };
        map.insert(StatHashes::HANDLING.into(), handling);
        map
    }

    add_hmr_steady_hands(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HandlingModifierResponse {
        let mut handling_mult = 1.0;
        let mut handling = 0;
        let duration = if _input.is_enhanced { 9.0 } else { 8.5 };
        if _input.value > 0 && _input.calc_data.time_total < duration {
            handling_mult = 0.825;
            handling = 100;
        };
        HandlingModifierResponse {
            stat_add: handling,
            stow_scale: handling_mult,
            draw_scale: handling_mult,
            ..Default::default()
        }
    }

    add_dmr_target_lock(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let buff;

        let enh_increase = if _input.is_enhanced { 1.125 } else { 1.0 };
        let low_end_dmg = 0.28 / 3.0 * enh_increase;
        let high_end_dmg = 0.40 * enh_increase;

        let formula_start = -0.3505;
        let formula_end = 1.1395;

        let percent_of_mag = _input.calc_data.shots_fired_this_mag / _input.calc_data.base_mag;

        if percent_of_mag < 0.125 {
            buff = 0.0;
        } else if percent_of_mag > formula_end {
            buff = high_end_dmg;
        } else {
            let x = (percent_of_mag - formula_start) / (formula_end - formula_start);
            let smoothstep = 3.0 * (x.powf(2.0)) - 2.0 * (x.powf(3.0));
            buff = low_end_dmg + (high_end_dmg - low_end_dmg) * smoothstep;
        }

        DamageModifierResponse {
            impact_dmg_scale: buff + 1.0,
            explosive_dmg_scale: buff + 1.0,
            crit_scale: 1.0,
        }
    }

    add_dmr_over_under(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let mut buff = 1.0_f64;
        if _input.calc_data.has_overshield {
            buff += 0.2;
        }
        if _input.is_enhanced {
            buff *= 1.05;
        }
        DamageModifierResponse {
            impact_dmg_scale: buff,
            explosive_dmg_scale: buff,
            crit_scale: 1.0,
        }
    }

    add_sbr_well_rounded(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let val = clamp(_input.value, 0, 2) as i32;
        let mut map = HashMap::new();
        let stat_base = if _input.is_enhanced { 12 } else { 10 };
        let stat_bump = stat_base * val;
        map.insert(StatHashes::STABILITY.into(), stat_bump);
        map.insert(StatHashes::RANGE.into(), stat_bump);
        map.insert(StatHashes::HANDLING.into(), stat_bump);
        map
    }

    add_hmr_well_rounded(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HandlingModifierResponse {
        let val = clamp(_input.value, 0, 2) as i32;
        //due to ease of activation and upkeep will assume its always active
        // let mut duration = if  _input.is_enhanced {9.0} else {8.5};
        let stat_base = if _input.is_enhanced { 12 } else { 10 };
        let handling = stat_base * val;
        HandlingModifierResponse {
            stat_add: handling,
            ..Default::default()
        }
    }

    add_rmr_well_rounded(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> RangeModifierResponse {
        let val = clamp(_input.value, 0, 2) as i32;
        let stat_base = if _input.is_enhanced { 12 } else { 10 };
        let range = stat_base * val;
        RangeModifierResponse {
            range_stat_add: range,
            range_all_scale: 1.0,
            range_hip_scale: 1.0,
            range_zoom_scale: 1.0,
        }
    }

    add_dmr_bait_and_switch(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        if _input.value > 0 {
            DamageModifierResponse {
                impact_dmg_scale: 1.35,
                explosive_dmg_scale: 1.35,
                crit_scale: 1.0,
            }
        } else {
            DamageModifierResponse::default()
        }
    }

    add_edr_bait_and_switch(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> ExtraDamageResponse {
        let time = _input.calc_data.handling_data.ready_time * 2.0
            + _input.calc_data.handling_data.stow_time * 2.0;
        let last_proc = _input.cached_data
            .get("bait_and_switch_last_proc")
            .unwrap_or(&0.0);
        if _input.calc_data.time_total - last_proc < 10.0 {
            return ExtraDamageResponse::default();
        }
        ExtraDamageResponse {
            additive_damage: 200.0,
            combatant_scale: true,
            crit_scale: true,
            increment_total_time: true,
            time_for_additive_damage: time,
            times_to_hit: 1,
            weapon_scale: true,
            hit_at_same_time: true,
            is_dot: false,
        }
    }

    add_rsmr_compulsive_reloader(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> ReloadModifierResponse {
        let reload_add = if _input.is_enhanced { 55 } else { 50 };
        if _input.calc_data.shots_fired_this_mag <= _input.calc_data.base_mag / 2.0 && _input.value > 0 {
            ReloadModifierResponse {
                reload_stat_add: reload_add,
                reload_time_scale: 0.95,
            }
        } else {
            ReloadModifierResponse::default()
        }
    }

    add_sbr_compulsive_reloader(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let reload_add = if _input.is_enhanced { 55 } else { 50 };
        let mut map = HashMap::new();
        if _input.calc_data.shots_fired_this_mag <= _input.calc_data.base_mag / 2.0 && _input.value > 0 {
            map.insert(StatHashes::RELOAD.into(), reload_add);
        }
        map
    }

    add_sbr_sleight_of_hand(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let val = clamp(_input.value, 0, 3) as i32;
        let mut map = HashMap::new();
        let stat_base = 10;
        let stat_bump = stat_base * val;
        map.insert(StatHashes::STABILITY.into(), stat_bump);
        map.insert(StatHashes::RELOAD.into(), stat_bump);
        map.insert(StatHashes::HANDLING.into(), stat_bump);
        map
    }

    add_hmr_sleight_of_hand(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HandlingModifierResponse {
        let val = clamp(_input.value, 0, 3) as i32;
        let stat_base = 10;
        let handling = stat_base * val;
        HandlingModifierResponse {
            stat_add: handling,
            ..Default::default()
        }
    }

    add_rsmr_sleight_of_hand(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> ReloadModifierResponse {
        let val = clamp(_input.value, 0, 3) as i32;
        let stat_base = 10;
        let reload = stat_base * val;
        ReloadModifierResponse {
            reload_stat_add: reload,
            ..Default::default()
        }
    }

    add_hmr_shot_swap(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HandlingModifierResponse {
        let mut handling_mult = 1.0;
        let mut handling = 0;
        if _input.value > 0 {
            handling_mult = 0.95;
            handling = 100;
        };
        HandlingModifierResponse {
            stat_add: handling,
            stow_scale: handling_mult,
            draw_scale: handling_mult,
            ..Default::default()
        }
    }

    add_sbr_shot_swap(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut map = HashMap::new();
        if _input.value > 0 {
            map.insert(StatHashes::HANDLING.into(), 100);
        }
        map
    }

    add_fmr_succesful_warmup(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> FiringModifierResponse {
        let fire_rate_buff = if _input.value > 0 { 0.625 } else { 1.0 };
        let duration = if _input.value > 0 {
            6_f64
                + (if _input.is_enhanced { 5_f64 } else { 4_f64 })
                    * clamp(_input.value as f64 - 1_f64, 0_f64, 4_f64)
        } else {
            0.0
        };
        if _input.calc_data.time_total < duration as f64 {
            FiringModifierResponse {
                burst_delay_scale: fire_rate_buff,
                ..Default::default()
            }
        } else {
            FiringModifierResponse::default()
        }
    }
}