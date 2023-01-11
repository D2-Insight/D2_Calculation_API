pub mod exotic_perks;
pub mod lib;
pub mod other_perks;
pub mod year_1_perks;
pub mod year_2_perks;
pub mod year_3_perks;
pub mod year_4_perks;
pub mod year_5_perks;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{perks::exotic_perks::sbr_roadborn, D2Enums::StatHashes, js_types::JsPerk};

use self::{
    lib::CalculationInput, other_perks::*, year_1_perks::*, year_2_perks::*, year_3_perks::*,
    year_4_perks::*, year_5_perks::*,
};

pub fn clamp<T: PartialOrd>(n: T, min: T, max: T) -> T {
    if n < min {
        min
    } else if n > max {
        max
    } else {
        n
    }
}

#[derive(Clone, Debug)]
pub struct Perk {
    pub stat_buffs: HashMap<u32, i32>,
    pub enhanced: bool,
    pub value: i32, //used for toggle and stacks
    pub name: String,
    pub hash: u32,
    pub perk_enum: Perks,
}
impl Perk {
    pub fn from_js(js_perk: JsPerk) -> Perk {
        let mut stat_buffs = HashMap::new();
        for (key, value) in js_perk.stat_buffs {
            let hash_enum = StatHashes::from_u32(key);
            if hash_enum.is_weapon_stat() {
                stat_buffs.insert(key, value);
            }
        }
        Perk {
            stat_buffs,
            enhanced: js_perk.enhanced,
            value: js_perk.value,
            name: js_perk.name,
            hash: js_perk.id,
            perk_enum: Perks::from_u32(js_perk.id),
        }
    }
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
    FirmlyPlanted,
    SlideShot,
    SlideWays,
    TapTheTrigger,
    PerfectFloat,
    OffhandStrike,
    //class
    Amplified,
    Tempering,
    HeatRises,
    Hedrons,

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
    ThreatDetector,
    AirAssault,
    //class
    OnYourMark,
    //weird
    ElementalCapacitor,

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
    HipFireGrip,

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
    OphidianAspect,
    DragonShadow,

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
    Roadborn,
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
}

//return HashMap<u32, i32> tuple
pub fn get_perk_stats(_perks: Vec<Perk>, _input_data: CalculationInput, _pvp: bool) -> [HashMap<u32, i32>; 2]{
    let mut dynamic_stats: HashMap<u32, i32> = HashMap::new();
    let mut static_stats: HashMap<u32, i32> = HashMap::new();
    for perk in _perks {
        let perk_stats = dyanmic_perk_stats(perk, _input_data, _pvp);
        for (key, value) in perk_stats {
            let entry = dynamic_stats.entry(key).or_insert(0);
            *entry += value;
        }
        for (key, value) in perk.stat_buffs {
            let entry = static_stats.entry(key).or_insert(0);
            *entry += value;
        }
    }
    [dynamic_stats, static_stats]
}

pub fn dyanmic_perk_stats(
    _perk: Perk,
    _input_data: CalculationInput,
    _pvp: bool,
) -> HashMap<u32, i32> {
    let perk_enum = Perks::from_u32(_perk.hash);
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::Roadborn => sbr_roadborn(_input_data, val, enhanced, _pvp),
        Perks::OphidianAspect => sbr_ophidian_aspects(_input_data, val, enhanced, _pvp),
        Perks::DragonShadow => sbr_dragon_shadow(_input_data, val, enhanced, _pvp),
        Perks::Amplified => sbr_amplified(_input_data, val, enhanced, _pvp),
        Perks::Tempering => sbr_tempering(_input_data, val, enhanced, _pvp),
        Perks::OnYourMark => sbr_on_your_mark(_input_data, val, enhanced, _pvp),
        Perks::HeatRises => sbr_heat_rises(_input_data, val, enhanced, _pvp),
        Perks::Hedrons => sbr_hedrons(_input_data, val, enhanced, _pvp),
        Perks::ThreatDetector => sbr_threat_detector(_input_data, val, enhanced, _pvp),
        Perks::FieldPrep => sbr_field_prep(_input_data, val, enhanced, _pvp),
        Perks::FirmlyPlanted => sbr_firmly_planted(_input_data, val, enhanced, _pvp),
        Perks::HipFireGrip => sbr_hip_fire_grip(_input_data, val, enhanced, _pvp),
        Perks::MovingTarget => sbr_moving_target(_input_data, val, enhanced, _pvp),
        Perks::OpeningShot => sbr_opening_shot(_input_data, val, enhanced, _pvp),
        Perks::Outlaw => sbr_outlaw(_input_data, val, enhanced, _pvp),
        Perks::SlideShot => sbr_slide_shot(_input_data, val, enhanced, _pvp),
        Perks::SlideWays => sbr_slide_ways(_input_data, val, enhanced, _pvp),
        Perks::TapTheTrigger => sbr_tap_the_trigger(_input_data, val, enhanced, _pvp),
        Perks::AirAssault => sbr_air_assault(_input_data, val, enhanced, _pvp),
        Perks::FeedingFrenzy => sbr_feeding_frenzy(_input_data, val, enhanced, _pvp),
        Perks::ElementalCapacitor => sbr_elemental_capacitor(_input_data, val, enhanced, _pvp),
        Perks::KillingWind => sbr_killing_wind(_input_data, val, enhanced, _pvp),
        Perks::AdrenalineJunkie => sbr_adrenaline_junkie(_input_data, val, enhanced, _pvp),
        Perks::Ensemble => sbr_ensemble(_input_data, val, enhanced, _pvp),
        Perks::Frenzy => sbr_frenzy(_input_data, val, enhanced, _pvp),
        Perks::PerpetualMotion => sbr_perpetual_motion(_input_data, val, enhanced, _pvp),
        Perks::PerfectFloat => sbr_perfect_float(_input_data, val, enhanced, _pvp),
        Perks::Pugilist => sbr_pugilist(_input_data, val, enhanced, _pvp),
        Perks::Encore => sbr_encore(_input_data, val, enhanced, _pvp),
        Perks::FragileFocus => sbr_fragile_focus(_input_data, val, enhanced, _pvp),
        Perks::OffhandStrike => sbr_offhand_strike(_input_data, val, enhanced, _pvp),
        Perks::StatsForAll => sbr_stats_for_all(_input_data, val, enhanced, _pvp),
        Perks::SteadyHands => sbr_steady_hands(_input_data, val, enhanced, _pvp),
        Perks::WellRounded => sbr_well_rounded(_input_data, val, enhanced, _pvp),
        _ => HashMap::new(),
    }
}






