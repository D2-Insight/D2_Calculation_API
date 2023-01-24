#![cfg(feature = "python")]

use pyo3::{prelude::*, types::PyDict};
use std::collections::HashMap;

use crate::{weapons::FiringConfig, perks::Perk};

use super::rs_types::{
    AmmoFormula, DamageMods, DpsResponse, HandlingFormula, HandlingResponse, MagazineResponse,
    RangeFormula, RangeResponse, ReloadFormula, ReloadResponse, ReserveResponse, TtkResponse, QuadraticFormula,
};

#[derive(Debug, Clone, Default)]
#[pyclass(name = "Weapon")]
pub struct PyWeapon {
    #[pyo3(get, set)]
    pub hash: u32,
    #[pyo3(get, set)]
    pub weapon_type: u32,
    #[pyo3(get, set)]
    pub damage_type: u32,
    #[pyo3(get, set)]
    pub weapon_slot: u32,
    #[pyo3(get, set)]
    pub ammo_type: u32,
    #[pyo3(get, set)]
    pub perks: HashMap<u32, PyPerk>,
    #[pyo3(get, set)]
    pub stats: HashMap<u32, i32>,
    #[pyo3(get, set)]
    pub damage_mods: PyDamageModifiers,
    #[pyo3(get, set)]
    pub formulas: PyWeaponFormula,
}

#[derive(Debug, Clone, Default)]
#[pyclass(name = "DamageModifiers", dict)]
pub struct PyDamageModifiers {
    pub pve: f64, // not reccomended to use but gives users a way to update stuff themselves
    pub vehicle: f64,
    pub boss: f64,
    pub miniboss: f64,
    pub champion: f64,
    pub elite: f64,
    pub minor: f64,
}
#[pymethods]
impl PyDamageModifiers {
    #[new]
    fn new() -> Self {
        PyDamageModifiers {
            pve: 1.0,
            vehicle: 1.0,
            boss: 1.0,
            miniboss: 1.0,
            champion: 1.0,
            elite: 1.0,
            minor: 1.0,
        }
    }
    #[staticmethod]
    fn from_dict(_dct: &PyDict) -> PyResult<Self> {
        let mut dmg_mods = PyDamageModifiers::new();
        for (key, value) in _dct.iter() {
            let key = key.extract::<String>()?;
            let value = value.extract::<f64>()?;
            match key.as_str() {
                "pve" => dmg_mods.pve = value,
                "vehicle" => dmg_mods.vehicle = value,
                "boss" => dmg_mods.boss = value,
                "miniboss" => dmg_mods.miniboss = value,
                "champion" => dmg_mods.champion = value,
                "elite" => dmg_mods.elite = value,
                "minor" => dmg_mods.minor = value,
                _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid key")),
            }
        }
        Ok(dmg_mods)
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "DamageModifiers(pve={}, vehicle={}, boss={}, miniboss={}, champion={}, elite={}, minor={})",
            self.pve, self.vehicle, self.boss, self.miniboss, self.champion, self.elite, self.minor
        ))
    }
}
impl Into<DamageMods> for PyDamageModifiers {
    fn into(self) -> DamageMods {
        DamageMods {
            pve: self.pve,
            vehicle: self.vehicle,
            boss: self.boss,
            miniboss: self.miniboss,
            champion: self.champion,
            elite: self.elite,
            minor: self.minor,
        }
    }
}

#[derive(Debug, Clone, Default)]
#[pyclass(name = "RangeFormula")]
pub struct PyRangeFormula {
    pub vpp_start: f64,
    pub offset_start: f64,
    pub vpp_end: f64,
    pub offset_end: f64,
    pub floor_percent: f64,
    pub is_fusion: bool,
}
#[pymethods]
impl PyRangeFormula {
    #[new]
    fn new() -> Self {
        PyRangeFormula {
            vpp_start: -1.0,
            offset_start: 0.0,
            vpp_end: 0.0,
            offset_end: -2.0,
            floor_percent: 1.0,
            is_fusion: false,
        }
    }
    #[staticmethod]
    fn from_dict(_dct: &PyDict) -> PyResult<Self> {
        let mut range_formula = PyRangeFormula::new();
        for (key, value) in _dct.iter() {
            let key = key.extract::<String>()?;
            let value = value.extract::<f64>()?;
            match key.as_str() {
                "vpp_start" => range_formula.vpp_start = value,
                "offset_start" => range_formula.offset_start = value,
                "vpp_end" => range_formula.vpp_end = value,
                "offset_end" => range_formula.offset_end = value,
                "floor_percent" => range_formula.floor_percent = value,
                "is_fusion" => range_formula.is_fusion = value > 0.0,
                _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid key")),
            }
        }
        Ok(range_formula)
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "RangeFormula(vpp_start={}, offset_start={}, vpp_end={}, offset_end={}, floor_percent={}, is_fusion={})",
            self.vpp_start, self.offset_start, self.vpp_end, self.offset_end, self.floor_percent, self.is_fusion
        ))
    }
}
impl Into<RangeFormula> for PyRangeFormula {
    fn into(self) -> RangeFormula {
        RangeFormula {
            start: QuadraticFormula {
                evpp: 0.0,
                vpp: self.vpp_start,
                offset: self.offset_start,
            },
            end: QuadraticFormula {
                evpp: 0.0,
                vpp: self.vpp_end,
                offset: self.offset_end,
            },
            floor_percent: self.floor_percent,
            is_fusion: self.is_fusion,
        }
    }
}

#[derive(Debug, Clone, Default)]
#[pyclass(name = "ReloadFormula")]
pub struct PyReloadFormula {
    pub evpp: f64,
    pub vpp: f64,
    pub offset: f64,
    pub ammo_percent: f64,
}
#[pymethods]
impl PyReloadFormula {
    #[new]
    fn new() -> Self {
        PyReloadFormula {
            evpp: 0.0,
            vpp: 0.0,
            offset: 0.0,
            ammo_percent: 1.0,
        }
    }
    #[staticmethod]
    fn from_dict(_dct: &PyDict) -> PyResult<Self> {
        let mut reload_formula = PyReloadFormula::new();
        for (key, value) in _dct.iter() {
            let key = key.extract::<String>()?;
            let value = value.extract::<f64>()?;
            match key.as_str() {
                "evpp" => reload_formula.evpp = value,
                "vpp" => reload_formula.vpp = value,
                "offset" => reload_formula.offset = value,
                "ammo_percent" => reload_formula.ammo_percent = value,
                _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid key")),
            }
        }
        Ok(reload_formula)
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "ReloadFormula(evpp={}, vpp={}, offset={}, ammo_percent={})",
            self.evpp, self.vpp, self.offset, self.ammo_percent
        ))
    }
}
impl Into<ReloadFormula> for PyReloadFormula {
    fn into(self) -> ReloadFormula {
        ReloadFormula {
            reload_data: QuadraticFormula {
                evpp: self.evpp,
                vpp: self.vpp,
                offset: self.offset,
            },
            ammo_percent: self.ammo_percent,
        }
    }
}

#[derive(Debug, Clone, Default)]
#[pyclass(name = "HandlingFormula")]
pub struct PyHandlingFormula {
    pub ready_vpp: f64,
    pub ready_offset: f64,
    pub stow_vpp: f64,
    pub stow_offset: f64,
    pub ads_vpp: f64,
    pub ads_offset: f64,
}
#[pymethods]
impl PyHandlingFormula {
    #[new]
    fn new() -> Self {
        PyHandlingFormula {
            ready_vpp: 0.0,
            ready_offset: 0.0,
            stow_vpp: 0.0,
            stow_offset: 0.0,
            ads_vpp: 0.0,
            ads_offset: 0.0,
        }
    }
    #[staticmethod]
    fn from_dict(_dct: &PyDict) -> PyResult<Self> {
        let mut handling_formula = PyHandlingFormula::new();
        for (key, value) in _dct.iter() {
            let key = key.extract::<String>()?;
            let value = value.extract::<HashMap<String, f64>>()?;
            match key.as_str() {
                "ready" => {handling_formula.ready_vpp = value["vpp"]; handling_formula.ready_offset = value["offset"];}
                "stow" => {handling_formula.stow_vpp = value["vpp"]; handling_formula.stow_offset = value["offset"];}
                "ads" => {handling_formula.ads_vpp = value["vpp"]; handling_formula.ads_offset = value["offset"];}
                _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid key")),
            }
        }
        Ok(handling_formula)
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "HandlingFormula(ready_vpp={}, ready_offset={}, stow_vpp={}, stow_offset={}, ads_vpp={}, ads_offset={})",
            self.ready_vpp, self.ready_offset, self.stow_vpp, self.stow_offset, self.ads_vpp, self.ads_offset
        ))
    }
}
impl Into<HandlingFormula> for PyHandlingFormula {
    fn into(self) -> HandlingFormula {
        HandlingFormula {
            ready: QuadraticFormula {
                evpp: 0.0,
                vpp: self.ready_vpp,
                offset: self.ready_offset,
            },
            stow: QuadraticFormula {
                evpp: 0.0,
                vpp: self.stow_vpp,
                offset: self.stow_offset,
            },
            ads: QuadraticFormula {
                evpp: 0.0,
                vpp: self.ads_vpp,
                offset: self.ads_offset,
            },
        }
    }
}

#[derive(Debug, Clone, Default)]
#[pyclass(name = "FiringData")]
pub struct PyFiringData {
    pub damage: f64,
    pub crit_mult: f64,
    pub burst_delay: f64,
    pub burst_duration: f64,
    pub burst_size: i32,
    pub one_ammo_burst: bool,
    pub is_charge: bool,
    pub is_explosive: bool,
}
#[pymethods]
impl PyFiringData {
    #[new]
    fn new() -> Self {
        PyFiringData {
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
    #[staticmethod]
    fn from_dict(_dct: &PyDict) -> PyResult<Self> {
        let mut firing_data = PyFiringData::new();
        for (key, value) in _dct.iter() {
            let key = key.extract::<String>()?;
            let f_value = value.extract::<f64>()?;
            let b_value = value.extract::<bool>()?;
            match key.as_str() {
                "damage" => firing_data.damage = f_value,
                "crit_mult" => firing_data.crit_mult = f_value,
                "burst_delay" => firing_data.burst_delay = f_value,
                "burst_duration" => firing_data.burst_duration = f_value,
                "burst_size" => firing_data.burst_size = f_value as i32,
                "one_ammo_burst" => firing_data.one_ammo_burst = b_value,
                "is_charge" => firing_data.is_charge = b_value,
                "is_explosive" => firing_data.is_explosive = b_value,
                _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid key")),
            }
        }
        Ok(firing_data)
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "FiringData(damage={}, crit_mult={}, burst_delay={}, burst_duration={}, burst_size={}, one_ammo_burst={}, is_charge={}, is_explosive={})",
            self.damage, self.crit_mult, self.burst_delay, self.burst_duration, self.burst_size, self.one_ammo_burst, self.is_charge, self.is_explosive
        ))
    }
}
impl Into<FiringConfig> for PyFiringData {
    fn into(self) -> FiringConfig {
        FiringConfig {
            burst_delay: self.burst_delay,
            burst_duration: self.burst_duration,
            burst_size: self.burst_size,
            one_ammo_burst: self.one_ammo_burst,
            is_charge: self.is_charge,
            is_explosive: self.is_explosive,
        }
    }
}

#[derive(Debug, Clone, Default)]
#[pyclass(name = "AmmoFormula")]
pub struct PyAmmoFormula {
    pub mag_evpp: f64,
    pub mag_vpp: f64,
    pub mag_offset: f64,
    pub mag_round_to_nearest: i32,
    pub reserve_formulas: HashMap<i32, (f64, f64)>,
}
#[pymethods]
impl PyAmmoFormula {
    #[new]
    pub fn new() -> Self {
        PyAmmoFormula {
            mag_evpp: 0.0,
            mag_vpp: 0.0,
            mag_offset: 0.0,
            mag_round_to_nearest: 0,
            reserve_formulas: HashMap::new(),
        }
    }
    //TODO
    #[staticmethod]
    pub fn from_dict(_dct: &PyDict) -> PyResult<Self> {
        let mut ammo_formula = PyAmmoFormula::new();
        for (key, value) in _dct.iter() {
            let key = key.extract::<String>()?;
            let f_value = value.extract::<f64>()?;
            let i_value = value.extract::<i32>()?;
            let h_value = value.extract::<HashMap<i32, (f64, f64)>>()?;
            match key.as_str() {
                "mag_evpp" => ammo_formula.mag_evpp = f_value,
                "mag_vpp" => ammo_formula.mag_vpp = f_value,
                "mag_offset" => ammo_formula.mag_offset = f_value,
                "mag_round_to_nearest" => ammo_formula.mag_round_to_nearest = i_value,
                "reserve_formulas" => ammo_formula.reserve_formulas = h_value,
                _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid key")),
            }
        }
        Ok(ammo_formula)
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "AmmoFormula(mag_evpp={}, mag_vpp={}, mag_offset={}, mag_round_to_nearest={}, reserve_formulas={:?})",
            self.mag_evpp, self.mag_vpp, self.mag_offset, self.mag_round_to_nearest, self.reserve_formulas
        ))
    }
}
impl Into<AmmoFormula> for PyAmmoFormula {
    fn into(self) -> AmmoFormula {
        AmmoFormula {
            mag: QuadraticFormula {
                evpp: self.mag_evpp,
                vpp: self.mag_vpp,
                offset: self.mag_offset,
            },
            round_to_nearest: self.mag_round_to_nearest,
            reserves: self
                .reserve_formulas
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        QuadraticFormula {
                            evpp: 0.0,
                            vpp: v.0,
                            offset: v.1,
                        },
                    )
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Default)]
#[pyclass(name = "WeaponFormula")]
pub struct PyWeaponFormula {
    #[pyo3(get, set)]
    pub range_data: PyRangeFormula,
    #[pyo3(get, set)]
    pub reload_data: PyReloadFormula,
    #[pyo3(get, set)]
    pub handling_data: PyHandlingFormula,
    #[pyo3(get, set)]
    pub firing_data: PyFiringData,
    #[pyo3(get, set)]
    pub ammo_data: PyAmmoFormula,
}

#[derive(Debug, Clone, Default)]
#[pyclass(name = "Perk")]
pub struct PyPerk {
    pub stat_buffs: HashMap<u32, i32>,
    pub enhanced: bool,
    pub value: u32, //used for toggle and stacks
    pub hash: u32,
}
#[pymethods]
impl PyPerk {
    #[new]
    pub fn new() -> Self {
        PyPerk {
            stat_buffs: HashMap::new(),
            enhanced: false,
            value: 0,
            hash: 0,
        }
    }
    #[staticmethod]
    pub fn __init__(_stat_buffs: HashMap<u32, f64>, _enhanced: bool, _value: u32, _hash: u32) -> PyResult<Self> {
        let perk = PyPerk {
            stat_buffs: _stat_buffs.into_iter().map(|(k, v)| (k, v as i32)).collect(),
            enhanced: _enhanced,
            value: _value,
            hash: _hash,
        };
        Ok(perk)
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "Perk(stat_buffs={:?}, enhanced={}, value={}, hash={})",
            self.stat_buffs, self.enhanced, self.value, self.hash
        ))
    }
}
impl Into<Perk> for PyPerk {
    fn into(self) -> Perk {
        Perk {
            stat_buffs: self.stat_buffs,
            enhanced: self.enhanced,
            value: self.value,
            hash: self.hash,
        }
    }
}
