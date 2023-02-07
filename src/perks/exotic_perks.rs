//This also includes intrinsic perks, not just exotic
use std::collections::HashMap;

use crate::{d2_enums::StatHashes, enemies::EnemyType};

use super::{
    clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExtraDamageResponse, FiringModifierResponse,
        HandlingModifierResponse, InventoryModifierResponse, MagazineModifierResponse,
        RangeModifierResponse, RefundResponse, ReloadModifierResponse, ReloadOverrideResponse,
    },
};

pub(super) fn rsmr_alloy_mag(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    //also works for rapid fire frames
    ReloadModifierResponse {
        reload_stat_add: 0,
        reload_time_scale: 0.85,
    }
}

pub(super) fn hmr_swap_mag(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HandlingModifierResponse {
    //also works for quick access sling
    HandlingModifierResponse {
        handling_stat_add: 0,
        handling_ads_scale: 1.0,
        handling_swap_scale: 0.9,
    }
}

pub(super) fn dmr_paracausal_shot(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let bufflist = vec![1.0, 2.92, 3.0, 3.4, 4.25, 6.67, 10.71, 17.36];
    let mut damage_buff = 1.0;
    if _input.curr_mag == 1.0 {
        let num_of_crits = clamp(_input.shots_fired_this_mag as i32, 0, 7);
        damage_buff = bufflist[num_of_crits as usize];
    };
    DamageModifierResponse {
        impact_dmg_scale: damage_buff,
        explosive_dmg_scale: damage_buff,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_momento_mori(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let mut damage_buff = 1.0;
    if _value > 0 && _input.total_shots_fired < 7.0 {
        damage_buff = if _pvp { 1.5 } else { 1.285 };
    };
    DamageModifierResponse {
        impact_dmg_scale: damage_buff,
        explosive_dmg_scale: damage_buff,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_agers_call(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let mut damage_buff = 1.0;
    if _value > 0 && _input.num_reloads == 0.0 {
        damage_buff = 1.8;
    };
    DamageModifierResponse {
        impact_dmg_scale: damage_buff,
        explosive_dmg_scale: damage_buff,
        crit_scale: 1.0,
    }
}

pub(super) fn mmr_agers_call(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> MagazineModifierResponse {
    let mut mag_buff = 1.0;
    if _value > 0 && _input.total_shots_fired == 0.0 {
        mag_buff = 2.0;
    };
    MagazineModifierResponse {
        magazine_scale: mag_buff,
        ..Default::default()
    }
}

pub(super) fn sbr_roadborn(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut out = HashMap::new();
    if _value > 0 {
        out.insert(StatHashes::HANDLING.to_u32(), 20);
        out.insert(StatHashes::RELOAD.to_u32(), 40);
    };
    out
}

pub(super) fn dmr_roadborn(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let mut crit_mult = 1.0;
    if _value > 0 {
        crit_mult = 1.17;
    };
    DamageModifierResponse {
        crit_scale: crit_mult,
        explosive_dmg_scale: 1.0,
        impact_dmg_scale: 1.0,
    }
}

pub(super) fn fmr_roadborn(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> FiringModifierResponse {
    let mut delay_mult = 1.0;
    if _value > 0 {
        delay_mult = 0.583;
    };
    FiringModifierResponse {
        burst_delay_scale: delay_mult,
        burst_delay_add: 0.0,
        inner_burst_scale: 1.0,
        burst_size_add: 0.0,
    }
}

pub(super) fn rmr_roadborn(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> RangeModifierResponse {
    let mut range_scale = 1.05;
    if _value > 0 {
        range_scale = 1.15; //roughly
    };
    RangeModifierResponse {
        range_stat_add: 0,
        range_all_scale: range_scale,
        range_hip_scale: 1.0,
        range_zoom_scale: 1.0,
    }
}

pub(super) fn rsmr_roadborn(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    let mut reload = 0;
    if _value > 0 {
        reload = 40;
    };
    ReloadModifierResponse {
        reload_stat_add: reload,
        reload_time_scale: 1.0,
    }
}

// pub(super) fn fmr_rat_pack(_input: CalculationInput, _value: i32, _is_enhanced: bool, _pvp: bool) -> FiringModifierResponse {
//     let mut burst_size = 0.0;
//     if _value > 0 {
//         burst_size = 1.0;
//     };
//     FiringModifierResponse {
//         burst_delay_scale: 1.0,
//         burst_duration_scale: 1.0,
//         burst_size_add: burst_size,
//     }
// }

pub(super) fn fmr_reign_havoc(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> FiringModifierResponse {
    let mut delay_mult = 1.0;
    if _input.shots_fired_this_mag >= _input.base_mag * 0.2 {
        delay_mult = 0.75;
    };
    if _input.shots_fired_this_mag >= _input.base_mag * 0.4 {
        delay_mult = 0.625;
    };
    FiringModifierResponse {
        burst_delay_scale: delay_mult,
        burst_delay_add: 0.0,
        inner_burst_scale: 1.0,
        burst_size_add: 0.0,
    }
}

pub(super) fn edr_reign_havoc(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ExtraDamageResponse {
    let dmg = if _pvp { 65.0 } else { 65.0 * 1.3 };
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

pub(super) fn dmr_worms_hunger(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let val = clamp(_value, 0, 20);
    DamageModifierResponse {
        impact_dmg_scale: 1.0 + (val as f64) * 0.1,
        explosive_dmg_scale: 1.0 + (val as f64) * 0.1,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_lagragian_sight(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let mut damage_buff = 1.0;
    if _value > 0 && _input.time_total < 30.0 {
        damage_buff = 1.4;
    };
    DamageModifierResponse {
        impact_dmg_scale: damage_buff,
        explosive_dmg_scale: damage_buff,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_tom(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let mut damage_buff = 1.0;
    if _input.curr_mag == 1.0 {
        damage_buff = 2.0;
    };
    DamageModifierResponse {
        impact_dmg_scale: damage_buff,
        explosive_dmg_scale: damage_buff,
        crit_scale: 1.0,
    }
}

pub(super) fn refund_tom(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> RefundResponse {
    RefundResponse {
        refund_mag: if _input.curr_mag == 0.0 { 1 } else { 0 },
        refund_reserves: 0,
        crit: false,
        requirement: 1,
    }
}

pub(super) fn edr_rocket_tracers(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ExtraDamageResponse {
    let dmg = if _pvp { 24.0 } else { 105.0 };
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

// pub(super) fn edr_guidance_ring(
//     _input: &CalculationInput,
//     _value: u32,
//     _is_enhanced: bool,
//     _pvp: bool,
//     _cached_data: &mut HashMap<String, f64>,
// ) -> ExtraDamageResponse {
//     ExtraDamageResponse {
//         additive_damage: if _value > 0 {
//             _input.base_damage * _input.base_crit_mult
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

// pub(super) fn edr_poison_arrows(
//     _input: &CalculationInput,
//     _value: u32,
//     _is_enhanced: bool,
//     _pvp: bool,
//     _cached_data: &mut HashMap<String, f64>,
// ) -> ExtraDamageResponse {
//     let last_proc = _cached_data.get("poison_arrows").unwrap_or(&0.0);
//     let time_diff = _input.time_total - last_proc;
//     return ExtraDamageResponse {
//         additive_damage: if _value > 0 {
//             _input.base_damage * _input.base_crit_mult
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

pub(super) fn rsmr_lunafaction(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    if _value > 0 {
        ReloadModifierResponse {
            reload_stat_add: 100,
            reload_time_scale: 0.9,
        }
    } else {
        ReloadModifierResponse::default()
    }
}


pub(super) fn fmr_hakke_heavy_burst(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> FiringModifierResponse {
    FiringModifierResponse {
        burst_size_add: -2.0,
        burst_delay_add: -1.0/30.0,
        ..Default::default()
    }
}

pub(super) fn dmr_hakke_heavy_burst(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    DamageModifierResponse {
        explosive_dmg_scale: 1.48,
        impact_dmg_scale: 1.48,
        crit_scale: 0.941,
    }
}