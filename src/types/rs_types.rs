use std::collections::HashMap;

#[cfg(target_arch = "wasm32")]
use super::js_types::{
    JsAmmoResponse, JsDamageModifiers, JsDpsResponse, JsHandlingResponse, JsReloadResponse,
};

#[derive(Debug, Clone)]
pub struct DamageMods {
    pub pve: f64,
    pub minor: f64,
    pub elite: f64,
    pub miniboss: f64,
    pub champion: f64,
    pub boss: f64,
    pub vehicle: f64,
}
impl Default for DamageMods {
    fn default() -> Self {
        DamageMods {
            pve: 1.0,
            minor: 1.0,
            elite: 1.0,
            miniboss: 1.0,
            champion: 1.0,
            boss: 1.0,
            vehicle: 1.0,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct RangeFormula {
    pub start: QuadraticFormula,
    pub end: QuadraticFormula,
    pub floor_percent: f64,
    pub is_fusion: bool,
}

//even if just linear use this
#[derive(Debug, Clone, Default)]
pub struct QuadraticFormula {
    pub evpp: f64,
    pub vpp: f64,
    pub offset: f64,
}
impl QuadraticFormula {
    pub fn solve_at(&self, _x: f64) -> f64 {
        self.evpp * _x * _x + self.vpp * _x + self.offset
    }
}

#[derive(Debug, Clone, Default)]
pub struct ReloadFormula {
    pub reload_data: QuadraticFormula,
    pub ammo_percent: f64,
}

#[derive(Debug, Clone, Default)]
pub struct HandlingFormula {
    pub ready: QuadraticFormula,
    pub stow: QuadraticFormula,
    pub ads: QuadraticFormula,
}

#[derive(Debug, Clone, Default)]
pub struct AmmoFormula {
    pub mag: QuadraticFormula,
    pub reserves: HashMap<i32, QuadraticFormula>,
}

#[derive(Debug, Clone, Default)]
pub struct RangeResponse {
    pub hip_falloff_start: f64,
    pub hip_falloff_end: f64,
    pub ads_falloff_start: f64,
    pub ads_falloff_end: f64,
}

#[derive(Debug, Clone, Default)]
pub struct HandlingResponse {
    pub ready_time: f64,
    pub stow_time: f64,
    pub ads_time: f64,
}

#[derive(Debug, Clone, Default)]
pub struct AmmoResponse {
    pub mag: i32,
    pub mag_perk: i32,
    pub reserves: i32,
}

#[derive(Debug, Clone, Default)]
pub struct ReloadResponse {
    pub reload_time: f64,
    pub ammo_time: f64,
}

#[derive(Debug, Clone, Default)]
pub struct DpsResponse {
    pub dps_per_mag: Vec<f64>,
    pub damage_time_data: Vec<(f64, f64)>,
    pub total_damage: f64,
    pub total_shots: f64,
}

#[derive(Debug, Clone, Default)]
pub struct TtkResponse {
    pub ammo_needed: i32,
    pub hits_needed: i32,
    pub optimal_ttk: f64,
    pub crit_percent: f64,
    pub bodyshot_ttk: f64,
}
