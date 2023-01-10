pub mod year_1_perks;
pub mod year_2_perks;
pub mod year_3_perks;
pub mod year_4_perks;
pub mod year_5_perks;
pub mod exotic_perks;
pub mod lib;
pub mod other_perks;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::D2Enums::StatHashes;





#[derive(Clone, Debug)]
pub struct Perk {
    pub stat_buffs: HashMap<StatHashes, i32>,
    pub enhanced: bool,
    pub value: i32, //used for toggle and stacks
    pub name: String,
    pub id: u32,
    pub perk_enum: Perks,
}
impl Perk {
    pub fn from_js(js_perk: JsPerk) -> Perk {
        let mut stat_buffs = HashMap::new();
        for (key, value) in js_perk.stat_buffs {
            let hash_enum = StatHashes::from_u32(key);
            if hash_enum.is_weapon_stat() {
                stat_buffs.insert(hash_enum, value);
            }
        }
        Perk {
            stat_buffs,
            enhanced: js_perk.enhanced,
            value: js_perk.value,
            name: js_perk.name,
            id: js_perk.id,
            perk_enum: Perks::from_u32(js_perk.id),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JsPerk {
    pub stat_buffs: HashMap<u32, i32>,
    pub enhanced: bool,
    pub value: i32, //used for toggle and stacks
    pub name: String,
    pub id: u32,
}

// all armor pekrs are for the future but wanted to started to compile them now
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Perks {
    ////TOGGLE////
    VeistStinger, //will give it 100% chance
    Surrounded,
    Harmony,
    Frenzy,       //if its disabled will still activate after 12s in dps anyways
    HakkeBreach,  // can't check if this is a viable option so will always allow it
    CloseToMelee, //such a stupid name
    SteadyHands,
    Cornered,
    KillClip,
    Ensemble,
    FiringLine,
    StatsForAll,
    FragileFocus,
    KillingWind,
    Desperado,
    Cascade,
    Outlaw,
    BackupPlan,     // will apply in dps no matter what
    BoxedBreathing, // will apply in dps no matter what
    Pugilist,
    WellRounded,
    ExplosiveLight,
    Adagio,
    Ambush,
    FieldPrep,
    OpeningShot,

    ////SLIDER////
    FeedingFrenzy,
    RunnethOver,
    MultikillClip,
    Encore,
    SuccesfulWarmup,
    Swashbuckler,
    Surplus,
    RapidHit, // dps will still start at 0 :)
    PerpetualMotion,
    AdrenalineJunkie,
    Rampage,

    //armor
    //slides between 1 and 2
    DexterityMod,
    ReloadMod,
    ReserveMod,
    TargetingMod,

    ////STATIC////
    GutShot,
    Vorpal,
    ImpulseAmplifier,
    MovingTarget,
    TargetLock,
    HighImpactReserves,
    FocusedFury,
    ChillClip,
    LastingImpression,
    TripleTap,
    FourthTimesTheCharm,

    //armor
    QuickCharge,

    ////MISC////
    WeaponPart,
    Masterwork,
    Ignore,
    ////////EXOTIC////////
    ////TOGGLE////
    CranialSpikeCat,
    AgersCatalyst,
    LagragianSight,

    ////SLIDER////
    RatPack,
    StringofCurses,
    WormsHunger,

    ////STATIC////
    RocketTracers,
    ParacausalShot,
    CorruptionSpreads,
    TimeSlip,
    ToM,
    IgnitionTrigger,
    GuidanceRing,
    ConserveMomentum,
    Impetus,
    LooksCanKill,
    PerfectFith,
    Broadside,
    Stormbringer,
    PrismaticInferno,
    ReignHavoc,
    WhisperCatalyst,
}

impl Perks {
    pub fn from_u32(_value: u32) -> Perks {
        match _value {
            3988215619 => Perks::VeistStinger,
            3708227201 => Perks::Surrounded,
            438098033 => Perks::Harmony,
            4104185692 => Perks::Frenzy,
            1607056502 => Perks::HakkeBreach,
            1782407750 => Perks::CloseToMelee,
            509074078 => Perks::SteadyHands,
            1799762209 => Perks::Cornered,
            1015611457 => Perks::KillClip,
            2621346526 => Perks::Ensemble,
            1771339417 => Perks::FiringLine,
            1583705720 => Perks::StatsForAll,
            2451262963 => Perks::FragileFocus,
            2450788523 => Perks::KillingWind,
            3047969693 => Perks::Desperado,
            3751912585 => Perks::Cascade,
            1168162263 => Perks::Outlaw,
            1600092898 => Perks::BackupPlan,
            2551157718 => Perks::BoxedBreathing,
            691659142 => Perks::Pugilist,
            744594675 => Perks::WellRounded,
            3194351027 => Perks::ExplosiveLight,
            3673922083 => Perks::Adagio,
            192157151 => Perks::Ambush,
            2869569095 => Perks::FieldPrep,
            2779035018 => Perks::FeedingFrenzy,
            120721526 => Perks::RunnethOver,
            2458213969 => Perks::MultikillClip,
            1195158366 => Perks::Encore,
            2652708987 => Perks::SuccesfulWarmup,
            4082225868 => Perks::Swashbuckler,
            3436462433 => Perks::Surplus,
            247725512 => Perks::RapidHit,
            1428297954 => Perks::PerpetualMotion,
            11612903 => Perks::AdrenalineJunkie,
            3425386926 => Perks::Rampage,
            1111111111 => Perks::DexterityMod,
            2222222222 => Perks::ReloadMod,
            3333333333 => Perks::ReserveMod,
            3333333334 => Perks::TargetingMod,
            1365187766 => Perks::GutShot,
            1546637391 => Perks::Vorpal,
            951095735 => Perks::ImpulseAmplifier,
            588594999 => Perks::MovingTarget,
            365154968 => Perks::TargetLock,
            2213355989 => Perks::HighImpactReserves,
            2896038713 => Perks::FocusedFury,
            2978966579 => Perks::ChillClip,
            3927722942 => Perks::LastingImpression,
            1484685884 => Perks::QuickCharge,
            1301843770 => Perks::CranialSpikeCat,
            970163821 => Perks::AgersCatalyst,
            2881100038 => Perks::LagragianSight,
            2121086290 => Perks::RatPack,
            4004944400 => Perks::StringofCurses,
            2812324400 => Perks::WormsHunger,
            3602718766 => Perks::RocketTracers,
            213689231 => Perks::ParacausalShot,
            4208418110 => Perks::CorruptionSpreads,
            3556949035 => Perks::TimeSlip,
            2724693746 => Perks::ToM,
            961505134 => Perks::IgnitionTrigger,
            2226793914 => Perks::GuidanceRing,
            656200654 => Perks::ConserveMomentum,
            2333607307 => Perks::Impetus,
            3174300811 => Perks::LooksCanKill,
            1000724343 => Perks::PerfectFith,
            407549716 => Perks::Broadside,
            3117514172 => Perks::Stormbringer,
            571267712 => Perks::PrismaticInferno,
            4148158229 => Perks::ReignHavoc,
            1340292993 => Perks::WhisperCatalyst,
            _ => Perks::Ignore,
        }
    }

    fn to_u32(&self) -> u32 {
        match self {
            Perks::VeistStinger => 3988215619,
            Perks::Surrounded => 3708227201,
            Perks::Harmony => 438098033,
            Perks::Frenzy => 4104185692,
            Perks::HakkeBreach => 1607056502,
            Perks::CloseToMelee => 1782407750,
            Perks::SteadyHands => 509074078,
            Perks::Cornered => 1799762209,
            Perks::KillClip => 1015611457,
            Perks::Ensemble => 2621346526,
            Perks::FiringLine => 1771339417,
            Perks::StatsForAll => 1583705720,
            Perks::FragileFocus => 2451262963,
            Perks::KillingWind => 2450788523,
            Perks::Desperado => 3047969693,
            Perks::Cascade => 3751912585,
            Perks::Outlaw => 1168162263,
            Perks::BackupPlan => 1600092898,
            Perks::BoxedBreathing => 2551157718,
            Perks::Pugilist => 691659142,
            Perks::WellRounded => 744594675,
            Perks::ExplosiveLight => 3194351027,
            Perks::Adagio => 3673922083,
            Perks::Ambush => 192157151,
            Perks::FieldPrep => 2869569095,
            Perks::FeedingFrenzy => 2779035018,
            Perks::RunnethOver => 120721526,
            Perks::MultikillClip => 2458213969,
            Perks::Encore => 1195158366,
            Perks::SuccesfulWarmup => 2652708987,
            Perks::Swashbuckler => 4082225868,
            Perks::Surplus => 3436462433,
            Perks::RapidHit => 247725512,
            Perks::PerpetualMotion => 1428297954,
            Perks::AdrenalineJunkie => 11612903,
            Perks::Rampage => 3425386926,
            Perks::DexterityMod => 1111111111,
            Perks::ReloadMod => 2222222222,
            Perks::ReserveMod => 3333333333,
            Perks::TargetingMod => 3333333334,
            Perks::GutShot => 1365187766,
            Perks::Vorpal => 1546637391,
            Perks::ImpulseAmplifier => 951095735,
            Perks::MovingTarget => 588594999,
            Perks::TargetLock => 365154968,
            Perks::HighImpactReserves => 2213355989,
            Perks::FocusedFury => 2896038713,
            Perks::ChillClip => 2978966579,
            Perks::LastingImpression => 3927722942,
            Perks::QuickCharge => 1484685884,
            Perks::Ignore => 0,
            Perks::CranialSpikeCat => 1301843770,
            Perks::AgersCatalyst => 970163821,
            Perks::LagragianSight => 2881100038,
            Perks::RatPack => 2121086290,
            Perks::StringofCurses => 4004944400,
            Perks::WormsHunger => 2812324400,
            Perks::RocketTracers => 3602718766,
            Perks::ParacausalShot => 213689231,
            Perks::CorruptionSpreads => 4208418110,
            Perks::TimeSlip => 3556949035,
            Perks::ToM => 2724693746,
            Perks::IgnitionTrigger => 961505134,
            Perks::GuidanceRing => 2226793914,
            Perks::ConserveMomentum => 656200654,
            Perks::Impetus => 2333607307,
            Perks::LooksCanKill => 3174300811,
            Perks::PerfectFith => 1000724343,
            Perks::Broadside => 407549716,
            Perks::Stormbringer => 3117514172,
            Perks::PrismaticInferno => 571267712,
            Perks::ReignHavoc => 4148158229,
            Perks::WhisperCatalyst => 1340292993,
            _ => 0,
        }
    }
}

pub fn clamp<T: PartialOrd>(n: T, min: T, max: T) -> T {
    if n < min {
        min
    } else if n > max {
        max
    } else {
        n
    }
}