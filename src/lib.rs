#![allow(dead_code)]
#![allow(unused_imports)]

use perks::enhanced_handler::enhanced_check;
use serde::{Deserialize, Serialize};
pub mod abilities;
pub mod activity;
pub mod d2_enums;
pub mod dps;
pub mod enemies;
pub mod perks;
pub mod types;
pub mod weapons;

use crate::perks::{Perk, Perks};
use crate::weapons::{Stat, Weapon};
use abilities::Ability;
use activity::Activity;
use d2_enums::StatHashes;
use enemies::Enemy;
use std::cell::RefCell;
use std::collections::HashMap;
use std::panic;

extern crate flexbuffers;
extern crate serde;

mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

//make type alias for u32 called hash
pub type HashId = u32;
pub type StatMap = HashMap<HashId, i32>;

//JavaScript
#[cfg(feature = "wasm")]
use crate::types::js_types::{
    JsAmmoResponse, JsDifficultyOptions, JsDpsResponse, JsEnemyType, JsFiringResponse,
    JsHandlingResponse, JsMetaData, JsRangeResponse, JsReloadResponse, JsResillienceSummary,
    JsStat,
};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

//python
#[cfg(feature = "python")]
use crate::types::py_types::{
    PyActivity, PyDifficultyOptions, PyDpsResponse, PyEnemy, PyEnemyType, PyFiringResponse,
    PyHandlingResponse, PyPerk, PyPlayer, PyPlayerClass, PyRangeResponse, PyResillienceSummary,
};
#[cfg(feature = "python")]
use pyo3::{prelude::*, types::PyDict};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PersistentData {
    pub main_weapon: Weapon,
    pub ability: Ability,
    pub activity: Activity,
    pub enemy: Enemy,
}
impl PersistentData {
    pub fn new() -> PersistentData {
        Self::default()
    }
}

thread_local! {
    static PERS_DATA: RefCell<PersistentData> = RefCell::new(PersistentData::new());
}

pub fn set_instance_data(input: Vec<u8>) -> Result<(), String> {
    let r = flexbuffers::Reader::get_root(input.as_slice());
    if r.is_err() {
        return Err("Failed to read flexbuffer".to_string());
    }
    let loaded_data = PersistentData::deserialize(r.unwrap()).unwrap();
    PERS_DATA.with(|data| {
        *data.borrow_mut() = loaded_data;
    });
    return Ok(());
}

pub fn snapshot_instance_data() -> Result<Vec<u8>, String> {
    let mut buffer = Vec::new();
    PERS_DATA.with(|pers_data| {
        let data = pers_data.borrow().to_owned();
        let mut writer = flexbuffers::FlexbufferSerializer::new();
        data.serialize(&mut writer).unwrap();
        buffer = writer.take_buffer();
    });
    return Ok(buffer);
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
extern "C" {
    //foreign function interface
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[cfg(feature = "wasm")]
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
#[cfg(feature = "wasm")]
#[wasm_bindgen(start)]
pub fn start() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log!("D2 Calculator Loaded");
}



//---------------WEAPONS---------------//

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "getMetadata")]
pub fn get_metadata() -> Result<JsMetaData, JsValue> {
    use weapons::weapon_formulas::DATABASE_TIMESTAMP;
    let metadata = JsMetaData {
        database_timestamp: DATABASE_TIMESTAMP,
        api_timestamp: built_info::BUILT_TIME_UTC,
        api_version: built_info::PKG_VERSION,
        api_commit: built_info::GIT_COMMIT_HASH.unwrap(),
        api_branch: built_info::GIT_HEAD_REF.unwrap(),
    };
    Ok(metadata)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "stringifyWeapon")]
pub fn weapon_as_string() -> Result<String, JsValue> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().main_weapon.to_owned());
    Ok(format!("{:?}", weapon))
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "weaponJSON")]
///Returns the weapon as a JSON structure, snake case fields
pub fn weapon_as_json() -> Result<JsValue, JsValue> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().main_weapon.to_owned());
    Ok(serde_wasm_bindgen::to_value(&weapon).unwrap())
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "setWeapon")]
pub fn set_weapon(
    _hash: u32,
    _weapon_type_id: u8,
    _intrinsic_hash: u32,
    _ammo_type_id: u32,
    _damage_type_id: u32,
) -> Result<(), JsValue> {
    PERS_DATA.with(|perm_data| {
        let new_weapon = Weapon::generate_weapon(
            _hash,
            _weapon_type_id,
            _intrinsic_hash,
            _ammo_type_id,
            _damage_type_id,
        );
        if new_weapon.is_err() {
            console_log!(
                "Could not find weapon data for type: {}, intrinsic: {}, Err: {:?}",
                _weapon_type_id,
                _intrinsic_hash,
                new_weapon
            );
            perm_data.borrow_mut().main_weapon = Weapon::default();
        } else {
            perm_data.borrow_mut().main_weapon = new_weapon.unwrap();
        };
    });
    Ok(())
}

// #[cfg(feature = "wasm")]
// #[wasm_bindgen(js_name = "getWeaponHash")]
// pub fn get_weapon_hash() -> Result<bool, JsValue> {
//     Ok(PERS_DATA.with(|weapon| weapon.borrow().weapon.hash != 0))
// }

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "getStats")]
pub fn get_stats() -> Result<JsValue, JsValue> {
    let stat_map = PERS_DATA.with(|perm_data| perm_data.borrow().main_weapon.stats.clone());
    let mut js_stat_map = HashMap::new();
    for (key, value) in stat_map {
        js_stat_map.insert(key, JsStat::from(value));
    }
    let value = serde_wasm_bindgen::to_value(&js_stat_map);
    if value.is_err() {
        return Err(JsValue::from_str("Could not convert stats to JsValue"));
    }
    Ok(value.unwrap())
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "setStats")]
pub fn set_stats(_stats: JsValue) -> Result<(), JsValue> {
    let in_stats: HashMap<u32, i32> = serde_wasm_bindgen::from_value(_stats).unwrap();
    let mut stats = HashMap::new();
    for (key, value) in in_stats {
        stats.insert(key, Stat::from(value));
    }
    PERS_DATA.with(|perm_data| perm_data.borrow_mut().main_weapon.stats = stats);
    Ok(())
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "addTrait")]
pub fn add_perk(_stats: JsValue, _value: u32, _hash: u32) -> Result<(), JsValue> {
    let data = enhanced_check(_hash);
    let inter_stat_buffs: HashMap<u32, i32> = serde_wasm_bindgen::from_value(_stats).unwrap();
    let perk = Perk {
        stat_buffs: inter_stat_buffs,
        enhanced: data.1,
        value: _value,
        hash: data.0,
    };
    PERS_DATA.with(|perm_data| perm_data.borrow_mut().main_weapon.add_perk(perk));
    Ok(())
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "getTraitHashes")]
pub fn query_perks() -> Vec<u32> {
    PERS_DATA.with(|perm_data| perm_data.borrow_mut().main_weapon.list_perk_ids())
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "setTraitValue")]
pub fn change_perk_value(perk_hash: u32, new_value: u32) {
    PERS_DATA.with(|perm_data| {
        perm_data
            .borrow_mut()
            .main_weapon
            .change_perk_val(perk_hash, new_value)
    });
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "getTraitOptions")]
pub fn get_perk_options_js(_perks: Vec<u32>) -> Result<JsValue, JsValue> {
    let options = perks::perk_options_handler::get_perk_options(_perks);
    let value = serde_wasm_bindgen::to_value(&options);
    if value.is_err() {
        return Err(JsValue::from_str(
            "Could not convert perk options to JsValue",
        ));
    }
    Ok(value.unwrap())
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "getWeaponRangeFalloff")]
pub fn get_weapon_range(_dynamic_traits: bool, _pvp: bool) -> Result<JsRangeResponse, JsValue> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().main_weapon.clone());
    if _dynamic_traits {
        Ok(weapon
            .calc_range_falloff(Some(weapon.static_calc_input()), None, _pvp)
            .into())
    } else {
        Ok(weapon.calc_range_falloff(None, None, _pvp).into())
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "getWeaponHandlingTimes")]
pub fn get_weapon_handling(
    _dynamic_traits: bool,
    _pvp: bool,
) -> Result<JsHandlingResponse, JsValue> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().main_weapon.clone());
    if _dynamic_traits {
        Ok(weapon
            .calc_handling_times(Some(weapon.static_calc_input()), None, _pvp)
            .into())
    } else {
        Ok(weapon.calc_handling_times(None, None, _pvp).into())
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "getWeaponReloadTimes")]
pub fn get_weapon_reload(_dynamic_traits: bool, _pvp: bool) -> Result<JsReloadResponse, JsValue> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().main_weapon.clone());
    if _dynamic_traits {
        Ok(weapon
            .calc_reload_time(Some(weapon.static_calc_input()), None, _pvp)
            .into())
    } else {
        Ok(weapon.calc_reload_time(None, None, _pvp).into())
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "getWeaponAmmoSizes")]
pub fn get_weapon_ammo(_dynamic_traits: bool, _pvp: bool) -> Result<JsAmmoResponse, JsValue> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().main_weapon.clone());
    if _dynamic_traits {
        Ok(weapon
            .calc_ammo_sizes(Some(weapon.static_calc_input()), None, _pvp)
            .into())
    } else {
        Ok(weapon.calc_ammo_sizes(None, None, _pvp).into())
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "getWeaponTtk")]
pub fn get_weapon_ttk(_overshield: f64) -> Result<JsValue, JsValue> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().main_weapon.clone());
    let ttk_data = weapon.calc_ttk(_overshield);
    let js_ttk_data: Vec<JsResillienceSummary> = ttk_data.into_iter().map(|r| r.into()).collect();
    Ok(serde_wasm_bindgen::to_value(&js_ttk_data).unwrap())
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "getWeaponDps")]
pub fn get_weapon_dps(_use_rpl: bool) -> Result<JsDpsResponse, JsValue> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().main_weapon.clone());
    let enemy = PERS_DATA.with(|perm_data| perm_data.borrow().enemy.clone());
    let pl_dmg_mult = PERS_DATA.with(|perm_data| perm_data.borrow().activity.get_pl_delta());
    let mut dps_response = weapon.calc_dps(enemy, pl_dmg_mult);
    let rpl_mult = PERS_DATA.with(|perm_data| perm_data.borrow().activity.get_rpl_mult());
    if _use_rpl {
        dps_response.apply_rpl(rpl_mult)
    }
    Ok(dps_response.into())
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "getWeaponFiringData")]
pub fn get_weapon_firing_data(
    _dynamic_traits: bool,
    _pvp: bool,
    _use_rpl: bool,
) -> Result<JsFiringResponse, JsValue> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().main_weapon.clone());
    let mut response: types::rs_types::FiringResponse;
    if _dynamic_traits {
        response = weapon.calc_firing_data(Some(weapon.static_calc_input()), None, _pvp);
    } else {
        response = weapon.calc_firing_data(None, None, _pvp);
    };
    PERS_DATA.with(|perm_data| {
        response.apply_pve_bonuses(
            perm_data.borrow().activity.get_rpl_mult(),
            perm_data.borrow().activity.get_pl_delta(),
            perm_data.borrow().main_weapon.damage_mods.pve,
            perm_data
                .borrow()
                .main_weapon
                .damage_mods
                .get_mod(&perm_data.borrow().enemy.type_),
        )
    });
    Ok(response.into())
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "getWeaponFlinch")]
pub fn get_weapon_flinch(_dynamic_traits: bool, _pvp: bool, _resilience: u8) -> Result<f64, JsValue> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    if _dynamic_traits {
        Ok(weapon
            .calc_flinch_resist(Some(weapon.static_calc_input()), _resilience as i32, _pvp, None))
    } else {
        Ok(weapon.calc_flinch_resist(None, _resilience as i32, _pvp, None))
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "setEncounter")]
pub fn set_encounter(
    _rpl: u32,
    _override_cap: i32,
    _difficulty: JsDifficultyOptions,
    _enemy_type: JsEnemyType,
) -> Result<(), JsValue> {
    PERS_DATA.with(|perm_data| {
        let mut activity = &mut perm_data.borrow_mut().activity;
        activity.rpl = _rpl;
        activity.cap = _override_cap;
        activity.difficulty = _difficulty.into();
        activity.player.pl = 2000;
    });
    PERS_DATA.with(|perm_data| {
        let mut enemy = &mut perm_data.borrow_mut().enemy;
        enemy.type_ = _enemy_type.into();
    });
    Ok(())
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "python")]
#[pyfunction(name = "get_hash")]
fn get_weapon_hash() -> PyResult<u32> {
    Ok(PERS_DATA.with(|perm_data| perm_data.borrow().weapon.hash))
}

#[cfg(feature = "python")]
#[pyfunction(name = "set_weapon")]
fn set_weapon(
    _hash: u32,
    _weapon_type_id: u8,
    _intrinsic_hash: u32,
    _ammo_type_id: u32,
    _damage_type_id: u32,
) -> PyResult<()> {
    PERS_DATA.with(|perm_data| {
        let new_weapon = Weapon::generate_weapon(
            _hash,
            _weapon_type_id,
            _intrinsic_hash,
            _ammo_type_id,
            _damage_type_id,
        );
        if new_weapon.is_err() {
            #[cfg(feature = "python")]
            println!(
                "Could not find weapon data for type: {}, intrinsic: {}",
                _weapon_type_id, _intrinsic_hash
            );
            perm_data.borrow_mut().weapon = Weapon::default();
        } else {
            perm_data.borrow_mut().weapon = new_weapon.unwrap();
        };
    });
    Ok(())
}

#[cfg(feature = "python")]
#[pyfunction(name = "add_trait")]
fn add_perk(_perk: PyPerk) -> PyResult<()> {
    PERS_DATA.with(|perm_data| perm_data.borrow_mut().weapon.add_perk(_perk.into()));
    Ok(())
}
#[cfg(feature = "python")]
#[pyfunction(name = "remove_trait")]
fn remove_perk(_perk_hash: u32) -> PyResult<()> {
    PERS_DATA.with(|perm_data| perm_data.borrow_mut().weapon.remove_perk(_perk_hash));
    Ok(())
}
#[cfg(feature = "python")]
#[pyfunction(name = "stringify_weapon")]
fn weapon_as_string() -> PyResult<String> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    Ok(format!("{:?}", weapon))
}
#[cfg(feature = "python")]
#[pyfunction(name = "get_range_falloff")]
fn get_weapon_range(_use_traits: bool, _pvp: bool) -> PyResult<PyRangeResponse> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    if _use_traits {
        Ok(weapon
            .calc_range_falloff(Some(weapon.static_calc_input()), None, _pvp)
            .into())
    } else {
        Ok(weapon.calc_range_falloff(None, None, _pvp).into())
    }
}
#[cfg(feature = "python")]
#[pyfunction(name = "get_handling_times")]
fn get_weapon_handling(_use_traits: bool, _pvp: bool) -> PyResult<PyHandlingResponse> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    if _use_traits {
        Ok(weapon
            .calc_handling_times(Some(weapon.static_calc_input()), None, _pvp)
            .into())
    } else {
        Ok(weapon.calc_handling_times(None, None, _pvp).into())
    }
}

#[cfg(feature = "python")]
#[pyfunction(name = "get_firing_data")]
fn get_firing_data(
    _dynamic_traits: bool,
    _pvp: bool,
    _use_rpl: bool,
) -> PyResult<PyFiringResponse> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    let mut response: types::rs_types::FiringResponse;
    if _dynamic_traits {
        response = weapon.calc_firing_data(Some(weapon.static_calc_input()), None, _pvp);
    } else {
        response = weapon.calc_firing_data(None, None, _pvp);
    };
    PERS_DATA.with(|perm_data| {
        response.apply_pve_bonuses(
            perm_data.borrow().activity.get_rpl_mult(),
            perm_data.borrow().activity.get_pl_delta(),
            perm_data.borrow().weapon.damage_mods.pve,
            perm_data
                .borrow()
                .weapon
                .damage_mods
                .get_mod(&perm_data.borrow().enemy.type_),
        )
    });
    Ok(response.into())
}

#[cfg(feature = "python")]
#[pyfunction(name = "get_dps")]
fn get_weapon_dps(_do_rpl_mult: bool) -> PyResult<PyDpsResponse> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    let enemy = PERS_DATA.with(|perm_data| perm_data.borrow().enemy.clone());
    let pl_dmg_mult = PERS_DATA.with(|perm_data| perm_data.borrow().activity.get_pl_delta());
    let mut dps_response = weapon.calc_dps(enemy, pl_dmg_mult);
    let rpl_mult = PERS_DATA.with(|perm_data| perm_data.borrow().activity.get_rpl_mult());
    if _do_rpl_mult {
        dps_response.apply_rpl(rpl_mult)
    }
    Ok(dps_response.into())
}

#[cfg(feature = "python")]
#[pyfunction(name = "get_ttk")]
fn get_weapon_ttk(_overshield: f64) -> PyResult<Vec<PyResillienceSummary>> {
    let rs_resill_array =
        PERS_DATA.with(|perm_data| perm_data.borrow().weapon.calc_ttk(_overshield));
    //call into on every item in the array
    let mut py_resill_array = Vec::new();
    for rs_resill in rs_resill_array {
        py_resill_array.push(rs_resill.into());
    }
    Ok(py_resill_array)
}

#[cfg(feature = "python")]
#[pyfunction(name = "set_stats")]
fn set_weapon_stats(_in: &PyDict) {
    let mut stats = HashMap::new();
    for (key, value) in _in.iter() {
        let key = key.extract::<u32>().unwrap();
        let value: Stat = value.extract::<i32>().unwrap().into();
        stats.insert(key, value);
    }
    PERS_DATA.with(|perm_data| {
        perm_data.borrow_mut().weapon.stats = stats;
    });
}

#[cfg(feature = "python")]
fn register_weapon_interface(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    let weapon_interface = PyModule::new(py, "WeaponInterface")?;
    //functions
    weapon_interface.add_function(wrap_pyfunction!(get_weapon_handling, weapon_interface)?)?;
    weapon_interface.add_function(wrap_pyfunction!(get_weapon_range, weapon_interface)?)?;
    weapon_interface.add_function(wrap_pyfunction!(weapon_as_string, weapon_interface)?)?;
    weapon_interface.add_function(wrap_pyfunction!(remove_perk, weapon_interface)?)?;
    weapon_interface.add_function(wrap_pyfunction!(add_perk, weapon_interface)?)?;
    weapon_interface.add_function(wrap_pyfunction!(get_weapon_hash, weapon_interface)?)?;
    weapon_interface.add_function(wrap_pyfunction!(get_weapon_dps, weapon_interface)?)?;
    weapon_interface.add_function(wrap_pyfunction!(set_weapon, weapon_interface)?)?;
    weapon_interface.add_function(wrap_pyfunction!(get_weapon_ttk, weapon_interface)?)?;
    weapon_interface.add_function(wrap_pyfunction!(set_weapon_stats, weapon_interface)?)?;
    weapon_interface.add_function(wrap_pyfunction!(get_firing_data, weapon_interface)?)?;

    //classes;
    weapon_interface.add_class::<PyPerk>()?;
    weapon_interface.add_class::<PyRangeResponse>()?;
    weapon_interface.add_class::<PyHandlingResponse>()?;
    weapon_interface.add_class::<PyDpsResponse>()?;
    weapon_interface.add_class::<PyResillienceSummary>()?;
    parent_module.add_submodule(weapon_interface)?;
    Ok(())
}

#[cfg(feature = "python")]
#[pyfunction(name = "set_activity")]
fn set_activity(_activity: PyActivity) -> PyResult<()> {
    PERS_DATA.with(|perm_data| {
        perm_data.borrow_mut().activity.cap = _activity.cap;
        perm_data.borrow_mut().activity.name = _activity.name;
        perm_data.borrow_mut().activity.rpl = _activity.rpl;
        perm_data.borrow_mut().activity.difficulty = _activity.difficulty.into();
    });
    Ok(())
}

#[cfg(feature = "python")]
#[pyfunction(name = "get_activity")]
fn get_activity() -> PyResult<PyActivity> {
    let activity = PERS_DATA.with(|perm_data| perm_data.borrow().activity.clone());
    Ok(activity.into())
}

#[cfg(feature = "python")]
#[pyfunction(name = "set_player")]
fn set_player(_player: PyPlayer) -> PyResult<()> {
    PERS_DATA.with(|perm_data| {
        perm_data.borrow_mut().activity.player = _player.into();
    });
    Ok(())
}

#[cfg(feature = "python")]
#[pyfunction(name = "get_player")]
fn get_player() -> PyResult<PyPlayer> {
    let player = PERS_DATA.with(|perm_data| perm_data.borrow().activity.player.clone());
    Ok(player.into())
}

#[cfg(feature = "python")]
fn register_activity_interface(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    let activity_interface = PyModule::new(py, "ActivityInterface")?;
    //functions
    activity_interface.add_function(wrap_pyfunction!(get_activity, activity_interface)?)?;
    activity_interface.add_function(wrap_pyfunction!(set_activity, activity_interface)?)?;
    activity_interface.add_function(wrap_pyfunction!(get_player, activity_interface)?)?;
    activity_interface.add_function(wrap_pyfunction!(set_player, activity_interface)?)?;

    //classes
    activity_interface.add_class::<PyActivity>()?;
    activity_interface.add_class::<PyPlayer>()?;
    activity_interface.add_class::<PyDifficultyOptions>()?;
    activity_interface.add_class::<PyPlayerClass>()?;

    parent_module.add_submodule(activity_interface)?;
    Ok(())
}

#[cfg(feature = "python")]
#[pyfunction(name = "get_enemy")]
fn get_enemy() -> PyResult<PyEnemy> {
    let enemy = PERS_DATA.with(|perm_data| perm_data.borrow().enemy.clone());
    Ok(enemy.into())
}

#[cfg(feature = "python")]
#[pyfunction(name = "set_enemy")]
fn set_enemy(_enemy: PyEnemy) -> PyResult<()> {
    PERS_DATA.with(|perm_data| {
        perm_data.borrow_mut().enemy = _enemy.into();
    });
    Ok(())
}

#[cfg(feature = "python")]
#[pyfunction(name = "set_enemy_type")]
fn set_enemy_type(_enemy_type: PyEnemyType) -> PyResult<()> {
    PERS_DATA.with(|perm_data| {
        perm_data.borrow_mut().enemy.type_ = _enemy_type.into();
    });
    Ok(())
}

#[cfg(feature = "python")]
fn register_enemy_interface(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    let enemy_interface = PyModule::new(py, "EnemyInterface")?;
    //functions
    enemy_interface.add_function(wrap_pyfunction!(get_enemy, enemy_interface)?)?;
    enemy_interface.add_function(wrap_pyfunction!(set_enemy, enemy_interface)?)?;
    enemy_interface.add_function(wrap_pyfunction!(set_enemy_type, enemy_interface)?)?;

    //classes
    enemy_interface.add_class::<PyEnemy>()?;
    enemy_interface.add_class::<PyEnemyType>()?;

    parent_module.add_submodule(enemy_interface)?;
    Ok(())
}

#[cfg(feature = "python")]
#[pymodule]
fn d2_calculation_api(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    register_weapon_interface(_py, m)?;
    register_activity_interface(_py, m)?;
    register_enemy_interface(_py, m)?;
    Ok(())
}
