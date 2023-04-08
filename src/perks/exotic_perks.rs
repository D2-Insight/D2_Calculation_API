//This also includes intrinsic perks, not just exotic
use std::collections::HashMap;

use crate::{d2_enums::StatHashes, enemies::EnemyType, weapons::Stat};

use super::{
    clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExtraDamageResponse, FiringModifierResponse,
        HandlingModifierResponse, InventoryModifierResponse, MagazineModifierResponse,
        RangeModifierResponse, RefundResponse, ReloadModifierResponse, ReloadOverrideResponse,
    }, ModifierResponsInput, Perks, add_dmr, add_hmr, add_rsmr, add_vmr, add_fmr, add_rmr, add_mmr, add_epr, add_sbr
};

pub fn exotic_perks() {
    add_dmr_paracausal_shot(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let bufflist_pve = vec![1.0, 3.92, 4.0, 4.4, 5.25, 7.67, 11.71, 18.36];
        let bufflist_pvp = vec![1.0, 1.01, 1.03, 1.13, 1.41, 1.96, 3.0, 4.73];
        let mut damage_buff = 1.0;
        if _input.calc_data.curr_mag == 1.0 {
            let num_of_crits = clamp(_input.calc_data.shots_fired_this_mag as i32, 0, 7);
            let bufflist = if _input.pvp {
                &bufflist_pvp
            } else {
                &bufflist_pve
            };
            damage_buff = bufflist[num_of_crits as usize];
        };
        if _input.calc_data.time_this_mag < 0.0 {
            let num_of_crits = clamp(_input.value as i32, 0, 7);
            let bufflist = if _input.pvp {
                &bufflist_pvp
            } else {
                &bufflist_pve
            };
            damage_buff = bufflist[num_of_crits as usize];
        }
        DamageModifierResponse {
            impact_dmg_scale: damage_buff,
            explosive_dmg_scale: damage_buff,
            crit_scale: 1.0,
        }
    }

    add_sbr_hunters_trance(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut out = HashMap::new();
        let inter_val = *_input.calc_data.perk_value_map.get(&213689231).unwrap_or(&0);
        let buff_val = (clamp(inter_val, 0, 7) * 5) as i32;
        out.insert(StatHashes::RELOAD.into(), buff_val);
        out.insert(StatHashes::RANGE.into(), buff_val);
        out.insert(StatHashes::HANDLING.into(), buff_val);
        out
    }

    add_rsmr_hunters_trance(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> ReloadModifierResponse {
        let inter_val = *_input.calc_data.perk_value_map.get(&213689231).unwrap_or(&0);
        let buff_val = (clamp(inter_val, 0, 7) * 5) as i32;
        ReloadModifierResponse {
            reload_stat_add: buff_val,
            ..Default::default()
        }
    }

    add_rmr_hunters_trance(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> RangeModifierResponse {
        let inter_val = *_input.calc_data.perk_value_map.get(&213689231).unwrap_or(&0);
        let buff_val = (clamp(inter_val, 0, 7) * 5) as i32;
        RangeModifierResponse {
            range_stat_add: buff_val,
            ..Default::default()
        }
    }

    add_hmr_hunters_trance(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HandlingModifierResponse {
        let inter_val = *_input.calc_data.perk_value_map.get(&213689231).unwrap_or(&0);
        let buff_val = (clamp(inter_val, 0, 7) * 5) as i32;
        HandlingModifierResponse {
            stat_add: buff_val,
            ..Default::default()
        }
    }

    add_dmr_momento_mori(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let mut damage_buff = 1.0;
        if _input.value > 0 && _input.calc_data.total_shots_fired < 7.0 {
            damage_buff = if _input.pvp { 1.5 } else { 1.285 };
        };
        DamageModifierResponse {
            impact_dmg_scale: damage_buff,
            explosive_dmg_scale: damage_buff,
            crit_scale: 1.0,
        }
    }

    add_dmr_agers_call(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let mut damage_buff = 1.0;
        if _input.value > 0 && _input.calc_data.num_reloads == 0.0 {
            damage_buff = 1.8;
        };
        DamageModifierResponse {
            impact_dmg_scale: damage_buff,
            explosive_dmg_scale: damage_buff,
            crit_scale: 1.0,
        }
    }

    add_mmr_agers_call(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> MagazineModifierResponse {
        let mut mag_buff = 1.0;
        if _input.value > 0 && _input.calc_data.total_shots_fired == 0.0 {
            mag_buff = 2.0;
        };
        MagazineModifierResponse {
            magazine_scale: mag_buff,
            ..Default::default()
        }
    }

    add_sbr_roadborn(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut out = HashMap::new();
        if _input.value > 0 {
            out.insert(StatHashes::HANDLING.into(), 20);
            out.insert(StatHashes::RELOAD.into(), 40);
        };
        out
    }

    add_dmr_roadborn(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let mut crit_mult = 1.0;
        if _input.value > 0 {
            crit_mult = 1.17;
        };
        DamageModifierResponse {
            crit_scale: crit_mult,
            explosive_dmg_scale: 1.0,
            impact_dmg_scale: 1.0,
        }
    }

    add_fmr_roadborn(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> FiringModifierResponse {
        let mut delay_mult = 1.0;
        if _input.value > 0 {
            delay_mult = 0.583;
        };
        FiringModifierResponse {
            burst_delay_scale: delay_mult,
            burst_delay_add: 0.0,
            inner_burst_scale: 1.0,
            burst_size_add: 0.0,
        }
    }

    add_rmr_roadborn(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> RangeModifierResponse {
        let mut range_scale = 1.05;
        if _input.value > 0 {
            range_scale = 1.15; //roughly
        };
        RangeModifierResponse {
            range_stat_add: 0,
            range_all_scale: range_scale,
            range_hip_scale: 1.0,
            range_zoom_scale: 1.0,
        }
    }

    add_rsmr_roadborn(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> ReloadModifierResponse {
        let mut reload = 0;
        if _input.value > 0 {
            reload = 40;
        };
        ReloadModifierResponse {
            reload_stat_add: reload,
            reload_time_scale: 1.0,
        }
    }

    add_fmr_reign_havoc(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> FiringModifierResponse {
        let mut delay_mult = 1.0;
        if _input.calc_data.shots_fired_this_mag >= _input.calc_data.base_mag * 0.2 {
            delay_mult = 0.75;
        };
        if _input.calc_data.shots_fired_this_mag >= _input.calc_data.base_mag * 0.4 {
            delay_mult = 0.625;
        };
        FiringModifierResponse {
            burst_delay_scale: delay_mult,
            burst_delay_add: 0.0,
            inner_burst_scale: 1.0,
            burst_size_add: 0.0,
        }
    }

    add_edr_reign_havoc(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> ExtraDamageResponse {
        let dmg = if _input.pvp { 65.0 } else { 65.0 * 1.3 };
        ExtraDamageResponse {
            additive_damage: dmg,
            increment_total_time: false,
            times_to_hit: 1,
            time_for_additive_damage: 0.0,
            hit_at_same_time: true,
            is_dot: false,
            weapon_scale: true,
            crit_scale: false,
            combatant_scale: true,
        }
    }

    add_dmr_worms_hunger(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let val = clamp(_input.value, 0, 20);
        DamageModifierResponse {
            impact_dmg_scale: 1.0 + (val as f64) * 0.1,
            explosive_dmg_scale: 1.0 + (val as f64) * 0.1,
            crit_scale: 1.0,
        }
    }

    add_dmr_lagragian_sight(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let mut damage_buff = 1.0;
        if _input.value > 0 && _input.calc_data.time_total < 30.0 {
            damage_buff = 1.4;
        };
        DamageModifierResponse {
            impact_dmg_scale: damage_buff,
            explosive_dmg_scale: damage_buff,
            crit_scale: 1.0,
        }
    }

    add_dmr_tom(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let mut damage_buff = 1.0;
        if _input.calc_data.curr_mag == 1.0 {
            damage_buff = 2.0;
        };
        DamageModifierResponse {
            impact_dmg_scale: damage_buff,
            explosive_dmg_scale: damage_buff,
            crit_scale: 1.0,
        }
    }

    add_refund_tom(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> RefundResponse {
        RefundResponse {
            refund_mag: if _input.calc_data.curr_mag == 0.0 {
                1
            } else {
                0
            },
            refund_reserves: 0,
            crit: false,
            requirement: 1,
        }
    }

    add_edr_rocket_tracers(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> ExtraDamageResponse {
        let dmg = if _input.pvp { 24.0 } else { 105.0 };
        ExtraDamageResponse {
            additive_damage: dmg,
            times_to_hit: 1,
            increment_total_time: false,
            time_for_additive_damage: 0.0,
            hit_at_same_time: true,
            is_dot: false,
            weapon_scale: true,
            crit_scale: false,
            combatant_scale: true,
        }
    }

    // add_edr_guidance_ring(
    //     _input: &CalculationInput,
    //     _input.value: u32,
    //     is_enhanced: bool,
    //     _pvp: bool,
    //     _cached_data: &mut HashMap<String, f64>,
    // ) -> ExtraDamageResponse {
    //     ExtraDamageResponse {
    //         additive_damage: if _input.value > 0 {
    //              _input.calc_data.base_damage *  _input.calc_data.base_crit_mult
    //         } else {
    //             0.0
    //         },
    //         times_to_hit: 2,
    //         increment_total_time: false,
    //         time_for_additive_damage: 0.1,
    //         hit_at_same_time: true,
    //         is_dot: false,
    //         weapon_scale: true,
    //         crit_scale: false,
    //         combatant_scale: true,
    //     }
    // }

    // add_edr_poison_arrows(
    //     _input: &CalculationInput,
    //     _input.value: u32,
    //     is_enhanced: bool,
    //     _pvp: bool,
    //     _cached_data: &mut HashMap<String, f64>,
    // ) -> ExtraDamageResponse {
    //     let last_proc = _cached_data.get("poison_arrows").unwrap_or(&0.0);
    //     let time_diff =  _input.calc_data.time_total - last_proc;
    //     return ExtraDamageResponse {
    //         additive_damage: if _input.value > 0 {
    //              _input.calc_data.base_damage *  _input.calc_data.base_crit_mult
    //         } else {
    //             0.0
    //         },
    //         times_to_hit: (time_diff / 0.5).ceil() as i32,
    //         increment_total_time: false,
    //         time_for_additive_damage: 0.5,
    //         hit_at_same_time: false,
    //         is_dot: true,
    //         weapon_scale: true,
    //         crit_scale: false,
    //         combatant_scale: true,
    //     };
    // }

    add_fmr_hakke_heavy_burst(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> FiringModifierResponse {
        FiringModifierResponse {
            burst_size_add: -2.0,
            burst_delay_add: -1.0 / 30.0,
            ..Default::default()
        }
    }

    add_dmr_hakke_heavy_burst(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let crit_scale = (1.5 + 5.0 / 51.0) / _input.calc_data.base_crit_mult;
        DamageModifierResponse {
            explosive_dmg_scale: 1.48,
            impact_dmg_scale: 1.48,
            crit_scale,
        }
    }

    add_dmr_swooping_talons(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let mut dmg_mult = 1.0;
        if _input.value > 0 {
            dmg_mult = 1.4;
        }
        dmg_mult += _input.calc_data.total_shots_fired * 0.04;
        dmg_mult = clamp(dmg_mult, 1.0, 1.4);
        DamageModifierResponse {
            impact_dmg_scale: dmg_mult,
            explosive_dmg_scale: dmg_mult,
            crit_scale: 1.0,
        }
    }

    add_dmr_ignition_trigger(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let mut dmg_mult = 1.0;
        if _input.value > 0 || _input.calc_data.total_shots_fired > 20.0 {
            dmg_mult = if _input.pvp { 1.55 } else { 1.99 };
        }
        DamageModifierResponse {
            impact_dmg_scale: dmg_mult,
            explosive_dmg_scale: dmg_mult,
            crit_scale: 1.0,
        }
    }

    add_dmr_vex_catalyst(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let mut damage_mult = if _input.value > 0 { 0.2 } else { 0.0 };
        let duration = 5.0;
        if _input.calc_data.time_total > duration {
            damage_mult = 0.0;
        };
        DamageModifierResponse {
            impact_dmg_scale: 1.0 + damage_mult,
            explosive_dmg_scale: 1.0 + damage_mult,
            crit_scale: 1.0,
        }
    }

    add_fmr_ravenous_beast(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> FiringModifierResponse {
        if _input.value > 0 {
            FiringModifierResponse {
                burst_delay_scale: 0.8,
                ..Default::default()
            }
        } else {
            FiringModifierResponse::default()
        }
    }

    add_dmr_ravenous_beast(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let mut damage_mult = 1.0;
        let mut crit_mult = 1.0;
        if _input.value > 0 {
            damage_mult = if _input.pvp { 2.2 } else { 2.87 };
            crit_mult = if _input.pvp {
                1.0 / (1.5 + -3.0 / 51.0)
            } else {
                1.99 / 2.87
            };
        }
        DamageModifierResponse {
            impact_dmg_scale: damage_mult,
            explosive_dmg_scale: damage_mult,
            crit_scale: crit_mult,
        }
    }

    add_sbr_release_the_wolves(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let has_cat = _input.calc_data.perk_value_map.contains_key(&431220296);
        let mut out = HashMap::new();
        if has_cat {
            if _input.value == 0 {
                out.insert(StatHashes::STABILITY.into(), 40);
            } else if _input.value == 1 {
                out.insert(StatHashes::RELOAD.into(), 100);
            }
        }
        out
    }

    add_rsmr_release_the_wolves(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> ReloadModifierResponse {
        let has_cat = _input.calc_data.perk_value_map.contains_key(&431220296);
        if _input.value == 1 && has_cat {
            ReloadModifierResponse {
                reload_stat_add: 100,
                reload_time_scale: 0.85,
            }
        } else {
            ReloadModifierResponse::default()
        }
    }

    add_fmr_release_the_wolves(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> FiringModifierResponse {
        if _input.value > 0 {
            FiringModifierResponse {
                burst_delay_scale: 0.4,
                ..Default::default()
            }
        } else {
            FiringModifierResponse::default()
        }
    }

    add_dmr_release_the_wolves(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let damage_mult = if _input.value > 0 { 1.4 } else { 1.0 };
        DamageModifierResponse {
            impact_dmg_scale: damage_mult,
            explosive_dmg_scale: damage_mult,
            crit_scale: 1.0,
        }
    }

    add_sbr_fundamentals(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut stats = HashMap::new();
        if _input.value == 1 {
            stats.insert(StatHashes::STABILITY.into(), 20);
            stats.insert(StatHashes::AIM_ASSIST.into(), 10);
        } else if _input.value == 2 {
            stats.insert(StatHashes::AIRBORNE.into(), 20);
            stats.insert(StatHashes::RELOAD.into(), 35);
        } else if _input.value == 3 {
            stats.insert(StatHashes::RANGE.into(), 5);
            stats.insert(StatHashes::HANDLING.into(), 25);
        };
        stats
    }

    add_hmr_fundamentals(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HandlingModifierResponse {
        let mut handling = 0;
        if _input.value == 3 {
            handling = 25;
        }
        HandlingModifierResponse {
            stat_add: handling,
            ..Default::default()
        }
    }

    add_rsmr_fundamentals(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> ReloadModifierResponse {
        let mut reload = 0;
        if _input.value == 2 {
            reload = 35;
        }
        ReloadModifierResponse {
            reload_stat_add: reload,
            ..Default::default()
        }
    }

    add_rmr_fundamentals(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> RangeModifierResponse {
        let mut range = 0;
        if _input.value == 3 {
            range = 5;
        }
        RangeModifierResponse {
            range_stat_add: range,
            ..Default::default()
        }
    }

    add_sbr_thin_the_herd(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut out = HashMap::new();
        if _input.value > 0 {
            out.insert(StatHashes::RELOAD.into(), 70);
        }
        out
    }

    add_rsmr_thin_the_herd(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> ReloadModifierResponse {
        if _input.value > 0 {
            ReloadModifierResponse {
                reload_stat_add: 70,
                ..Default::default()
            }
        } else {
            ReloadModifierResponse::default()
        }
    }

    add_hmr_chimera(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HandlingModifierResponse {
        if _input.value > 0 {
            HandlingModifierResponse {
                stat_add: 100,
                ..Default::default()
            }
        } else {
            HandlingModifierResponse::default()
        }
    }

    add_sbr_chimera(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut out = HashMap::new();
        if _input.value > 0 {
            out.insert(StatHashes::RELOAD.into(), 100);
        }
        out
    }

    add_dmr_first_glance(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let mut damage_mult = 1.0;
        let mut crit_mult = 1.0;
        if _input.value > 0 {
            if _input.calc_data.total_shots_fired == 0.0 {
                damage_mult = 1.33;
            } else {
                crit_mult = 1.33;
            };
        };
        DamageModifierResponse {
            explosive_dmg_scale: damage_mult,
            impact_dmg_scale: damage_mult,
            crit_scale: crit_mult,
        }
    }

    add_dmr_fate_of_all_fools(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let mut damage_mult = 1.0;
        let mut crit_mult = 1.0;
        if _input.value as f64 > _input.calc_data.total_shots_fired {
            let cc = _input.calc_data.base_crit_mult;
            damage_mult = cc;
            crit_mult = 1.0 / cc;
        };
        DamageModifierResponse {
            explosive_dmg_scale: damage_mult,
            impact_dmg_scale: damage_mult,
            crit_scale: crit_mult,
        }
    }

    add_dmr_honed_edge(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let mut damage_mult = 1.0;
        let has_cat = _input.calc_data.perk_value_map.contains_key(&529188544);
        if _input.value == 2 {
            damage_mult = if _input.pvp { 1.183 } else { 2.0 };
        } else if _input.value == 3 {
            damage_mult = if _input.pvp { 1.412 } else { 3.0 };
        } else if _input.value == 4 && has_cat {
            damage_mult = if _input.pvp { 1.504 * 1.2 } else { 4.0 * 1.2 };
        } else if _input.value == 4 {
            damage_mult = if _input.pvp { 1.504 } else { 4.0 };
        };
        DamageModifierResponse {
            explosive_dmg_scale: damage_mult,
            impact_dmg_scale: damage_mult,
            crit_scale: 1.0,
        }
    }

    add_dmr_taken_predator(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let mut damage_mult = 1.0;
        if _input.value == 1 || _input.value == 2 {
            damage_mult = 1.25;
        } else if _input.value == 3 {
            damage_mult = 1.25 * 1.25;
        };
        DamageModifierResponse {
            explosive_dmg_scale: damage_mult,
            impact_dmg_scale: damage_mult,
            crit_scale: 1.0,
        }
    }

    add_dmr_markov_chain(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let val = clamp(_input.value, 0, 5);
        let damage_mult = (1.0 / 15.0) * val as f64;
        DamageModifierResponse {
            explosive_dmg_scale: 1.0 + damage_mult,
            impact_dmg_scale: 1.0 + damage_mult,
            crit_scale: 1.0,
        }
    }

    add_dmr_storm_and_stress(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let mut damage_mult = 1.0;
        if _input.value > 0 {
            damage_mult = if _input.pvp { 3.62 } else { 1.8 };
        };
        DamageModifierResponse {
            explosive_dmg_scale: damage_mult,
            impact_dmg_scale: damage_mult,
            crit_scale: 1.0,
        }
    }

    add_rmr_dual_speed_receiver(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> RangeModifierResponse {
        let zoom_stat = _input
            .calc_data
            .stats
            .get(&StatHashes::ZOOM.into())
            .unwrap_or(&Stat::new())
            .val() as f64;
        let zoom_mult = (zoom_stat + 3.0) / zoom_stat;
        if _input.value > 0 {
            RangeModifierResponse {
                range_stat_add: 30,
                range_zoom_scale: zoom_mult,
                ..Default::default()
            }
        } else {
            RangeModifierResponse::default()
        }
    }

    add_sbr_dual_speed_receiver(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut out = HashMap::new();
        if _input.value > 0 {
            out.insert(StatHashes::ZOOM.into(), 3);
            out.insert(StatHashes::RANGE.into(), 30);
        }
        out
    }

    add_dmr_full_stop(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        DamageModifierResponse {
            explosive_dmg_scale: 1.0,
            impact_dmg_scale: 1.0,
            crit_scale: if !_input.pvp { 2.9 } else { 1.0 },
        }
    }

    add_fmr_rat_pack(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> FiringModifierResponse {
        let val;
        if _input.value > 0 {
            val = clamp(_input.value - 1, 0, 4);
        } else {
            val = 0;
        }
        FiringModifierResponse {
            burst_delay_add: val as f64 * (-0.625 / 30.0),
            ..Default::default()
        }
    }

    add_mmr_rat_pack(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> MagazineModifierResponse {
        let val = clamp(_input.value - 1, 0, 4);
        MagazineModifierResponse {
            magazine_add: val as f64 * if val == 4 { 2.25 } else { 2.0 },
            ..Default::default()
        }
    }

    add_fmr_ride_the_bull(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> FiringModifierResponse {
        let extra_value = _input.calc_data.shots_fired_this_mag as f64 / 10.0;
        let val = clamp(_input.value + extra_value as u32, 0, 2);
        FiringModifierResponse {
            burst_delay_add: val as f64 * (-0.25 / 30.0),
            ..Default::default()
        }
    }

    add_fmr_spinning_up(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> FiringModifierResponse {
        let extra_value = _input.calc_data.shots_fired_this_mag as f64 / 12.0;
        let val = clamp(_input.value + extra_value as u32, 0, 2);
        FiringModifierResponse {
            burst_delay_add: val as f64 * (-0.5 / 30.0),
            ..Default::default()
        }
    }

    // add_dmr_noble_rounds(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
    //     if _input.value > 0 {
    //         dmr_blessing_of_the_sky(_input, 1, false, _pvp, _cached_data)
    //     } else {
    //         DamageModifierResponse::default()
    //     }
    // }
}