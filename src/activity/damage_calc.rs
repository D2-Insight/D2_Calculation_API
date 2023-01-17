#![allow(dead_code)]
use crate::{enemies::EnemyType, types::rs_types::DamageMods};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
struct TableKey {
    time: f64,
    value: f64,
}
#[derive(Debug, Clone)]
struct LinearTable {
    table: Vec<TableKey>,
}
impl LinearTable {
    fn evaluate(&self, time: f64) -> f64 {
        let mut index = 0;
        for i in 0..self.table.len() {
            if self.table[i].time > time {
                index = i;
                break;
            }
        }
        if index == 0 {
            return self.table[0].value;
        }
        if index == self.table.len() {
            return self.table[self.table.len() - 1].value;
        }
        let a = self.table[index - 1];
        let b = self.table[index];
        let t = (time - a.time) / (b.time - a.time);
        return a.value + (b.value - a.value) * t;
    }
    fn from_vecs(_times: [f64; 11], _values: [f64; 11]) -> LinearTable {
        let mut table = Vec::new();
        let times = _times.clone().to_vec();
        let values = _values.clone().to_vec();
        for i in 0..times.len() {
            table.push(TableKey {
                time: times[i],
                value: values[i],
            });
        }
        table.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
        return LinearTable { table };
    }
}

#[derive(Debug, Clone)]
pub struct DifficultyData {
    name: String,
    cap: i32,
    table: LinearTable,
}

const MASTER_VALUES: [f64; 11] = [
    0.85, 0.68, 0.58, 0.5336, 0.505, 0.485, 0.475, 0.46, 0.44, 0.42, 0.418,
];
const MASTER_TIMES: [f64; 11] = [
    0.0, -10.0, -20.0, -30.0, -40.0, -50.0, -60.0, -70.0, -80.0, -90.0, -99.0,
];

const NORMAL_VALUES: [f64; 11] = [
    1.0, 0.78, 0.66, 0.5914, 0.5405, 0.5, 0.475, 0.46, 0.44, 0.42, 0.418,
];
const NORMAL_TIMES: [f64; 11] = [
    0.0, -10.0, -20.0, -30.0, -40.0, -50.0, -60.0, -70.0, -80.0, -90.0, -99.0,
];

const RAID_VALUES: [f64; 11] = [
    0.925, 0.74, 0.62, 0.5623, 0.5225, 0.4925, 0.475, 0.46, 0.44, 0.42, 0.418,
];
const RAID_TIMES: [f64; 11] = [
    0.0, -10.0, -20.0, -30.0, -40.0, -50.0, -60.0, -70.0, -80.0, -90.0, -99.0,
];

const WWEAPON_DELTA_EXPONENT: f64 = 1.006736;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DifficultyOptions {
    NORMAL = 1,
    RAID = 2,
    MASTER = 3,
}
impl Default for DifficultyOptions {
    fn default() -> Self {
        DifficultyOptions::NORMAL
    }
}
impl DifficultyOptions {
    fn get_difficulty_data(&self) -> DifficultyData {
        match self {
            DifficultyOptions::NORMAL => DifficultyData {
                name: "Normal".to_string(),
                cap: 50,
                table: LinearTable::from_vecs(NORMAL_TIMES, NORMAL_VALUES),
            },
            DifficultyOptions::MASTER => DifficultyData {
                name: "Master".to_string(),
                cap: 20,
                table: LinearTable::from_vecs(MASTER_TIMES, MASTER_VALUES),
            },
            DifficultyOptions::RAID => DifficultyData {
                name: "Raid & Dungeon".to_string(),
                cap: 20,
                table: LinearTable::from_vecs(RAID_TIMES, RAID_VALUES),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    name: String,
    difficulty: DifficultyOptions,
    rpl: f64,
    cap: f64,
}
impl Default for Activity {
    fn default() -> Self {
        Activity {
            name: "Default".to_string(),
            difficulty: DifficultyOptions::default(),
            rpl: 1350.0,
            cap: 100.0,
        }
    }
}

fn rpl_mult(_rpl: f64) -> f64 {
    return (1.0 + ((1.0 / 30.0) * _rpl)) / (1.0 + 1.0 / 3.0);
}

fn pl_delta(_activity: Activity, _gpl: f64) -> f64 {
    let difficulty_data = _activity.difficulty.get_difficulty_data();
    let curve = difficulty_data.table;
    let rpl = _activity.rpl;
    let activity_cap = _activity.cap;
    let mut delta = _gpl - rpl;
    if delta < -99.0 {
        return 0.0;
    } else if delta > activity_cap {
        delta = activity_cap;
    } else {
        delta = delta; //no pass in rust?
    };
    let wep_delta_mult = WWEAPON_DELTA_EXPONENT.powf(rpl);
    let gear_delta_mult = curve.evaluate(delta);
    rpl_mult(rpl) * wep_delta_mult * gear_delta_mult
}
