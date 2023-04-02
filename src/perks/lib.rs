use crate::{
    d2_enums::{AmmoType, DamageType, StatHashes, WeaponType, StatBump, BungieHash, DamageSource},
    enemies::EnemyType,
    types::rs_types::HandlingResponse,
    weapons::{FiringData, Stat},
};
use std::{cell::RefCell, collections::HashMap, ops::Mul};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct CalculationInput<'a> {
    pub intrinsic_hash: u32,
    pub curr_firing_data: &'a FiringData,
    pub base_crit_mult: f64,
    pub shots_fired_this_mag: f64,
    pub total_shots_fired: f64,
    pub total_shots_hit: f64,
    pub base_mag: f64,
    pub curr_mag: f64,
    pub reserves_left: f64,
    pub time_total: f64,
    pub time_this_mag: f64,
    pub stats: &'a HashMap<u32, Stat>,
    pub weapon_type: &'a WeaponType,
    pub damage_type: &'a DamageType,
    pub ammo_type: &'a AmmoType,
    pub handling_data: HandlingResponse,
    pub num_reloads: f64,
    pub enemy_type: &'a EnemyType,
    pub perk_value_map: &'a HashMap<u32, u32>,
    pub has_overshield: bool,
}
impl<'a> CalculationInput<'a> {
    //stuff like mag size can use this, not reload, damage, etc.
    pub fn construct_pve_sparse(
        _intrinsic_hash: u32,
        _firing_data: &'a FiringData,
        _stats: &'a HashMap<u32, Stat>,
        _perk_value_map: &'a HashMap<u32, u32>,
        _weapon_type: &'a WeaponType,
        _ammo_type: &'a AmmoType,
        _damage_type: &'a DamageType,
        _base_damage: f64,
        _base_crit_mult: f64,
        _base_mag_size: i32,
        _total_shots_hit: i32,
        _total_time: f64,
    ) -> Self {
        Self {
            intrinsic_hash: _intrinsic_hash,
            curr_firing_data: &_firing_data,
            base_crit_mult: _base_crit_mult,
            shots_fired_this_mag: 0.0,
            total_shots_fired: _total_shots_hit as f64,
            total_shots_hit: _total_shots_hit as f64,
            base_mag: _base_mag_size as f64,
            curr_mag: _base_mag_size as f64,
            reserves_left: 100.0,
            time_total: _total_time,
            time_this_mag: -1.0,
            stats: &_stats,
            weapon_type: &_weapon_type,
            damage_type: _damage_type,
            ammo_type: &_ammo_type,
            handling_data: HandlingResponse::default(),
            num_reloads: 0.0,
            enemy_type: &EnemyType::BOSS,
            perk_value_map: _perk_value_map,
            has_overshield: false,
        }
    }
    pub fn construct_pvp(
        _intrinsic_hash: u32,
        _firing_data: &'a FiringData,
        _stats: &'a HashMap<u32, Stat>,
        _perk_value_map: &'a HashMap<u32, u32>,
        _weapon_type: &'a WeaponType,
        _ammo_type: &'a AmmoType,
        _base_damage: f64,
        _base_crit_mult: f64,
        _mag_size: f64,
        _has_overshield: bool,
        _handling_data: HandlingResponse,
    ) -> Self {
        Self {
            intrinsic_hash: _intrinsic_hash,
            curr_firing_data: _firing_data,
            base_crit_mult: _base_crit_mult,
            shots_fired_this_mag: 0.0,
            total_shots_fired: 0.0,
            total_shots_hit: 0.0,
            base_mag: _mag_size,
            curr_mag: _mag_size,
            reserves_left: 999.0,
            time_total: 0.0,
            time_this_mag: 0.0,
            stats: _stats,
            weapon_type: _weapon_type,
            damage_type: &DamageType::STASIS,
            ammo_type: _ammo_type,
            handling_data: _handling_data,
            num_reloads: 0.0,
            enemy_type: &EnemyType::PLAYER,
            perk_value_map: _perk_value_map,
            has_overshield: _has_overshield,
        }
    }
    pub fn construct_static(
        _intrinsic_hash: u32,
        _firing_data: &'a FiringData,
        _stats: &'a HashMap<u32, Stat>,
        _perk_value_map: &'a HashMap<u32, u32>,
        _weapon_type: &'a WeaponType,
        _ammo_type: &'a AmmoType,
        _crit_mult: f64,
    ) -> Self {
        Self {
            intrinsic_hash: _intrinsic_hash,
            curr_firing_data: _firing_data,
            base_crit_mult: _crit_mult,
            shots_fired_this_mag: 0.0,
            total_shots_fired: 0.0,
            total_shots_hit: 0.0,
            base_mag: 10.0,
            curr_mag: 10.0,
            reserves_left: 100.0,
            time_total: 0.0,
            time_this_mag: 0.0,
            stats: _stats,
            weapon_type: _weapon_type,
            damage_type: &DamageType::STASIS,
            ammo_type: _ammo_type,
            handling_data: HandlingResponse::default(),
            num_reloads: 0.0,
            enemy_type: &EnemyType::ENCLAVE,
            perk_value_map: _perk_value_map,
            has_overshield: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DamageModifierResponse {
    pub impact_dmg_scale: f64,
    pub explosive_dmg_scale: f64,
    pub crit_scale: f64,
}
impl Default for DamageModifierResponse {
    fn default() -> Self {
        Self {
            impact_dmg_scale: 1.0,
            explosive_dmg_scale: 1.0,
            crit_scale: 1.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExtraDamageResponse {
    pub additive_damage: f64,
    pub time_for_additive_damage: f64,
    //basically is this happening concurrently with the main damage?
    pub increment_total_time: bool,
    // will increment shots hit but not shots fired, shots fired is what *most*
    // perks use for calculation EDR shouldn't mess with other perks in unwanted ways
    pub times_to_hit: i32,
    //is_dot takes priority; makes it put dmg*count at in-time+time_for_additive_damage
    //instead of adding time_for_additive_damage between each count
    pub hit_at_same_time: bool,
    //if its a dot the dps calculator will count backwards and apply the dmg
    pub is_dot: bool,
    //pl scalling will apply no matter what
    pub weapon_scale: bool,
    pub crit_scale: bool,
    pub combatant_scale: bool,
}
impl Default for ExtraDamageResponse {
    fn default() -> Self {
        Self {
            additive_damage: 0.0,
            time_for_additive_damage: 0.0,
            increment_total_time: false,
            times_to_hit: 0,
            hit_at_same_time: true,
            is_dot: false,
            weapon_scale: false,
            crit_scale: false,
            combatant_scale: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ReloadModifierResponse {
    pub reload_stat_add: i32,
    pub reload_time_scale: f64,
}
impl Default for ReloadModifierResponse {
    fn default() -> Self {
        Self {
            reload_stat_add: 0,
            reload_time_scale: 1.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct FiringModifierResponse {
    pub burst_delay_scale: f64,
    pub burst_delay_add: f64,
    pub inner_burst_scale: f64,
    pub burst_size_add: f64,
}
impl Default for FiringModifierResponse {
    fn default() -> Self {
        Self {
            burst_delay_scale: 1.0,
            burst_delay_add: 0.0,
            inner_burst_scale: 1.0,
            burst_size_add: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct HandlingModifierResponse {
    pub handling_stat_add: i32,
    pub handling_swap_scale: f64,
    pub handling_ads_scale: f64,
}
impl Default for HandlingModifierResponse {
    fn default() -> Self {
        Self {
            handling_stat_add: 0,
            handling_swap_scale: 1.0,
            handling_ads_scale: 1.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RangeModifierResponse {
    pub range_stat_add: i32,
    pub range_all_scale: f64,
    pub range_hip_scale: f64,
    pub range_zoom_scale: f64,
}
impl Default for RangeModifierResponse {
    fn default() -> Self {
        Self {
            range_stat_add: 0,
            range_all_scale: 1.0,
            range_hip_scale: 1.0,
            range_zoom_scale: 1.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RefundResponse {
    pub crit: bool,
    pub requirement: i32,
    pub refund_mag: i32,
    pub refund_reserves: i32,
}
impl Default for RefundResponse {
    fn default() -> Self {
        Self {
            crit: false,
            requirement: 0,
            refund_mag: 0,
            refund_reserves: 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MagazineModifierResponse {
    pub magazine_stat_add: i32,
    pub magazine_scale: f64,
    pub magazine_add: f64,
}
impl Default for MagazineModifierResponse {
    fn default() -> Self {
        Self {
            magazine_stat_add: 0,
            magazine_scale: 1.0,
            magazine_add: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct InventoryModifierResponse {
    pub inv_stat_add: i32,
    pub inv_scale: f64,
    pub inv_add: f64,
}
impl Default for InventoryModifierResponse {
    fn default() -> Self {
        Self {
            inv_stat_add: 0,
            inv_scale: 1.0,
            inv_add: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct FlinchModifierResponse {
    pub flinch_scale: f64,
}
impl Default for FlinchModifierResponse {
    fn default() -> Self {
        Self { flinch_scale: 1.0 }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VelocityModifierResponse {
    pub velocity_scaler: f64,
}
impl Default for VelocityModifierResponse {
    fn default() -> Self {
        Self {
            velocity_scaler: 1.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReloadOverrideResponse {
    pub valid: bool,
    pub reload_time: f64,
    pub ammo_to_reload: i32,
    pub priority: i32,
    pub count_as_reload: bool,
    pub uses_ammo: bool,
}
impl ReloadOverrideResponse {
    pub fn invalid() -> Self {
        Self {
            //an easy way for dps calculator to throw out
            valid: false,
            reload_time: 0.0,
            ammo_to_reload: 0,
            priority: 0,
            //this will also reset mag stats
            count_as_reload: false,
            uses_ammo: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExplosivePercentResponse {
    pub percent: f64,
    pub delyed: f64,
    pub retain_base_total: bool,
}
impl Default for ExplosivePercentResponse {
    fn default() -> Self {
        Self {
            percent: 0.0,
            delyed: 0.0,
            retain_base_total: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]

pub struct DamageResistModifierResponse {
    pub body_shot_resist: f64,
    pub head_shot_resist: f64,
    pub element: Option<DamageType>,
    pub source: Option<DamageSource>,
}
impl Default for DamageResistModifierResponse {
    fn default() -> Self { 
        Self {
            body_shot_resist: 1.0,
            head_shot_resist: 1.0,
            element: None,
            source: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ModifierResponse{
    pub rmr: Option<RangeModifierResponse>,
    pub dmr: Option<DamageModifierResponse>, 
    pub hmr: Option<HandlingModifierResponse>, 
    pub fmr: Option<FiringModifierResponse>,
    pub flmr: Option<FlinchModifierResponse>,
    pub rsmr: Option<ReloadModifierResponse>,
    pub mmr: Option<MagazineModifierResponse>,
    pub imr: Option<InventoryModifierResponse>,
    pub drmr: Option<DamageResistModifierResponse>,
    pub statbump: Option<HashMap<BungieHash, StatBump>>,
}
    

impl Default for ModifierResponse {
    fn default() -> Self {
        Self {
            rmr: None,
            dmr: None,
            hmr: None,
            fmr: None,
            flmr: None,
            rsmr: None,
            mmr: None,
            imr: None,
            drmr: None,
            statbump: None,
        }
    }
}

