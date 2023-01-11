//This also includes intrinsic perks, not just exotic
use std::collections::HashMap;

use crate::{D2Enemy::EnemyType, D2Enums::StatHashes};

use super::{
    clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExtraDamageResponse, FiringModifierResponse,
        HandlingModifierResponse, MagazineModifierResponse, RangeModifierResponse, RefundResponse,
        ReloadModifierResponse, ReloadOverideResponse, ReserveModifierResponse,
    },
};

// def emptyMagBuff(_input: FunctionInputData, _perkValue: int) -> ReloadModifierResponse:
//     return ReloadModifierResponse(0, 0.85)

pub(super) fn rsmr_alloy_mag(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> ReloadModifierResponse {
    //also works for rapid fire frames
    ReloadModifierResponse {
        reload_stat_add: 0,
        reload_time_scale: 0.85,
    }
}

pub(super) fn hmr_swap_mag(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
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
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> DamageModifierResponse {
    let bufflist = vec![1.0, 2.92, 3.0, 3.4, 4.25, 6.67, 10.71, 17.36];
    let mut damage_buff = 1.0;
    if _input.curr_mag == 1.0 {
        let num_of_crits = clamp(_input.shots_hit_this_mag as i32, 0, 7);
        damage_buff = bufflist[num_of_crits as usize];
    };
    DamageModifierResponse {
        damage_scale: damage_buff,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_momento_mori(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> DamageModifierResponse {
    let mut damage_buff = 1.0;
    if _value > 0 && _input.total_shots_hit < 7.0 {
        damage_buff = if _pvp { 1.5 } else { 1.285 };
    };
    DamageModifierResponse {
        damage_scale: damage_buff,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_agers_call(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> DamageModifierResponse {
    let mut damage_buff = 1.0;
    if _value > 0 && _input.num_reloads == 0.0 {
        damage_buff = 1.8;
    };
    DamageModifierResponse {
        damage_scale: damage_buff,
        crit_scale: 1.0,
    }
}

pub(super) fn mmr_agers_call(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> MagazineModifierResponse {
    let mut mag_buff = 1.0;
    if _value > 0 && _input.num_reloads == 0.0 {
        mag_buff = 2.0;
    };
    MagazineModifierResponse {
        magazine_stat_add: 0,
        magazine_add: 0.0,
        magazine_scale: mag_buff,
    }
}

pub(super) fn dmr_arbys(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> DamageModifierResponse {
    let mut damage_buff = 1.0;
    if _input.enemy_type == EnemyType::CHAMPION {
        damage_buff = 0.75;
    };
    DamageModifierResponse {
        damage_scale: damage_buff,
        crit_scale: 1.0,
    }
}

pub(super) fn sbr_roadborn(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
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
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> DamageModifierResponse {
    let mut crit_mult = 1.0;
    if _value > 0 {
        crit_mult = 1.17;
    };
    DamageModifierResponse {
        damage_scale: 1.0,
        crit_scale: crit_mult,
    }
}

pub(super) fn fmr_roadborn(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> FiringModifierResponse {
    let mut delay_mult = 1.0;
    if _value > 0 {
        delay_mult = 0.583;
    };
    FiringModifierResponse {
        burst_delay_scale: delay_mult,
        burst_duration_scale: 1.0,
        burst_size_add: 0.0,
    }
}

pub(super) fn rmr_roadborn(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
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
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
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
