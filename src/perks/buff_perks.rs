use std::collections::HashMap;

use crate::d2_enums::{StatHashes, WeaponType, DamageType, AmmoType};

use super::{
    clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExtraDamageResponse, FiringModifierResponse,
        HandlingModifierResponse, RangeModifierResponse, RefundResponse, ReloadModifierResponse,
        ReloadOverrideResponse,
    },
};

fn emp_buff(_cached_data: &mut HashMap<String, f64>, _desired_buff: f64) -> f64 {
    let current_buff = _cached_data.get("empowering").unwrap_or(&1.0).to_owned();
    if current_buff >= _desired_buff {
        return 1.0;
    } else {
        _cached_data.insert("empowering".to_string(), _desired_buff);
        return _desired_buff / current_buff;
    }
}

fn gbl_debuff(_cached_data: &mut HashMap<String, f64>, _desired_buff: f64) -> f64 {
    let current_buff = _cached_data.get("debuff").unwrap_or(&1.0).to_owned();
    if current_buff >= _desired_buff {
        return 1.0;
    } else {
        _cached_data.insert("debuff".to_string(), _desired_buff);
        return _desired_buff / current_buff;
    }
}

//surge mod dmr is in meta_perks.rs

//
// BUFFS
//
pub(super) fn dmr_well_of_radiance(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let buff = emp_buff(_cached_data, 1.25);
    DamageModifierResponse {
        impact_dmg_scale: buff,
        explosive_dmg_scale: buff,
        ..Default::default()
    }
}

pub(super) fn dmr_blessing_of_the_sky(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    if _value == 0 {return DamageModifierResponse::default()}
    let des_buff = if _pvp { 1.15 } else { 1.35 };
    let buff = emp_buff(_cached_data, des_buff);
    DamageModifierResponse {
        impact_dmg_scale: buff,
        explosive_dmg_scale: buff,
        ..Default::default()
    }
}

pub(super) fn dmr_radiant(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let des_buff = if _pvp { 1.1 } else { 1.25 };
    let buff = emp_buff(_cached_data, des_buff);
    _cached_data.insert("radiant".to_string(), 1.0);
    DamageModifierResponse {
        impact_dmg_scale: buff,
        explosive_dmg_scale: buff,
        ..Default::default()
    }
}

pub(super) fn dmr_path_of_burning_steps(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    if _value == 0 {return DamageModifierResponse::default()}
    let pvp_values = [1.15, 1.25, 1.2, 1.35];
    let pve_values = [1.2, 1.25, 1.35, 1.4];
    let des_buff = if _pvp {
        pvp_values[clamp(_value-1, 0, 3) as usize]
    } else {
        pve_values[clamp(_value-1, 0, 3) as usize]
    };
    let buff = emp_buff(_cached_data, des_buff);
    DamageModifierResponse {
        impact_dmg_scale: buff,
        explosive_dmg_scale: buff,
        ..Default::default()
    }
}

pub(super) fn dmr_banner_shield(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let des_buff = if _pvp { 1.35 } else { 1.4 };
    let buff = emp_buff(_cached_data, des_buff);
    DamageModifierResponse {
        impact_dmg_scale: buff,
        explosive_dmg_scale: buff,
        ..Default::default()
    }
}

pub(super) fn dmr_empowering_rift(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let des_buff = if _pvp { 1.15 } else { 1.2 };
    let buff = emp_buff(_cached_data, des_buff);
    DamageModifierResponse {
        impact_dmg_scale: buff,
        explosive_dmg_scale: buff,
        ..Default::default()
    }
}

pub(super) fn dmr_mantle_of_battle_harmony(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let des_buff = if _pvp { 1.15 } else { 1.2 };
    let buff = emp_buff(_cached_data, des_buff);
    DamageModifierResponse {
        impact_dmg_scale: buff,
        explosive_dmg_scale: buff,
        ..Default::default()
    }
}

pub(super) fn dmr_ward_of_dawn(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let buff = emp_buff(_cached_data, 1.25);
    DamageModifierResponse {
        impact_dmg_scale: buff,
        explosive_dmg_scale: buff,
        ..Default::default()
    }
}

pub(super) fn dmr_gyrfalcon(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let des_buff = if _pvp { 1.0 } else { 1.35 };
    let buff = emp_buff(_cached_data, des_buff);
    DamageModifierResponse {
        impact_dmg_scale: buff,
        explosive_dmg_scale: buff,
        ..Default::default()
    }
}

pub(super) fn dmr_aeon_insight(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let des_buff = if _pvp { 1.0 } else { 1.35 };
    let buff = emp_buff(_cached_data, des_buff);
    DamageModifierResponse {
        impact_dmg_scale: buff,
        explosive_dmg_scale: buff,
        ..Default::default()
    }
}

pub(super) fn dmr_umbral_vow_mod(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let pve_values = [1.2, 1.25, 1.35, 1.4];
    let des_buff = if _pvp { 1.0 } else { pve_values[clamp(_value, 0, 3) as usize] };
    let buff = emp_buff(_cached_data, des_buff);
    DamageModifierResponse {
        impact_dmg_scale: buff,
        explosive_dmg_scale: buff,
        ..Default::default()
    }
}

//
// DEBUFFS
//

pub(super) fn dmr_weaken(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let des_debuff = if _pvp { 1.075 } else { 1.15 };
    let debuff = gbl_debuff(_cached_data, des_debuff);
    DamageModifierResponse {
        impact_dmg_scale: debuff,
        explosive_dmg_scale: debuff,
        ..Default::default()
    }
}

pub(super) fn dmr_tractor_cannon(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let des_debuff = if _pvp { 1.5 } else { 1.3 };
    let debuff = gbl_debuff(_cached_data, des_debuff);
    DamageModifierResponse {
        impact_dmg_scale: debuff,
        explosive_dmg_scale: debuff,
        ..Default::default()
    }
}

pub(super) fn dmr_tether(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let des_debuff = if _pvp { 1.5 } else { 1.3 };
    let debuff = gbl_debuff(_cached_data, des_debuff);
    DamageModifierResponse {
        impact_dmg_scale: debuff,
        explosive_dmg_scale: debuff,
        ..Default::default()
    }
}

pub(super) fn dmr_felwinters_helm(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let debuff = gbl_debuff(_cached_data, 1.3);
    DamageModifierResponse {
        impact_dmg_scale: debuff,
        explosive_dmg_scale: debuff,
        ..Default::default()
    }
}

pub(super) fn dmr_dsc_scanner_mod(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let pve_values = [1.08, 1.137, 1.173, 1.193, 1.2];
    let des_debuff = if _pvp { 1.0 } else { pve_values[clamp(_value, 0, 4) as usize] };
    let debuff = gbl_debuff(_cached_data, des_debuff);
    DamageModifierResponse {
        impact_dmg_scale: debuff,
        explosive_dmg_scale: debuff,
        ..Default::default()
    }
}
