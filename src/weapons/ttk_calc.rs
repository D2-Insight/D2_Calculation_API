use std::collections::HashMap;

use crate::{types::rs_types::{TtkResponse, HandlingResponse}, perks::{lib::CalculationInput, get_dmg_modifier, get_firing_modifier}};

use super::{FiringConfig, Weapon};

//just to make code cleaner for now
fn ceil(x: f64) -> f64 {
    x.ceil()
}

const RESILIENCE_VALUES: [f64; 11] = [
    185.01, 186.01, 187.01, 188.01, 189.01, 190.01, 192.01, 194.01, 196.01, 198.01, 200.01,
];

fn kills_what_resil(_damage: f64, _overshield: f64) -> i32 {
    let mut i = 0;
    while i < 11 {
        if _damage < (RESILIENCE_VALUES[i] + _overshield) && _damage > (RESILIENCE_VALUES[0] + _overshield) {
            return i as i32 - 1;
        }
        i += 1;
    }
    return 11 - 1;
}



pub fn calc_ttk(_weapon: &Weapon, _accuracy: f64, _overshield: f64) -> HashMap<i32, (i32, i32, f64)>{

    let mut damage_dealt =  0.0_f64;
    let mut time_taken =    0.0_f64;
    let mut bullets_hit =   0.0_f64;
    let mut bullets_fired = 0.0_f64;


    let mut last_ressil_killed = 0;
    let mut resil_killed: HashMap<i32, (i32, i32, f64)> = HashMap::new();

    //Optimal ttk
    while last_ressil_killed < 10 {
        //PERK CALCULATIONS////////////
        let calc_input = _weapon.pvp_calc_input(
            bullets_fired,
            bullets_hit,
            time_taken,
            (_overshield-damage_dealt)>0.0
        );
        let dmg_mods = get_dmg_modifier(_weapon.list_perks().clone(), &calc_input, true);
        let firing_mods = get_firing_modifier(_weapon.list_perks().clone(), &calc_input, true);
        ///////////////////////////////

        let body_damage = _weapon.firing_data.damage*dmg_mods.dmg_scale;
        let critical_multiplier = _weapon.firing_data.crit_mult*dmg_mods.crit_scale;

        let shot_burst_delay =
                (_weapon.firing_data.burst_delay + firing_mods.burst_delay_add) * firing_mods.burst_delay_scale;
        let shot_burst_duration = _weapon.firing_data.burst_duration * firing_mods.burst_duration_scale;
        let shot_burst_size = _weapon.firing_data.burst_size as f64 + firing_mods.burst_size_add;
        let shot_inner_burst_delay = shot_burst_duration / (shot_burst_size - 1.0);

        let shot_delay = if bullets_hit%shot_burst_size > 0.0 {
            shot_inner_burst_delay
        } else {
            shot_burst_delay
        };

        
        let resil_kill_body = kills_what_resil(damage_dealt+body_damage, _overshield);
        if resil_kill_body > last_ressil_killed {
            resil_killed.insert(resil_kill_body, (bullets_hit as i32, 1, time_taken));
            last_ressil_killed = resil_kill_body;
        }
        
        let resil_kill_crit = kills_what_resil(damage_dealt+body_damage*critical_multiplier, _overshield);
        if resil_kill_crit > last_ressil_killed {
            resil_killed.insert(resil_kill_crit, (bullets_hit as i32, 0, time_taken));
            last_ressil_killed = resil_kill_crit;
        }

        time_taken += shot_delay;

        if bullets_hit%shot_burst_size == 0.0 {
            bullets_fired += 1.0;
            bullets_hit += 1.0;
        } else {
            bullets_hit += 1.0;
        };
    }
    resil_killed
}




