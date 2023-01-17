// use std::cell::RefCell;
// use std::collections::HashMap;

// use crate::D2Enemy::Enemy;
// use crate::perks::lib::CalculationInput;
// use crate::perks::*;
// use crate::D2Structs::FiringConfig;
// use crate::D2Weapon::Weapon;

// #[allow(dead_code)]
// // fn time_to_empty(_mag: i32, _burst_delay: f64, _burst_duration: f64) -> f64 {
// //     let mag = _mag as f64;
// //     mag*_burst_duration + (mag-1.0)*_burst_delay
// // }
// // #[allow(dead_code)]
// // pub fn calc_refund(_mag: i32, _crits_missed: i32, _refunds: Vec<RefundResponse>) -> i32 {
// //     if _refunds.len() == 0 {
// //         return 0;
// //     };
// //     let mut curr_mag = _mag;
// //     let mut shots_fired = 0;
// //     while curr_mag > 0 {
// //         let mut is_crit = true;
// //         if shots_fired < _crits_missed {
// //             is_crit = false;
// //         };
// //         shots_fired += 1;
// //         for refund in &_refunds {
// //             if refund.crit == is_crit || refund.crit == false{
// //                 let req =  refund.requirement as f64;
// //                 if shots_fired as f64 % req == 0.0 {
// //                     curr_mag += refund.refund;
// //                 };
// //             };
// //         };
// //         curr_mag -= 1;
// //     };
// //     return shots_fired;
// // }

// // pub fn calc_dps(_firing_settings: FiringConfig, _damage: f64, _crit_mult: f64, _reload: f64, _mag: i32,
// //                     _reserves: i32, _crits_missed: i32, _refunds: Vec<RefundResponse>) -> Vec<f64>{
// //     let num_mags = (_reserves as f64 / _mag as f64).ceil() as i32;//too much casting
// //     let size_of_last_mag = _reserves % _mag;

// //     let dmg = if _firing_settings.burst_duration == 0.0 {_damage*_firing_settings.burst_size as f64} else {_damage};
// //     let reload_time = if _firing_settings.is_charge {_reload+_firing_settings.burst_delay} else {_reload};

// //     let mut time_taken = 0.0_f64;
// //     let mut damage_dealt = 0.0_f64;
// //     let mut dps_per_mag: Vec<f64> = Vec::new();

// //     let adj_mag_size_last = calc_refund(size_of_last_mag, _crits_missed, _refunds.clone());
// //     let adj_mag_size = calc_refund(_mag, _crits_missed, _refunds);

// //     for i in 0..num_mags {
// //         let mag_idx = i+1;
// //         let mag_size = if mag_idx == num_mags {adj_mag_size_last} else {adj_mag_size};
// //         if mag_idx > 1 {
// //             time_taken += reload_time;
// //         };

// //         time_taken += time_to_empty(mag_size, _firing_settings.burst_delay, _firing_settings.burst_duration);
// //         damage_dealt += mag_size as f64*(dmg*(_crit_mult*((mag_size-_crits_missed) as f64/mag_size as f64)));

// //         dps_per_mag.push(damage_dealt/time_taken);
// //     };
// //     return dps_per_mag;
// // }
// #[derive(Debug, Clone)]
// pub struct DpsReturn {
//     dps_per_mag: Vec<f64>,

//     damage_vec: Vec<f64>,
//     time_vec: Vec<f64>,

//     total_damage: f64,
//     total_shots: f64,
// }
// impl DpsReturn {
//     pub fn new() -> Self {
//         DpsReturn {
//             dps_per_mag: Vec::new(),

//             damage_vec: Vec::new(),
//             time_vec: Vec::new(),

//             total_damage: 0.0,
//             total_shots: 0.0,
//         }
//     }
// }

// pub fn complex_dps_calc(_weapon: Weapon, _enemy: Enemy) -> DpsReturn {
//     //-> DpsReturn
//     // if _weapon.formulas.is_none() {
//     //     return DpsReturn::new();
//     // };
//     let weapon = RefCell::new(_weapon.clone());
//     let stats = weapon.borrow().stats.clone();
//     let weapon_type = weapon.borrow().weapon_type.clone();
//     let ammo_type = weapon.borrow().ammo_type.clone();
//     let base_dmg = weapon.borrow().formulas.firing_data.damage;
//     let base_crit_mult = weapon.borrow().formulas.firing_data.crit_mult;

//     let base_mag = weapon.borrow().magsize(false, 0);

//     let firing_settings = _weapon.firing_data.clone();
//     let perks = weapon.borrow().list_perks();

//     let burst_size = firing_settings.burst_size as f64;
//     let burst_delay = firing_settings.burst_delay;
//     let burst_duration = firing_settings.burst_duration;

//     let mut total_damage = 0.0;
//     let mut time_taken = 0.0;

//     let mut damage_vec: Vec<f64> = Vec::new(); //used for chart stuff
//     let mut time_vec: Vec<f64> = Vec::new(); //used for chart stuff

//     let mut shots_fired = 0;
//     let mut shots_hit = 0;


//     let mut reserve = weapon.borrow().reserves(true);

//     let mut persistent_calc_data: HashMap<String, f64> = HashMap::new();
//     while reserve > 0 {
//         let mut shots_this_mag = 0;
//         //MAGAZINE/////////////////////
//         let mut mag = weapon.borrow().magsize(true,  shots_fired);
//         if mag > reserve {
//             mag = reserve
//         }
//         ///////////////////////////////

//         //HANDLING/////////////////////
//         //This is for stuff like weapon swapping, demo or trench barrel
//         let sparse_calc_input = weapon.borrow().sparse_calc_input(shots_fired);
//         let handling_mod_details = get_handling_modifier(perks.clone(), sparse_calc_input, false);

//         ///////////////////////////////
//         let start_time = time_taken.clone();
//         while mag > 0 {
//             let mut dmg_this_shot = base_dmg*base_crit_mult;
//             //DMG MODIFIERS////////////////
//             let before_shot_input_data = CalculationInput{
//                 curr_firing_data: firing_settings.clone(),
//                 base_damage: base_dmg.clone(),
//                 base_crit_mult: base_crit_mult.clone(),
//                 base_mag: base_mag.clone() as f64,
//                 curr_mag: mag.clone() as f64,
//                 ammo_type: ammo_type.clone(),
//                 weapon_type: weapon_type.clone(),
//                 stats: stats.clone(),
//                 enemy_type: _enemy.type_.clone(),
//                 shots_fired_this_mag: shots_this_mag.clone() as f64,
//                 total_shots_fired: shots_fired.clone() as f64,
//                 total_shots_hit: shots_hit.clone() as f64,
//                 cached_data: Option::from(&mut persistent_calc_data),
//                 reserves_left: reserve as f64,
//                 time_total: time_taken,
//                 time_this_mag: time_taken-start_time,
//                 weapon_slot: weapon.borrow().weapon_slot.clone(),
//                 handling_data: ,
//                 num_reloads: todo!(),
//                 has_overshield: todo!(),
//             };
//             let dmg_mods = get_dmg_modifier(perks.clone(), input_data, false);
//             ///////////////////////////////

//             mag -= 1;

//             //REFUNDS//////////////////////
//             ;
//             ///////////////////////////////


//             //EXTRA DMG////////////////////
//             ;
//             ///////////////////////////////
//         }
//         reserve -= mag;

//         //RELOAD///////////////////////
//         ;
//         ///////////////////////////////
//     }
//     return DpsReturn::new();
// }
