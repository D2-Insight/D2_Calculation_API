#![cfg(feature = "wasm")]

use std::collections::HashMap;

use crate::{
    types::rs_types::StatQuadraticFormula,
    weapons::{FiringConfig, Stat, ttk_calc::ResillienceSummary}, perks::Perk, activity::damage_calc::DifficultyOptions, enemies::EnemyType,
};
use serde::{Deserialize, Serialize};
// use tsify::Tsify;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use super::rs_types::{
    AmmoFormula, AmmoResponse, DamageMods, DpsResponse, HandlingFormula, HandlingResponse,
    RangeFormula, RangeResponse, ReloadFormula, ReloadResponse, FiringResponse,
};

#[derive(Debug, Clone, Copy, Serialize)]
#[wasm_bindgen(js_name = "HandlingResponse", inspectable)]
pub struct JsHandlingResponse {
    #[wasm_bindgen(readonly)]
    pub ready_time: f64,
    #[wasm_bindgen(readonly)]
    pub stow_time: f64,
    #[wasm_bindgen(readonly)]
    pub ads_time: f64,
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

#[derive(Debug, Clone, Copy, Serialize)]
#[wasm_bindgen(js_name = "RangeResponse", inspectable)]
pub struct JsRangeResponse {
    #[wasm_bindgen(readonly)]
    pub hip_falloff_start: f64,
    #[wasm_bindgen(readonly)]
    pub hip_falloff_end: f64,
    #[wasm_bindgen(readonly)]
    pub ads_falloff_start: f64,
    #[wasm_bindgen(readonly)]
    pub ads_falloff_end: f64,
    #[wasm_bindgen(readonly)]
    pub floor_percent: f64,
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

#[derive(Debug, Clone, Copy, Serialize)]
#[wasm_bindgen(js_name = "ReloadResponse", inspectable)]
pub struct JsReloadResponse {
    #[wasm_bindgen(readonly)]
    pub reload_time: f64,
    #[wasm_bindgen(readonly)]
    pub ammo_time: f64,
}
impl From<ReloadResponse> for JsReloadResponse {
    fn from(reload: ReloadResponse) -> Self {
        JsReloadResponse {
            reload_time: reload.reload_time,
            ammo_time: reload.ammo_time,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
#[wasm_bindgen(js_name = "AmmoResponse", inspectable)]
pub struct JsAmmoResponse {
    #[wasm_bindgen(readonly)]
    pub mag_size: i32,
    #[wasm_bindgen(readonly)]
    pub reserve_size: i32,
}
impl From<AmmoResponse> for JsAmmoResponse {
    fn from(ammo: AmmoResponse) -> Self {
        JsAmmoResponse {
            mag_size: ammo.mag_size,
            reserve_size: ammo.reserve_size,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[wasm_bindgen(js_name = "DpsResponse")]
pub struct JsDpsResponse {
    #[wasm_bindgen(skip)]
    pub dps_per_mag: Vec<f64>,
    #[wasm_bindgen(skip)]
    pub time_damage_data: Vec<(f64, f64)>,
    #[wasm_bindgen(readonly)]
    pub total_damage: f64,
    #[wasm_bindgen(readonly)]
    pub total_time: f64,
    #[wasm_bindgen(readonly)]
    pub total_shots: i32,
}
#[wasm_bindgen(js_class = "DpsResponse")]
impl JsDpsResponse {
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(self) -> String {
        format!("{:?}", self)
    }
    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(self) -> String {
        serde_wasm_bindgen::to_value(&self).unwrap().as_string().unwrap()
    }
    ///Returns a list of tuples of time and damage
    #[wasm_bindgen(getter)]
    pub fn time_damage_data(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.time_damage_data).unwrap()
    }
    ///Returns a list of dps values for each magazine
    #[wasm_bindgen(getter)]
    pub fn dps_per_mag(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.dps_per_mag).unwrap()
    }
}
impl From<DpsResponse> for JsDpsResponse {
    fn from(dps: DpsResponse) -> Self {
        JsDpsResponse {
            dps_per_mag: dps.dps_per_mag,
            time_damage_data: dps.time_damage_data,
            total_damage: dps.total_damage,
            total_time: dps.total_time,
            total_shots: dps.total_shots,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[wasm_bindgen(js_name = "TtkResponse")]
pub struct JsTtkResponse {
    #[wasm_bindgen(skip)]
    pub data: Vec<ResillienceSummary>,
}
#[wasm_bindgen(js_class = "TtkResponse")]
impl JsTtkResponse {
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(self) -> String {
        format!("{:?}", self)
    }
    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(self) -> String {
        serde_wasm_bindgen::to_value(&self).unwrap().as_string().unwrap()
    }
    #[wasm_bindgen(getter)]
    pub fn data(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.data).unwrap()
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[wasm_bindgen(js_name = "FiringResponse", inspectable)]
pub struct JsFiringResponse {
    pub pvp_damage: f64,
    pub pvp_crit_mult: f64,

    pub pve_damage: f64,
    pub pve_crit_mult: f64,

    pub burst_delay: f64,
    pub burst_duration: f64,
    pub burst_size: i32,

    pub rpm: f64,
}
impl From<FiringResponse> for JsFiringResponse {
    fn from(firing: FiringResponse) -> Self {
        JsFiringResponse {
            pvp_damage: firing.pvp_damage,
            pvp_crit_mult: firing.pvp_crit_mult,
            pve_damage: firing.pve_damage,
            pve_crit_mult: firing.pve_crit_mult,
            burst_delay: firing.burst_delay,
            burst_duration: firing.burst_duration,
            burst_size: firing.burst_size,
            rpm: firing.rpm,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[wasm_bindgen(js_name = "Stat")]
pub struct JsStat {
    pub base_value: i32,
    pub part_value: i32,
    pub trait_value: i32,
}
#[wasm_bindgen(js_class = "Stat")]
impl JsStat {
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(self) -> String {
        format!("{:?}", self)
    }
    #[wasm_bindgen(constructor)]
    pub fn new(base_value: i32, part_value: i32, trait_value: i32) -> Self {
        JsStat {
            base_value,
            part_value,
            trait_value,
        }
    }
}
impl From<Stat> for JsStat {
    fn from(stat: Stat) -> Self {
        JsStat {
            base_value: stat.base_value,
            part_value: stat.part_value,
            trait_value: stat.perk_value,
        }
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen(js_name = "DifficultyOptions")]
pub enum JsDifficultyOptions {
    NORMAL = 1,
    RAID = 2,
    MASTER = 3,
}
impl Into<DifficultyOptions> for JsDifficultyOptions {
    fn into(self) -> DifficultyOptions {
        match self {
            JsDifficultyOptions::NORMAL => DifficultyOptions::NORMAL,
            JsDifficultyOptions::RAID => DifficultyOptions::RAID,
            JsDifficultyOptions::MASTER => DifficultyOptions::MASTER,
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[wasm_bindgen(js_name = "EnemyType")]
pub enum JsEnemyType {
    MINOR,
    ELITE,
    MINIBOSS,
    BOSS,
    VEHICLE,
    ENCLAVE,
    PLAYER,
    CHAMPION
}
impl Into<EnemyType> for JsEnemyType {
    fn into(self) -> EnemyType {
        match self {
            JsEnemyType::MINOR => EnemyType::MINOR,
            JsEnemyType::ELITE => EnemyType::ELITE,
            JsEnemyType::MINIBOSS => EnemyType::MINIBOSS,
            JsEnemyType::BOSS => EnemyType::BOSS,
            JsEnemyType::VEHICLE => EnemyType::VEHICLE,
            JsEnemyType::ENCLAVE => EnemyType::ENCLAVE,
            JsEnemyType::PLAYER => EnemyType::PLAYER,
            JsEnemyType::CHAMPION => EnemyType::CHAMPION,
        }
    }
}

