#![cfg(feature = "python")]

use pyo3::{prelude::*, types::PyDict};
use std::collections::HashMap;

use crate::{
    activity::{damage_calc::DifficultyOptions, Activity, Player, PlayerClass},
    enemies::{Enemy, EnemyType},
    perks::Perk,
    weapons::{ttk_calc::ResillienceSummary, FiringData},
};

use super::rs_types::{
    AmmoFormula, AmmoResponse, DamageMods, DpsResponse, HandlingFormula, HandlingResponse,
    RangeFormula, RangeResponse, ReloadFormula, ReloadResponse, StatQuadraticFormula, FiringResponse
};



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
    pub pvp_impact_damage: f64,
    #[pyo3(get)]
    pub pvp_explosion_damage: f64,
    #[pyo3(get)]
    pub pvp_crit_mult: f64,
    #[pyo3(get)]
    pub pve_impact_damage: f64,
    #[pyo3(get)]
    pub pve_explosion_damage: f64,
    #[pyo3(get)]
    pub pve_crit_mult: f64,
    #[pyo3(get)]
    pub burst_delay: f64,
    #[pyo3(get)]
    pub inner_burst_delay: f64,
    #[pyo3(get)]
    pub burst_size: i32,
    #[pyo3(get)]
    pub rpm: f64,
}
#[pymethods]
impl PyFiringResponse {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "FiringResponse(pvp_impact_damage={}, pvp_explosion_damage={}, pvp_crit_mult={}, pve_impact_damage={}, pve_explosion_damage={}, pve_crit_mult={}, burst_delay={}, inner_burst_delay={}, burst_size={}, rpm={})",
            self.pvp_impact_damage, self.pvp_explosion_damage, self.pvp_crit_mult, self.pve_impact_damage, self.pve_explosion_damage, self.pve_crit_mult, self.burst_delay, self.inner_burst_delay, self.burst_size, self.rpm
        ))
    }
}
impl From<FiringResponse> for PyFiringResponse {
    fn from(r: FiringResponse) -> Self {
        PyFiringResponse {
            pvp_impact_damage: r.pvp_impact_damage,
            pvp_explosion_damage: r.pvp_explosion_damage,
            pvp_crit_mult: r.pvp_crit_mult,
            pve_impact_damage: r.pve_impact_damage,
            pve_explosion_damage: r.pve_explosion_damage,
            pve_crit_mult: r.pve_crit_mult,
            burst_delay: r.burst_delay,
            inner_burst_delay: r.inner_burst_delay,
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
