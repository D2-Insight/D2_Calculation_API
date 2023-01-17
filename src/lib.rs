#![allow(dead_code)]
#![allow(unused_imports)]
pub mod abilities;
pub mod enemies;
pub mod d2_enums;
pub mod weapons;
pub mod js_types;
pub mod perks;
pub mod activity;

use crate::perks::{Perk, Perks};
use crate::weapons::Weapon;
use js_types::{JsPerk, JsStat};
use activity::damage_calc::Activity;
use std::cell::RefCell;
use std::collections::HashMap;
use std::panic;
use wasm_bindgen::prelude::*;
use abilities::Ability;
use enemies::Enemy;
use d2_enums::StatHashes;

#[wasm_bindgen]
extern "C" {
    //foreign function interface
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

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

#[wasm_bindgen]
pub fn is_weapon_init() -> bool {
    PERS_DATA.with(|weapon| weapon.borrow().weapon.id != 0)
}

#[wasm_bindgen]
pub fn weapon_id() -> u32 {
    PERS_DATA.with(|perm_data| perm_data.borrow().weapon.id)
}

#[wasm_bindgen]
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

#[wasm_bindgen]
pub fn is_perk_implemented(perk_hash: u32) -> bool {
    // let rust_hash = perk_hash.as_f64().unwrap() as u32;
    Perks::from_u32(perk_hash) != Perks::Ignore
}

#[wasm_bindgen]
pub fn add_perk(perk_data: JsPerk, add: bool) {
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

#[wasm_bindgen]
pub fn query_perks() -> Vec<u32> {
    //let perk_list =
    PERS_DATA.with(|perm_data| perm_data.borrow_mut().weapon.list_perk_ids())
    // return Ok(serde_wasm_bindgen::to_value(&perk_list)?);
}

#[wasm_bindgen]
pub fn change_perk_value(perk_hash: u32, new_value: i32) {
    PERS_DATA.with(|perm_data| {
        perm_data
            .borrow_mut()
            .weapon
            .change_perk_val(perk_hash, new_value)
    });
}

//---------------ACTIVITY---------------//
#[wasm_bindgen]
pub fn set_activity(activity_data: JsValue) {
    let r_activity_data: Activity = serde_wasm_bindgen::from_value(activity_data).unwrap();
    PERS_DATA.with(|perm_data| {
        perm_data.borrow_mut().activity = r_activity_data;
    });
}

#[wasm_bindgen]
pub fn get_activity() -> JsValue {
    let activity = PERS_DATA.with(|perm_data| perm_data.borrow().activity.clone());
    serde_wasm_bindgen::to_value(&activity).unwrap()
}
