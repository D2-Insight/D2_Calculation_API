use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

use super::{FiringConfig, Weapon};
use crate::d2_enums::AmmoType;
use crate::enemies::Enemy;
use crate::perks::lib::{CalculationInput, RefundResponse, ExtraDamageResponse};
use crate::perks::*;
use crate::types::rs_types::DpsResponse;

//first entry in tuple is refund to mag, second is too reserves
pub fn calc_refund(_shots_hit_this_mag: i32, _refunds: Vec<RefundResponse>) -> (i32, i32) {
    let mut refund_ammount = (0, 0);
    for refund in _refunds {
        if _shots_hit_this_mag % refund.requirement == 0 {
            refund_ammount.0 += refund.refund_mag;
            refund_ammount.1 += refund.refund_reserves;
        }
    }
    return refund_ammount;
}

#[derive(Debug, Clone)]
pub struct ExtraDamageResult {
    pub extra_time: f64,
    pub extra_dmg: f64,
    pub extra_hits: i32,
    pub extra_time_dmg: Vec<(f64,f64)>
}
#[derive(Debug, Clone)]
pub struct ExtraDamageBuffInfo {
    pub pl_buff: f64,
    pub weapon_buff: f64,
    pub crit_buff: f64,
    pub combatant_buff: f64,
}
impl ExtraDamageBuffInfo {
    pub fn get_buff_amount(&self, entry: &ExtraDamageResponse) -> f64{
        let mut buff = self.pl_buff;
        if entry.weapon_scale { buff *= self.weapon_buff };
        if entry.crit_scale { buff *= self.crit_buff };
        if entry.combatant_scale { buff *= self.combatant_buff };
        buff
    }
}
pub fn calc_extra_dmg(_total_time: f64, _extra_dmg_entries: Vec<ExtraDamageResponse>, _dmg_buffs: ExtraDamageBuffInfo) -> ExtraDamageResult {
    let mut extra_time = 0.0;
    let mut extra_dmg = 0.0;
    let mut extra_hits = 0;
    let mut extra_time_dmg: Vec<(f64,f64)> = Vec::new();
    for entry in _extra_dmg_entries {
        if entry.additive_damage > 0.0 {
            if entry.hit_at_same_time {
                let mut bonus_dmg = entry.additive_damage * entry.times_to_hit as f64;
                bonus_dmg *= _dmg_buffs.get_buff_amount(&entry);
                extra_dmg += bonus_dmg;
                if entry.increment_total_time { extra_time += entry.time_for_additive_damage};
                extra_time_dmg.push((_total_time + entry.time_for_additive_damage, bonus_dmg));
                extra_hits += entry.times_to_hit;
            } else if entry.is_dot == false{
                for i in 0..entry.times_to_hit {
                    let mut bonus_dmg = entry.additive_damage;
                    bonus_dmg *= _dmg_buffs.get_buff_amount(&entry);
                    extra_dmg += bonus_dmg;
                    if entry.increment_total_time { extra_time += entry.time_for_additive_damage};
                    extra_time_dmg.push((_total_time + entry.time_for_additive_damage * i as f64, bonus_dmg));
                    extra_hits += 1;
                };
            } else {
                //all dot does is increment time backwards
                for i in 0..entry.times_to_hit {
                    let mut bonus_dmg = entry.additive_damage;
                    bonus_dmg *= _dmg_buffs.get_buff_amount(&entry);
                    extra_dmg += bonus_dmg;
                    if entry.increment_total_time { extra_time += entry.time_for_additive_damage};
                    extra_time_dmg.push((_total_time - entry.time_for_additive_damage * i as f64, bonus_dmg));
                    extra_hits += 1;
                };
            }
        }
    }
    ExtraDamageResult { extra_time, extra_dmg, extra_hits, extra_time_dmg}
}

pub fn complex_dps_calc(_weapon: Weapon, _enemy: Enemy, _pl_dmg_mult: f64) -> DpsResponse {

    let weapon = Rc::new(_weapon.clone());
    let stats = weapon.stats.clone();
    let weapon_type = weapon.weapon_type.clone();
    let ammo_type = weapon.ammo_type.clone();
    let base_dmg = weapon.base_damage;
    let base_crit_mult = weapon.base_crit_mult;

    let base_mag = weapon.calc_ammo_sizes(None).mag_size;
    let maximum_shots = if base_mag*5 < 15 {15} else {base_mag*5};

    let firing_settings = _weapon.firing_data.clone();
    let perks = weapon.list_perks();

    let burst_size = firing_settings.burst_size as f64;
    let burst_delay = firing_settings.burst_delay;
    let burst_duration = firing_settings.burst_duration;

    let mut total_damage = 0.0_f64;
    let mut total_time = 0.0_f64;

    let mut time_damage_data: Vec<(f64, f64)> = Vec::new(); //used for chart stuff
    let mut dps_per_mag: Vec<f64> = Vec::new(); //used for chart stuff

    let mut total_shots_fired = 0_i32;
    let mut total_shots_hit = 0_i32;
    let mut num_reloads = 0_i32;

    let mut reserve = weapon
        .calc_ammo_sizes(Some(weapon.static_calc_input()))
        .reserve_size;

    #[allow(unused_mut)]
    let mut persistent_calc_data: HashMap<String, f64> = HashMap::new();
    while reserve > 0 {
        let mut shots_this_mag = 0;
        //MAGAZINE/////////////////////
        let mag_calc_input = weapon.sparse_calc_input(total_shots_fired, total_time);
        let mut mag = weapon.calc_ammo_sizes(Some(mag_calc_input)).mag_size;
        if mag > reserve {
            mag = reserve
        }
        ///////////////////////////////

        //HANDLING/////////////////////
        //This is for stuff like weapon swapping, demo or trench barrel
        let handling_calc_input = weapon.sparse_calc_input(total_shots_fired, total_time);
        let handling_data = weapon.calc_handling_times(Some(handling_calc_input));
        ///////////////////////////////
        let start_time = total_time.clone();
        while mag > 0 {
            //DMG MODIFIERS////////////////
            let before_shot_input_data = CalculationInput {
                curr_firing_data: &firing_settings,
                base_damage: base_dmg,
                base_crit_mult: base_crit_mult,
                base_mag: base_mag as f64,
                curr_mag: mag as f64,
                ammo_type: &ammo_type,
                weapon_type: &weapon_type,
                stats: &stats,
                enemy_type: &_enemy.type_,
                shots_fired_this_mag: shots_this_mag as f64,
                total_shots_fired: total_shots_fired as f64,
                total_shots_hit: total_shots_hit as f64,
                reserves_left: reserve as f64,
                time_total: total_time,
                time_this_mag: (total_time - start_time),
                weapon_slot: &weapon.weapon_slot,
                handling_data: handling_data,
                num_reloads: num_reloads as f64,
                has_overshield: false,
            };
            let dmg_mods = get_dmg_modifier(perks.clone(), &before_shot_input_data, false);
            ///////////////////////////////

            //FIRING MODIFIERS/////////////
            let firing_mods = get_firing_modifier(perks.clone(), &before_shot_input_data, false);
            ///////////////////////////////

            let dmg = (base_dmg * dmg_mods.dmg_scale)
                * (base_crit_mult * dmg_mods.crit_scale)
                * _pl_dmg_mult
                * weapon.damage_mods.get_mod(&_enemy.type_)
                * weapon.damage_mods.pve;

            let shot_burst_delay = (burst_delay + firing_mods.burst_delay_add) * firing_mods.burst_delay_scale;
            let shot_burst_duration = burst_duration*firing_mods.burst_duration_scale;
            let shot_burst_size = burst_size + firing_mods.burst_size_add;
            let shot_inner_burst_delay = shot_burst_duration / (shot_burst_size - 1.0);

            if firing_settings.one_ammo_burst && burst_size > 1.0 {
                total_shots_fired += 1;
                shots_this_mag += 1;
                total_shots_hit += shot_burst_size as i32;
                total_damage += dmg * shot_burst_size;
                for i in 0..firing_settings.burst_size {
                    time_damage_data.push((total_time + shot_inner_burst_delay * i as f64, dmg));
                }
                total_time += firing_settings.burst_duration
            } else {
                let spec_delay = if shots_this_mag % burst_size as i32 == 0 {
                    shot_burst_delay
                } else {
                    shot_inner_burst_delay
                };
                total_shots_fired += 1;
                shots_this_mag += 1;
                total_shots_hit += 1;
                if burst_duration == 0.0 {
                    total_damage += dmg * burst_size;
                    time_damage_data.push((total_time, dmg * burst_size));
                } else {
                    total_damage += dmg;
                    time_damage_data.push((total_time, dmg));
                }
                total_time += spec_delay;
            }
            mag -= 1;

            //REFUNDS//////////////////////
            let mut refund_calc_input = weapon.sparse_calc_input(total_shots_fired, total_time);
            refund_calc_input.shots_fired_this_mag = shots_this_mag as f64;
            let refunds = get_refund_modifier(perks.clone(), &refund_calc_input, false);
            let ammo_to_refund = calc_refund(shots_this_mag, refunds);
            mag += ammo_to_refund.0;
            reserve += ammo_to_refund.1;
            ///////////////////////////////

            //COMPLEX CALC PRECURSOR//////
            let after_shot_input_data = CalculationInput {
                curr_firing_data: &firing_settings,
                base_damage: base_dmg,
                base_crit_mult: base_crit_mult,
                base_mag: base_mag as f64,
                curr_mag: mag as f64,
                ammo_type: &ammo_type,
                weapon_type: &weapon_type,
                stats: &stats,
                enemy_type: &_enemy.type_,
                shots_fired_this_mag: shots_this_mag as f64,
                total_shots_fired: total_shots_fired as f64,
                total_shots_hit: total_shots_hit as f64,
                reserves_left: reserve as f64,
                time_total: total_time,
                time_this_mag: (total_time - start_time),
                weapon_slot: &weapon.weapon_slot,
                handling_data: handling_data,
                num_reloads: num_reloads as f64,
                has_overshield: false,
            };
            ///////////////////////////////

            //EXTRA DMG////////////////////
            let extra_dmg_responses = get_extra_damage(perks.clone(), &after_shot_input_data, false, &persistent_calc_data);
            let buffs = ExtraDamageBuffInfo{ 
                pl_buff: _pl_dmg_mult, 
                weapon_buff: weapon.damage_mods.pve * dmg_mods.dmg_scale,
                crit_buff: base_crit_mult * dmg_mods.crit_scale,
                combatant_buff: weapon.damage_mods.get_mod(&_enemy.type_),};
            let tmp_out_data = calc_extra_dmg(total_time, extra_dmg_responses, buffs);
            total_damage += tmp_out_data.extra_dmg;
            total_time += tmp_out_data.extra_time;
            total_shots_hit += tmp_out_data.extra_hits;
            time_damage_data.extend(tmp_out_data.extra_time_dmg);
            ///////////////////////////////

            //RELOAD OVERRIDE//////////////
            if mag == 0 {}
            ///////////////////////////////
        }

        reserve -= mag;
        dps_per_mag.push(total_damage / total_time);
        if weapon.ammo_type == AmmoType::PRIMARY {
            if total_shots_fired > maximum_shots {
                break;
            }
        }

        //RELOAD///////////////////////
        let reload_input_data = CalculationInput {
            curr_firing_data: &firing_settings,
            base_damage: base_dmg,
            base_crit_mult: base_crit_mult,
            base_mag: base_mag as f64,
            curr_mag: mag as f64,
            ammo_type: &ammo_type,
            weapon_type: &weapon_type,
            stats: &stats,
            enemy_type: &_enemy.type_,
            shots_fired_this_mag: shots_this_mag as f64,
            total_shots_fired: total_shots_fired as f64,
            total_shots_hit: total_shots_hit as f64,
            reserves_left: reserve as f64,
            time_total: total_time,
            time_this_mag: (total_time - start_time),
            weapon_slot: &weapon.weapon_slot,
            handling_data: handling_data,
            num_reloads: num_reloads as f64,
            has_overshield: false,
        };
        let reload_responses = weapon.calc_reload_time(Some(reload_input_data));
        total_time += reload_responses.reload_time;
        ///////////////////////////////
        num_reloads += 1;
    }
    //sort time_damage_data by time
    time_damage_data.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    DpsResponse {
        dps_per_mag,
        time_damage_data,
        total_damage,
        total_time,
        total_shots: total_shots_fired,
    }
}
