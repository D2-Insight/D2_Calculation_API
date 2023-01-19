#![cfg(target_arch = "wasm32")]

use std::collections::HashMap;

use crate::{types::rs_types::QuadraticFormula, weapons::FiringConfig};
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use super::{
    rs_types::{
        AmmoFormula, DamageMods, DpsResponse, HandlingFormula, HandlingResponse, MagazineResponse,
        RangeFormula, RangeResponse, ReloadFormula, ReloadResponse, ReserveResponse, TtkResponse,
    },
    ToRs,
};

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct JsWeapon {
    pub hash: u32,
    pub intrinsic: u32,
    pub weapon_type: u32,
    pub damage_type: u32,
    pub weapon_slot: u32,
    pub ammo_type: u32,
    pub stats: HashMap<u32, i32>,
    pub damage_modifiers: JsDamageModifiers,
    pub formulas: JsWeaponFormula,
}

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
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
impl JsDamageModifiers {
    pub fn is_null(&self) -> bool {
        self.global == 0.0
            && self.vehicle == 0.0
            && self.boss == 0.0
            && self.miniboss == 0.0
            && self.champion == 0.0
            && self.elite == 0.0
            && self.minor == 0.0
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

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct JsAmmoData {
    pub mag_evpp: f64,
    pub mag_vpp: f64,
    pub mag_offset: f64,
    pub mag_round_to_nearest: i32,
    pub reserve_formulas: HashMap<i32, (f64, f64)>,
}
#[wasm_bindgen]
impl JsAmmoData {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsAmmoData {
        JsAmmoData {
            mag_evpp: 0.0,
            mag_vpp: 0.0,
            mag_offset: 0.0,
            mag_round_to_nearest: 1,
            reserve_formulas: HashMap::new(),
        }
    }
}
impl JsAmmoData {
    pub fn is_null(&self) -> bool {
        self.mag_evpp == 0.0
            && self.mag_vpp == 0.0
            && self.mag_offset == 0.0
            && self.mag_round_to_nearest < 2
            && self.reserve_formulas.is_empty()
    }
}
impl Into<AmmoFormula> for JsAmmoData {
    fn into(self) -> AmmoFormula {
        AmmoFormula {
            mag: QuadraticFormula {
                evpp: self.mag_evpp,
                vpp: self.mag_vpp,
                offset: self.mag_offset,
            },
            round_to_nearest: self.mag_round_to_nearest,
            reserves: self
                .reserve_formulas
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        QuadraticFormula {
                            evpp: 0.0,
                            vpp: v.0,
                            offset: v.1,
                        },
                    )
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct JsWeaponFormula {
    pub range_data: JsRangeFormula,
    pub reload_data: JsReloadFormula,
    pub handling_data: JsHandlingFormula,
    pub firing_data: JsFiringData,
    pub ammo_data: JsAmmoData,
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
            ammo_data: JsAmmoData::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct JsPerk {
    pub stat_buffs: HashMap<u32, i32>,
    pub enhanced: bool,
    pub value: i32, //used for toggle and stacks
    pub id: u32,
}
#[wasm_bindgen]
impl JsPerk {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsPerk {
        JsPerk {
            stat_buffs: HashMap::new(),
            enhanced: false,
            value: 0,
            id: 0,
        }
    }
}

//
////Serialize Only
//

#[derive(Debug, Clone, Serialize, Tsify, Default)]
#[tsify(into_wasm_abi)]
pub struct JsHandlingResponse {
    pub ready_time: f64,
    pub stow_time: f64,
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

#[derive(Debug, Clone, Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct JsRangeResponse {
    pub hip_falloff_start: f64,
    pub hip_falloff_end: f64,
    pub ads_falloff_start: f64,
    pub ads_falloff_end: f64,
}
impl From<RangeResponse> for JsRangeResponse {
    fn from(range: RangeResponse) -> Self {
        JsRangeResponse {
            hip_falloff_start: range.hip_falloff_start,
            hip_falloff_end: range.hip_falloff_end,
            ads_falloff_start: range.ads_falloff_start,
            ads_falloff_end: range.ads_falloff_end,
        }
    }
}

#[derive(Debug, Clone, Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct JsReloadResponse {
    pub reload_time: f64,
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

#[derive(Debug, Clone, Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct JsTtkResponse {
    pub ammo_needed: i32,
    pub hits_needed: i32,
    pub optimal_ttk: f64,
    pub crit_percent: f64,
    pub bodyshot_ttk: f64,
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

#[derive(Debug, Clone, Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct JsDpsResponse {
    pub dps_per_mag: Vec<f64>,
    pub time_damage_data: Vec<(f64, f64)>,
    pub total_damage: f64,
    pub total_time: f64,
    pub total_shots: i32,
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

#[derive(Debug, Clone, Serialize, Tsify, Default)]
#[tsify(into_wasm_abi)]
pub struct JsMagazineResponse {
    pub mag_size: i32,
}
impl From<MagazineResponse> for JsMagazineResponse {
    fn from(ammo: MagazineResponse) -> Self {
        JsMagazineResponse {
            mag_size: ammo.mag_size,
        }
    }
}

#[derive(Debug, Clone, Serialize, Tsify, Default)]
#[tsify(into_wasm_abi)]
pub struct JsReserveResponse {
    pub reserve_size: i32,
}
impl From<ReserveResponse> for JsReserveResponse {
    fn from(ammo: ReserveResponse) -> Self {
        JsReserveResponse {
            reserve_size: ammo.reserve_size,
        }
    }
}

#[derive(Debug, Clone, Serialize, Tsify, Default)]
#[tsify(into_wasm_abi)]
pub struct JsStat {
    pub stat_hash: u32,
    pub base_value: i32,
    pub part_value: i32,
    pub perk_value: i32,
}
