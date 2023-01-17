use crate::D2Structs::FiringConfig;

use super::ceil;

static RESILIENCE_VALUES: [f64; 11] = [
    185.01, 186.01, 187.01, 188.01, 189.01, 190.01, 192.01, 194.01, 196.01, 198.01, 200.01,
];

fn extra_burst_bullets(_hits_needed: f64, _burst_size: f64) -> f64 {
    if _hits_needed % _burst_size > 0.0 {
        (_hits_needed % _burst_size) - 1.0
    } else {
        0.0
    }
}

fn calc_crit_percent(_health: f64, _hits_needed: f64, _dmg: f64, _crit_mult: f64) -> f64 {
    if _crit_mult <= 1.0 {
        return 0.0;
    }
    let crits_needed =
        ceil((_health - (_hits_needed as f64 * _dmg)) / ((_dmg * _crit_mult) - _dmg))
            / _hits_needed;
    (crits_needed as f64 * 100.0).round() / 100.0
}

pub struct TtkResponse {
    pub ammo_needed: i32,
    pub hits_needed: i32,
    pub optimal_ttk: f64,
    pub crit_percent: f64,
    pub bodyshot_ttk: f64,
}

pub fn simple_calc_ttk(
    _firing_config: FiringConfig,
    _resilience: i32,
    _damage: f64,
    _crit_mult: f64,
    _mag_size: i32,
) -> TtkResponse {
    let mut ammo_needed: f64 = 0.0;
    let mut hits_needed: f64 = 0.0;
    let mut optimal_ttk: f64 = 0.0;
    let mut crit_percent: f64 = 0.0;
    let mut bodyshot_ttk: f64 = 0.0;

    let crit_dmg: f64 = _damage * _crit_mult;
    let dmg: f64 = _damage;
    let health: f64 = RESILIENCE_VALUES[_resilience as usize] as f64;

    let burst_delay: f64 = _firing_config.burst_delay;
    let burst_duration: f64 = _firing_config.burst_duration;
    let burst_size: f64 = _firing_config.burst_size as f64;

    let dmg_per_ammo: f64 = if _firing_config.one_ammo_burst {
        dmg * (burst_size as f64)
    } else {
        dmg
    };
    if _mag_size as f64 * dmg_per_ammo < health {
        //do nothing
    }
    // Theres 4 categoris of firing,
    // Standard: 1 bullet and ammo per shot, no duration
    // Scatter: many bullets but 1 ammo per shot, no duration
    // Pulse: many bullet and ammo per shot, has duration
    // Fusion: many bullets but 1 ammo per shot, has duration
    else if burst_size > 1.0 && burst_duration > 0.0 {
        // Fusion and Pulse
        hits_needed = ceil(health / crit_dmg);
        ammo_needed = ceil(hits_needed / burst_size);
        optimal_ttk = (ammo_needed * burst_delay)
            + (hits_needed * burst_delay)
            + (extra_burst_bullets(hits_needed, burst_size) * burst_delay);
        let body_hits_needed = ceil(health / dmg);
        let body_bursts_needed = ceil(body_hits_needed / burst_size);
        bodyshot_ttk = (body_bursts_needed * burst_delay)
            + (body_hits_needed * burst_delay)
            + (extra_burst_bullets(body_hits_needed, burst_size) * burst_delay);
        crit_percent = calc_crit_percent(health, hits_needed, dmg, _crit_mult);
    } else if burst_size == 1.0 && burst_duration == 0.0 {
        // Standard
        hits_needed = ceil(health / crit_dmg);
        ammo_needed = hits_needed;
        optimal_ttk = (hits_needed - 1.0) * burst_delay;
        bodyshot_ttk = ceil(health / dmg) * burst_delay - burst_delay;
        crit_percent = calc_crit_percent(health, hits_needed, dmg, _crit_mult);
    } else if burst_size > 1.0 && burst_duration == 0.0 {
        // Scatter
        hits_needed = ceil(health / crit_dmg);
        ammo_needed = ceil(hits_needed / burst_size);
        optimal_ttk = (ceil(hits_needed / burst_size) - 1.0) * burst_delay;
        bodyshot_ttk = ceil(ceil(health / dmg) / burst_size) * burst_delay - burst_delay;
        crit_percent = calc_crit_percent(health, hits_needed, dmg, _crit_mult);
    };
    return TtkResponse {
        ammo_needed: ammo_needed as i32,
        hits_needed: hits_needed as i32,
        optimal_ttk: optimal_ttk,
        crit_percent: crit_percent,
        bodyshot_ttk: bodyshot_ttk,
    };
}
