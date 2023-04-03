use std::{collections::HashMap, sync::Arc};

use crate::{PERS_DATA, d2_enums::{DamageType, AmmoType, WeaponType, StatHashes}, weapons::{Weapon, Stat}};

const FLOAT_DELTA: f32 = 0.001;


fn setup() {
    let vec = Vec::<u8>::from("bozo".to_string());
    let mut hash = 0;
    for i in 0..vec.len() {
        hash += vec[i] as u32;
        if i < vec.len() - 1 {
            hash <<= 8;
        }
    }
    let mut new_weapon = Weapon::generate_weapon(
        hash,//bozo as u32 :)
        13,//pulse
        69420,//test pulse
        1,//primary
        3373582085,//kinetic
    ).unwrap();
    let mut stats = HashMap::new();
    stats.insert(StatHashes::RELOAD.into(), Stat::from(50));
    stats.insert(StatHashes::HANDLING.into(), Stat::from(50));
    stats.insert(StatHashes::RANGE.into(), Stat::from(50));
    new_weapon.set_stats(stats);
    PERS_DATA.with(|perm_data| {
        perm_data.borrow_mut().weapon = new_weapon;
    });
}

#[test]
fn test_weapon_setup() {
    setup();
    PERS_DATA.with(|perm_data| {
        let mut weapon = perm_data.borrow().weapon.clone();
        assert_eq!(weapon.damage_type, DamageType::KINETIC);
        assert_eq!(weapon.ammo_type, AmmoType::PRIMARY);
        assert_eq!(weapon.intrinsic_hash, 69420);
        assert_eq!(weapon.weapon_type, WeaponType::PULSERIFLE);
        assert_eq!(weapon.get_stats().get(&(StatHashes::HANDLING.into())).unwrap().val(), 50);
    });
}

// #[test]
// fn test_reload() {
//     setup();
//     PERS_DATA.with(|perm_data| {
//         let mut weapon = perm_data.borrow_mut().weapon.clone();
//         assert_eq!(weapon.calc_reload_time(None, None, true), )
//     });
// }