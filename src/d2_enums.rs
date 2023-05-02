#![allow(dead_code)]

use num_enum::{FromPrimitive, IntoPrimitive};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, FromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum AmmoType {
    Primary = 1,
    Special = 2,
    Heavy = 3,
    #[num_enum(default)]
    Unknown = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, FromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum WeaponType {
    AutoRifle = 6,
    Bow = 31,
    FusionRifle = 11,
    Glaive = 33,
    GrenadeLauncher = 23,
    HandCannon = 9,
    LinearFusionRifle = 22,
    MachineGun = 8,
    PulseRifle = 13,
    Rocket = 10,
    ScoutRifle = 14,
    Shotgun = 7,
    Sidearm = 17,
    Sniper = 12,
    SubMachineGun = 24,
    Sword = 18,
    Tracerifle = 25,
    #[num_enum(default)]
    Unknown = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum StatHashes {
    Accuracy = 1591432999,
    AimAssist = 1345609583,
    AirborneEffect = 2714457168,
    AmmoCap = 925767036,
    ATTACK = 1480404414,
    BlastRadius = 3614673599,
    ChargeRate = 3022301683,
    ChargeTime = 2961396640,
    Discipline = 1735777505,
    DrawTime = 447667954,
    GuardEfficency = 2762071195,
    GuardEndurance = 3736848092,
    GuardResistance = 209426660,
    Handling = 943549884,
    Impact = 4043523819,
    Intellect = 144602215,
    InventorySize = 1931675084,
    Magazine = 3871231066,
    Mobility = 2996146975,
    Power = 1935470627,
    Range = 1240592695,
    RecoilDir = 2715839340,
    Recovery = 1943323491,
    Reload = 4188031367,
    Resilience = 392767087,
    RoundsPerMinute = 4284893193,
    ShieldDuration = 1842278586,
    Stability = 155624089,
    Strength = 4244567218,
    SwingSpeed = 2837207746,
    Velocity = 2523465841,
    Zoom = 3555269338,
    #[num_enum(default)]
    Unknown = 0,
}
impl StatHashes {
    pub fn is_weapon_stat(&self) -> bool {
        match self {
            StatHashes::Accuracy => true,
            StatHashes::AimAssist => true,
            StatHashes::AirborneEffect => true,
            StatHashes::AmmoCap => true,
            StatHashes::Zoom => true,
            StatHashes::Range => true,
            StatHashes::Stability => true,
            StatHashes::Reload => true,
            StatHashes::Magazine => true,
            StatHashes::Handling => true,
            StatHashes::Velocity => true,
            StatHashes::BlastRadius => true,
            StatHashes::ChargeTime => true,
            StatHashes::InventorySize => true,
            StatHashes::RecoilDir => true,
            StatHashes::RoundsPerMinute => true,
            StatHashes::GuardEfficency => true,
            StatHashes::GuardEndurance => true,
            StatHashes::GuardResistance => true,
            StatHashes::DrawTime => true,
            StatHashes::SwingSpeed => true,
            StatHashes::ShieldDuration => true,
            StatHashes::Impact => true,
            StatHashes::ChargeRate => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, FromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum DamageType {
    Arc = 2303181850,
    Void = 3454344768,
    Solar = 1847026933,
    Stasis = 151347233,
    Kinetic = 3373582085,
    Strand = 3949783978,
    #[num_enum(default)]
    Unknown = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum DamageSource {
    SNIPER,
    MELEE,
    EXPLOSION,
    ENVIRONMENTAL,
    UNKNOWN,
}

pub type Seconds = f64;
pub type MetersPerSecond = f64;
pub type StatBump = i32;
pub type BungieHash = u32;
