use std::collections::HashMap;

use serde::Serialize;

use crate::{
    d2_enums::WeaponType,
    perks::{get_dmg_modifier, get_firing_modifier, lib::CalculationInput},
};

use super::{FiringData, Weapon};

//just to make code cleaner for now
fn ceil(x: f64) -> f64 {
    x.ceil()
}

const RESILIENCE_VALUES: [f64; 11] = [
    185.00, 186.00, 187.00, 188.00, 189.00, 190.00, 192.00, 194.00, 196.00, 198.00, 200.00,
];

fn average_range(_range_data: &Vec<(f64, f64)>, _wanted_percent: f64, _dmagae_floor: f64) -> f64 {
    let mut total_range = 0.0;
    let mut num_entries = 0.0;
    for range_pair in _range_data {
        total_range += (range_pair.1 - range_pair.0) * ((1.0 - _dmagae_floor) * _wanted_percent)
            + range_pair.0;
        num_entries += 1.0;
    }
    total_range / num_entries
}

#[derive(Debug, Clone, Serialize)]
pub struct OptimalKillData {
    pub headshots: i32,
    pub bodyshots: i32,
    #[serde(rename = "timeTaken")]
    pub time_taken: f64,
    //defines how far away this ttk is achievalbe if all hits ar crits
    #[serde(rename = "achievableRange")]
    pub achievable_range: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct BodyKillData {
    pub bodyshots: i32,
    #[serde(rename = "timeTaken")]
    pub time_taken: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResillienceSummary {
    pub value: i32,
    #[serde(rename = "bodyTtk")]
    pub body_ttk: BodyKillData,
    #[serde(rename = "optimalTtk")]
    pub optimal_ttk: OptimalKillData,
}

pub fn calc_ttk(_weapon: &Weapon, _overshield: f64) -> Vec<ResillienceSummary> {
    let mut ttk_data: Vec<ResillienceSummary> = Vec::new();
    let mut persistent_data: HashMap<String, f64> = HashMap::new();

    let tmp_dmg_prof = _weapon.get_damage_profile();
    let impact_dmg = tmp_dmg_prof.0;
    let explosion_dmg = tmp_dmg_prof.1;
    let mut crit_mult = tmp_dmg_prof.2;
    // let damage_delay = tmp_dmg_prof.3;
    if _weapon.weapon_type == WeaponType::SHOTGUN && _weapon.firing_data.burst_size == 12 {
        crit_mult = 1.0; // shawty has no crits
    }

    for i in 0..RESILIENCE_VALUES.len() {
        let health = RESILIENCE_VALUES[i] + _overshield;

        let mut opt_infnite_range = false;
        let mut opt_damage_dealt = 0.0_f64;
        let mut opt_time_taken = 0.0_f64;
        let mut opt_bullets_fired = 0.0_f64;
        let mut opt_bullets_hit = 0.0_f64;
        let mut opt_range_data_vec: Vec<(f64, f64)> = Vec::new();
        let opt_bodyshots = 0;
        let mut opt_headshots = 0;
        let mut opt_bullet_timeline: Vec<(f64, f64)> = Vec::new();
        
        //Optimal ttk
        while opt_bullets_hit < 50.0 {
            //PERK CALCULATIONS////////////
            
            persistent_data.insert("health%".to_string(), opt_damage_dealt/health);
            let calc_input = _weapon.pvp_calc_input(
                opt_bullets_fired,
                opt_bullets_hit,
                opt_time_taken,
                (_overshield - opt_damage_dealt) > 0.0,
            );
            let dmg_mods = get_dmg_modifier(
                _weapon.list_perks().clone(),
                &calc_input,
                true,
                &mut persistent_data,
            );
            let firing_mods = get_firing_modifier(
                _weapon.list_perks().clone(),
                &calc_input,
                true,
                &mut persistent_data,
            );
            let tmp_range_data =
                _weapon.calc_range_falloff(Some(calc_input.clone()), Some(&mut persistent_data), true);
            if tmp_range_data.ads_falloff_start > 998.0 {
                opt_infnite_range = true;
            } else {
                opt_range_data_vec.push((
                    tmp_range_data.ads_falloff_start,
                    tmp_range_data.ads_falloff_end,
                ));
            }
            ///////////////////////////////

            let body_damage = (impact_dmg * dmg_mods.impact_dmg_scale)
                + (explosion_dmg * dmg_mods.explosive_dmg_scale);
            let critical_multiplier = crit_mult * dmg_mods.crit_scale;
            let head_diff = ((impact_dmg * dmg_mods.impact_dmg_scale) * critical_multiplier)
                - (impact_dmg * dmg_mods.impact_dmg_scale);

            let shot_burst_delay = (_weapon.firing_data.burst_delay + firing_mods.burst_delay_add)
                * firing_mods.burst_delay_scale;
            let shot_inner_burst_delay =
                _weapon.firing_data.inner_burst_delay * firing_mods.inner_burst_scale;
            let shot_burst_size =
                _weapon.firing_data.burst_size as f64 + firing_mods.burst_size_add;

            let shot_delay = if opt_bullets_hit % shot_burst_size > 0.0 && opt_bullets_hit > 0.0 {
                shot_inner_burst_delay
            } else if opt_bullets_hit == 0.0 {
                0.0
            } else {
                shot_burst_delay
            };

            if opt_bullets_hit % shot_burst_size == 0.0 {
                opt_bullets_fired += 1.0;
                opt_bullets_hit += 1.0;
            } else {
                opt_bullets_hit += 1.0;
            };

            opt_time_taken += shot_delay;

            opt_bullet_timeline.push((body_damage, head_diff));

            // assume all headshots for first pass
            if (opt_damage_dealt + body_damage + head_diff) >= health {
                opt_headshots += 1;
                opt_damage_dealt += body_damage + head_diff;
                break;
            } else {
                opt_headshots += 1;
                opt_damage_dealt += body_damage + head_diff;
                if _weapon.weapon_type == WeaponType::BOW {
                    opt_time_taken += _weapon
                        .calc_reload_time(Some(calc_input.clone()), Some(&mut persistent_data), true)
                        .reload_time;
                }
            }
        }

        let mut opt_timeline_damage_dealt = opt_damage_dealt;
        let mut opt_timeline_bodyshots = opt_bodyshots;
        let mut opt_timeline_headshots = opt_headshots;

        // walk back and turn headshots to bodyshots
        for timeline_snapshot in opt_bullet_timeline.iter().rev() {
            let _body_damage = timeline_snapshot.0;
            let headshot_diff = timeline_snapshot.1;

            if opt_timeline_damage_dealt - headshot_diff >= health {
                opt_timeline_bodyshots += 1;
                opt_timeline_headshots -= 1;
                opt_timeline_damage_dealt -= headshot_diff;
            } else {
                break;
            }
        }

        let expolsive_percent = explosion_dmg / (impact_dmg + explosion_dmg);
        if expolsive_percent >= 0.9 {
            opt_infnite_range = true;
        }
        let dropoff_wanted: f64 =
            ((opt_damage_dealt - health) / opt_damage_dealt) / (1.0 - expolsive_percent);
        let range_possible = if !opt_infnite_range {
            average_range(
                &opt_range_data_vec,
                dropoff_wanted,
                _weapon.range_formula.floor_percent,
            )
        } else {
            999.9
        };
        let optimal_ttk = OptimalKillData {
            headshots: opt_timeline_headshots,
            bodyshots: opt_timeline_bodyshots,
            time_taken: opt_time_taken,
            achievable_range: range_possible,
        };

        let mut bdy_bullets_hit = 0.0;
        let mut bdy_bullets_fired = 0.0;
        let mut bdy_time_taken = 0.0;
        let mut bdy_damage_dealt = 0.0;
        while bdy_bullets_hit < 50.0 {
            //PERK CALCULATIONS////////////
            persistent_data.insert("health%".to_string(), bdy_damage_dealt/health);
            let calc_input = _weapon.pvp_calc_input(
                bdy_bullets_fired,
                bdy_bullets_hit,
                bdy_time_taken,
                (_overshield - bdy_damage_dealt) > 0.0,
            );
            let dmg_mods = get_dmg_modifier(
                _weapon.list_perks().clone(),
                &calc_input,
                true,
                &mut persistent_data,
            );
            let firing_mods = get_firing_modifier(
                _weapon.list_perks().clone(),
                &calc_input,
                true,
                &mut persistent_data,
            );
            ///////////////////////////////

            let tmp_dmg_prof = _weapon.get_damage_profile();
            let impact_dmg = tmp_dmg_prof.0;
            let explosion_dmg = tmp_dmg_prof.1;

            let body_damage = (impact_dmg * dmg_mods.impact_dmg_scale)
                + (explosion_dmg * dmg_mods.explosive_dmg_scale);

            let shot_burst_delay = (_weapon.firing_data.burst_delay + firing_mods.burst_delay_add)
                * firing_mods.burst_delay_scale;
            let shot_inner_burst_delay =
                _weapon.firing_data.inner_burst_delay * firing_mods.inner_burst_scale;
            let shot_burst_size =
                _weapon.firing_data.burst_size as f64 + firing_mods.burst_size_add;

            let shot_delay = if bdy_bullets_hit % shot_burst_size > 0.0 && bdy_bullets_hit > 0.0 {
                shot_inner_burst_delay
            } else if bdy_bullets_hit == 0.0 {
                0.0
            } else {
                shot_burst_delay
            };

            bdy_time_taken += shot_delay;
            if bdy_bullets_hit % shot_burst_size == 0.0 {
                bdy_bullets_fired += 1.0;
                bdy_bullets_hit += 1.0;
            } else {
                bdy_bullets_hit += 1.0;
            };

            if (bdy_damage_dealt + body_damage) >= health {
                break;
            } else {
                bdy_damage_dealt += body_damage;
            }
        }
        let body_ttk = BodyKillData {
            time_taken: bdy_time_taken,
            bodyshots: bdy_bullets_hit as i32,
        };
        ttk_data.push(ResillienceSummary {
            value: i as i32,
            body_ttk,
            optimal_ttk,
        });
    }
    ttk_data
}

impl Weapon {
    pub fn calc_ttk(&self, _overshield: f64) -> Vec<ResillienceSummary> {
        calc_ttk(self, _overshield)
    }
}
