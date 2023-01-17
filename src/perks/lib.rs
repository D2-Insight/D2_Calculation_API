use crate::{
    enemies::EnemyType,
    d2_enums::{AmmoType, StatHashes, WeaponSlot, WeaponType},
    weapons::{Stat, FiringConfig},types::rs_types::HandlingResponse
};
use std::{collections::HashMap, ops::Mul};

#[derive(Debug)]
pub struct CalculationInput<'a> {
    pub curr_firing_data: FiringConfig,
    pub base_damage: f64,
    pub base_crit_mult: f64,
    pub shots_fired_this_mag: f64,
    pub total_shots_fired: f64,
    pub total_shots_hit: f64,
    pub base_mag: f64,
    pub curr_mag: f64,
    pub reserves_left: f64,
    pub time_total: f64,
    pub time_this_mag: f64,
    pub stats: HashMap<u32, Stat>,
    pub weapon_type: WeaponType,
    pub weapon_slot: WeaponSlot,
    pub ammo_type: AmmoType,
    pub handling_data: HandlingResponse,
    pub num_reloads: f64,
    pub enemy_type: EnemyType,
    pub has_overshield: bool,
    pub cached_data: Option<&'a mut HashMap<String, f64>>,
}
impl CalculationInput<'_> {
    //stuff like mag size can use this, not reload, damage, etc.
    pub fn construct_pve_sparse(
        _firing_data: FiringConfig,
        _stats: HashMap<u32, Stat>,
        _weapon_type: WeaponType,
        _ammo_type: AmmoType,
        _base_damage: f64,
        _base_crit_mult: f64,
        _base_mag_size: i32,
        _total_shots_hit: i32,
        _total_time: f64,
    ) -> Self{
        Self {
            curr_firing_data: _firing_data,
            base_damage: _base_damage,
            base_crit_mult: _base_crit_mult,
            shots_fired_this_mag: 0.0,
            total_shots_fired: _total_shots_hit as f64,
            total_shots_hit: _total_shots_hit as f64,
            base_mag: _base_mag_size as f64,
            curr_mag: _base_mag_size as f64,
            reserves_left: 100.0,
            time_total: _total_time,
            time_this_mag: 0.0,
            stats: _stats,
            weapon_type: _weapon_type,
            weapon_slot: WeaponSlot::KINETIC,
            ammo_type: _ammo_type,
            handling_data: HandlingResponse::default(),
            num_reloads: 0.0,
            enemy_type: EnemyType::BOSS,
            has_overshield: false,
            cached_data: None,
        }
    }
    pub fn construct_pvp(
        _firing_data: FiringConfig,
        _stats: HashMap<u32, Stat>,
        _weapon_type: WeaponType,
        _ammo_type: AmmoType,
        _base_damage: f64,
        _base_crit_mult: f64,
        _mag_size: f64,
        _has_overshield: bool,
        _handling_data: HandlingResponse,
    ) -> Self {
        Self {
            curr_firing_data: _firing_data,
            base_damage: _base_damage,
            base_crit_mult: _base_crit_mult,
            shots_fired_this_mag: 0.0,
            total_shots_fired: 0.0,
            total_shots_hit: 0.0,
            base_mag: _mag_size,
            curr_mag: _mag_size,
            reserves_left: 100.0,
            time_total: 0.0,
            time_this_mag: 0.0,
            stats: _stats,
            weapon_type: _weapon_type,
            weapon_slot: WeaponSlot::KINETIC,
            ammo_type: _ammo_type,
            handling_data: _handling_data,
            num_reloads: 0.0,
            enemy_type: EnemyType::PLAYER,
            has_overshield: _has_overshield,
            cached_data: None,
        }
    }
    pub fn construct_static(
        _firing_data: FiringConfig,
        _stats: HashMap<u32, Stat>,
        _weapon_type: WeaponType,
        _ammo_type: AmmoType,
    ) -> Self {
        Self {
            curr_firing_data: _firing_data,
            base_damage: 0.0,
            base_crit_mult: 0.0,
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
            weapon_slot: WeaponSlot::KINETIC,
            ammo_type: _ammo_type,
            handling_data: HandlingResponse::default(),
            num_reloads: 0.0,
            enemy_type: EnemyType::ENCLAVE,
            has_overshield: false,
            cached_data: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DamageModifierResponse {
    pub damage_scale: f64,
    pub crit_scale: f64,
}
impl Mul for DamageModifierResponse {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            damage_scale: self.damage_scale * rhs.damage_scale,
            crit_scale: self.crit_scale * rhs.crit_scale,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExtraDamageResponse {
    pub additive_damage: f64,
    pub time_for_additive_damage: f64,
    pub additive_damage_hits: f64,
    pub weapon_scale: bool,
    pub crit_scale: bool,
}

#[derive(Debug, Clone)]
pub struct ReloadModifierResponse {
    pub reload_stat_add: i32,
    pub reload_time_scale: f64,
}

#[derive(Debug, Clone)]
pub struct FiringModifierResponse {
    pub burst_delay_scale: f64,
    pub burst_size_add: f64,
    pub burst_duration_scale: f64,
}

#[derive(Debug, Clone)]
pub struct HandlingModifierResponse {
    pub handling_stat_add: i32,
    pub handling_swap_scale: f64,
    pub handling_ads_scale: f64,
}

#[derive(Debug, Clone)]
pub struct RangeModifierResponse {
    pub range_stat_add: i32,
    pub range_all_scale: f64,
    pub range_hip_scale: f64,
    pub range_zoom_scale: f64,
}

#[derive(Debug, Clone)]
pub struct RefundResponse {
    pub crit: bool,
    pub requirement: i32,
    pub refund: i32,
    pub generate_ammo: bool,
}

#[derive(Debug, Clone)]
pub struct MagazineModifierResponse {
    pub magazine_stat_add: i32,
    pub magazine_scale: f64,
    pub magazine_add: f64,
}

#[derive(Debug, Clone)]
pub struct InventoryModifierResponse {
    pub ammo_stat_add: i32,
    pub ammo_scale: f64,
    pub ammo_add: f64,
}

#[derive(Debug, Clone)]
pub struct ReloadOverideResponse {
    pub valid: bool,
    pub reload_time: f64,
    pub ammo_to_reload: f64,
    pub priority: i32,
    pub increments_reload_count: bool,
    pub uses_ammo: bool,
}
impl ReloadOverideResponse {
    pub fn invalid() -> Self {
        Self {
            valid: false,
            reload_time: 0.0,
            ammo_to_reload: 0.0,
            priority: 0,
            increments_reload_count: false,
            uses_ammo: false,
        }
    }
}
