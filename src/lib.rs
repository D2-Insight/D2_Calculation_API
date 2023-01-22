#![allow(dead_code)]
#![allow(unused_imports)]
pub mod abilities;
pub mod enemies;
pub mod d2_enums;
pub mod weapons;
pub mod perks;
pub mod activity;
pub mod types;

use crate::perks::{Perk, Perks};
use crate::weapons::Weapon;
use activity::Activity;
use std::cell::RefCell;
use std::collections::HashMap;
use std::panic;
use abilities::Ability;
use enemies::Enemy;
use d2_enums::StatHashes;


//JavaScript
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use crate::types::js_types::{JsPerk, JsStat};

//python
#[cfg(not(target_arch = "wasm32"))]
use pyo3::prelude::*;


#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    //foreign function interface
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
#[cfg(target_arch = "wasm32")]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log!("D2 Calculator Loaded");
}

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

//---------------WEAPONS---------------//
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = "isWeaponInitialized")]
pub fn is_weapon_init() -> bool {
    PERS_DATA.with(|weapon| weapon.borrow().weapon.id != 0)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = "getWeaponId")]
pub fn get_weapon_id() -> u32 {
    PERS_DATA.with(|perm_data| perm_data.borrow().weapon.id)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = "getStat")]
pub fn get_stat(_stat: u32) -> JsStat {
    let stats = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.stats.clone());
    let stat = stats.get(&_stat);
    if stat.is_some() {
        stat.unwrap().to_js(_stat)
    } else {
        JsStat {
            stat_hash: _stat,
            base_value: 0,
            perk_value: 0,
            part_value: 0,
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = "isPerkImplemented")]
pub fn is_perk_implemented(perk_hash: u32) -> bool {
    //really meant for debugging
    Perks::from_u32(perk_hash) != Perks::Ignore
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = "addPerk")]
pub fn modify_perks(perk_data: JsPerk, add: bool) {
    let perk = Perk::from_js(perk_data);
    if is_weapon_init() {
        if add {
            PERS_DATA.with(|perm_data| perm_data.borrow_mut().weapon.add_perk(perk));
        } else {
            PERS_DATA.with(|perm_data| perm_data.borrow_mut().weapon.remove_perk(perk));
        }
    } else {
        console_log!("Weapon is not ready for perks");
    };
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = "queryPerks")]
pub fn query_perks() -> Vec<u32> {
    PERS_DATA.with(|perm_data| perm_data.borrow_mut().weapon.list_perk_ids())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = "changePerkValue")]
pub fn change_perk_value(perk_hash: u32, new_value: i32) {
    PERS_DATA.with(|perm_data| {
        perm_data
            .borrow_mut()
            .weapon
            .change_perk_val(perk_hash, new_value)
    });
}

//---------------ACTIVITY---------------//

