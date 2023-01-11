use crate::D2Structs::{FiringConfig};
use crate::perks::Perk;
use crate::perks::lib::RefundResponse; 


#[allow(dead_code)]


fn time_to_empty(_mag: i32, _burst_delay: f64, _burst_duration: f64) -> f64 {
    let mag = _mag as f64;
    mag*_burst_duration + (mag-1.0)*_burst_delay
}
#[allow(dead_code)]
pub fn calc_refund(_mag: i32, _crits_missed: i32, _refunds: Vec<RefundResponse>) -> i32 {
    if _refunds.len() == 0 {
        return 0;
    };
    let mut curr_mag = _mag;
    let mut shots_fired = 0;
    while curr_mag > 0 {
        let mut is_crit = true;
        if shots_fired < _crits_missed {
            is_crit = false;
        };
        shots_fired += 1;
        for refund in &_refunds {
            if refund.crit == is_crit || refund.crit == false{
                let req =  refund.requirement as f64;
                if shots_fired as f64 % req == 0.0 {
                    curr_mag += refund.refund;
                };
            };
        };
        curr_mag -= 1;
    };
    return shots_fired;
}



pub fn calc_dps(_firing_settings: FiringConfig, _damage: f64, _crit_mult: f64, _reload: f64, _mag: i32,
                    _reserves: i32, _crits_missed: i32, _refunds: Vec<RefundResponse>) -> Vec<f64>{
    let num_mags = (_reserves as f64 / _mag as f64).ceil() as i32;//too much casting
    let size_of_last_mag = _reserves % _mag;

    let dmg = if _firing_settings.burst_duration == 0.0 {_damage*_firing_settings.burst_size as f64} else {_damage};
    let reload_time = if _firing_settings.is_charge {_reload+_firing_settings.burst_delay} else {_reload};

    let mut time_taken = 0.0_f64;
    let mut damage_dealt = 0.0_f64;
    let mut dps_per_mag: Vec<f64> = Vec::new();

    let adj_mag_size_last = calc_refund(size_of_last_mag, _crits_missed, _refunds.clone());
    let adj_mag_size = calc_refund(_mag, _crits_missed, _refunds);


    for i in 0..num_mags {
        let mag_idx = i+1;
        let mag_size = if mag_idx == num_mags {adj_mag_size_last} else {adj_mag_size};
        if mag_idx > 1 {
            time_taken += reload_time;
        };

        time_taken += time_to_empty(mag_size, _firing_settings.burst_delay, _firing_settings.burst_duration);
        damage_dealt += mag_size as f64*(dmg*(_crit_mult*((mag_size-_crits_missed) as f64/mag_size as f64)));

        dps_per_mag.push(damage_dealt/time_taken);
    };
    return dps_per_mag;
}



fn collect_perk_damage(_perks: Vec<Perk>) {
    
}








pub struct DpsReturn {
    dps_per_mag: Vec<f64>,

    damage_vec: Vec<f64>,
    time_vec: Vec<f64>,

    total_damage: f64,
    total_shots: f64
}


pub fn complex_dps_calc(_firing_settings: FiringConfig, _damage: f64, _crit_mult: f64, _reload: f64, _mag: f64,
    _reserves: f64,  _refunds: Vec<RefundResponse>, _perks: Vec<Perk>)  {//-> DpsReturn

    let num_mags = (_reserves as f64 / _mag as f64).ceil() as i32;//too much casting
    let size_of_last_mag = _reserves % _mag;

    let mut burst_size = _firing_settings.burst_size as f64;
    let mut burst_delay = _firing_settings.burst_delay;
    let mut burst_duration = _firing_settings.burst_duration;

    let mut total_damage = 0.0;
    let mut time_taken = 0.0;

    let mut damage_vec: Vec<f64> = Vec::new();
    let mut time_vec: Vec<f64> = Vec::new();

    let mut shots_fired = 0;
    
    if _firing_settings.burst_delay == 0.0 { //makes doing things like shotguns easier
        let mut damage = _damage*burst_size as f64;
        burst_size = 1.0;
    } else {
        let mut damage = _damage;
    }


    for i in 0..num_mags {//itterates through mags
        let mag_idx = i+1;
        let mut curr_mag = if mag_idx == num_mags {size_of_last_mag} else {_mag};;

        while curr_mag > 0.0 {//itterates through bullets in mag
            for perk in &_perks {
                
            }


            shots_fired += 1;
            curr_mag -= 1.0;
        };
    }
}











