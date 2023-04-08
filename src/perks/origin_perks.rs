use std::collections::HashMap;

use crate::d2_enums::{StatHashes, WeaponType};

use super::{
    clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExtraDamageResponse, FiringModifierResponse,
        FlinchModifierResponse, HandlingModifierResponse, MagazineModifierResponse,
        RangeModifierResponse, RefundResponse, ReloadModifierResponse, ReloadOverrideResponse,
    }, ModifierResponsInput, Perks, add_dmr, add_hmr, add_rsmr, add_vmr, add_fmr, add_rmr, add_mmr, add_epr, add_sbr
};

pub fn origin_perks() {
    add_rr_veist_stinger(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> RefundResponse {
        let data = _input.cached_data.get("veist_stinger");
        let last_proc;
        if data.is_none() {
            last_proc = 0.0;
        } else {
            last_proc = *data.unwrap();
        };
        let time_since_last_proc = _input.calc_data.time_total - last_proc;
        if time_since_last_proc >= 4.0 && _input.value > 0 {
            let max_refund = _input.calc_data.base_mag - _input.calc_data.curr_mag;
            let refund_amount = (_input.calc_data.base_mag / 4.0).ceil() as i32;
            if max_refund > 0.0 {
                _input.cached_data.insert("veist_stinger".to_string(), _input.calc_data.time_total);
                let final_refund_ammount = clamp(refund_amount, 0, max_refund as i32);
                return RefundResponse {
                    requirement: 1,
                    crit: false,
                    refund_mag: refund_amount,
                    refund_reserves: -final_refund_ammount,
                };
            } else {
                RefundResponse::default()
            }
        } else {
            RefundResponse::default()
        }
    }

    add_dmr_hakke_breache(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let damage_mult = if _input.value > 0 { 0.3 } else { 0.0 };
        DamageModifierResponse {
            impact_dmg_scale: 1.0 + damage_mult,
            explosive_dmg_scale: 1.0 + damage_mult,
            crit_scale: 1.0,
        }
    }

    add_rmr_alacrity(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> RangeModifierResponse {
        let range_add = if _input.value > 0 { 20 } else { 0 };
        RangeModifierResponse {
            range_stat_add: range_add,
            ..Default::default()
        }
    }

    add_rsmr_alacrity(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> ReloadModifierResponse {
        let reload_add = if _input.value > 0 { 50 } else { 0 };
        ReloadModifierResponse {
            reload_stat_add: reload_add,
            ..Default::default()
        }
    }

    add_sbr_alacrity(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut map = HashMap::new();
        let range = if _input.value > 0 { 20 } else { 0 };
        let reload = if _input.value > 0 { 50 } else { 0 };
        let stability = if _input.value > 0 { 20 } else { 0 };
        let aim_assit = if _input.value > 0 { 10 } else { 0 };
        map.insert(StatHashes::RANGE.into(), range);
        map.insert(StatHashes::RELOAD.into(), reload);
        map.insert(StatHashes::STABILITY.into(), stability);
        map.insert(StatHashes::AIM_ASSIST.into(), aim_assit);
        map
    }

    add_sbr_ambush(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut map = HashMap::new();
        let range = if _input.is_enhanced { 30 } else { 20 };
        let handling = if _input.is_enhanced { 40 } else { 20 };
        if _input.calc_data.time_total < 2.0 && _input.value > 0 {
            map.insert(StatHashes::RANGE.into(), range);
            map.insert(StatHashes::HANDLING.into(), handling);
        }
        map
    }

    add_dmr_ambush(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        let damage_mult = if _input.value > 0 { 0.095 } else { 0.0 };
        DamageModifierResponse {
            impact_dmg_scale: 1.0 + damage_mult,
            explosive_dmg_scale: 1.0 + damage_mult,
            crit_scale: 1.0,
        }
    }

    add_hmr_hot_swap(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HandlingModifierResponse {
        let handling_add = if _input.is_enhanced { 60 } else { 30 };
        if _input.value > 0 {
            HandlingModifierResponse {
                stat_add: handling_add,
                ..Default::default()
            }
        } else {
            HandlingModifierResponse::default()
        }
    }

    add_rsmr_fluid_dynamics(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> ReloadModifierResponse {
        let reload_add = if _input.is_enhanced { 35 } else { 30 };
        if _input.calc_data.shots_fired_this_mag <= _input.calc_data.base_mag / 2.0 {
            ReloadModifierResponse {
                reload_stat_add: reload_add,
                reload_time_scale: 1.0,
            }
        } else {
            ReloadModifierResponse::default()
        }
    }

    add_sbr_fluid_dynamics(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut map = HashMap::new();
        let reload = if _input.is_enhanced { 35 } else { 30 };
        let stability = if _input.is_enhanced { 25 } else { 20 };
        if _input.calc_data.shots_fired_this_mag <= _input.calc_data.base_mag / 2.0 && _input.value > 0 {
            map.insert(StatHashes::RELOAD.into(), reload);
            map.insert(StatHashes::STABILITY.into(), stability);
        }
        map
    }

    add_rsmr_quiet_moment(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> ReloadModifierResponse {
        if _input.value > 0 {
            ReloadModifierResponse {
                reload_stat_add: 40,
                reload_time_scale: 0.95,
            }
        } else {
            ReloadModifierResponse::default()
        }
    }

    add_sbr_quiet_moment(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut map = HashMap::new();
        if _input.value > 0 {
            map.insert(StatHashes::RELOAD.into(), 40);
        }
        map
    }

    add_rsmr_bitter_spite(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> ReloadModifierResponse {
        let val = clamp(_input.value, 0, 5) as i32;
        let mult = match val {
            0 => 1.0,
            1 => 0.97,
            2 => 0.96,
            3 => 0.95,
            4 => 0.92,
            5 => 0.90,
            _ => 0.90,
        };
        ReloadModifierResponse {
            reload_stat_add: val * 10,
            reload_time_scale: mult,
        }
    }

    add_sbr_bitter_spite(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut map = HashMap::new();
        let val = clamp(_input.value, 0, 5) as i32;
        map.insert(StatHashes::RELOAD.into(), val * 10);
        map
    }

    add_rmr_right_hook(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> RangeModifierResponse {
        let range_add = if _input.is_enhanced { 20 } else { 10 };
        if _input.value > 0 {
            RangeModifierResponse {
                range_stat_add: range_add,
                ..Default::default()
            }
        } else {
            RangeModifierResponse::default()
        }
    }

    add_sbr_right_hook(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut map = HashMap::new();
        let stat_bump = if _input.is_enhanced { 20 } else { 10 };
        if _input.value > 0 {
            map.insert(StatHashes::AIM_ASSIST.into(), stat_bump);
            map.insert(StatHashes::RANGE.into(), stat_bump);
        }
        map
    }

    add_hmr_search_party(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HandlingModifierResponse {
        HandlingModifierResponse {
            ads_scale: 0.85,
            ..Default::default()
        }
    }

    add_mmr_runneth_over(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> MagazineModifierResponse {
        let val = clamp(_input.value, 0, 5) as f64;
        MagazineModifierResponse {
            magazine_scale: 1.0 + val * 0.1,
            ..Default::default()
        }
    }

    add_sbr_tex_balanced_stock(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut map = HashMap::new();
        if _input.value > 0 {
            map.insert(StatHashes::HANDLING.into(), 20);
            map.insert(StatHashes::RELOAD.into(), 20);
        }
        map
    }

    add_hmr_tex_balanced_stock(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HandlingModifierResponse {
        if _input.value > 0 {
            HandlingModifierResponse {
                stat_add: 50,
                ..Default::default()
            }
        } else {
            HandlingModifierResponse::default()
        }
    }

    add_rsmr_tex_balanced_stock(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> ReloadModifierResponse {
        if _input.value > 0 {
            ReloadModifierResponse {
                reload_stat_add: 20,
                reload_time_scale: 0.9,
                ..Default::default()
            }
        } else {
            ReloadModifierResponse::default()
        }
    }

    add_sbr_suros_synergy(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
        let mut out = HashMap::new();
        if _input.value > 0 {
            out.insert(StatHashes::HANDLING.into(), 40);
        }
        out
    }

    add_hmr_suros_synergy(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> HandlingModifierResponse {
        if _input.value > 0 {
            HandlingModifierResponse {
                stat_add: 40,
                ..Default::default()
            }
        } else {
            HandlingModifierResponse::default()
        }
    }

    add_flmr_suros_synergy(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> FlinchModifierResponse {
        if _input.value > 0 {
            FlinchModifierResponse { flinch_scale: 0.80 }
        } else {
            FlinchModifierResponse::default()
        }
    }
}
