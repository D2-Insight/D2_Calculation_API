
#![cfg(not(target_arch = "wasm32"))]

use pyo3::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
#[pyclass(name = "Weapon")]
pub struct PyWeapon {
    pub hash: u32,
    pub intrinsic: u32,
    pub weapon_type: u32,
    pub damage_type: u32,
    pub weapon_slot: u32,
    pub ammo_type: u32,
    pub stats: HashMap<u32, i32>,
    pub damage_modifiers: PyDamageModifiers,
    // pub formulas: JsWeaponFormula,
}

#[pyclass(name = "DamageModifiers")]
#[derive(Debug, Clone, Default)]
pub struct PyDamageModifiers {
    pub global: f64, // not reccomended to use but gives users a way to update stuff themselves
    pub vehicle: f64,
    pub boss: f64,
    pub miniboss: f64,
    pub champion: f64,
    pub elite: f64,
    pub minor: f64,
}

