use std::collections::HashMap;


use crate::D2Enums::StatHashes;
use crate::D2Structs::FrameData;
use crate::D2Structs::WeaponData;
use serde::{Serialize, Deserialize};
use crate::perks::Perk;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct JS_Stat {
    base_value: i32,
    part_values: i32,
    perk_values: i32,
}
impl JS_Stat {
    pub fn from_stat(stat: &Stat) -> JS_Stat {
        JS_Stat {
            base_value: stat.base_value,
            part_values: stat.part_values.iter().sum(),
            perk_values: stat.perk_values.iter().sum(),
        }
    }
}

#[derive(Debug,Clone)]
pub struct Stat {
    pub base_value: i32,
    pub part_values: Vec<i32>,
    pub perk_values: Vec<i32>,
    //this is just for organization, it's not used in the calculation
    //can be called for use elsewhere
    pub associated_scalars: Vec<f64>,
}
impl Stat {
    pub fn new() -> Stat {
        Stat {
            base_value: 0,
            part_values: vec![0],
            perk_values: vec![0],
            associated_scalars: vec![1.0],
        }
    }
    pub fn add_mod(&mut self, mod_value: i32) {
        self.part_values.push(mod_value);
    }
    pub fn remove_mod(&mut self, mod_value: i32) {
        //remove 1 instance of mod_value from mod_values
        let mut index = 0;
        for i in 0..self.part_values.len() {
            if self.part_values[i] == mod_value {
                index = i;
                break;
            }
        }
        self.part_values.remove(index);
    }
    pub fn add_scalar(&mut self, associated_scalar: f64) {
        self.associated_scalars.push(associated_scalar);
    }
    pub fn remove_scalar(&mut self, associated_scalar: f64) {
        //remove 1 instance of associated_scalar from associated_scalars
        let mut index = 0;
        for i in 0..self.associated_scalars.len() {
            if self.associated_scalars[i] == associated_scalar {
                index = i;
                break;
            }
        }
        self.associated_scalars.remove(index);
    }
    pub fn val(&self) -> i32 {
        let mut total_mod = 0;
        for i in 0..self.part_values.len() {
            total_mod += self.part_values[i];
        }
        self.base_value + total_mod
    }
    pub fn scalar(&self) -> f64 {
        let mut total_scalar = 1.0;
        for i in 0..self.associated_scalars.len() {
            total_scalar *= self.associated_scalars[i];
        }
        total_scalar
    }
}

#[derive(Debug,Clone)]
pub struct Weapon {
    //ideally entirely interfaced with through funcs
    pub name: String,

    pub perks: HashMap<u32, Perk>,
    pub stats: HashMap<u32, Stat>,
    pub id: u32,

}
impl Weapon {
    pub fn add_perk(&mut self, _perk: Perk) {
        self.perks.insert(_perk.id, _perk);
    }
    pub fn remove_perk(&mut self, _perk: Perk) {
        self.perks.remove(&_perk.id);
    }
    pub fn list_perks(&self) -> Vec<u32> {
        let mut perk_list: Vec<u32> = Vec::new();
        for (key, _perk) in &self.perks {
            perk_list.push(*key);
        }
        perk_list
    }
    pub fn change_perk_val(&mut self, _perk_hash: u32, _val: i32) {
        let perk_opt = self.perks.get_mut(&_perk_hash);
        if perk_opt.is_some() {
            perk_opt.unwrap().value = _val;
        }
    }
}
impl Default for Weapon {
    fn default() -> Weapon {
        Weapon {
            name: String::from(""),
            perks: HashMap::new(),
            stats: HashMap::new(),
            id: 0
        }
    }
}
impl Weapon{
    pub fn reset(&mut self){//TODO: make this a trait
        self.name = String::from("");
        self.perks = HashMap::new();
        self.stats = HashMap::new();
        self.id = 0;
    }
}


