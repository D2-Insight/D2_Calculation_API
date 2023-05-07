// pub mod dps_calc;
// pub mod reserve_calc;
// pub mod stat_calc;
// pub mod ttk_calc;
// pub mod weapon_constructor;

use std::cell::RefCell;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::attribute_framework::attributes::Attribute;
use crate::d2_enums::{AmmoType, DamageType, StatHashes, WeaponType};
use crate::enemies::Enemy;
use crate::perks::{
    get_magazine_modifier, get_reserve_modifier, get_stat_bumps, lib::CalculationInput, Perk,
};

use crate::types::rs_types::{
    AmmoFormula, DamageMods, DpsResponse, FiringData, HandlingFormula, RangeFormula, ReloadFormula,
};

#[derive(Debug, Clone)]
pub struct PsuedoWeapon {}

#[derive(Debug, Clone, Serialize)]
pub struct Stat {
    pub base_value: i32,
    pub part_value: i32,
    pub perk_value: i32,
}
impl Stat {
    pub fn new() -> Stat {
        Stat {
            base_value: 0,
            part_value: 0,
            perk_value: 0,
        }
    }
    pub fn val(&self) -> i32 {
        (self.base_value + self.part_value).clamp(0, 100)
    }
    pub fn perk_val(&self) -> i32 {
        (self.base_value + self.part_value + self.perk_value).clamp(0, 100)
    }
}
impl From<i32> for Stat {
    fn from(_val: i32) -> Self {
        Stat {
            base_value: _val,
            part_value: 0,
            perk_value: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Weapon {
    hash: u32,
    intrinsic_hash: u32,

    perks: RefCell<HashMap<u32, Perk>>,
    base_stats: HashMap<StatHashes, i32>,
    #[serde(skip)]
    perk_value_map: HashMap<u32, u32>,

    damage_mods: DamageMods,
    firing_data: FiringData,
    range_formula: RangeFormula,
    ammo_formula: AmmoFormula,
    handling_formula: HandlingFormula,
    reload_formula: ReloadFormula,

    weapon_type: WeaponType,
    damage_type: DamageType,
    ammo_type: AmmoType,
}
impl Weapon {
    fn get_weapon_type(&self) -> WeaponType { self.weapon_type }
    fn get_damage_type(&self) -> DamageType { self.damage_type }
    fn get_ammo_type(&self) -> AmmoType { self.ammo_type }

    pub fn attr_reload<'a>(&self, bonus_stat_attr: &Attribute<'a>) -> Attribute<'a> {
        let base_stat = self.base_stats.get(&StatHashes::Reload);
        if base_stat.is_none() {
            return Attribute::PrimF(0.0);
        } else {
            let base_stat: Attribute = (*base_stat.unwrap()).into();
            let stat = base_stat.add(bonus_stat_attr).clamp(0f64, 100f64);
            let evpp = stat.pow(&2.into()).mul(&self.reload_formula.reload_data.evpp.into());
            let vpp = stat.mul(&self.reload_formula.reload_data.vpp.into());
            vpp.add(&evpp).add(&self.reload_formula.reload_data.offset.into())
        }
    }

    pub fn attr_range_start<'a>(&self, bonus_stat_attr: &Attribute<'a>) -> Attribute<'a> {
        let base_stat = self.base_stats.get(&StatHashes::Range);
        if base_stat.is_none() {
            return Attribute::PrimF(0.0);
        } else {
            let base_stat: Attribute = (*base_stat.unwrap()).into();
            let stat = base_stat.add(bonus_stat_attr).clamp(0f64, 100f64);
            let evpp = stat.pow(&2.into()).mul(&self.range_formula.start.evpp.into());
            let vpp = stat.mul(&self.range_formula.start.vpp.into());
            vpp.add(&evpp).add(&self.range_formula.start.offset.into())
        }
    }

    pub fn attr_range_end<'a>(&self, bonus_stat_attr: &Attribute<'a>) -> Attribute<'a> {
        let base_stat = self.base_stats.get(&StatHashes::Range);
        if base_stat.is_none() {
            return Attribute::PrimF(0.0);
        } else {
            let base_stat: Attribute = (*base_stat.unwrap()).into();
            let stat = base_stat.add(bonus_stat_attr).clamp(0f64, 100f64);
            let evpp = stat.pow(&2.into()).mul(&self.range_formula.end.evpp.into());
            let vpp = stat.mul(&self.range_formula.end.vpp.into());
            vpp.add(&evpp).add(&self.range_formula.end.offset.into())
        }
    }

    pub fn attr_range_floor_percent<'a>(&self) -> Attribute<'a> {
        Attribute::PrimF(self.range_formula.floor_percent)
    }

    pub fn attr_range_to_ads<'a>(&self,  bonus_stat_attr: &Attribute<'a>, hip_range_attr: &Attribute<'a>) -> Attribute<'a> {
        let base_stat = self.base_stats.get(&StatHashes::Zoom);
        if base_stat.is_none() {
            return Attribute::PrimF(0.0);
        } else {
            let base_stat: Attribute = (*base_stat.unwrap()).into();
            let zoom_mult = base_stat.add(bonus_stat_attr)
                                        .clamp(0f64, 100f64)
                                        .div(&10.into()).sub(&0.025.into());
            hip_range_attr.mul(&zoom_mult)
        }
    }

    pub fn attr_magazine<'a>(&self, bonus_stat_attr: &Attribute<'a>) -> Attribute<'a> {
        let base_stat = self.base_stats.get(&StatHashes::Magazine);
        if base_stat.is_none() {
            return Attribute::PrimF(0.0);
        } else {
            let base_stat: Attribute = (*base_stat.unwrap()).into();
            let stat = base_stat.add(bonus_stat_attr).clamp(0f64, 100f64);
            let evpp = stat.pow(&2.into()).mul(&self.ammo_formula.mag.evpp.into());
            let vpp = stat.mul(&self.ammo_formula.mag.vpp.into());
            vpp.add(&evpp).add(&self.ammo_formula.mag.offset.into())
                .round(0, self.ammo_formula.round_to as u32)
        }
    }
}
