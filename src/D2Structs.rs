use std::collections::HashMap;

use crate::js_types::JsDamageModifiers;
use crate::D2Enemy::EnemyType;
use crate::D2Enums;

fn vec_product(vec: Vec<f64>) -> f64 {
    let mut product = 1.0;
    for i in vec {
        product *= i;
    }
    product
}

#[derive(Debug, Clone)]
pub struct BuffPackage {
    pub pve: f64,
    pub minor: f64,
    pub elite: f64,
    pub miniboss: f64,
    pub boss: f64,
    pub vehicle: f64,
}
impl BuffPackage {
    pub fn new() -> BuffPackage {
        BuffPackage {
            pve: 1.0,
            minor: 1.0,
            elite: 1.0,
            miniboss: 1.0,
            boss: 1.0,
            vehicle: 1.0,
        }
    }
    pub fn from_js(js_equiv: &JsDamageModifiers) -> BuffPackage {
        BuffPackage {
            pve: js_equiv.global,
            minor: js_equiv.minor,
            elite: js_equiv.elite,
            miniboss: js_equiv.miniboss,
            boss: js_equiv.boss,
            vehicle: js_equiv.vehicle,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct FiringConfig {
    pub burst_delay: f64,
    pub burst_duration: f64,
    pub burst_size: i32,
    pub one_ammo_burst: bool,
    pub is_charge: bool,
    pub is_explosive: bool,
}

// #[derive(Debug, Clone)]
// pub struct FrameData {
//     pub frame_name: String,
//     pub weapon_type: D2Enums::WeaponType,
//     pub base_damage: f64,
//     pub crit_mult: f64,
//     pub range_data: Option<HashMap<String, f64>>,
//     pub reload_data: Option<HashMap<String, f64>>,
//     pub handling_data: Option<HashMap<String, f64>>,
//     pub damage_modifiers: BuffPackage,
//     pub firing_settings: FiringConfig,
// }
// impl Default for FrameData {
//     fn default() -> Self {
//         FrameData {
//             frame_name: String::from(""),
//             weapon_type: D2Enums::WeaponType::UNKNOWN,
//             base_damage: 0.0,
//             crit_mult: 1.0,
//             range_data: None,
//             reload_data: None,
//             handling_data: None,
//             damage_modifiers: BuffPackage::new(),
//             firing_settings: FiringConfig {
//                 burst_delay: 0.0,
//                 burst_duration: 0.0,
//                 burst_size: 0,
//                 one_ammo_burst: false,
//                 is_charge: false,
//                 is_explosive: false,
//             },
//         }
//     }
// }

// #[derive(Debug, Clone)]
// pub struct WeaponData {
//     pub name: String,
//     pub weapon_type_enum: D2Enums::WeaponType,
//     pub intrinsic_hash: i32,
//     pub stats: HashMap<u32, i32>,
//     pub stat_layout: HashMap<String, String>,
//     // pub perks: HashMap<i32, Vec<(i32, bool)>>,
//     // pub image: HashMap<String, String>,
//     pub slot: D2Enums::WeaponSlot,
//     pub damage_type: D2Enums::DamageType,
//     pub ammo_type: D2Enums::AmmoType,
// }
// impl Default for WeaponData {
//     fn default() -> Self {
//         WeaponData {
//             name: String::from(""),
//             weapon_type_enum: D2Enums::WeaponType::UNKNOWN,
//             intrinsic_hash: 0,
//             stats: HashMap::new(),
//             stat_layout: HashMap::new(),
//             // perks: HashMap::new(),
//             // image: HashMap::new(),
//             slot: D2Enums::WeaponSlot::UNKNOWN,
//             damage_type: D2Enums::DamageType::UNKNOWN,
//             ammo_type: D2Enums::AmmoType::UNKNOWN,
//         }
//     }
// }
// #[derive(Debug, Clone, Default)]
// pub struct HandlingOut {
//     pub ready: f64,
//     pub stow: f64,
//     pub ads: f64,
// }
