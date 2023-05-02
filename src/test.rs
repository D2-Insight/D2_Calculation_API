use std::{borrow::Borrow, cell::RefCell, collections::HashMap, pin::Pin, rc::Rc};

use num_traits::{Float, Zero};

use crate::{
    attributes::*,
    d2_enums::{AmmoType, DamageType, StatHashes, WeaponType},
    weapons::{Stat, Weapon},
    PERS_DATA,
};

const FLOAT_DELTA: f32 = 0.0001;
fn cmp_floats<T: Float + Zero>(a: T, b: T) -> bool {
    let delta = T::from(FLOAT_DELTA).unwrap();
    (a - b).abs() < delta
}

fn cmp_floats_delta<T: Float + Zero>(a: T, b: T, delta: T) -> bool {
    (a - b).abs() < delta
}

fn setup_pulse() {
    let vec = Vec::<u8>::from("bozo".to_string());
    let mut hash = 0;
    for i in 0..vec.len() {
        hash += vec[i] as u32;
        if i < vec.len() - 1 {
            hash <<= 8;
        }
    }
    let mut new_weapon = Weapon::generate_weapon(
        hash,       //bozo as u32 :)
        13,         //pulse
        69420,      //test pulse
        1,          //primary
        3373582085, //kinetic
    )
    .unwrap();
    let mut stats = HashMap::new();
    stats.insert(StatHashes::RELOAD.into(), Stat::from(50));
    stats.insert(StatHashes::HANDLING.into(), Stat::from(50));
    stats.insert(StatHashes::RANGE.into(), Stat::from(50));
    stats.insert(StatHashes::ZOOM.into(), Stat::from(15));
    new_weapon.set_stats(stats);
    PERS_DATA.with(|perm_data| {
        perm_data.borrow_mut().weapon = new_weapon;
    });
}

#[test]
fn test_pulse_setup() {
    setup_pulse();
    PERS_DATA.with(|perm_data| {
        let mut weapon = perm_data.borrow().weapon.clone();
        assert_eq!(weapon.damage_type, DamageType::KINETIC);
        assert_eq!(weapon.ammo_type, AmmoType::PRIMARY);
        assert_eq!(weapon.intrinsic_hash, 69420);
        assert_eq!(weapon.weapon_type, WeaponType::PULSERIFLE);
        let test_stat = weapon
            .get_stats()
            .get(&(StatHashes::HANDLING.into()))
            .unwrap()
            .val();
        assert_eq!(test_stat, 50, "test_stat: {}", test_stat);
    });
}

#[test]
fn test_pulse_reload() {
    setup_pulse();
    PERS_DATA.with(|perm_data| {
        let weapon = perm_data.borrow_mut().weapon.clone();
        let response = weapon.calc_reload_time(None, None, true);
        assert!(
            cmp_floats(response.reload_time, 5.0),
            "reload time: {}",
            response.reload_time
        );
    });
}

#[test]
fn test_pulse_handling() {
    setup_pulse();
    PERS_DATA.with(|perm_data| {
        let weapon = perm_data.borrow_mut().weapon.clone();
        let response = weapon.calc_handling_times(None, None, true);
        assert!(
            cmp_floats(response.ads_time, 5.0),
            "ads time: {}",
            response.ads_time
        );
        assert!(
            cmp_floats(response.ready_time, 5.0),
            "ready time: {}",
            response.ready_time
        );
        assert!(
            cmp_floats(response.stow_time, 5.0),
            "stow time: {}",
            response.stow_time
        );
    });
}

#[test]
fn test_pulse_range() {
    setup_pulse();
    PERS_DATA.with(|perm_data| {
        let weapon = perm_data.borrow_mut().weapon.clone();
        let response = weapon.calc_range_falloff(None, None, true);
        assert!(
            cmp_floats(response.hip_falloff_start, 15.0),
            "hip falloff start: {}",
            response.hip_falloff_start
        );
        assert!(
            cmp_floats(response.ads_falloff_start, 15.0 * (1.5 - 0.025)),
            "ads falloff start: {}",
            response.ads_falloff_start
        );
        assert!(
            cmp_floats(response.hip_falloff_end, 30.0),
            "hip falloff end: {}",
            response.hip_falloff_end
        );
        assert!(
            cmp_floats(response.ads_falloff_end, 30.0 * (1.5 - 0.025)),
            "ads falloff end: {}",
            response.ads_falloff_end
        );
    });
}

#[test]
fn test_pulse_firing_data() {
    setup_pulse();
    PERS_DATA.with(|perm_data| {
        let weapon = perm_data.borrow_mut().weapon.clone();
        let mut response = weapon.calc_firing_data(None, None, true);
        PERS_DATA.with(|perm_data| {
            response.apply_pve_bonuses(
                perm_data.borrow().activity.get_rpl_mult(),
                perm_data.borrow().activity.get_pl_delta(),
                perm_data.borrow().weapon.damage_mods.pve,
                perm_data
                    .borrow()
                    .weapon
                    .damage_mods
                    .get_mod(&perm_data.borrow().enemy.type_),
            )
        });
        assert!(
            cmp_floats(response.pvp_impact_damage, 10.0),
            "impact damage: {}",
            response.pvp_impact_damage
        );
        assert!(
            cmp_floats(response.pvp_explosion_damage, 0.0),
            "explosive damage: {}",
            response.pvp_explosion_damage
        );
        assert!(cmp_floats(response.rpm, 900.0), "rpm: {}", response.rpm);
        assert!(
            cmp_floats(response.pvp_crit_mult, 2.0),
            "crit mult: {}",
            response.pvp_crit_mult
        );
    });
}

fn setup_bow() {
    let vec = Vec::<u8>::from("harm".to_string());
    let mut hash = 0;
    for i in 0..vec.len() {
        hash += vec[i] as u32;
        if i < vec.len() - 1 {
            hash <<= 8;
        }
    }
    let mut new_weapon = Weapon::generate_weapon(
        hash,       //harm turned himslf into a u32! Funniest shit I've ever seen
        31,         //bow
        696969,     //test bow
        2,          //special
        3949783978, //strand
    )
    .unwrap();
    let mut stats = HashMap::new();
    stats.insert(StatHashes::RELOAD.into(), Stat::from(50));
    stats.insert(StatHashes::HANDLING.into(), Stat::from(50));
    stats.insert(StatHashes::RANGE.into(), Stat::from(50));
    stats.insert(StatHashes::ZOOM.into(), Stat::from(15));
    new_weapon.set_stats(stats);
    PERS_DATA.with(|perm_data| {
        perm_data.borrow_mut().weapon = new_weapon;
    });
}

#[test]
fn test_bow_setup() {
    setup_bow();
    PERS_DATA.with(|perm_data| {
        let mut weapon = perm_data.borrow().weapon.clone();
        assert_eq!(weapon.damage_type, DamageType::STRAND);
        assert_eq!(weapon.ammo_type, AmmoType::SPECIAL);
        assert_eq!(weapon.intrinsic_hash, 696969);
        assert_eq!(weapon.weapon_type, WeaponType::BOW);
        let test_stat = weapon
            .get_stats()
            .get(&(StatHashes::HANDLING.into()))
            .unwrap()
            .val();
        assert_eq!(test_stat, 50, "test_stat: {}", test_stat);
    });
}

#[test]
fn test_bow_reload() {
    setup_bow();
    PERS_DATA.with(|perm_data| {
        let weapon = perm_data.borrow_mut().weapon.clone();
        let response = weapon.calc_reload_time(None, None, true);
        assert!(
            cmp_floats(response.reload_time, 5.0),
            "reload time: {}",
            response.reload_time
        );
    });
}

#[test]
fn test_bow_handling() {
    setup_bow();
    PERS_DATA.with(|perm_data| {
        let weapon = perm_data.borrow_mut().weapon.clone();
        let response = weapon.calc_handling_times(None, None, true);
        assert!(
            cmp_floats(response.ads_time, 5.0),
            "ads time: {}",
            response.ads_time
        );
        assert!(
            cmp_floats(response.ready_time, 5.0),
            "ready time: {}",
            response.ready_time
        );
        assert!(
            cmp_floats(response.stow_time, 5.0),
            "stow time: {}",
            response.stow_time
        );
    });
}

#[test]
fn test_bow_range() {
    setup_bow();
    PERS_DATA.with(|perm_data| {
        let weapon = perm_data.borrow_mut().weapon.clone();
        let response = weapon.calc_range_falloff(None, None, true);
        assert!(
            response.ads_falloff_start > 998.0,
            "ads falloff start: {}",
            response.ads_falloff_start
        );
        assert!(
            response.hip_falloff_end > 998.0,
            "hip falloff end: {}",
            response.hip_falloff_end
        );
    });
}

#[test]
fn test_bow_firing_data() {
    setup_bow();
    PERS_DATA.with(|perm_data| {
        let weapon = perm_data.borrow_mut().weapon.clone();
        let mut response = weapon.calc_firing_data(None, None, true);
        PERS_DATA.with(|perm_data| {
            response.apply_pve_bonuses(
                perm_data.borrow().activity.get_rpl_mult(),
                perm_data.borrow().activity.get_pl_delta(),
                perm_data.borrow().weapon.damage_mods.pve,
                perm_data
                    .borrow()
                    .weapon
                    .damage_mods
                    .get_mod(&perm_data.borrow().enemy.type_),
            )
        });
        assert!(
            cmp_floats(response.pvp_impact_damage, 100.0),
            "impact damage: {}",
            response.pvp_impact_damage
        );
        assert!(
            cmp_floats(response.pvp_explosion_damage, 0.0),
            "explosive damage: {}",
            response.pvp_explosion_damage
        );
        assert!(
            cmp_floats(response.burst_delay, 20.0 / 30.0),
            "draw time: {}",
            response.burst_delay
        );
        assert!(
            cmp_floats(response.pvp_crit_mult, 1.5 + (2.0 / 51.0)),
            "crit mult: {}",
            response.pvp_crit_mult
        );
    });
}

struct TestyTest<'a> {
    pub attr_val: Attribute<'a>,
    pub prim_val: f64,
}
impl TestyTest<'_> {
    pub fn get_val(&self) -> f64 {
        self.prim_val
    }
    pub fn new(val: f64) -> Self {
        Self {
            attr_val: Attribute::Ref(RefCell::new(val)),
            prim_val: val,
        }
    }
    pub fn attr(&self) -> &Attribute {
        &self.attr_val
    }
    pub fn attr_prim(&self) -> Attribute {
        Attribute::Lambda(Box::new(|| self.get_val()))
    }
    pub fn set_prim_val(&mut self, val: f64) {
        self.prim_val = val;
    }
    pub fn set_attr_val(&self, val: f64) {
        self.attr_val.inner().unwrap().replace(val);
    }
}

#[test]
fn attr_test() {
    let tst = TestyTest::new(5.0);
    let attr = tst.attr_prim();
    let prim = Attribute::PrimF(5.0);
    let sum = attr.add(&prim);
    let sum2 = sum.add(&Attribute::PrimI(10));
    assert_eq!(sum2.val(), 20.0);
    // assert_eq!(sum2.val(), 25.0);
}
