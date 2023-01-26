#![allow(dead_code)]
#![allow(unused_imports)]
pub mod abilities;
pub mod activity;
pub mod d2_enums;
pub mod enemies;
pub mod perks;
pub mod types;
pub mod weapons;

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
    PyReloadFormula, PyWeapon, PyWeaponFormula, PyRangeResponse, PyHandlingResponse
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
#[derive(Debug, Clone, Default)]
#[pyclass(name = "WeaponInterface")]
pub struct PyWeaponFuncs;
#[cfg(feature = "python")]
#[pymethods]
impl PyWeaponFuncs {
    #[pyo3(name = "isWeaponSet")]
    #[staticmethod]
    fn is_weapon_assigned() -> PyResult<bool> {
        let val = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.hash);
        Ok(val != 0)
    }

    #[pyo3(name = "setWeapon")]
    #[staticmethod]
    fn set_weapon(weapon: PyWeapon) -> PyResult<()> {
        PERS_DATA.with(|perm_data| {
            let new_weapon: Weapon = weapon.into();
            perm_data.borrow_mut().weapon = new_weapon;
        });
        Ok(())
    }

    #[pyo3(name = "addPerk")]
    #[staticmethod]
    fn add_perk(_perk: PyPerk) -> PyResult<()> {
        PERS_DATA.with(
            |perm_data| 
            perm_data.borrow_mut().weapon.add_perk(_perk.into()));
        Ok(())
    }

    #[pyo3(name = "removePerk")]
    #[staticmethod]
    fn remove_perk(_perk: PyPerk) -> PyResult<()> {
        PERS_DATA.with(
            |perm_data| 
            perm_data.borrow_mut().weapon.remove_perk(_perk.into()));
        Ok(())
    }

    #[pyo3(name = "stringifyWeapon")]
    #[staticmethod]
    fn weapon_as_string() -> PyResult<String> {
        let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
        Ok(format!("{:?}", weapon))
    }

    #[pyo3(name = "getWeaponRange")]
    #[staticmethod]
    fn get_weapon_range(_dynamic_perks: bool) -> PyResult<PyRangeResponse> {
        let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
        if _dynamic_perks {
            Ok(weapon.calc_range_falloff(Some(weapon.static_calc_input())).into())
        } else {
            Ok(weapon.calc_range_falloff(None).into())
        }
    }

    #[pyo3(name = "getWeaponHandling")]
    #[staticmethod]
    fn get_weapon_handling(_dynamic_perks: bool) -> PyResult<PyHandlingResponse> {
        let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
        if _dynamic_perks {
            Ok(weapon.calc_handling_times(Some(weapon.static_calc_input())).into())
        } else {
            Ok(weapon.calc_handling_times(None).into())
        }
    }
}


#[cfg(feature = "python")]
#[pymodule]
fn d2_calculation_api(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyWeapon>()?;
    m.add_class::<PyPerk>()?;
    m.add_class::<PyWeaponFormula>()?;
    m.add_class::<PyDamageModifiers>()?;
    m.add_class::<PyFiringData>()?;
    m.add_class::<PyAmmoFormula>()?;
    m.add_class::<PyHandlingFormula>()?;
    m.add_class::<PyRangeFormula>()?;
    m.add_class::<PyReloadFormula>()?;
    m.add_class::<PyWeaponFuncs>()?;
    m.add_class::<PyRangeResponse>()?;
    m.add_class::<PyHandlingResponse>()?;
    Ok(())
}
