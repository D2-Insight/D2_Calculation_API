use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::D2Weapon::JS_Stat;

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

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct JsRangeFormula {
    pub zrm: f64,
    pub zrm_tier: i32,
    pub vpp: f64,
    pub base_min: f64,
    pub base_max: f64,
    pub scale: bool,
    pub floor_percent: f64,
}
#[wasm_bindgen]
impl JsRangeFormula {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsRangeFormula {
        JsRangeFormula {
            zrm: 0.0,
            zrm_tier: 0,
            vpp: 0.0,
            base_min: 0.0,
            base_max: 0.0,
            scale: false,
            floor_percent: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct JsReloadFormula {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}
#[wasm_bindgen]
impl JsReloadFormula {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsReloadFormula {
        JsReloadFormula {
            a: 0.0,
            b: 0.0,
            c: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct JsHandlingFormula {
    pub ready_m: f64,
    pub ready_b: f64,
    pub stow_m: f64,
    pub stow_b: f64,
    pub ads_m: f64,
    pub ads_b: f64,
}
#[wasm_bindgen]
impl JsHandlingFormula {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsHandlingFormula {
        JsHandlingFormula {
            ready_m: 0.0,
            ready_b: 0.0,
            stow_m: 0.0,
            stow_b: 0.0,
            ads_m: 0.0,
            ads_b: 0.0,
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

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct JsWeaponFormula {
    #[tsify(optional)]
    pub range_data: Option<JsRangeFormula>,
    #[tsify(optional)]
    pub reload_data: Option<JsReloadFormula>,
    #[tsify(optional)]
    pub handling_data: Option<JsHandlingFormula>,
    pub firing_data: JsFiringData,
}
#[wasm_bindgen]
impl JsWeaponFormula {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsWeaponFormula {
        JsWeaponFormula {
            range_data: None,
            reload_data: None,
            handling_data: None,
            firing_data: JsFiringData::new(),
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

#[derive(Debug, Clone, Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct JsRangeResponse {
    pub hip_falloff_start: f64,
    pub hip_falloff_end: f64,
    pub ads_falloff_start: f64,
    pub ads_falloff_end: f64,
}

#[derive(Debug, Clone, Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct JsReloadResponse {
    pub reaload_time: f64,
    pub ammo_time: f64,
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

#[derive(Debug, Clone, Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct JsDpsResponse {
    dps_per_mag: Vec<f64>,

    // damage_vec: Vec<f64>,
    // time_vec: Vec<f64>,
    damage_time_data: Vec<(f64, f64)>,

    total_damage: f64,
    total_shots: f64,
}