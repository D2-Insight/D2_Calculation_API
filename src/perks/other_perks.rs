use std::{collections::HashMap, default::default};

use crate::{
    d2_enums::{DamageType, StatHashes, WeaponType},
    enemies::EnemyType,
};

use super::{
    clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExtraDamageResponse, FiringModifierResponse,
        FlinchModifierResponse, HandlingModifierResponse, InventoryModifierResponse,
        MagazineModifierResponse, RangeModifierResponse, RefundResponse, ReloadModifierResponse,
        ReloadOverrideResponse,
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
    HandlingModifierResponse {
        handling_stat_add: 0,
        handling_ads_scale: 1.0,
        handling_swap_scale: 0.9,
    }
}

pub(super) fn hmr_ophidian_aspects(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HandlingModifierResponse {
    HandlingModifierResponse {
        handling_stat_add: 35,
        handling_ads_scale: 1.0,
        handling_swap_scale: 1.0,
    }
}

pub(super) fn rsmr_ophidian_aspects(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    ReloadModifierResponse {
        reload_stat_add: 35,
        reload_time_scale: 1.0,
    }
}

pub(super) fn sbr_ophidian_aspects(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    stats.insert(StatHashes::HANDLING.into(), 35);
    stats.insert(StatHashes::RELOAD.into(), 35);
    stats.insert(StatHashes::AIRBORNE.into(), 10);
    stats
}

pub(super) fn sbr_dragon_shadow(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    stats.insert(StatHashes::HANDLING.into(), 100);
    stats.insert(StatHashes::RELOAD.into(), 100);
    stats
}

pub(super) fn hmr_dragon_shadow(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HandlingModifierResponse {
    HandlingModifierResponse {
        handling_stat_add: 100,
        handling_ads_scale: 1.0,
        handling_swap_scale: 0.95,
    }
}

pub(super) fn rsmr_dragon_shadow(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    ReloadModifierResponse {
        reload_stat_add: 100,
        reload_time_scale: 1.0,
    }
}

pub(super) fn sbr_amplified(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    stats.insert(StatHashes::HANDLING.into(), 40);
    stats
}

pub(super) fn hmr_amplified(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HandlingModifierResponse {
    HandlingModifierResponse {
        handling_stat_add: 40,
        handling_ads_scale: 1.0,
        handling_swap_scale: 0.95,
    }
}

pub(super) fn rsmr_frequency(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    //far far too lazy to do this properly
    ReloadModifierResponse {
        reload_stat_add: 100,
        reload_time_scale: 0.8,
    }
}

pub(super) fn rsmr_flow_state(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    //far far too lazy to do this properly
    ReloadModifierResponse {
        reload_stat_add: 55,
        reload_time_scale: 0.87,
    }
}

pub(super) fn sbr_tempering(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    if _value > 0 {
        stats.insert(StatHashes::AIRBORNE.into(), 20);
    };
    stats
}

pub(super) fn sbr_on_your_mark(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    let val = clamp(_value, 0, 3) as i32;
    if _value > 0 {
        stats.insert(StatHashes::HANDLING.into(), 20 * val);
        stats.insert(StatHashes::RELOAD.into(), 20 * val);
    };
    stats
}

pub(super) fn hmr_on_your_mark(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HandlingModifierResponse {
    let val = clamp(_value, 0, 3) as i32;
    HandlingModifierResponse {
        handling_stat_add: 20 * val,
        handling_ads_scale: 1.0,
        handling_swap_scale: 1.0,
    }
}

pub(super) fn rsmr_on_your_mark(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    let val = clamp(_value, 0, 3) as i32;
    ReloadModifierResponse {
        reload_stat_add: 20 * val,
        reload_time_scale: 0.93,
    }
}

pub(super) fn sbr_heat_rises(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    let mut buff = 20;
    if _value > 0 {
        buff += 50;
    };
    stats.insert(StatHashes::AIRBORNE.into(), buff);
    stats
}

pub(super) fn sbr_hedrons(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    if _value > 0 {
        stats.insert(StatHashes::AIRBORNE.into(), 20);
        stats.insert(StatHashes::AIM_ASSIST.into(), 15);
        stats.insert(StatHashes::STABILITY.into(), 30);
    };
    stats
}

pub(super) fn sbr_quick_charge(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    if *_input.weapon_type == WeaponType::FUSIONRIFLE
        || *_input.weapon_type == WeaponType::SHOTGUN
        || *_input.weapon_type == WeaponType::SIDEARM
    {
        stats.insert(StatHashes::HANDLING.into(), 25);
    };
    stats
}

pub(super) fn dmr_boss_spec(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let damage_mult = if *_input.enemy_type == EnemyType::BOSS {
        1.077
    } else {
        1.0
    };
    DamageModifierResponse {
        impact_dmg_scale: damage_mult,
        explosive_dmg_scale: damage_mult,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_major_spec(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let damage_mult;
    if *_input.enemy_type == EnemyType::MINIBOSS
        || *_input.enemy_type == EnemyType::ELITE
        || *_input.enemy_type == EnemyType::CHAMPION
    {
        damage_mult = 1.077;
    } else {
        damage_mult = 1.0;
    };
    DamageModifierResponse {
        impact_dmg_scale: damage_mult,
        explosive_dmg_scale: damage_mult,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_big_ones_spec(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let damage_mult;
    if *_input.enemy_type == EnemyType::MINIBOSS
        || *_input.enemy_type == EnemyType::ELITE
        || *_input.enemy_type == EnemyType::CHAMPION
        || *_input.enemy_type == EnemyType::BOSS
    {
        damage_mult = 1.077;
    } else {
        damage_mult = 1.0;
    };
    DamageModifierResponse {
        impact_dmg_scale: damage_mult,
        explosive_dmg_scale: damage_mult,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_minor_spec(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let damage_mult = if *_input.enemy_type == EnemyType::MINOR {
        1.077
    } else {
        1.0
    };
    DamageModifierResponse {
        impact_dmg_scale: damage_mult,
        explosive_dmg_scale: damage_mult,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_taken_spec(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let damage_mult = if _value > 0 && !_pvp { 1.1 } else { 1.0 };
    DamageModifierResponse {
        impact_dmg_scale: damage_mult,
        explosive_dmg_scale: damage_mult,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_spike_grenades(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    DamageModifierResponse {
        impact_dmg_scale: 1.5,
        explosive_dmg_scale: 1.0,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_disorienting_grenades(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    DamageModifierResponse {
        impact_dmg_scale: 0.75,
        explosive_dmg_scale: 0.75,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_full_choke(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    if _input.weapon_type == &WeaponType::SHOTGUN && _input.base_crit_mult < 1.15 {
        DamageModifierResponse {
            impact_dmg_scale: 1.0,
            explosive_dmg_scale: 1.0,
            crit_scale: 0.92,
        }
    } else {
        DamageModifierResponse::default()
    }
}

pub(super) fn fmr_accelerated_coils(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> FiringModifierResponse {
    if _input.weapon_type == &WeaponType::LINEARFUSIONRIFLE {
        return FiringModifierResponse {
            burst_delay_add: -0.033,
            ..Default::default()
        };
    }
    FiringModifierResponse {
        burst_delay_add: -0.040,
        ..Default::default()
    }
}

pub(super) fn fmr_liquid_coils(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> FiringModifierResponse {
    if _input.weapon_type == &WeaponType::LINEARFUSIONRIFLE {
        return FiringModifierResponse {
            burst_delay_add: 0.033,
            ..Default::default()
        };
    }
    FiringModifierResponse {
        burst_delay_add: 0.040,
        ..Default::default()
    }
}

pub(super) fn dmr_liquid_coils(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    DamageModifierResponse {
        impact_dmg_scale: 1.02,
        explosive_dmg_scale: 1.02,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_accelerated_coils(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    DamageModifierResponse {
        impact_dmg_scale: 0.982,
        explosive_dmg_scale: 0.982,
        crit_scale: 1.0,
    }
}

pub(super) fn fmr_faster_string_t2(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> FiringModifierResponse {
    FiringModifierResponse {
        burst_delay_add: -2.0 / 30.0,
        ..Default::default()
    }
}

pub(super) fn fmr_faster_string_t1(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> FiringModifierResponse {
    FiringModifierResponse {
        burst_delay_add: -1.0 / 30.0,
        ..Default::default()
    }
}

pub(super) fn fmr_slower_string_t1(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> FiringModifierResponse {
    FiringModifierResponse {
        burst_delay_add: 1.0 / 30.0,
        ..Default::default()
    }
}

pub(super) fn fmr_slower_string_t2(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> FiringModifierResponse {
    FiringModifierResponse {
        burst_delay_add: 2.0 / 30.0,
        ..Default::default()
    }
}

pub(super) fn fmr_assault_mag(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> FiringModifierResponse {
    let hash = _input.intrinsic_hash;
    let tick_amount = if hash == 904 {
        3.0
    } else if hash == 906 {
        2.0
    } else {
        1.0
    };
    if _input.weapon_type == &WeaponType::SHOTGUN {
        FiringModifierResponse {
            burst_delay_add: -(tick_amount / 30.0),
            ..Default::default()
        }
    } else {
        FiringModifierResponse::default()
    }
}

pub(super) fn sbr_tome_of_dawn(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    if _value > 0 {
        stats.insert(StatHashes::AIRBORNE.into(), 50);
    }
    stats
}

pub(super) fn flmr_tome_of_dawn(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> FlinchModifierResponse {
    if _value > 0 {
        FlinchModifierResponse { flinch_scale: 0.80 }
    } else {
        FlinchModifierResponse::default()
    }
}

pub(super) fn sbr_foetracer(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
  ) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    stats.insert(StatHashes::AIRBORNE.into(), 20);
    stats
  }
  
  //SAME AS LUCKY PANTS, NEED TO BE ABLE TO MULTIPLY DMG INCREASE BY STACKS (I WOULD RATHER NOT TYPE OUT 29 IF STATEMENTS)

  /*pub(super) fn dmr_Foetracer(
    _input: &CalculationInput,
    _value: u32,
    _is_enahanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
    ) -> DamageModifierResponse {
    let damage_mult = 
    */
  
  pub(super) fn sbr_mechaneers_tricksleeves(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
  ) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    if _input.weapon_type == &WeaponType::SIDEARM {
       stats.insert(StatHashes::AIRBORNE.into(), 50);
       stats.insert(StatHashes::HANDLING.into(), 100);
       stats.insert(StatHashes::RELOAD.into(), 100);
       //AUTORELOAD NEEDED
    };
    stats
  }
  
  pub(super) fn dmr_mechaneers_tricksleeves(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
  )  -> DamageModifierResponse {
    let mut dmr = DamageModifierResponse::default();
    if _value <= 0 || _input.weapon_type != &WeaponType::SIDEARM {return dmr;};
    let damage_mult = if _pvp { 1.35 } else { 2.0 };
    dmr.explosive_dmg_scale = damage_mult;
    dmr.impact_dmg_scale = damage_mult;
    return dmr;
  }
  
  pub(super) fn sbr_oathkeeper(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
  ) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    if _input.weapon_type == &WeaponType::BOW {
      stats.insert(StatHashes::AIRBORNE.into(), 40);
    };
    stats
  }
     // NEED OATHKEEPER PERFECT DRAW SHENANIGANS
  
  
  
  pub(super) fn sbr_sealed_ahamkara_grasps(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
  ) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    if _value > 0 {
      stats.insert(StatHashes::AIRBORNE.into(), 50);
      //AUTORELOAD NEEDED
     };
     stats
  }
  //LUCKY PANTS ONLY WORKS FOR READY ?!?!?! crazy :(
  pub(super) fn sbr_lucky_pants(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
  ) -> HashMap<u32, i32> {
    if _value > 0 && _input.weapon_type == &WeaponType::HANDCANNON {
        return HashMap::from([(StatHashes::AIRBORNE.into(), 20)]);
    };
    HashMap::new()
  }
  
  pub(super) fn hmr_lucky_pants(
     _input: &CalculationInput,
     _value: u32,
     _is_enhanced: bool,
     _pvp: bool,
     _cached_data: &mut HashMap<String, f64>,
  ) -> HandlingModifierResponse {
    if _input.weapon_type == &WeaponType::HANDCANNON {
      HandlingModifierResponse {
        handling_stat_add: 100,
        handling_ads_scale: 1.0,
        handling_swap_scale: 0.6,
      };
    }
    return HandlingModifierResponse::default();
  }
  

  // LUCKY PANTS DMR, HOW TO MULTIPLY THE 60% DMG INCREASE BY NUMBER OF STACKS

  /* pub(super) fn dmr_lucky_pants(
    _input: &CalculationInput,
     _value: u32,
     _is_enhanced: bool,
     _pvp: bool,
     _cached_data: &mut HashMap<String, f64>,
  ) -> DamageModifierResponse {
    if _input.enemy_type != &EnemyType::PLAYER {
        if _value > 0 && _value < 10 {
            DamageModifierResponse {
                impact_dmg_scale: 1.0 + (0.6 * _value),
                explosive_dmg_scale: 1.0 + (0.6 * _value),
                crit_scale: 1.0,
            };
        }
        else if _value > 9 {
            DamageModifierResponse {
                impact_dmg_scale: 6.0,
                explosive_dmg_scale: 6.0,
                crit_scale: 1.0,
            };
        };
    }
    return DamageModifierResponse::default();
  }    */
  
  // NEED LUCKY PANTS ACCURACY CONE struct thing
  

  pub(super) fn sbr_stompees(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
  ) -> HashMap<u32, i32> {
    HashMap::from([(StatHashes::AIRBORNE.into(), -50)])
  }
  
  pub(super) fn sbr_no_backup_plans(
    _input: &CalculationInput,
    _value: u32,
    _is_enahanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
  ) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    if _input.weapon_type == &WeaponType::SHOTGUN {
        stats.insert(StatHashes::AIRBORNE.into(), 30);
    };
    stats
  }

  pub(super) fn sbr_actium_war_rig(
    _input: &CalculationInput,
    _value: u32,
    _is_enahanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
  ) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    if _input.weapon_type == &WeaponType::AUTORIFLE || _input.weapon_type == &WeaponType::MACHINEGUN {
        stats.insert(StatHashes::AIRBORNE.into(), 30);
    }
    stats
  }

// need autoreload on actium - 10% every 1.5s

pub(super) fn sbr_hallowfire_heart(
    _input: &CalculationInput,
    _value: u32,
    _is_enahanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
  ) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    stats.insert(StatHashes::AIRBORNE.into(), 20);
    stats
  }

  pub(super) fn sbr_lion_rampants(
    _input: &CalculationInput,
    _value: u32,
    _is_enahanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
  ) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    //hipfire only
    if _value > 0 {
    stats.insert(StatHashes::AIRBORNE.into(), 50);
    };
    stats
  }

  pub(super) fn sbr_peacekeepers(
    _input: &CalculationInput,
    _value: u32,
    _is_enahanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
  ) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    if _input.weapon_type == &WeaponType::SUBMACHINEGUN{
        stats.insert(StatHashes::AIRBORNE.into(), 40);
        stats.insert(StatHashes::HANDLING.into(), 100);
    };
    stats
  }

  pub(super) fn hmr_peacekeepers(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
 ) -> HandlingModifierResponse {
    if _input.weapon_type == &WeaponType::SUBMACHINEGUN {
     HandlingModifierResponse {
       handling_stat_add: 100,
       handling_ads_scale: 1.0,
       handling_swap_scale: 0.6,
     };
    }
   return HandlingModifierResponse::default();
 }

 pub(super) fn sbr_peregrine_greaves(
    _input: &CalculationInput,
    _value: u32,
    _is_enahanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
  ) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    stats.insert(StatHashes::AIRBORNE.into(), 20);
    stats
  }

  pub(super) fn sbr_eye_of_another_world(
    _input: &CalculationInput,
    _value: u32,
    _is_enahanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
  ) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    stats.insert(StatHashes::AIRBORNE.into(), 15);
    stats
  }  

  pub(super) fn sbr_astrocyte_verse(
    _input: &CalculationInput,
    _value: u32,
    _is_enahanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
  ) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    stats.insert(StatHashes::AIRBORNE.into(), 30);
    if _value > 0 {
        stats.insert(StatHashes::HANDLING.into(), 100);
    }
    stats
  }

  pub(super) fn sbr_necrotic_grip(
    _input: &CalculationInput,
    _value: u32,
    _is_enahanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
  ) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    if _input.intrinsic_hash == 1863355414 || _input.intrinsic_hash == 2965975126 || _input.intrinsic_hash == 2724693746 {
        stats.insert(StatHashes::AIRBORNE.into(), 30);
    };
    stats
 } 

 pub(super) fn sbr_boots_of_the_assembler(
    _input: &CalculationInput,
    _value: u32,
    _is_enahanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
  ) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    if _input.intrinsic_hash == 2144092201 {
        stats.insert(StatHashes::AIRBORNE.into(), 30);
    };
    stats
 }

 pub(super) fn sbr_rain_of_fire(
    _input: &CalculationInput,
    _value: u32,
    _is_enahanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
  ) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    if _input.weapon_type == &WeaponType::FUSIONRIFLE || _input.weapon_type == &WeaponType::LINEARFUSIONRIFLE {
        stats.insert(StatHashes::AIRBORNE.into(), 30);
    }
    stats
  }

  pub(super) fn sbr_speedloader_slacks(
    _input: &CalculationInput,
    _value: u32,
    _is_enahanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
  ) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    if _value == 1 {
        stats.insert(StatHashes::RELOAD.into(), 40);
        stats.insert(StatHashes::HANDLING.into(), 40); //?
        stats.insert(StatHashes::AIRBORNE.into(), 30);
    }
    else if _value == 2 {
        stats.insert(StatHashes::RELOAD.into(), 40);
        stats.insert(StatHashes::HANDLING.into(), 40); //?
        stats.insert(StatHashes::AIRBORNE.into(), 35);
    }
    else if _value == 3 {
        stats.insert(StatHashes::RELOAD.into(), 45);
        stats.insert(StatHashes::HANDLING.into(), 45); //?
        stats.insert(StatHashes::AIRBORNE.into(), 40);
    }
    else if _value == 4 {
        stats.insert(StatHashes::RELOAD.into(), 50);
        stats.insert(StatHashes::HANDLING.into(), 50); //?
        stats.insert(StatHashes::AIRBORNE.into(), 45);
    }
    else if _value > 4 {
        stats.insert(StatHashes::RELOAD.into(), 55);
        stats.insert(StatHashes::HANDLING.into(), 55); //?
        stats.insert(StatHashes::AIRBORNE.into(), 50);
    }
    stats
  }

  pub(super) fn rsmr_speedloader_slacks(
    _input: &CalculationInput,
    _value: u32,
    _is_enahanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
  ) -> ReloadModifierResponse {
    if _value == 1 {
        ReloadModifierResponse {
            reload_stat_add: 40,
            reload_time_scale: 1.0,
        };
    }
    else if _value == 2 {
        ReloadModifierResponse {
            reload_stat_add: 40,
            reload_time_scale: 0.925,
        };
    }
    else if _value == 3 {
        ReloadModifierResponse {
            reload_stat_add: 45,
            reload_time_scale: 0.915,
        };
    }
    else if _value == 4 {
        ReloadModifierResponse {
            reload_stat_add: 50,
            reload_time_scale: 0.91,
        };   
    }
    else if _value > 4 {
        ReloadModifierResponse {
            reload_stat_add: 55,
            reload_time_scale: 0.89,
        };
    }
    return ReloadModifierResponse::default();
  }

  pub(super) fn dmr_mantle_of_battle_harmony (
    _input: &CalculationInput,
    _value: u32,
    _is_enahanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
  ) -> DamageModifierResponse {
    if _value > 0 {
        if _pvp {
            DamageModifierResponse {
                impact_dmg_scale: 1.15,
                explosive_dmg_scale: 1.15,
                crit_scale: 1.0,
            };
        }
        else if !_pvp {
            DamageModifierResponse {
                impact_dmg_scale: 1.20,
                explosive_dmg_scale: 1.20,
                crit_scale: 1.0,
            };
        }
    }
    return DamageModifierResponse::default();
  }

  pub(super) fn dmr_mask_of_bakris (
    _input: &CalculationInput,
    _value: u32,
    _is_enahanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
  ) -> DamageModifierResponse {
    if _input.enemy_type != &EnemyType::PLAYER {
        if _input.damage_type == &DamageType::ARC {
            DamageModifierResponse {
                impact_dmg_scale: 1.20,
                explosive_dmg_scale: 1.20,
                crit_scale: 1.0,
            };
        }
        // NEED TO BE ABLE TO ACCONT FOR TARGETS BEING SLOWED/FROZEN
    }
    return DamageModifierResponse::default();
  }

  pub(super) fn dmr_ballidorse_wrathweavers (
    _input: &CalculationInput,
    _value: u32,
    _is_enahanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
  ) -> DamageModifierResponse {
    if _input.damage_type == &DamageType::STASIS && !_pvp {
        DamageModifierResponse {
            impact_dmg_scale: 1.15,
            explosive_dmg_scale: 1.15,
            crit_scale: 1.0,
        };
    }
    else if _input.damage_type == &DamageType::STASIS && _pvp {
        DamageModifierResponse {
            impact_dmg_scale: 1.05,
            explosive_dmg_scale: 1.05,
            crit_scale: 1.0,
        };
    }
    return DamageModifierResponse::default();
  }

pub(super) fn sbr_lunafaction_boots (
    _input: &CalculationInput,
    _value: u32,
    _is_enahanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    stats.insert(StatHashes::RELOAD.into(), 100);
    return stats;
}

pub(super) fn rdr_lunafaction_boots (
    _input: &CalculationInput,
    _value: u32,
    _is_enahanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    return ReloadModifierResponse { 
        reload_stat_add: 100,
        reload_time_scale: 0.9, 
        };
}

//SAME AS LUCKY PANTS, NEED TO KNOW HOW TO MULTIPLY THE INCREMENTS BY THE VALUE

/* pub(super) fn dmr_the_path_of_burning_steps (
    _input: &CalculationInput,
    _value: u32,
    _is_enahanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    if _value > 0 {
        if !_pvp {
            DamageModifierResponse {
                impact_dmg_scale: 1.2 + (0.05  * _value),
                explosive_dmg_scale: 1.2 + (0.05 * _value),
                crit_scale: 1.0,
            };
        }
        else if _pvp {
            DamageModifierResponse {
                impact_dmg_scale: 1.15 + (0.05 * _value),
                explosive_dmg_scale: 1.15 + (0.05 * _value),
                crit_scale: 1.0,
            };
        }
    }
    return DamageModifierResponse::default();
} */