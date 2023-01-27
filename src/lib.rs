#![allow(dead_code)]
#![allow(unused_imports)]
pub mod abilities;
pub mod activity;
pub mod d2_enums;
pub mod enemies;
pub mod perks;
pub mod types;
pub mod weapons;
pub mod json;

use crate::perks::{Perk, Perks};
use crate::weapons::Weapon;
use abilities::Ability;
use activity::Activity;
use d2_enums::StatHashes;
use enemies::Enemy;
use std::cell::RefCell;
use std::collections::HashMap;
use std::panic;

//JavaScript
#[cfg(feature = "wasm")]
use crate::types::js_types::{JsPerk, JsStat};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

//python
#[cfg(feature = "python")]
use crate::types::py_types::{
    PyAmmoFormula, PyDamageModifiers, PyFiringData, PyHandlingFormula, PyPerk, PyRangeFormula,
    PyReloadFormula, PyWeapon, PyWeaponFormula, PyRangeResponse, PyHandlingResponse, PyActivity,
    PyPlayer, PyEnemy, PyEnemyType, PyDpsResponse, PyDifficultyOptions, PyPlayerClass
};
#[cfg(feature = "python")]
use pyo3::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct PersistentData {
    pub weapon: Weapon,
    pub activity: Activity,
    pub ability: Ability,
    pub enemy: Enemy,
}
thread_local! {
    static PERS_DATA: RefCell<PersistentData> = RefCell::new(PersistentData::default());
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
extern "C" {
    //foreign function interface
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
#[cfg(feature = "wasm")]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
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
#[wasm_bindgen(js_name = "isWeaponInitialized")]
pub fn is_weapon_init() -> bool {
    PERS_DATA.with(|weapon| weapon.borrow().weapon.hash != 0)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "getString")]
pub fn get_string() -> String {
    json::get_data().to_string()
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "getWeaponId")]
pub fn get_weapon_id() -> u32 {
    PERS_DATA.with(|perm_data| perm_data.borrow().weapon.hash)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "getStat")]
pub fn get_stat(_stat: u32) -> JsStat {
    let stats = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.stats.clone());
    let stat = stats.get(&_stat);
    if stat.is_some() {
        stat.unwrap().clone().into()
    } else {
        JsStat {
            base_value: 0,
            perk_value: 0,
            part_value: 0,
        }
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "isPerkImplemented")]
pub fn is_perk_implemented(perk_hash: u32) -> bool {
    //really meant for debugging
    Perks::from_u32(perk_hash) != Perks::Ignore
}

// #[cfg(feature = "wasm")]
// #[wasm_bindgen(js_name = "addPerk")]
// pub fn modify_perks(perk_data: JsPerk, add: bool) {
//     let perk = Perk::from_js(perk_data);
//     if is_weapon_init() {
//         if add {
//             PERS_DATA.with(|perm_data| perm_data.borrow_mut().weapon.add_perk(perk));
//         } else {
//             PERS_DATA.with(|perm_data| perm_data.borrow_mut().weapon.remove_perk(perk));
//         }
//     } else {
//         console_log!("Weapon is not ready for perks");
//     };
// }

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "queryPerks")]
pub fn query_perks() -> Vec<u32> {
    PERS_DATA.with(|perm_data| perm_data.borrow_mut().weapon.list_perk_ids())
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "changePerkValue")]
pub fn change_perk_value(perk_hash: u32, new_value: u32) {
    PERS_DATA.with(|perm_data| {
        perm_data
            .borrow_mut()
            .weapon
            .change_perk_val(perk_hash, new_value)
    });
}

//---------------ACTIVITY---------------//

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


#[cfg(feature = "python")]
#[pyfunction(name = "is_weapon_set")]
fn is_weapon_assigned() -> PyResult<bool> {
    let val = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.hash);
    Ok(val != 0)
}
#[cfg(feature = "python")]
#[pyfunction(name = "set_weapon")]
fn set_weapon(weapon: PyWeapon) -> PyResult<()> {
    PERS_DATA.with(|perm_data| {
        let new_weapon: Weapon = weapon.into();
        perm_data.borrow_mut().weapon = new_weapon;
    });
    Ok(())
}
#[cfg(feature = "python")]
#[pyfunction(name = "add_perk")]
fn add_perk(_perk: PyPerk) -> PyResult<()> {
    PERS_DATA.with(
        |perm_data| 
        perm_data.borrow_mut().weapon.add_perk(_perk.into()));
    Ok(())
}
#[cfg(feature = "python")]
#[pyfunction(name = "remove_perk")]
fn remove_perk(_perk: PyPerk) -> PyResult<()> {
    PERS_DATA.with(
        |perm_data| 
        perm_data.borrow_mut().weapon.remove_perk(_perk.into()));
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
fn get_weapon_range(_dynamic_perks: bool) -> PyResult<PyRangeResponse> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    if _dynamic_perks {
        Ok(weapon.calc_range_falloff(Some(weapon.static_calc_input())).into())
    } else {
        Ok(weapon.calc_range_falloff(None).into())
    }
}
#[cfg(feature = "python")]
#[pyfunction(name = "get_handling_times")]
fn get_weapon_handling(_dynamic_perks: bool) -> PyResult<PyHandlingResponse> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    if _dynamic_perks {
        Ok(weapon.calc_handling_times(Some(weapon.static_calc_input())).into())
    } else {
        Ok(weapon.calc_handling_times(None).into())
    }
}

#[cfg(feature = "python")]
#[pyfunction(name = "get_dps")]
fn get_weapon_dps(_do_rpl_mult: bool) -> PyResult<PyDpsResponse> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    let enemy = PERS_DATA.with(|perm_data| perm_data.borrow().enemy.clone());
    let pl_dmg_mult = PERS_DATA.with(|perm_data| perm_data.borrow().activity.get_pl_delta());
    let mut dps_response = weapon.calc_dps(enemy, pl_dmg_mult);
    let rpl_mult = PERS_DATA.with(|perm_data| perm_data.borrow().activity.get_rpl_mult());
    if _do_rpl_mult{
        dps_response.apply_rpl(rpl_mult)
    }
    Ok(dps_response.into())
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
    weapon_interface.add_function(wrap_pyfunction!(set_weapon, weapon_interface)?)?;
    weapon_interface.add_function(wrap_pyfunction!(is_weapon_assigned, weapon_interface)?)?;
    weapon_interface.add_function(wrap_pyfunction!(get_weapon_dps, weapon_interface)?)?;

    //classes
    weapon_interface.add_class::<PyWeapon>()?;
    weapon_interface.add_class::<PyPerk>()?;
    weapon_interface.add_class::<PyWeaponFormula>()?;
    weapon_interface.add_class::<PyDamageModifiers>()?;
    weapon_interface.add_class::<PyFiringData>()?;
    weapon_interface.add_class::<PyAmmoFormula>()?;
    weapon_interface.add_class::<PyHandlingFormula>()?;
    weapon_interface.add_class::<PyRangeFormula>()?;
    weapon_interface.add_class::<PyReloadFormula>()?;
    weapon_interface.add_class::<PyRangeResponse>()?;
    weapon_interface.add_class::<PyHandlingResponse>()?;
    weapon_interface.add_class::<PyDpsResponse>()?;
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
