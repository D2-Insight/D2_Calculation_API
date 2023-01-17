use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;


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
impl JsRangeFormula {
    pub fn is_null(&self) -> bool {
        self.zrm == 0.0
            && self.zrm_tier == 0
            && self.vpp == 0.0
            && self.base_min == 0.0
            && self.base_max == 0.0
            && self.scale == false
            && self.floor_percent == 0.0
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct JsReloadFormula {
    pub evpp: f64,
    pub vpp: f64,
    pub offset: f64,
}
#[wasm_bindgen]
impl JsReloadFormula {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsReloadFormula {
        JsReloadFormula {
            evpp: 0.0,
            vpp: 0.0,
            offset: 0.0,
        }
    }
}
impl JsReloadFormula {
    pub fn is_null(&self) -> bool {
        self.evpp == 0.0 && self.vpp == 0.0 && self.offset == 0.0
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


#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct JsAmmoData {
    pub mag_evpp: f64,
    pub mag_vpp: f64,
    pub mag_offset: f64,
    pub reserve_formulas: HashMap<i32, (f64, f64)>
}
#[wasm_bindgen]
impl JsAmmoData {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsAmmoData {
        JsAmmoData {
            mag_evpp: 0.0,
            mag_vpp: 0.0,
            mag_offset: 0.0,
            reserve_formulas: HashMap::new(),
        }
    }
}
impl JsAmmoData {
    pub fn is_null(&self) -> bool {
        self.mag_evpp == 0.0
            && self.mag_vpp == 0.0
            && self.mag_offset == 0.0
            && self.reserve_formulas.is_empty()
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

#[derive(Debug, Clone, Serialize, Tsify, Default)]
#[tsify(into_wasm_abi)]
pub struct JsAmmoResponse {
    pub mag_size: i32,
    pub mag_size_perk: i32,
    pub reserve_size: i32,
}

#[derive(Debug, Clone, Serialize, Tsify, Default)]
#[tsify(into_wasm_abi)]
pub struct JsStat {
    pub stat_hash: u32,
    pub base_value: i32,
    pub part_value: i32,
    pub perk_value: i32,
}
