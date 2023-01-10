#![allow(dead_code)]
pub mod D2Ability;
pub mod D2Enemy;
pub mod D2Enums;
pub mod D2Structs;
pub mod D2Weapon;
pub mod mathUtil;
pub mod perks;

use crate::perks::{JsPerk, Perk, Perks};
use crate::D2Weapon::Weapon;
use mathUtil::DamageCalc::Activity;
use std::cell::RefCell;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use D2Ability::Ability;
use D2Enemy::Enemy;
use D2Enums::StatHashes;
use D2Weapon::JS_Stat;

// #[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    Ok(())
}

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
pub fn weapon_id() -> JsValue {
    let id = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.id);
    return JsValue::from_f64(id as f64);
}

#[wasm_bindgen]
pub fn send_stats() -> Result<JsValue, JsValue> {
    let stats = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.stats.clone());
    let mut js_hashmap = HashMap::new();
    for (stat_hash, stat_value) in stats {
        let js_stat = JS_Stat::from_stat(&stat_value);
        js_hashmap.insert(stat_hash, js_stat);
    }
    let js_stats = serde_wasm_bindgen::to_value(&js_hashmap)?;
    Ok(js_stats)
}

#[wasm_bindgen]
pub fn is_perk_implemented(perk_hash: u32) -> bool {
    // let rust_hash = perk_hash.as_f64().unwrap() as u32;
    Perks::from_u32(perk_hash) != Perks::Ignore
}

#[wasm_bindgen]
pub fn receive_perk(perk_data: JsValue, add: bool) -> Result<(), JsValue> {
    let r_perk_data: JsPerk = serde_wasm_bindgen::from_value(perk_data)?;
    let perk = Perk::from_js(r_perk_data);
    if is_weapon_init() {
        if add {
            PERS_DATA.with(|perm_data| perm_data.borrow_mut().weapon.add_perk(perk));
        } else {
            PERS_DATA.with(|perm_data| perm_data.borrow_mut().weapon.remove_perk(perk));
        }
    } else {
        console_log!("Weapon is not ready for perks");
    }
    Ok(())
}

#[wasm_bindgen]
pub fn query_perks() -> Result<JsValue, JsValue> {
    let perk_list = PERS_DATA.with(|perm_data| perm_data.borrow_mut().weapon.list_perks());
    return Ok(serde_wasm_bindgen::to_value(&perk_list)?);
}

#[wasm_bindgen]
pub fn change_perk_value(perk_hash: u32, new_value: i32) -> Result<(), JsValue> {
    PERS_DATA.with(|perm_data| {
        perm_data
            .borrow_mut()
            .weapon
            .change_perk_val(perk_hash, new_value)
    });
    Ok(())
}

//---------------ACTIVITY---------------//
#[wasm_bindgen]
pub fn receive_activity(activity_data: JsValue) -> Result<(), JsValue> {
    let r_activity_data: Activity = serde_wasm_bindgen::from_value(activity_data)?;
    PERS_DATA.with(|perm_data| {
        perm_data.borrow_mut().activity = r_activity_data;
    });
    Ok(())
}
#[wasm_bindgen]
pub fn send_activity() -> Result<JsValue, JsValue> {
    let activity = PERS_DATA.with(|perm_data| perm_data.borrow().activity.clone());
    return Ok(serde_wasm_bindgen::to_value(&activity)?);
}
