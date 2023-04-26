
use num_enum::IntoPrimitive;
use serde::{Serialize, Deserialize};



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoPrimitive)]
#[repr(u8)]
pub enum EventType {
    Wait = 0,
    //firing
    FireKinetic = 1,
    FireEnergy = 2,
    FirePower = 3,
    ExtraPerkDamage = 4,
    DamageOverTime = 5,
    //swapping
    SwapToKinetic = 11,
    SwapToEnergy = 12,
    SwapToPower = 13,
    //abilities
    Grenade = 21,
    Super = 22,
    Melee = 23,
    UnchargedMelee = 24,
    ClassAbility = 25,
    //reload
    Reload = 31,
    ReloadCancel = 32,
    ReloadOverride = 33,
    //verbs
    Shatter = 41,
    Ignition = 42,
    Volatile = 43,
    Unravel = 44,
    Jolt = 45,
}
impl Serialize for EventType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer
    {
        serializer.serialize_u8(self.clone().into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoPrimitive)]
#[repr(u8)]
pub enum DamageEventType {
    None,
    WeaponImpact,
    WeaponExplosion,
    WeaponPerk,
    Grenade,
    RangedMelee,
    CloseMelee,
    UnchargedMelee,
    Super,
    ClassAbility
}