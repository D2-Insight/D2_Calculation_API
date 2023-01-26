#![cfg(feature = "wasm")]

use std::collections::HashMap;

use crate::{types::rs_types::QuadraticFormula, weapons::{FiringConfig, Stat}};
use serde::{Deserialize, Serialize};
// use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use super::{
    rs_types::{
        AmmoFormula, DamageMods, DpsResponse, HandlingFormula, HandlingResponse, AmmoResponse,
        RangeFormula, RangeResponse, ReloadFormula, ReloadResponse, TtkResponse,
    }
};

#[derive(Debug, Clone)]
#[wasm_bindgen(js_name = "Weapon")]
pub struct JsWeapon {
    pub hash: u32,
    pub weapon_type: u32,
    pub damage_type: u32,
    pub weapon_slot: u32,
    pub ammo_type: u32,
    #[wasm_bindgen(skip)]
    pub stats: Vec<js_sys::BigInt64Array>,
    pub damage_modifiers: JsDamageModifiers,
    pub formulas: JsWeaponFormula,
}
#[wasm_bindgen]
impl JsWeapon {
    #[wasm_bindgen(constructor)]
    pub fn new(
        hash: u32,
        weapon_type: u32,
        damage_type: u32,
        weapon_slot: u32,
        ammo_type: u32,
        stats: Vec<js_sys::BigInt64Array>,
        damage_modifiers: JsDamageModifiers,
        formulas: JsWeaponFormula,
    ) -> JsWeapon {
        JsWeapon {
            hash,
            weapon_type,
            damage_type,
            weapon_slot,
            ammo_type,
            stats,
            damage_modifiers,
            formulas,
        }
    }
    #[wasm_bindgen(getter)]
    pub fn stats(&self) -> Vec<js_sys::BigInt64Array> {
        self.stats.clone()
    }
    #[wasm_bindgen(setter)]
    pub fn set_stats(&mut self, stats: Vec<js_sys::BigInt64Array>) {
        self.stats = stats;
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[wasm_bindgen(js_name = "DamageModifiers")]
pub struct JsDamageModifiers {
    pub global: f64, // not reccomended to use but gives users a way to update sstuff themselves
    pub vehicle: f64,
    pub boss: f64,
    pub miniboss: f64,
    pub champion: f64,
    pub elite: f64,
    pub minor: f64,
}
#[wasm_bindgen]
impl JsDamageModifiers {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsDamageModifiers {
        JsDamageModifiers {
            global: 1.0,
            vehicle: 1.0,
            boss: 1.0,
            miniboss: 1.0,
            champion: 1.0,
            elite: 1.0,
            minor: 1.0,
        }
    }
}
impl Into<DamageMods> for JsDamageModifiers {
    fn into(self) -> DamageMods {
        DamageMods {
            pve: self.global,
            minor: self.minor,
            elite: self.elite,
            miniboss: self.miniboss,
            champion: self.champion,
            boss: self.boss,
            vehicle: self.vehicle,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[wasm_bindgen(js_name = "RangeFormula")]
pub struct JsRangeFormula {
    pub vpp_start: f64,
    pub offset_start: f64,
    pub vpp_end: f64,
    pub offset_end: f64,
    pub floor_percent: f64,
    pub is_fusion: bool,
}
#[wasm_bindgen]
impl JsRangeFormula {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsRangeFormula {
        JsRangeFormula {
            vpp_start: 0.0,
            offset_start: 0.0,
            vpp_end: 0.0,
            offset_end: 0.0,
            floor_percent: 0.0,
            is_fusion: false,
        }
    }
}
impl JsRangeFormula {
    pub fn is_null(&self) -> bool {
        self.vpp_start == 0.0
            && self.offset_start == 0.0
            && self.vpp_end == 0.0
            && self.offset_end == 0.0
            && self.floor_percent == 0.0
            && self.is_fusion == false
    }
}
impl Into<RangeFormula> for JsRangeFormula {
    fn into(self) -> RangeFormula {
        RangeFormula {
            start: QuadraticFormula {
                evpp: 0.0,
                vpp: self.vpp_start,
                offset: self.offset_start,
            },
            end: QuadraticFormula {
                evpp: 0.0,
                vpp: self.vpp_end,
                offset: self.offset_end,
            },
            floor_percent: self.floor_percent,
            is_fusion: self.is_fusion,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
// #[serde(rename = "ReloadFormula")]
#[wasm_bindgen(js_name = "ReloadFormula")]
pub struct JsReloadFormula {
    pub evpp: f64,
    pub vpp: f64,
    pub offset: f64,
    pub ammo_percent: f64,
}
#[wasm_bindgen]
impl JsReloadFormula {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsReloadFormula {
        JsReloadFormula {
            evpp: 0.0,
            vpp: 0.0,
            offset: 0.0,
            ammo_percent: 1.0,
        }
    }
}
impl JsReloadFormula {
    pub fn is_null(&self) -> bool {
        self.evpp == 0.0 && self.vpp == 0.0 && self.offset == 0.0
    }
}
impl Into<ReloadFormula> for JsReloadFormula {
    fn into(self) -> ReloadFormula {
        ReloadFormula {
            reload_data: QuadraticFormula {
                evpp: self.evpp,
                vpp: self.vpp,
                offset: self.offset,
            },
            ammo_percent: self.ammo_percent,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
// #[serde(rename = "HandlingFormula")]
#[wasm_bindgen(js_name = "HandlingFormula")]
pub struct JsHandlingFormula {
    pub ready_vpp: f64,
    pub ready_offset: f64,
    pub stow_vpp: f64,
    pub stow_offset: f64,
    pub ads_vpp: f64,
    pub ads_offset: f64,
}
#[wasm_bindgen]
impl JsHandlingFormula {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsHandlingFormula {
        JsHandlingFormula {
            ready_vpp: 0.0,
            ready_offset: 0.0,
            stow_vpp: 0.0,
            stow_offset: 0.0,
            ads_vpp: 0.0,
            ads_offset: 0.0,
        }
    }
}
impl JsHandlingFormula {
    pub fn is_null(&self) -> bool {
        self.ready_vpp == 0.0
            && self.ready_offset == 0.0
            && self.stow_vpp == 0.0
            && self.stow_offset == 0.0
            && self.ads_vpp == 0.0
            && self.ads_offset == 0.0
    }
}
impl Into<HandlingFormula> for JsHandlingFormula {
    fn into(self) -> HandlingFormula {
        HandlingFormula {
            ready: QuadraticFormula {
                evpp: 0.0,
                vpp: self.ready_vpp,
                offset: self.ready_offset,
            },
            stow: QuadraticFormula {
                evpp: 0.0,
                vpp: self.stow_vpp,
                offset: self.stow_offset,
            },
            ads: QuadraticFormula {
                evpp: 0.0,
                vpp: self.ads_vpp,
                offset: self.ads_offset,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
// #[serde(rename = "FiringData")]
#[wasm_bindgen(js_name = "FiringData")]
pub struct JsFiringData {
    pub damage: f64,
    pub crit_mult: f64,
    pub burst_delay: f64,
    pub burst_duration: f64,
    pub burst_size: i32,
    pub one_ammo_burst: bool,
    pub is_charge: bool,
    pub is_explosive: bool,
}
#[wasm_bindgen]
impl JsFiringData {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsFiringData {
        JsFiringData {
            damage: 0.0,
            crit_mult: 0.0,
            burst_delay: 0.0,
            burst_duration: 0.0,
            burst_size: 0,
            one_ammo_burst: false,
            is_charge: false,
            is_explosive: false,
        }
    }
}
impl JsFiringData {
    pub fn is_null(&self) -> bool {
        self.damage == 0.0
            && self.crit_mult == 0.0
            && self.burst_delay == 0.0
            && self.burst_duration == 0.0
            && self.burst_size == 0
            && self.one_ammo_burst == false
            && self.is_charge == false
            && self.is_explosive == false
    }
}
impl Into<FiringConfig> for JsFiringData {
    fn into(self) -> FiringConfig {
        FiringConfig {
            burst_delay: self.burst_delay,
            burst_duration: self.burst_duration,
            burst_size: self.burst_size,
            one_ammo_burst: self.one_ammo_burst,
            is_charge: self.is_charge,
            is_explosive: self.is_explosive,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[wasm_bindgen(js_name = "AmmoData")]
pub struct JsAmmoFormula {
    pub mag_evpp: f64,
    pub mag_vpp: f64,
    pub mag_offset: f64,
    pub mag_round_to_nearest: i32,
    pub is_primary: bool,
    pub reserve_id: i32,
}
#[wasm_bindgen]
impl JsAmmoFormula {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsAmmoFormula {
        JsAmmoFormula {
            mag_evpp: 0.0,
            mag_vpp: 0.0,
            mag_offset: 0.0,
            mag_round_to_nearest: 1,
            is_primary: false,
            reserve_id: 0,
        }
    }
}
impl Into<AmmoFormula> for JsAmmoFormula {
    fn into(self) -> AmmoFormula {
        AmmoFormula {
            mag: QuadraticFormula {
                evpp: self.mag_evpp,
                vpp: self.mag_vpp,
                offset: self.mag_offset,
            },
            round_to_nearest: self.mag_round_to_nearest,
            is_primary: self.is_primary,
            reserve_id: self.reserve_id,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[wasm_bindgen(js_name = "WeaponFormula")]
pub struct JsWeaponFormula {
    pub range_data: JsRangeFormula,
    pub reload_data: JsReloadFormula,
    pub handling_data: JsHandlingFormula,
    pub firing_data: JsFiringData,
    pub ammo_data: JsAmmoFormula,
}
#[wasm_bindgen]
impl JsWeaponFormula {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsWeaponFormula {
        JsWeaponFormula {
            range_data: JsRangeFormula::new(),
            reload_data: JsReloadFormula::new(),
            handling_data: JsHandlingFormula::new(),
            firing_data: JsFiringData::new(),
            ammo_data: JsAmmoFormula::new(),
        }
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen(js_name = "Perk")]
pub struct JsPerk {
    //first entry in array is key next is value
    #[wasm_bindgen(skip)]
    pub stat_buffs: Vec<js_sys::BigInt64Array>,
    pub enhanced: bool,
    pub value: i32, //used for toggle and stacks
    pub id: u32,
}
#[wasm_bindgen]
impl JsPerk {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsPerk {
        JsPerk {
            stat_buffs: Vec::new(),
            enhanced: false,
            value: 0,
            id: 0,
        }
    }
    #[wasm_bindgen(getter)]
    pub fn stat_buffs(&self) -> Vec<js_sys::BigInt64Array> {
        self.stat_buffs
            .iter()
            .map(|x| {
                let arr = js_sys::BigInt64Array::new_with_length(2);
                arr.set_index(0, x.get_index(0));
                arr.set_index(1, x.get_index(1));
                arr
            })
            .collect()
    }
    #[wasm_bindgen(setter)]
    pub fn set_stat_buffs(&mut self, stat_buffs: Vec<js_sys::BigInt64Array>) {
        self.stat_buffs = stat_buffs
            .iter()
            .map(|x| {
                let arr = js_sys::BigInt64Array::new_with_length(2);
                arr.set_index(0, x.get_index(0));
                arr.set_index(1, x.get_index(1));
                arr
            })
            .collect();
    }
}



#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[wasm_bindgen(js_name = "HandlingResponse")]
pub struct JsHandlingResponse {
    pub ready_time: f64,
    pub stow_time: f64,
    pub ads_time: f64,
}
#[wasm_bindgen]
impl JsHandlingResponse {
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(self) -> String {
        format!("{:?}", self)
    }
}
impl From<HandlingResponse> for JsHandlingResponse {
    fn from(handling: HandlingResponse) -> Self {
        JsHandlingResponse {
            ready_time: handling.ready_time,
            stow_time: handling.stow_time,
            ads_time: handling.ads_time,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[wasm_bindgen(js_name = "RangeResponse")]
pub struct JsRangeResponse {
    pub hip_falloff_start: f64,
    pub hip_falloff_end: f64,
    pub ads_falloff_start: f64,
    pub ads_falloff_end: f64,
    pub floor_percent: f64,
}
#[wasm_bindgen]
impl JsRangeResponse {
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(self) -> String {
        format!("{:?}", self)
    }
}
impl From<RangeResponse> for JsRangeResponse {
    fn from(range: RangeResponse) -> Self {
        JsRangeResponse {
            hip_falloff_start: range.hip_falloff_start,
            hip_falloff_end: range.hip_falloff_end,
            ads_falloff_start: range.ads_falloff_start,
            ads_falloff_end: range.ads_falloff_end,
            floor_percent: range.floor_percent,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[wasm_bindgen(js_name = "ReloadResponse")]
pub struct JsReloadResponse {
    pub reload_time: f64,
    pub ammo_time: f64,
}
#[wasm_bindgen]
impl JsReloadResponse {
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(self) -> String {
        format!("{:?}", self)
    }
}
impl From<ReloadResponse> for JsReloadResponse {
    fn from(reload: ReloadResponse) -> Self {
        JsReloadResponse {
            reload_time: reload.reload_time,
            ammo_time: reload.ammo_time,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[wasm_bindgen(js_name = "TtkResponse")]
pub struct JsTtkResponse {
    pub ammo_needed: i32,
    pub hits_needed: i32,
    pub optimal_ttk: f64,
    pub crit_percent: f64,
    pub bodyshot_ttk: f64,
}
#[wasm_bindgen]
impl JsTtkResponse {
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(self) -> String {
        format!("{:?}", self)
    }
}
impl From<TtkResponse> for JsTtkResponse {
    fn from(ttk: TtkResponse) -> Self {
        JsTtkResponse {
            ammo_needed: ttk.ammo_needed,
            hits_needed: ttk.hits_needed,
            optimal_ttk: ttk.optimal_ttk,
            crit_percent: ttk.crit_percent,
            bodyshot_ttk: ttk.bodyshot_ttk,
        }
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen(js_name = "DpsResponse")]
pub struct JsDpsResponse {
    #[wasm_bindgen(skip)]
    pub dps_per_mag: Vec<f64>,
    #[wasm_bindgen(skip)]
    pub time_damage_data: Vec<js_sys::Float64Array>,
    pub total_damage: f64,
    pub total_time: f64,
    pub total_shots: i32,
}
#[wasm_bindgen]
impl JsDpsResponse {
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(self) -> String {
        format!("{:?}", self)
    }
    #[wasm_bindgen(getter)]
    pub fn time_damage_data(&self) -> Vec<js_sys::Float64Array> {
        self.time_damage_data.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn dps_per_mag(&self) -> Vec<f64> {
        self.dps_per_mag.clone()
    }
}
impl From<DpsResponse> for JsDpsResponse {
    fn from(dps: DpsResponse) -> Self {
        //turn dps.time_damage_data into vec of js_sys::Float64Array
        let mut time_damage_data = Vec::new();
        for time_data in dps.time_damage_data {
            let time_data_array = js_sys::Float64Array::new_with_length(2);
            time_data_array.set_index(0 as u32, time_data.0);
            time_data_array.set_index(1 as u32, time_data.1);
            time_damage_data.push(time_data_array);
        }
        JsDpsResponse {
            dps_per_mag: dps.dps_per_mag,
            time_damage_data: time_damage_data,
            total_damage: dps.total_damage,
            total_time: dps.total_time,
            total_shots: dps.total_shots,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[wasm_bindgen(js_name = "AmmoResponse")]
pub struct JsAmmoResponse {
    pub mag_size: i32,
    pub reserve_size: i32,

}
#[wasm_bindgen]
impl JsAmmoResponse {
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(self) -> String {
        format!("{:?}", self)
    }
}
impl From<AmmoResponse> for JsAmmoResponse {
    fn from(ammo: AmmoResponse) -> Self {
        JsAmmoResponse {
            mag_size: ammo.mag_size,
            reserve_size: ammo.reserve_size,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[wasm_bindgen(js_name = "Stat")]
pub struct JsStat {
    pub base_value: i32,
    pub part_value: i32,
    pub perk_value: i32,
}
#[wasm_bindgen]
impl JsStat {
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(self) -> String {
        format!("{:?}", self)
    }
}
impl From<Stat> for JsStat {
    fn from(stat: Stat) -> Self {
        JsStat {
            base_value: stat.base_value,
            part_value: stat.part_value,
            perk_value: stat.perk_value,
        }
    }
}