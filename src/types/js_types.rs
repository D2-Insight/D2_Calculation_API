#![cfg(feature = "wasm")]

use std::collections::HashMap;

use crate::{
    types::rs_types::StatQuadraticFormula,
    weapons::{FiringConfig, Stat},
};
use serde::{Deserialize, Serialize};
// use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use super::rs_types::{
    AmmoFormula, AmmoResponse, DamageMods, DpsResponse, HandlingFormula, HandlingResponse,
    RangeFormula, RangeResponse, ReloadFormula, ReloadResponse, TtkResponse,
};

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
