use std::collections::HashMap;

use crate::D2Enums;
use crate::D2Enemy::EnemyType;

fn vec_product(vec: Vec<f64>) -> f64 {
    let mut product = 1.0;
    for i in vec {
        product *= i;
    }
    product
}

#[derive(Debug,Clone)]
pub struct BuffPackage {
    pve: Vec<f64>,
    pvp: Vec<f64>,
    minor: Vec<f64>,
    elite: Vec<f64>,
    miniboss: Vec<f64>,
    boss: Vec<f64>,
    vehicle: Vec<f64>,
}
impl BuffPackage {
    pub fn new() -> BuffPackage {
        BuffPackage {
            pve: vec![1.0],
            pvp: vec![1.0],
            minor: vec![1.0],
            elite: vec![1.0],
            miniboss: vec![1.0],
            boss: vec![1.0],
            vehicle: vec![1.0],
        }
    }
    pub fn new_params(
        pve: f64,
        pvp: f64,
        minor: f64,
        elite: f64,
        miniboss: f64,
        boss: f64,
        vehicle: f64,
    ) -> BuffPackage {
        BuffPackage {
            pve: vec![pve],
            pvp: vec![pvp],
            minor: vec![minor],
            elite: vec![elite],
            miniboss: vec![miniboss],
            boss: vec![boss],
            vehicle: vec![vehicle],
        }
    }
    pub fn add_enemy_modifier(&mut self, enemy: &EnemyType, value: f64) {
        match enemy {
            EnemyType::MINOR => self.minor.push(value),
            EnemyType::ELITE => self.elite.push(value),
            EnemyType::MINIBOSS => self.miniboss.push(value),
            EnemyType::BOSS => self.boss.push(value),
            EnemyType::VEHICLE => self.vehicle.push(value),
            _ => (),
        }
    }
    pub fn add_pve_modifier(&mut self, value: f64) {
        self.pve.push(value);
    }
    pub fn add_pvp_modifier(&mut self, value: f64) {
        self.pvp.push(value);
    }
    pub fn remove_enemy_modifier(&mut self, enemy: &EnemyType, value: f64) {
        let wanted_vec = match enemy {
            EnemyType::MINOR => &mut self.minor,
            EnemyType::ELITE => &mut self.elite,
            EnemyType::MINIBOSS => &mut self.miniboss,
            EnemyType::BOSS => &mut self.boss,
            EnemyType::VEHICLE => &mut self.vehicle,
            _ => return,
        };
        for i in 0..wanted_vec.len() {
            if wanted_vec[i] == value {
                wanted_vec.remove(i);
                break;
            }
        }
    }
    pub fn remove_pve_modifier(&mut self, value: f64) {
        for i in 0..self.pve.len() {
            if self.pve[i] == value {
                self.pve.remove(i);
                break;
            }
        }
    }
    pub fn remove_pvp_modifier(&mut self, value: f64) {
        for i in 0..self.pvp.len() {
            if self.pvp[i] == value {
                self.pvp.remove(i);
                break;
            }
        }
    }
    pub fn get_pve_buff(&self, enemy: &EnemyType) -> f64 {
        let wanted_vec = match enemy {
            EnemyType::MINOR => self.minor.clone(),
            EnemyType::ELITE => self.elite.clone(),
            EnemyType::MINIBOSS => self.miniboss.clone(),
            EnemyType::BOSS => self.boss.clone(),
            EnemyType::VEHICLE => self.vehicle.clone(),
            _ => vec![1.0],
        };
        vec_product(self.pve.clone()) * vec_product(wanted_vec)
    }
    pub fn get_pvp_buff(&self) -> f64 {
        vec_product(self.pvp.clone())
    }
}

#[derive(Debug,Clone)]
pub struct FiringConfig {
    pub burst_delay: f64,
    pub burst_duration: f64,
    pub burst_size: i32,
    pub one_ammo_burst: bool,
    pub is_charge: bool,
    pub is_explosive: bool,
}

#[derive(Debug,Clone)]
pub struct FrameData {
    pub frame_name: String,
    pub weapon_type: D2Enums::WeaponType,
    pub base_damage: f64,
    pub crit_mult: f64,
    pub range_data: Option<HashMap<String, f64>>,
    pub reload_data: Option<HashMap<String, f64>>,
    pub handling_data: Option<HashMap<String, f64>>,
    pub damage_modifiers: BuffPackage,
    pub firing_settings: FiringConfig,
}
impl Default for FrameData {
    fn default() -> Self {
        FrameData {
            frame_name: String::from(""),
            weapon_type: D2Enums::WeaponType::UNKNOWN,
            base_damage: 0.0,
            crit_mult: 1.0,
            range_data: None,
            reload_data: None,
            handling_data: None,
            damage_modifiers: BuffPackage::new(),
            firing_settings: FiringConfig {
                burst_delay: 0.0,
                burst_duration: 0.0,
                burst_size: 0,
                one_ammo_burst: false,
                is_charge: false,
                is_explosive: false,
            },
        }
    }
}

#[derive(Debug,Clone)]
pub struct WeaponData {
    pub name: String,
    pub weapon_type_enum: D2Enums::WeaponType,
    pub intrinsic_hash: i32,
    pub stats: HashMap<u32, i32>,
    pub stat_layout: HashMap<String, String>,
    // pub perks: HashMap<i32, Vec<(i32, bool)>>,
    // pub image: HashMap<String, String>,
    pub slot: D2Enums::WeaponSlot,
    pub damage_type: D2Enums::DamageType,
    pub ammo_type: D2Enums::AmmoType,
}
impl Default for WeaponData {
    fn default() -> Self {
        WeaponData {
            name: String::from(""),
            weapon_type_enum: D2Enums::WeaponType::UNKNOWN,
            intrinsic_hash: 0,
            stats: HashMap::new(),
            stat_layout: HashMap::new(),
            // perks: HashMap::new(),
            // image: HashMap::new(),
            slot: D2Enums::WeaponSlot::UNKNOWN,
            damage_type: D2Enums::DamageType::UNKNOWN,
            ammo_type: D2Enums::AmmoType::UNKNOWN,
        }
    }
}
#[derive(Debug,Clone,Default)]
pub struct HandlingOut {
    pub ready: f64,
    pub stow: f64,
    pub ads: f64,
}


