use std::collections::HashMap;

use crate::{d2_enums::{StatHashes, WeaponType, DamageType, AmmoType}, logging::{extern_log, LogLevel}};

use super::{
    clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExtraDamageResponse, FiringModifierResponse,
        HandlingModifierResponse, RangeModifierResponse, RefundResponse, ReloadModifierResponse,
        ReloadOverrideResponse, FlinchModifierResponse
    },
};

pub(super) fn dmr_ballidorse_wrathweavers( //BALLIDORSE WRATHWEAVERS
  _input: &CalculationInput,
  _value: u32,
  _is_enhanced: bool,
  _pvp: bool,
  _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
  let mut modifier = DamageModifierResponse::default();
  let value = if _pvp {1.05} else {1.15};
  if _input.damage_type == &DamageType::STASIS && _value >=1 {
    modifier.impact_dmg_scale = value;
    modifier.explosive_dmg_scale = value;
  }
  return modifier;
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
    dmr
}

//doesnt work for sturm overcharge, (maybe) memento 
pub(super) fn dmr_lucky_pants(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
  ) -> DamageModifierResponse {
    let mut modifier = 1.0;
    let special_multiplier = if _input.ammo_type == &AmmoType::SPECIAL {0.5} else {1.0};

    if !_pvp {
        modifier += special_multiplier * 0.6 * _value.clamp(0, 10) as f64;
    }

    DamageModifierResponse {
        impact_dmg_scale: modifier,
        explosive_dmg_scale: modifier,
        crit_scale: 1.0,
    }
  } 

pub(super) fn dmr_mask_of_bakris (
  _input: &CalculationInput,
  _value: u32,
  _is_enahanced: bool,
  _pvp: bool,
  _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
  let mut dmr = DamageModifierResponse::default();
  let modifier = if _value > 0 && !_pvp { 1.1 } else { 1.0 };

  if _input.damage_type == &DamageType::ARC {
    dmr.impact_dmg_scale =  modifier * modifier;
    dmr.explosive_dmg_scale = modifier * modifier;

  } else {
    dmr.impact_dmg_scale = modifier;
    dmr.explosive_dmg_scale = modifier;
  }
  dmr
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
  HashMap::from([(StatHashes::AIRBORNE.into(), 20)])
}


  pub(super) fn dmr_foetracer(
    _input: &CalculationInput,
    _value: u32,
    _is_enahanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
    ) -> DamageModifierResponse {
        if _value == 0 {return DamageModifierResponse::default();}
        let health_percent = _cached_data.get("health%").unwrap_or(&0.0).clone();
        if health_percent <= 0.7 {return DamageModifierResponse::default();}
        let modifier = 1.0 + (0.01 * (health_percent-0.7)*100.0);
        return DamageModifierResponse {
            impact_dmg_scale: modifier,
            explosive_dmg_scale: modifier,
            crit_scale: 1.0,
        }
    }
  
//TODO: MECHANEER'S TRICKSLEEVES AUTORELOAD

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
  };
  stats
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
  };
  stats
  }

//TODO: AUTORELOAD FOR SEALED AHAMKARA GRASPS
//TODO: LUCKY PANTS AFFECTING ACCURACY CONE 
  //LUCKY PANTS ONLY WORKS FOR READY ?!?!?! crazy :(
pub(super) fn sbr_lucky_pants(
  _input: &CalculationInput,
  _value: u32,
  _is_enhanced: bool,
  _pvp: bool,
  _cached_data: &mut HashMap<String, f64>,
  ) -> HashMap<u32, i32> {
    let mut stat = HashMap::new();
    if _value > 0 && _input.weapon_type == &WeaponType::HANDCANNON {
      stat.insert(StatHashes::AIRBORNE.into(), 20);
      stat.insert(StatHashes::HANDLING.into(), 100);
    };
    stat
  }
  
  pub(super) fn hmr_lucky_pants(
     _input: &CalculationInput,
     _value: u32,
     _is_enhanced: bool,
     _pvp: bool,
     _cached_data: &mut HashMap<String, f64>,
  ) -> HandlingModifierResponse {
    if _value > 0 && _input.weapon_type == &WeaponType::HANDCANNON {
      return HandlingModifierResponse {
        handling_stat_add: 100,
        handling_ads_scale: 1.0,
        handling_swap_scale: 0.6,
      };
    }
    return HandlingModifierResponse::default();
  }

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

//TODO: AUTORELOAD ON ACTIUM WAR RIG

pub(super) fn sbr_hallowfire_heart(
    _input: &CalculationInput,
    _value: u32,
    _is_enahanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
  ) -> HashMap<u32, i32> {
    HashMap::from([(StatHashes::AIRBORNE.into(), 20)])
  }

  pub(super) fn sbr_lion_rampants(
    _input: &CalculationInput,
    _value: u32,
    _is_enahanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
  ) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
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
      return HandlingModifierResponse {
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
    HashMap::from([(StatHashes::AIRBORNE.into(), 20)])
  }

  pub(super) fn sbr_eye_of_another_world(
    _input: &CalculationInput,
    _value: u32,
    _is_enahanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
  ) -> HashMap<u32, i32> {
    HashMap::from([(StatHashes::AIRBORNE.into(), 15)])
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
    if _input.intrinsic_hash == 1863355414 || _input.intrinsic_hash == 2965975126 || _input.intrinsic_hash == 2724693746 { //Thorn, Osteo Striga, Touch of Malice
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
  if _input.intrinsic_hash == 2144092201 { //Lumina
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
    let modifiers= match _value {
        0 => ( 0,  0,  0),
        1 => (40, 40, 30),
        2 => (40, 40, 35),
        3 => (45, 45, 40),
        4 => (50, 50, 45),
        5 => (55, 55, 50),
        _ => (55, 55, 50),
    };

    HashMap::from([
    (StatHashes::RELOAD.into(), modifiers.0),
    (StatHashes::HANDLING.into(), modifiers.1), //?
    (StatHashes::AIRBORNE.into(), modifiers.2)
    ])
  }

  pub(super) fn rsmr_speedloader_slacks(
    _input: &CalculationInput,
    _value: u32,
    _is_enahanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
  ) -> ReloadModifierResponse {
    let modifiers= match _value {
        0 => ( 0, 1.0),
        1 => (40, 1.0),
        2 => (40, 0.925),
        3 => (45, 0.915),
        4 => (50, 0.91),
        5 => (55, 0.89),
        _ => (55, 0.89),
    };

    
    ReloadModifierResponse { 
        reload_stat_add: modifiers.0, 
        reload_time_scale: modifiers.1 
    }
  }

pub(super) fn sbr_lunafaction_boots (
    _input: &CalculationInput,
    _value: u32,
    _is_enahanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
  let mut stat = HashMap::new();
  if _value >= 1 {
    stat.insert(StatHashes::RELOAD.into(), 100);
  }
  stat
}

pub(super) fn rsmr_lunafaction_boots(
  _input: &CalculationInput,
  _value: u32,
  _is_enhanced: bool,
  _pvp: bool,
  _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
  if _value >= 1 {
      ReloadModifierResponse {
          reload_stat_add: 100,
          reload_time_scale: 0.9,
      }
  } else {
      ReloadModifierResponse::default()
  }
}

pub(super) fn rmr_lunafaction_boots(
    _input: &CalculationInput,
    _value: u32,
    _is_enahanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> RangeModifierResponse {
    if _value >= 2 {
        return RangeModifierResponse{
            range_all_scale: 2.0,
            ..Default::default()
        }
    }
    RangeModifierResponse::default()
}