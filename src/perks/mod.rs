
#![allow(clippy::all)]

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

use crate::d2_enums::StatHashes;

use self::{
    exotic_perks::*,
    lib::{
        CalculationInput, DamageModifierResponse, ExtraDamageResponse, FiringModifierResponse,
        HandlingModifierResponse, InventoryModifierResponse, MagazineModifierResponse,
        RangeModifierResponse, RefundResponse, ReloadModifierResponse, ReloadOverrideResponse,
    },
    other_perks::*,
    year_1_perks::*,
    year_2_perks::*,
    year_3_perks::*,
    year_4_perks::*,
    year_5_perks::*,
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

#[derive(Clone, Debug, Default)]
pub struct Perk {
    pub stat_buffs: HashMap<u32, i32>,
    pub enhanced: bool,
    pub value: u32, //used for toggle and stacks
    pub hash: u32,
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
    CascadePoint,
    Outlaw,
    BackupPlan,   // will apply in dps no matter what
    BoxBreathing, // will apply in dps no matter what
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
    TakenSpec,
    OverFlow, // new
    Reconstruction,
    //class
    Amplified,
    Tempering,
    HeatRises,
    Hedrons,
    Frequency,
    FlowState,

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
    KillingTally,
    AmbitiousAssassin,
    //class
    OnYourMark,
    //weird
    Demolitionist,
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
    RewindRounds,
    ExplosivePayload,
    TimedPayload,
    ExplosiveHead,
    SpikeGrenades,
    AlloyMag,
    RapidFireFrame,
    SwapMag,
    QuickAccessSling,
    BossSpec,
    MinorSpec,
    MajorSpec,
    BigOnesSpec,
    QuickDraw,
    ImpactCasing,
    FullChoke,
    AlloyMagazine,
    ResevoirBurst,
    OverUnder,
    ArchersTempo,
    Snapshot,
    Slickdraw,
    ClownCartridge, // new
    RangeFinder,

    //armor
    QuickCharge,

    ////MISC////
    Ignore,
    MasterWork,
    BuiltIn,
    EmpowermentBuffs,
    WeakenDebuffs,
    ////////EXOTIC////////
    ////TOGGLE////
    CranialSpikeCat,
    AgersCall,
    LagragianSight,
    OphidianAspect,
    DragonShadow,
    LunaFaction,

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
            0 => Perks::BuiltIn,
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
            3751912585 => Perks::CascadePoint,
            1168162263 => Perks::Outlaw,
            1600092898 => Perks::BackupPlan,
            2551157718 => Perks::BoxBreathing,
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
            970163821 => Perks::AgersCall,
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
            3066103999 => Perks::OnYourMark,
            1727069361 => Perks::Frequency,
            362132290 => Perks::Tempering,
            593361144 => Perks::DragonShadow,
            1147638875 => Perks::OphidianAspect,
            1431678320 => Perks::AlloyMag,
            1047830412 => Perks::FullChoke,
            3721627275 => Perks::SwapMag,
            1334978104 => Perks::QuickAccessSling,
            2788909693 => Perks::BossSpec,
            984527513 => Perks::MajorSpec,
            4091000557 => Perks::MinorSpec,
            3018373291 => Perks::BigOnesSpec,
            3400784728 => Perks::TripleTap,
            1354429876 => Perks::FourthTimesTheCharm,
            1866048759 => Perks::HipFireGrip,
            706527188 => Perks::QuickDraw,
            1821614984 => Perks::Slickdraw,
            957782887 => Perks::Snapshot,
            3301904089 => Perks::SpikeGrenades,
            3796465595 => Perks::ImpactCasing,
            3038247973 => Perks::ExplosivePayload,
            3365897133 => Perks::ExplosiveHead,
            1954620775 => Perks::TimedPayload,
            47981717 => Perks::OpeningShot,
            280464955 => Perks::FirmlyPlanted,
            3161816588 => Perks::SlideShot,
            2039302152 => Perks::SlideWays,
            1890422124 => Perks::TapTheTrigger,
            2416023159 => Perks::OffhandStrike,
            2272927194 => Perks::PerfectFloat,
            1513326571 => Perks::TakenSpec,
            3469412970 => Perks::Hedrons,
            83039194 => Perks::HeatRises,
            4194622036 => Perks::FlowState,
            4071163871 => Perks::ThreatDetector,
            222 => Perks::EmpowermentBuffs,
            333 => Perks::WeakenDebuffs, //also stuff like tractor and div, any non stacking ones
            _ => Perks::Ignore,
        }
    }
}

pub fn get_perk_stats(
    _perks: Vec<Perk>,
    _input_data: CalculationInput,
    _pvp: bool,
) -> [HashMap<u32, i32>; 2] {
    let mut dynamic_stats: HashMap<u32, i32> = HashMap::new();
    let mut static_stats: HashMap<u32, i32> = HashMap::new();
    for perk in _perks {
        let perk_stats = dyanmic_perk_stats(&perk, &_input_data, _pvp);
        for (key, value) in perk_stats {
            let entry = dynamic_stats.entry(key).or_insert(0);
            *entry += value;
        }
        for (key, value) in perk.stat_buffs {
            let entry = static_stats.entry(key.clone()).or_insert(0);
            *entry += value;
        }
    }
    [dynamic_stats, static_stats]
}
fn dyanmic_perk_stats(
    _perk: &Perk,
    _input_data: &CalculationInput,
    _pvp: bool,
) -> HashMap<u32, i32> {
    let perk_enum = Perks::from_u32(_perk.hash);
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::Roadborn => sbr_roadborn(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::OphidianAspect => {
            sbr_ophidian_aspects(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        Perks::DragonShadow => sbr_dragon_shadow(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::Amplified => sbr_amplified(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::Tempering => sbr_tempering(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::OnYourMark => sbr_on_your_mark(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::HeatRises => sbr_heat_rises(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::Hedrons => sbr_hedrons(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::ThreatDetector => {
            sbr_threat_detector(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        Perks::FieldPrep => sbr_field_prep(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::FirmlyPlanted => {
            sbr_firmly_planted(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        Perks::HipFireGrip => sbr_hip_fire_grip(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::MovingTarget => sbr_moving_target(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::OpeningShot => sbr_opening_shot(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::Outlaw => sbr_outlaw(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::SlideShot => sbr_slide_shot(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::SlideWays => sbr_slide_ways(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::TapTheTrigger => {
            sbr_tap_the_trigger(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        Perks::AirAssault => sbr_air_assault(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::FeedingFrenzy => {
            sbr_feeding_frenzy(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        Perks::ElementalCapacitor => {
            sbr_elemental_capacitor(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        Perks::KillingWind => sbr_killing_wind(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::AdrenalineJunkie => {
            sbr_adrenaline_junkie(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        Perks::Ensemble => sbr_ensemble(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::Frenzy => sbr_frenzy(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::PerpetualMotion => {
            sbr_perpetual_motion(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        Perks::PerfectFloat => sbr_perfect_float(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::Pugilist => sbr_pugilist(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::Encore => sbr_encore(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::FragileFocus => sbr_fragile_focus(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::OffhandStrike => {
            sbr_offhand_strike(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        Perks::StatsForAll => sbr_stats_for_all(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::SteadyHands => sbr_steady_hands(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::WellRounded => sbr_well_rounded(_input_data, val, enhanced, _pvp, &HashMap::new()),
        _ => HashMap::new(),
    }
}

pub fn get_dmg_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
) -> DamageModifierResponse {
    let mut dmg_modifier = DamageModifierResponse::default();
    for perk in _perks {
        let tmp = get_perk_dmr(perk.clone(), _input_data, _pvp);
        // println!{"{:?}: {:?}", Perks::from_u32(perk.hash), tmp}
        dmg_modifier.dmg_scale *= tmp.dmg_scale;
        dmg_modifier.crit_scale *= tmp.crit_scale;
    }
    dmg_modifier
}
fn get_perk_dmr(_perk: Perk, _input_data: &CalculationInput, _pvp: bool) -> DamageModifierResponse {
    let perk_enum = Perks::from_u32(_perk.hash);
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::HighImpactReserves => {
            dmr_high_impact_reserves(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        Perks::BoxBreathing => dmr_box_breathing(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::ExplosivePayload => {
            dmr_explosive_payload(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        Perks::TimedPayload => dmr_timed_payload(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::ImpactCasing => dmr_impact_casing(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::ExplosiveHead => {
            dmr_explosive_head(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        Perks::FiringLine => dmr_firing_line(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::KillingTally => dmr_killing_tally(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::ResevoirBurst => {
            dmr_resevoir_burst(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        Perks::Surrounded => dmr_surrounded(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::LastingImpression => {
            dmr_lasting_impressions(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        Perks::Vorpal => dmr_vorpal(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::Adagio => dmr_adagio(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::AdrenalineJunkie => {
            dmr_adrenaline_junkie(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        Perks::Frenzy => dmr_frenzy(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::FocusedFury => dmr_focused_fury(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::GutShot => dmr_gutshot_straight(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::TargetLock => dmr_target_lock(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::OverUnder => dmr_over_under(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::WormsHunger => dmr_worms_hunger(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::LagragianSight => {
            dmr_lagragian_sight(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        Perks::EmpowermentBuffs => {
            dmr_empowerment_buffs(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        Perks::WeakenDebuffs => {dmr_weaken_debuffs(_input_data, val, enhanced, _pvp, &HashMap::new())}
        Perks::BuiltIn => dmr_built_in(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::BossSpec => dmr_boss_spec(_input_data, val, enhanced, _pvp, &HashMap::new()),
        _ => DamageModifierResponse::default(),
    }
}

pub fn get_reload_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
) -> ReloadModifierResponse {
    let mut reload_modifier = ReloadModifierResponse::default();
    for perk in _perks {
        let tmp = get_perk_rsmr(perk, _input_data, _pvp);
        reload_modifier.reload_stat_add += tmp.reload_stat_add;
        reload_modifier.reload_time_scale *= tmp.reload_time_scale;
    }
    reload_modifier
}
fn get_perk_rsmr(
    _perk: Perk,
    _input_data: &CalculationInput,
    _pvp: bool,
) -> ReloadModifierResponse {
    let perk_enum = Perks::from_u32(_perk.hash);
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::RapidFireFrame => rsmr_alloy_mag(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::AlloyMagazine => rsmr_alloy_mag(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::Roadborn => rsmr_roadborn(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::OphidianAspect => {
            rsmr_ophidian_aspects(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        Perks::DragonShadow => {
            rsmr_dragon_shadow(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        Perks::Frequency => rsmr_frequency(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::FlowState => rsmr_flow_state(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::OnYourMark => rsmr_on_your_mark(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::ThreatDetector => {
            rsmr_threat_detector(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        Perks::FieldPrep => rsmr_field_prep(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::FeedingFrenzy => {
            rsmr_feeding_frenzy(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        Perks::RapidHit => rsmr_rapid_hit(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::ElementalCapacitor => {
            rsmr_elemental_capacitor(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        Perks::Ensemble => rsmr_ensemble(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::Frenzy => rsmr_frenzy(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::ImpulseAmplifier => {
            rsmr_impulse_amplifier(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        Perks::PerpetualMotion => {
            rsmr_perpetual_motion(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        Perks::StatsForAll => rsmr_stats_for_all(_input_data, val, enhanced, _pvp, &HashMap::new()),
        _ => ReloadModifierResponse::default(),
    }
}

pub fn get_firing_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
) -> FiringModifierResponse {
    let mut firing_modifier = FiringModifierResponse::default();
    for perk in _perks {
        let tmp = get_perk_fmr(perk, _input_data, _pvp);
        firing_modifier.burst_delay_scale *= tmp.burst_delay_scale;
        firing_modifier.burst_duration_scale *= tmp.burst_duration_scale;
        firing_modifier.burst_size_add += tmp.burst_size_add;
    }
    firing_modifier
}
fn get_perk_fmr(_perk: Perk, _input_data: &CalculationInput, _pvp: bool) -> FiringModifierResponse {
    let perk_enum = Perks::from_u32(_perk.hash);
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::Roadborn => fmr_roadborn(_input_data, val, enhanced, _pvp, &HashMap::new()),
        // Perks::RatPack => fmr_rat_pack(_input_data, val, enhanced, _pvp),
        // Perks::RideTheBull
        Perks::Desperado => fmr_desperado(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::ArchersTempo => fmr_archers_tempo(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::Adagio => fmr_adagio(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::Cornered => fmr_cornered(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::CascadePoint => fmr_cascade_point(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::ReignHavoc => fmr_reign_havoc(_input_data, val, enhanced, _pvp, &HashMap::new()),
        _ => FiringModifierResponse::default(),
    }
}

pub fn get_handling_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
) -> HandlingModifierResponse {
    let mut handling_modifier = HandlingModifierResponse::default();
    for perk in _perks {
        let tmp = get_perk_hmr(perk, _input_data, _pvp);
        handling_modifier.handling_stat_add += tmp.handling_stat_add;
        handling_modifier.handling_swap_scale *= tmp.handling_swap_scale;
        handling_modifier.handling_ads_scale *= tmp.handling_ads_scale;
    }
    handling_modifier
}
fn get_perk_hmr(
    _perk: Perk,
    _input_data: &CalculationInput,
    _pvp: bool,
) -> HandlingModifierResponse {
    let perk_enum = Perks::from_u32(_perk.hash);
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::SwapMag => hmr_swap_mag(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::OphidianAspect => {
            hmr_ophidian_aspects(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        Perks::DragonShadow => hmr_dragon_shadow(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::Amplified => hmr_amplified(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::OnYourMark => hmr_on_your_mark(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::ThreatDetector => {
            hmr_threat_detector(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        Perks::FirmlyPlanted => {
            hmr_firmly_planted(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        Perks::Snapshot => hmr_snapshot(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::ElementalCapacitor => {
            hmr_elemental_capacitor(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        Perks::AdrenalineJunkie => {
            hmr_adrenaline_junkie(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        Perks::Ensemble => hmr_ensemble(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::Frenzy => hmr_frenzy(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::PerpetualMotion => {
            hmr_perpetual_motion(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        Perks::Slickdraw => hmr_slickdraw(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::StatsForAll => hmr_stats_for_all(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::SteadyHands => hmr_steady_hands(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::WellRounded => hmr_well_rounded(_input_data, val, enhanced, _pvp, &HashMap::new()),
        _ => HandlingModifierResponse::default(),
    }
}

pub fn get_magazine_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
) -> MagazineModifierResponse {
    let mut magazine_modifier = MagazineModifierResponse::default();
    for perk in _perks {
        let tmp = get_perk_mmr(perk, _input_data, _pvp);
        magazine_modifier.magazine_stat_add += tmp.magazine_stat_add;
        magazine_modifier.magazine_add += tmp.magazine_add;
        magazine_modifier.magazine_scale *= tmp.magazine_scale;
    }
    magazine_modifier
}
fn get_perk_mmr(
    _perk: Perk,
    _input_data: &CalculationInput,
    _pvp: bool,
) -> MagazineModifierResponse {
    let perk_enum = Perks::from_u32(_perk.hash);
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::AgersCall => mmr_agers_call(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::AmbitiousAssassin => {
            mmr_abitious_assassin(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        Perks::OverFlow => mmr_overflow(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::ClownCartridge => {
            mmr_clown_cartridge(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        Perks::Reconstruction => {
            mmr_reconstruction(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        _ => MagazineModifierResponse::default(),
    }
}

pub fn get_reserve_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
) -> InventoryModifierResponse {
    let mut reserve_modifier = InventoryModifierResponse::default();
    for perk in _perks {
        let tmp = get_perk_imr(perk, _input_data, _pvp);
        reserve_modifier.inv_stat_add += tmp.inv_stat_add;
        reserve_modifier.inv_add += tmp.inv_add;
        reserve_modifier.inv_scale *= tmp.inv_scale;
    }
    reserve_modifier
}
fn get_perk_imr(
    _perk: Perk,
    _input_data: &CalculationInput,
    _pvp: bool,
) -> InventoryModifierResponse {
    let perk_enum = Perks::from_u32(_perk.hash);
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::FieldPrep => imr_field_prep(_input_data, val, enhanced, _pvp, &HashMap::new()),
        _ => InventoryModifierResponse::default(),
    }
}

pub fn get_range_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
) -> RangeModifierResponse {
    let mut range_modifier = RangeModifierResponse::default();
    for perk in _perks {
        let tmp = get_perk_rmr(perk, _input_data, _pvp);
        range_modifier.range_stat_add += tmp.range_stat_add;
        range_modifier.range_all_scale *= tmp.range_all_scale;
        range_modifier.range_hip_scale *= tmp.range_hip_scale;
        range_modifier.range_zoom_scale *= tmp.range_zoom_scale;
    }
    range_modifier
}
fn get_perk_rmr(_perk: Perk, _input_data: &CalculationInput, _pvp: bool) -> RangeModifierResponse {
    let perk_enum = Perks::from_u32(_perk.hash);
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::HipFireGrip => rmr_hip_fire_grip(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::OpeningShot => rmr_opening_shot(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::RangeFinder => rmr_range_finder(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::SlideShot => rmr_slide_shot(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::KillingWind => rmr_killing_wind(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::FragileFocus => rmr_fragile_focus(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::OffhandStrike => {rmr_offhand_strike(_input_data, val, enhanced, _pvp, &HashMap::new())}
        Perks::StatsForAll => rmr_stats_for_all(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::WellRounded => rmr_well_rounded(_input_data, val, enhanced, _pvp, &HashMap::new()),
        _ => RangeModifierResponse::default(),
    }
}

pub fn get_refund_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
) -> Vec<RefundResponse> {
    let mut refund_modifier = vec![];
    for perk in _perks {
        let tmp = get_perk_refund(perk, _input_data, _pvp);
        if tmp.requirement > 0 {
            refund_modifier.push(tmp);
        }
    }
    refund_modifier
}
fn get_perk_refund(_perk: Perk, _input_data: &CalculationInput, _pvp: bool) -> RefundResponse {
    let perk_enum = Perks::from_u32(_perk.hash);
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::TripleTap => rr_triple_tap(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::FourthTimesTheCharm => {
            rr_fourth_times(_input_data, val, enhanced, _pvp, &HashMap::new())
        }
        _ => RefundResponse::default(),
    }
}

pub fn get_extra_damage(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> Vec<ExtraDamageResponse> {
    let mut extra_damage = vec![];
    for perk in _perks {
        let tmp = get_perk_edr(perk, _input_data, _pvp, _cached_data);
        if tmp.additive_damage > 0.0 {
            extra_damage.push(tmp);
        }
    }
    extra_damage
}
fn get_perk_edr(
    _perk: Perk,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> ExtraDamageResponse {
    let perk_enum = Perks::from_u32(_perk.hash);
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::ReignHavoc => edr_reign_havoc(_input_data, val, enhanced, _pvp, &_cached_data),
        _ => ExtraDamageResponse::default(),
    }
}

pub fn get_reload_overrides(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &HashMap<String, f64>,
) -> Vec<ReloadOverrideResponse> {
    let mut reload_overrides = vec![];
    for perk in _perks {
        let tmp = get_perk_ror(perk, _input_data, _pvp);
        if tmp.valid{
            reload_overrides.push(tmp);
        }
    }
    reload_overrides
}
fn get_perk_ror(
    _perk: Perk,
    _input_data: &CalculationInput,
    _pvp: bool,
) -> ReloadOverrideResponse {
    let perk_enum = Perks::from_u32(_perk.hash);
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::Demolitionist => ror_demolitionist(_input_data, val, enhanced, _pvp, &HashMap::new()),
        Perks::VeistStinger => ror_veist_stinger(_input_data, val, enhanced, _pvp, &HashMap::new()),
        _ => ReloadOverrideResponse::invalid(),
    }
}
