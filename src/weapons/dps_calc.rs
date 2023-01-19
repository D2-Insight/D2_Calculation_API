
use std::{cell::RefCell, rc::Rc};
use std::collections::HashMap;

use crate::enemies::Enemy;
use crate::perks::lib::{CalculationInput, RefundResponse};
use crate::perks::*;
use crate::types::rs_types::DpsResponse;
use super::{FiringConfig, Weapon};


//first entry in tuple is refund to mag, second is too reserves
pub fn calc_refund(_shots_hit_this_mag: i32, _refunds: Vec<RefundResponse>) -> (i32,i32) {
    let mut refund_ammount = (0,0);
    for refund in _refunds {
        if _shots_hit_this_mag%refund.requirement == 0 {
            refund_ammount.0 += refund.refund_mag;
            refund_ammount.1 += refund.refund_reserves;
        }
    }
    return refund_ammount;
}



pub fn complex_dps_calc(_weapon: Weapon, _enemy: Enemy) -> DpsResponse {
    let weapon = Rc::new(_weapon.clone());
    let stats = weapon.stats.clone();
    let weapon_type = weapon.weapon_type.clone();
    let ammo_type = weapon.ammo_type.clone();
    let base_dmg = weapon.base_damage;
    let base_crit_mult = weapon.base_crit_mult;

    let base_mag = weapon.calc_mag_size(None).mag_size;

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


    let mut reserve = weapon.calc_reserve_size(Some(weapon.static_calc_input())).reserve_size;

    let mut persistent_calc_data: HashMap<String, f64> = HashMap::new();
    while reserve > 0 {
        let mut shots_this_mag = 0;
        //MAGAZINE/////////////////////
        let mag_calc_input = weapon.sparse_calc_input(&total_shots_fired, &total_time);
        let mut mag = weapon.calc_mag_size(Some(mag_calc_input)).mag_size;
        if mag > reserve {
            mag = reserve
        }
        ///////////////////////////////

        //HANDLING/////////////////////
        //This is for stuff like weapon swapping, demo or trench barrel
        let handling_calc_input = weapon.sparse_calc_input(&total_shots_fired, &total_time);
        let handling_data = weapon.calc_handling_times(Some(handling_calc_input));
        ///////////////////////////////
        let start_time = total_time.clone();
        while mag > 0 {
            //DMG MODIFIERS////////////////
            let before_shot_input_data = CalculationInput{
                curr_firing_data: firing_settings.clone(),
                base_damage: base_dmg.clone(),
                base_crit_mult: base_crit_mult.clone(),
                base_mag: base_mag.clone() as f64,
                curr_mag: mag.clone() as f64,
                ammo_type: ammo_type.clone(),
                weapon_type: weapon_type.clone(),
                stats: stats.clone(),
                enemy_type: _enemy.type_.clone(),
                shots_fired_this_mag: shots_this_mag.clone() as f64,
                total_shots_fired: total_shots_fired.clone() as f64,
                total_shots_hit: total_shots_hit.clone() as f64,
                cached_data: Some(&mut persistent_calc_data),
                reserves_left: reserve.clone() as f64,
                time_total: total_time.clone(),
                time_this_mag: (total_time-start_time).clone(),
                weapon_slot: weapon.weapon_slot.clone(),
                handling_data: handling_data.clone(),
                num_reloads: num_reloads.clone() as f64,
                has_overshield: false,
            };
            let dmg_mods = get_dmg_modifier(perks.clone(), &before_shot_input_data, false);
            ///////////////////////////////

            //FIRING MODIFIERS/////////////
            let firing_mods = get_firing_modifier(perks.clone(), &before_shot_input_data, false);
            ///////////////////////////////
            let dmg = (base_dmg*dmg_mods.damage_scale)*(base_crit_mult*dmg_mods.crit_scale);
            let inner_burst_delay = burst_delay*firing_mods.burst_duration_scale / 
                                            (burst_size + firing_mods.burst_size_add - 1.0);
            if firing_settings.one_ammo_burst && burst_size > 1.0 {
                total_shots_fired += 1;
                shots_this_mag += 1;
                total_shots_hit += burst_size as i32;
                total_damage += dmg*burst_size;
                for i in 0..firing_settings.burst_size {
                    time_damage_data.push((total_time + inner_burst_delay*i as f64, dmg));
                }
                total_time += firing_settings.burst_duration
            } else {
                let spec_delay = if shots_this_mag%burst_size as i32 == 0 {
                    burst_delay
                } else {
                    inner_burst_delay
                };
                total_shots_fired += 1;
                shots_this_mag += 1;
                total_shots_hit += burst_size as i32;
                if burst_duration == 0.0 {
                    total_damage += dmg*burst_size;
                    time_damage_data.push((total_time, dmg*burst_size));
                } else {
                    total_damage += dmg;
                    time_damage_data.push((total_time, dmg));
                }
                total_time += spec_delay;
            }
            mag -= 1;

            //REFUNDS//////////////////////
            let mut refund_calc_input = weapon.sparse_calc_input(&total_shots_fired, &total_time);
            refund_calc_input.shots_fired_this_mag = shots_this_mag as f64;
            let refunds = get_refund_modifier(perks.clone(), &refund_calc_input, false);
            let ammo_to_refund = calc_refund(shots_this_mag, refunds);
            mag += ammo_to_refund.0;
            reserve += ammo_to_refund.1;
            ///////////////////////////////


            //EXTRA DMG////////////////////
            
            ///////////////////////////////
            
            //RELOAD OVERRIDE//////////////
            if mag == 0 {

            }
            ///////////////////////////////
        }



        reserve -= mag;

        //RELOAD///////////////////////
        
        ///////////////////////////////
        num_reloads += 1;
        dps_per_mag.push(total_damage/total_time);
    }
    DpsResponse { dps_per_mag, time_damage_data, total_damage, total_time,total_shots: total_shots_fired }
}
