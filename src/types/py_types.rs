#![cfg(feature = "python")]

use pyo3::{prelude::*, types::PyDict};
use std::collections::HashMap;

use crate::{
    activity::{damage_calc::DifficultyOptions, Activity, Player, PlayerClass},
    enemies::{Enemy, EnemyType},
    perks::Perk,
    weapons::{FiringConfig, ttk_calc::ResillienceSummary},
};

use super::rs_types::{
    AmmoFormula, AmmoResponse, DamageMods, DpsResponse, HandlingFormula, HandlingResponse,
    RangeFormula, RangeResponse, ReloadFormula, ReloadResponse, StatQuadraticFormula, FiringResponse
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
#[pymethods]
impl PyWeapon {
    #[new]
    fn new(
        _hash: u32,
        _weapon_type: u32,
        _damage_type: u32,
        _ammo_type: u32,
        _perks: HashMap<u32, PyPerk>,
        _stats: HashMap<u32, i32>,
        _damage_mods: PyDamageModifiers,
        _formulas: PyWeaponFormula,
    ) -> Self {
        PyWeapon {
            hash: _hash,
            weapon_type: _weapon_type,
            damage_type: _damage_type,
            ammo_type: _ammo_type,
            perks: _perks,
            stats: _stats,
            damage_mods: _damage_mods,
            formulas: _formulas,
        }
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "Weapon(hash={}, weapon_type={}, damage_type={}, ammo_type={}, perks={:?}, stats={:?}, damage_mods={}, formulas={})",
            self.hash,
            self.weapon_type,
            self.damage_type,
            self.ammo_type,
            self.perks,
            self.stats,
            self.damage_mods.__repr__().unwrap(),
            self.formulas.__repr__().unwrap()
        ))
    }
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
    fn new(
        _pve: f64,
        _vehicle: f64,
        _boss: f64,
        _minboss: f64,
        _champion: f64,
        _elite: f64,
        _minor: f64,
    ) -> Self {
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
    #[pyo3(name = "default")]
    #[staticmethod]
    fn py_default() -> Self {
        PyDamageModifiers::default()
    }
    #[staticmethod]
    fn from_dict(_dct: &PyDict) -> PyResult<Self> {
        let mut dmg_mods = PyDamageModifiers::default();
        for (key, value) in _dct.iter() {
            let key = key.extract::<String>()?;
            let err_str = format!("Invalid key: {}", key);
            let value = value.extract::<f64>()?;
            match key.as_str() {
                "pve" => dmg_mods.pve = value,
                "vehicle" => dmg_mods.vehicle = value,
                "boss" => dmg_mods.boss = value,
                "miniboss" => dmg_mods.miniboss = value,
                "champion" => dmg_mods.champion = value,
                "elite" => dmg_mods.elite = value,
                "minor" => dmg_mods.minor = value,
                _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(err_str)),
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
    fn new(
        _vpp_start: f64,
        _offset_start: f64,
        _vpp_end: f64,
        _offset_end: f64,
        _floor_percent: f64,
        _is_fusion: bool,
    ) -> Self {
        PyRangeFormula {
            vpp_start: _vpp_start,
            offset_start: _offset_start,
            vpp_end: _vpp_end,
            offset_end: _offset_end,
            floor_percent: _floor_percent,
            is_fusion: _is_fusion,
        }
    }
    #[pyo3(name = "default")]
    #[staticmethod]
    fn py_default() -> Self {
        PyRangeFormula::default()
    }
    #[staticmethod]
    fn from_dict(_dct: &PyDict) -> PyResult<Self> {
        let mut range_formula = PyRangeFormula::default();
        for (key, value) in _dct.iter() {
            let key = key.extract::<String>()?;
            let err_str = format!("Invalid key: {}", key);
            let value = value.extract::<f64>()?;
            match key.as_str() {
                "vpp_start" => range_formula.vpp_start = value,
                "offset_start" => range_formula.offset_start = value,
                "vpp_end" => range_formula.vpp_end = value,
                "offset_end" => range_formula.offset_end = value,
                "floor_percent" => range_formula.floor_percent = value,
                "is_fusion" => range_formula.is_fusion = value > 0.0,
                _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(err_str)),
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
            start: StatQuadraticFormula {
                evpp: 0.0,
                vpp: self.vpp_start,
                offset: self.offset_start,
            },
            end: StatQuadraticFormula {
                evpp: 0.0,
                vpp: self.vpp_end,
                offset: self.offset_end,
            },
            floor_percent: self.floor_percent,
            fusion: self.is_fusion,
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
    pub mag_multiplier: bool,
}
#[pymethods]
impl PyReloadFormula {
    #[new]
    fn new(_evpp: f64, _vpp: f64, _offset: f64, _ammo_percent: f64, _mag_multiplier: bool) -> Self {
        PyReloadFormula {
            evpp: _evpp,
            vpp: _vpp,
            offset: _offset,
            ammo_percent: _ammo_percent,
            mag_multiplier: _mag_multiplier,
        }
    }
    #[pyo3(name = "default")]
    #[staticmethod]
    fn py_default() -> Self {
        PyReloadFormula::default()
    }
    #[staticmethod]
    fn from_dict(_dct: &PyDict) -> PyResult<Self> {
        let mut reload_formula = PyReloadFormula::default();
        for (key, value) in _dct.iter() {
            let key = key.extract::<String>()?;
            let err_str = format!("Invalid key: {}", key);
            match key.as_str() {
                "evpp" => reload_formula.evpp = value.extract::<f64>()?,
                "vpp" => reload_formula.vpp = value.extract::<f64>()?,
                "offset" => reload_formula.offset = value.extract::<f64>()?,
                "ammo_percent" => reload_formula.ammo_percent = value.extract::<f64>()?,
                "mag_multiplier" => reload_formula.mag_multiplier = value.extract::<bool>()?,
                _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(err_str)),
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
            reload_data: StatQuadraticFormula {
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
    fn new(
        _ready_vpp: f64,
        _ready_offset: f64,
        _stow_vpp: f64,
        _stow_offset: f64,
        _ads_vpp: f64,
        _ads_offset: f64,
    ) -> Self {
        PyHandlingFormula {
            ready_vpp: _ready_vpp,
            ready_offset: _ready_offset,
            stow_vpp: _stow_vpp,
            stow_offset: _stow_offset,
            ads_vpp: _ads_vpp,
            ads_offset: _ads_offset,
        }
    }
    #[pyo3(name = "default")]
    #[staticmethod]
    fn py_default() -> Self {
        PyHandlingFormula::default()
    }
    #[staticmethod]
    fn from_dict(_dct: &PyDict) -> PyResult<Self> {
        let mut handling_formula = PyHandlingFormula::default();
        for (key, value) in _dct.iter() {
            let key = key.extract::<String>()?;
            let err_str = format!("Invalid key: {}", key);
            let value = value.extract::<HashMap<String, f64>>()?;
            match key.as_str() {
                "ready" => {
                    handling_formula.ready_vpp = value["vpp"];
                    handling_formula.ready_offset = value["offset"];
                }
                "stow" => {
                    handling_formula.stow_vpp = value["vpp"];
                    handling_formula.stow_offset = value["offset"];
                }
                "ads" => {
                    handling_formula.ads_vpp = value["vpp"];
                    handling_formula.ads_offset = value["offset"];
                }
                _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(err_str)),
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
            ready: StatQuadraticFormula {
                evpp: 0.0,
                vpp: self.ready_vpp,
                offset: self.ready_offset,
            },
            stow: StatQuadraticFormula {
                evpp: 0.0,
                vpp: self.stow_vpp,
                offset: self.stow_offset,
            },
            ads: StatQuadraticFormula {
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
    fn new(
        _damage: f64,
        _crit_mult: f64,
        _burst_delay: f64,
        _burst_duration: f64,
        _burst_size: i32,
        _one_ammo_burst: bool,
        _is_charge: bool,
        _is_explosive: bool,
    ) -> Self {
        PyFiringData {
            damage: _damage,
            crit_mult: _crit_mult,
            burst_delay: _burst_delay,
            burst_duration: _burst_duration,
            burst_size: _burst_size,
            one_ammo_burst: _one_ammo_burst,
            is_charge: _is_charge,
            is_explosive: _is_explosive,
        }
    }
    #[pyo3(name = "default")]
    #[staticmethod]
    fn py_default() -> Self {
        PyFiringData::default()
    }
    #[staticmethod]
    fn from_dict(_dct: &PyDict) -> PyResult<Self> {
        let mut firing_data = PyFiringData::default();
        for (key, value) in _dct.iter() {
            let key = key.extract::<String>()?;
            let err_str = format!("Invalid key: {}", key);
            match key.as_str() {
                "damage" => firing_data.damage = value.extract::<f64>()?,
                "crit_mult" => firing_data.crit_mult = value.extract::<f64>()?,
                "burst_delay" => firing_data.burst_delay = value.extract::<f64>()?,
                "burst_duration" => firing_data.burst_duration = value.extract::<f64>()?,
                "burst_size" => firing_data.burst_size = value.extract::<f64>()? as i32,
                "one_ammo_burst" => firing_data.one_ammo_burst = value.extract::<bool>()?,
                "is_charge" => firing_data.is_charge = value.extract::<bool>()?,
                "is_explosive" => firing_data.is_explosive = value.extract::<bool>()?,
                _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(err_str)),
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
            damage: self.damage,
            crit_mult: self.crit_mult,
            burst_delay: self.burst_delay,
            burst_duration: self.burst_duration,
            burst_size: self.burst_size,
            one_ammo: self.one_ammo_burst,
            charge: self.is_charge,
            explosive: self.is_explosive,
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
    pub reserve_id: u32,
}
#[pymethods]
impl PyAmmoFormula {
    #[new]
    pub fn new(
        mag_evpp: f64,
        mag_vpp: f64,
        mag_offset: f64,
        mag_round_to_nearest: i32,
        reserve_id: u32,
    ) -> Self {
        PyAmmoFormula {
            mag_evpp,
            mag_vpp,
            mag_offset,
            mag_round_to_nearest,
            reserve_id,
        }
    }
    #[pyo3(name = "default")]
    #[staticmethod]
    fn py_default() -> Self {
        PyAmmoFormula::default()
    }
    //TODO
    #[staticmethod]
    pub fn from_dict(_dct: &PyDict) -> PyResult<Self> {
        let mut ammo_formula = PyAmmoFormula::default();
        for (key, value) in _dct.iter() {
            let key = key.extract::<String>()?;
            let err_str = format!("Invalid key: {}", key);
            if key.as_str() == "magazine" {
                for (mag_key, mag_value) in value.extract::<HashMap<String, f64>>()?.iter() {
                    let err_str = format!("Invalid key: {}", mag_key);
                    match mag_key.as_str() {
                        "evpp" => ammo_formula.mag_evpp = *mag_value,
                        "vpp" => ammo_formula.mag_vpp = *mag_value,
                        "offset" => ammo_formula.mag_offset = *mag_value,
                        "round_to_nearest" => ammo_formula.mag_round_to_nearest = *mag_value as i32,
                        _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(err_str)),
                    }
                }
            }
            match key.as_str() {
                "magazine" => (),
                "reserve_id" => ammo_formula.reserve_id = value.extract::<u32>()?,
                _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(err_str)),
            }
        }
        Ok(ammo_formula)
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "AmmoFormula(mag_evpp={}, mag_vpp={}, mag_offset={}, mag_round_to_nearest={}, reserve_id={})",
            self.mag_evpp, self.mag_vpp, self.mag_offset, self.mag_round_to_nearest, self.reserve_id
        ))
    }
}
impl Into<AmmoFormula> for PyAmmoFormula {
    fn into(self) -> AmmoFormula {
        AmmoFormula {
            mag: StatQuadraticFormula {
                evpp: self.mag_evpp,
                vpp: self.mag_vpp,
                offset: self.mag_offset,
            },
            round_to: self.mag_round_to_nearest,
            reserve_id: self.reserve_id,
        }
    }
}

#[derive(Debug, Clone, Default)]
#[pyclass(name = "WeaponFormula")]
pub struct PyWeaponFormula {
    pub range_data: PyRangeFormula,
    pub reload_data: PyReloadFormula,
    pub handling_data: PyHandlingFormula,
    pub firing_data: PyFiringData,
    pub ammo_data: PyAmmoFormula,
}
#[pymethods]
impl PyWeaponFormula {
    #[new]
    pub fn new(
        _range_data: PyRangeFormula,
        _reload_data: PyReloadFormula,
        _handling_data: PyHandlingFormula,
        _firing_data: PyFiringData,
        _ammo_data: PyAmmoFormula,
    ) -> Self {
        PyWeaponFormula {
            range_data: _range_data,
            reload_data: _reload_data,
            handling_data: _handling_data,
            firing_data: _firing_data,
            ammo_data: _ammo_data,
        }
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "WeaponFormula(range_data={}, reload_data={}, handling_data={}, firing_data={}, ammo_data={})",
            self.range_data.__repr__().unwrap(),
            self.reload_data.__repr__().unwrap(),
            self.handling_data.__repr__().unwrap(),
            self.firing_data.__repr__().unwrap(),
            self.ammo_data.__repr__().unwrap()
        ))
    }
}

#[derive(Debug, Clone, Default)]
#[pyclass(name = "Trait")]
pub struct PyPerk {
    pub stat_buffs: HashMap<u32, i32>,
    pub enhanced: bool,
    pub value: u32, //used for toggle and stacks
    pub hash: u32,
}
#[pymethods]
impl PyPerk {
    #[new]
    pub fn new(_stat_buffs: HashMap<u32, i32>, _enhanced: bool, _value: u32, _hash: u32) -> Self {
        PyPerk {
            stat_buffs: _stat_buffs,
            enhanced: _enhanced,
            value: _value,
            hash: _hash,
        }
    }
    #[pyo3(name = "default")]
    #[staticmethod]
    fn py_default() -> Self {
        PyPerk::default()
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

#[derive(Debug, Clone, Default)]
#[pyclass(name = "RangeResponse")]
pub struct PyRangeResponse {
    #[pyo3(get)]
    pub hip_falloff_start: f64,
    #[pyo3(get)]
    pub hip_falloff_end: f64,
    #[pyo3(get)]
    pub ads_falloff_start: f64,
    #[pyo3(get)]
    pub ads_falloff_end: f64,
    #[pyo3(get)]
    pub floor_percent: f64,
}
#[pymethods]
impl PyRangeResponse {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "RangeResponse(hip_falloff_start={}, hip_falloff_end={}, ads_falloff_start={}, ads_falloff_end={})",
            self.hip_falloff_start, self.hip_falloff_end, self.ads_falloff_start, self.ads_falloff_end
        ))
    }
}
impl From<RangeResponse> for PyRangeResponse {
    fn from(r: RangeResponse) -> Self {
        PyRangeResponse {
            hip_falloff_start: r.hip_falloff_start,
            hip_falloff_end: r.hip_falloff_end,
            ads_falloff_start: r.ads_falloff_start,
            ads_falloff_end: r.ads_falloff_end,
            floor_percent: r.floor_percent,
        }
    }
}

#[derive(Debug, Clone, Default)]
#[pyclass(name = "HandlingResponse")]
pub struct PyHandlingResponse {
    #[pyo3(get)]
    pub ready_time: f64,
    #[pyo3(get)]
    pub stow_time: f64,
    #[pyo3(get)]
    pub ads_time: f64,
}
#[pymethods]
impl PyHandlingResponse {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "HandlingResponse(ready_time={}, stow_time={}, ads_time={})",
            self.ready_time, self.stow_time, self.ads_time
        ))
    }
}
impl From<HandlingResponse> for PyHandlingResponse {
    fn from(r: HandlingResponse) -> Self {
        PyHandlingResponse {
            ready_time: r.ready_time,
            stow_time: r.stow_time,
            ads_time: r.ads_time,
        }
    }
}

#[derive(Debug, Clone, Default)]
#[pyclass(name = "FiringResponse")]
pub struct PyFiringResponse {
    #[pyo3(get)]
    pub pvp_damage: f64,
    #[pyo3(get)]
    pub pvp_crit_mult: f64,
    #[pyo3(get)]
    pub pve_damage: f64,
    #[pyo3(get)]
    pub pve_crit_mult: f64,
    #[pyo3(get)]
    pub burst_delay: f64,
    #[pyo3(get)]
    pub burst_duration: f64,
    #[pyo3(get)]
    pub burst_size: i32,
    #[pyo3(get)]
    pub rpm: f64,
}
#[pymethods]
impl PyFiringResponse {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "FiringResponse(pvp_damage={}, pvp_crit_mult={}, pve_damage={}, pve_crit_mult={}, burst_delay={}, burst_duration={}, burst_size={}, rpm={})",
            self.pvp_damage, self.pvp_crit_mult, self.pve_damage, self.pve_crit_mult, self.burst_delay, self.burst_duration, self.burst_size, self.rpm
        ))
    }
}
impl From<FiringResponse> for PyFiringResponse {
    fn from(r: FiringResponse) -> Self {
        PyFiringResponse {
            pvp_damage: r.pvp_damage,
            pvp_crit_mult: r.pvp_crit_mult,
            pve_damage: r.pve_damage,
            pve_crit_mult: r.pve_crit_mult,
            burst_delay: r.burst_delay,
            burst_duration: r.burst_duration,
            burst_size: r.burst_size,
            rpm: r.rpm,
        }
    }
}

#[derive(Debug, Clone, Default)]
#[pyclass(name = "DpsResponse")]
pub struct PyDpsResponse {
    #[pyo3(get)]
    pub dps_per_mag: Vec<f64>,
    #[pyo3(get)]
    pub time_damage_data: Vec<(f64, f64)>,
    #[pyo3(get)]
    pub total_damage: f64,
    #[pyo3(get)]
    pub total_time: f64,
    #[pyo3(get)]
    pub total_shots: i32,
}
#[pymethods]
impl PyDpsResponse {
    fn over_time_span(&self, _start: f64, _end: f64) -> PyResult<(f64, f64)> {
        let mut total = 0.0;
        for (time, damage) in &self.time_damage_data {
            if *time >= _start && *time <= _end {
                total += damage;
            }
        }
        Ok((total, total / (_end - _start)))
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "DpsResponse(dps_per_mag={:?}, time_damage_data={:?}, total_damage={}, total_time={}, total_shots={})",
            self.dps_per_mag, self.time_damage_data, self.total_damage, self.total_time, self.total_shots
        ))
    }
}
impl From<DpsResponse> for PyDpsResponse {
    fn from(r: DpsResponse) -> Self {
        PyDpsResponse {
            dps_per_mag: r.dps_per_mag,
            time_damage_data: r.time_damage_data,
            total_damage: r.total_damage,
            total_time: r.total_time,
            total_shots: r.total_shots,
        }
    }
}

#[derive(Debug, Clone)]
#[pyclass(name = "OptimalKillData")]
pub struct PyOptimalKillData{
    headshots: i32,
    bodyshots: i32,
    time_taken: f64,
    //defines how far away this ttk is achievalbe if all hits ar crits
    all_crit_range: f64,
}
#[pymethods]
impl PyOptimalKillData {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "OptimalKillData(headshots={}, bodyshots={}, time_taken={}, all_crit_range={})",
            self.headshots, self.bodyshots, self.time_taken, self.all_crit_range
        ))
    }
}

#[derive(Debug, Clone)]
#[pyclass(name = "BodyKillData")]
pub struct PyBodyKillData{
    bodyshots: i32,
    time_taken: f64,
}
#[pymethods]
impl PyBodyKillData {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "BodyKillData(bodyshots={}, time_taken={})",
            self.bodyshots, self.time_taken
        ))
    }
}

#[derive(Debug, Clone)]
#[pyclass(name = "ResillienceSummary")]
pub struct PyResillienceSummary{
    value: i32,
    body_ttk: PyBodyKillData,
    optimal_ttk: PyOptimalKillData,
}
#[pymethods]
impl PyResillienceSummary {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "ResillienceSummary(value={}, body_ttk={:?}, optimal_ttk={:?})",
            self.value, self.body_ttk, self.optimal_ttk
        ))
    }
}
impl From<ResillienceSummary> for PyResillienceSummary {
    fn from(r: ResillienceSummary) -> Self {
        PyResillienceSummary {
            value: r.value,
            body_ttk: PyBodyKillData{
                bodyshots: r.body_ttk.bodyshots,
                time_taken: r.body_ttk.time_taken,
            },
            optimal_ttk: PyOptimalKillData{
                headshots: r.optimal_ttk.headshots,
                bodyshots: r.optimal_ttk.bodyshots,
                time_taken: r.optimal_ttk.time_taken,
                all_crit_range: r.optimal_ttk.all_crit_range,
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
#[pyclass(name = "Activity")]
pub struct PyActivity {
    #[pyo3(get, set)]
    pub name: String,
    #[pyo3(get, set)]
    pub difficulty: PyDifficultyOptions,
    #[pyo3(get, set)]
    pub rpl: u32,
    #[pyo3(get, set)]
    pub cap: i32,
}
#[pymethods]
impl PyActivity {
    #[new]
    pub fn new(_name: String, _difficulty: PyDifficultyOptions, _rpl: u32, _cap: i32) -> Self {
        PyActivity {
            name: _name,
            difficulty: _difficulty,
            rpl: _rpl,
            cap: _cap,
        }
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "Activity(name={}, difficulty={:?}, rpl={}, cap={})",
            self.name, self.difficulty, self.rpl, self.cap
        ))
    }
}
impl From<Activity> for PyActivity {
    fn from(a: Activity) -> Self {
        let diff = match a.difficulty {
            DifficultyOptions::NORMAL => PyDifficultyOptions::NORMAL,
            DifficultyOptions::RAID => PyDifficultyOptions::RAID,
            DifficultyOptions::MASTER => PyDifficultyOptions::MASTER,
        };
        PyActivity {
            name: a.name,
            difficulty: diff,
            rpl: a.rpl,
            cap: a.cap,
        }
    }
}

#[derive(Debug, Clone, Default)]
#[pyclass(name = "Player")]
pub struct PyPlayer {
    #[pyo3(get, set)]
    pub powerl_level: u32,
    #[pyo3(get, set)]
    pub class: PyPlayerClass,
}
#[pymethods]
impl PyPlayer {
    #[new]
    pub fn new(_power_level: u32, _class: PyPlayerClass) -> Self {
        PyPlayer {
            powerl_level: _power_level,
            class: _class,
        }
    }
    #[pyo3(name = "default")]
    #[staticmethod]
    fn py_default() -> Self {
        PyPlayer::default()
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "Player(power_level={}, class={:?})",
            self.powerl_level, self.class
        ))
    }
}
impl Into<Player> for PyPlayer {
    fn into(self) -> Player {
        Player {
            pl: self.powerl_level,
            class: self.class.into(),
        }
    }
}
impl From<Player> for PyPlayer {
    fn from(p: Player) -> Self {
        PyPlayer {
            powerl_level: p.pl,
            class: p.class.into(),
        }
    }
}

#[derive(Debug, Clone, Default)]
#[pyclass(name = "Enemy")]
pub struct PyEnemy {
    #[pyo3(get, set)]
    pub health: f64,
    #[pyo3(get, set)]
    pub damage: f64,
    #[pyo3(get, set)]
    pub damage_resistance: f64,
    #[pyo3(get, set)]
    pub type_: PyEnemyType,
    #[pyo3(get, set)]
    pub tier: u8,
}
#[pymethods]
impl PyEnemy {
    #[new]
    pub fn new(
        _health: f64,
        _damage: f64,
        _damage_resistance: f64,
        _type_: PyEnemyType,
        _tier: u8,
    ) -> Self {
        PyEnemy {
            health: _health,
            damage: _damage,
            damage_resistance: _damage_resistance,
            type_: _type_,
            tier: _tier,
        }
    }
    #[pyo3(name = "default")]
    #[staticmethod]
    fn py_default() -> Self {
        PyEnemy::default()
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "Enemy(health={}, damage={}, damage_resistance={}, type_={:?}, tier={})",
            self.health, self.damage, self.damage_resistance, self.type_, self.tier
        ))
    }
}
impl From<Enemy> for PyEnemy {
    fn from(e: Enemy) -> Self {
        PyEnemy {
            health: e.health,
            damage: e.damage,
            damage_resistance: e.damage_resistance,
            type_: e.type_.into(),
            tier: e.tier,
        }
    }
}
impl Into<Enemy> for PyEnemy {
    fn into(self) -> Enemy {
        Enemy {
            health: self.health,
            damage: self.damage,
            damage_resistance: self.damage_resistance,
            type_: self.type_.into(),
            tier: self.tier,
        }
    }
}

//ENUMS///////////////////////
#[derive(Debug, Clone)]
#[pyclass(name = "DifficultyOptions")]
pub enum PyDifficultyOptions {
    NORMAL = 1,
    RAID = 2,
    MASTER = 3,
}
#[pymethods]
impl PyDifficultyOptions {
    #[pyo3(name = "default")]
    #[staticmethod]
    fn py_default() -> Self {
        PyDifficultyOptions::default()
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "DifficultyOptions({})",
            match self {
                PyDifficultyOptions::NORMAL => "NORMAL",
                PyDifficultyOptions::RAID => "RAID",
                PyDifficultyOptions::MASTER => "MASTER",
            }
        ))
    }
}
impl Default for PyDifficultyOptions {
    fn default() -> Self {
        PyDifficultyOptions::NORMAL
    }
}
impl Into<DifficultyOptions> for PyDifficultyOptions {
    fn into(self) -> DifficultyOptions {
        match self {
            PyDifficultyOptions::NORMAL => DifficultyOptions::NORMAL,
            PyDifficultyOptions::RAID => DifficultyOptions::RAID,
            PyDifficultyOptions::MASTER => DifficultyOptions::MASTER,
        }
    }
}
impl From<DifficultyOptions> for PyDifficultyOptions {
    fn from(d: DifficultyOptions) -> Self {
        match d {
            DifficultyOptions::NORMAL => PyDifficultyOptions::NORMAL,
            DifficultyOptions::RAID => PyDifficultyOptions::RAID,
            DifficultyOptions::MASTER => PyDifficultyOptions::MASTER,
        }
    }
}

#[derive(Debug, Clone)]
#[pyclass(name = "PlayerClass")]
pub enum PyPlayerClass {
    Unknown = 0,
    Titan = 1,
    Hunter = 2,
    Warlock = 3,
}
#[pymethods]
impl PyPlayerClass {
    #[pyo3(name = "default")]
    #[staticmethod]
    fn py_default() -> Self {
        PyPlayerClass::default()
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "PlayerClass({})",
            match self {
                PyPlayerClass::Unknown => "Unknown",
                PyPlayerClass::Titan => "Titan",
                PyPlayerClass::Hunter => "Hunter",
                PyPlayerClass::Warlock => "Warlock",
            }
        ))
    }
}
impl Default for PyPlayerClass {
    fn default() -> Self {
        PyPlayerClass::Unknown
    }
}
impl Into<PlayerClass> for PyPlayerClass {
    fn into(self) -> PlayerClass {
        match self {
            PyPlayerClass::Unknown => PlayerClass::Unknown,
            PyPlayerClass::Titan => PlayerClass::Titan,
            PyPlayerClass::Hunter => PlayerClass::Hunter,
            PyPlayerClass::Warlock => PlayerClass::Warlock,
        }
    }
}
impl From<PlayerClass> for PyPlayerClass {
    fn from(p: PlayerClass) -> Self {
        match p {
            PlayerClass::Unknown => PyPlayerClass::Unknown,
            PlayerClass::Titan => PyPlayerClass::Titan,
            PlayerClass::Hunter => PyPlayerClass::Hunter,
            PlayerClass::Warlock => PyPlayerClass::Warlock,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[pyclass(name = "EnemyType")]
pub enum PyEnemyType {
    MINOR,
    ELITE,
    MINIBOSS,
    BOSS,
    VEHICLE,
    ENCLAVE,
    PLAYER,
    CHAMPION,
}
#[pymethods]
impl PyEnemyType {
    #[pyo3(name = "default")]
    #[staticmethod]
    fn py_default() -> Self {
        PyEnemyType::default()
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "EnemyType({})",
            match self {
                PyEnemyType::MINOR => "MINOR",
                PyEnemyType::ELITE => "ELITE",
                PyEnemyType::MINIBOSS => "MINIBOSS",
                PyEnemyType::BOSS => "BOSS",
                PyEnemyType::VEHICLE => "VEHICLE",
                PyEnemyType::ENCLAVE => "ENCLAVE",
                PyEnemyType::PLAYER => "PLAYER",
                PyEnemyType::CHAMPION => "CHAMPION",
            }
        ))
    }
}
impl Default for PyEnemyType {
    fn default() -> Self {
        PyEnemyType::ENCLAVE
    }
}
impl Into<EnemyType> for PyEnemyType {
    fn into(self) -> EnemyType {
        match self {
            PyEnemyType::MINOR => EnemyType::MINOR,
            PyEnemyType::ELITE => EnemyType::ELITE,
            PyEnemyType::MINIBOSS => EnemyType::MINIBOSS,
            PyEnemyType::BOSS => EnemyType::BOSS,
            PyEnemyType::VEHICLE => EnemyType::VEHICLE,
            PyEnemyType::ENCLAVE => EnemyType::ENCLAVE,
            PyEnemyType::PLAYER => EnemyType::PLAYER,
            PyEnemyType::CHAMPION => EnemyType::CHAMPION,
        }
    }
}
impl From<EnemyType> for PyEnemyType {
    fn from(e: EnemyType) -> Self {
        match e {
            EnemyType::MINOR => PyEnemyType::MINOR,
            EnemyType::ELITE => PyEnemyType::ELITE,
            EnemyType::MINIBOSS => PyEnemyType::MINIBOSS,
            EnemyType::BOSS => PyEnemyType::BOSS,
            EnemyType::VEHICLE => PyEnemyType::VEHICLE,
            EnemyType::ENCLAVE => PyEnemyType::ENCLAVE,
            EnemyType::PLAYER => PyEnemyType::PLAYER,
            EnemyType::CHAMPION => PyEnemyType::CHAMPION,
        }
    }
}
