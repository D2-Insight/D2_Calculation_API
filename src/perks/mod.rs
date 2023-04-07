#![allow(clippy::all)]

pub mod buff_perks;
pub mod exotic_armor;
pub mod exotic_perks;
pub mod lib;
pub mod meta_perks;
pub mod origin_perks;
pub mod other_perks;
pub mod perk_options_handler;
pub mod year_1_perks;
pub mod year_2_perks;
pub mod year_3_perks;
pub mod year_4_perks;
pub mod year_5_perks;
pub mod year_6_perks;

use std::collections::HashMap;

use num_enum::FromPrimitive;
use serde::{Deserialize, Serialize};

use crate::d2_enums::{BungieHash, StatBump, StatHashes, WeaponType};
use crate::database;

use self::{
    buff_perks::*,
    exotic_armor::*,
    exotic_perks::*,
    lib::{
        CalculationInput, DamageModifierResponse, ExplosivePercentResponse, ExtraDamageResponse,
        FiringModifierResponse, FlinchModifierResponse, HandlingModifierResponse,
        InventoryModifierResponse, MagazineModifierResponse, ModifierResponseSummary,
        RangeModifierResponse, RefundResponse, ReloadModifierResponse, ReloadOverrideResponse,
        VelocityModifierResponse,
    },
    meta_perks::*,
    origin_perks::*,
    other_perks::*,
    year_1_perks::*,
    year_2_perks::*,
    year_3_perks::*,
    year_4_perks::*,
    year_5_perks::*,
    year_6_perks::*,
};

use crate::weapons::Weapon;

pub fn clamp<T: PartialOrd>(n: T, min: T, max: T) -> T {
    if n < min {
        min
    } else if n > max {
        max
    } else {
        n
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Perk {
    pub stat_buffs: HashMap<u32, i32>,
    pub enhanced: bool,
    pub value: u32, //used for toggle and stacks
    pub hash: u32,
}

pub fn enhanced_check(_hash: u32) -> (u32, bool) {
    let mut result = _hash;
    let mut found = false;
    for (_, (h, r)) in database::ENHANCE_PERK_MAPPING.iter().enumerate() {
        if _hash == *h {
            result = *r;
            found = true;
            break;
        }
    }
    (result, found)
}

// all armor pekrs are for the future but wanted to started to compile them now

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, FromPrimitive)]
#[repr(u32)]
pub enum Perks {
    //Meta perks
    BuiltIn = 0,
    RallyBarricade = 444,
    EmpRift = 555,

    Radiant = 1380009033,
    Weaken = 1464159054,
    WellOfRadiance = 2274196887,
    WardOfDawn = 4260353953,
    BannerShield = 4260353952,

    //intrinsics
    RapidFireFrame = 902,

    //armor
    DexterityMod = 1001,
    TargetingMod = 1002,
    ReserveMod = 1003,
    LoaderMod = 1004,
    UnflinchingMod = 1005,
    SurgeMod = 1006,
    DragonShadow = 593361144,
    OphidianAspect = 1147638875,
    LunaFaction = 3347978672,
    TomeOfDawn = 926349160,
    PathOfTheBurningSteps = 2500502982,
    Foetracer = 2663272109,
    MechaneersTricksleeves = 481860151,
    Oathkeeper = 1449897496,
    SealedAhamkaraGrasps = 2805134531,
    LuckyPants = 1694242448,
    Stompees = 1694242450,
    NoBackupPlans = 569260333,
    ActiumWarRig = 1667892711,
    HallowfireHeart = 1667892708,
    LionRampart = 3241194940,
    Peacekeepers = 3241194941,
    PeregrineGreaves = 235075862,
    EyeOfAnotherWorld = 3927963100,
    AstrocyteVerse = 3295796664,
    NecroticGrips = 3824622015,
    BootsOfTheAssembler = 902934539,
    RainOfFire = 4222205045,
    SpeedloaderSlacks = 858592012,
    MantleOfBattleHarmony = 2618534366,
    MaskOfBakris = 692285813,
    BallindorseWrathweavers = 2894608781,

    //parts
    ImpactCasing = 3796465595,
    SwapMag = 3721627275,
    FullChoke = 1047830412,
    SpikeGrenades = 3301904089,
    AlloyMag = 1431678320,
    LiquidCoils = 1687452232,
    AcceleratedCoils = 689005463,
    ChargetimeMW = 3128594062,
    DisorientingGrenades = 3032599245,
    AssaultMag = 791862061,
    //bow strings
    SlowerStringT1 = 3371775011,
    FasterStringT2 = 2801223209,
    #[num_enum(alternatives = [
        1639384016,
        4067834857,
        852209214,
    ])]
    FasterStringT1 = 1885045197,

    //mods
    QuickAccessSling = 1334978104,
    BossSpec = 2788909693,
    MajorSpec = 984527513,
    MinorSpec = 4091000557,
    BigOnesSpec = 3018373291,
    TakenSpec = 1513326571,
    AdeptChargeTime = 744770875,
    FreehandGrip = 736000386,

    //origin | year 5+
    VeistStinger = 3988215619,
    HakkeBreach = 1607056502,
    Alacrity = 2988596335,
    FluidDynamics = 2839173408,
    QuietMoment = 4091460919,
    SurosSynergy = 4008116374,
    BitterSpite = 4154828211,
    RunnethOver = 120721526,
    HotSwap = 1260401931,
    RightHook = 3907865655,
    Ambush = 192157151,
    TexBalancedStock = 2437618208,
    SearchParty = 2250679103,
    FieldTested = 2120661319,

    //season 1 | year 1
    KillClip = 1015611457,
    #[num_enum(alternatives = [
        1528281896, //rose
        3124871000, //redrix
        1266037487, //R0
    ])]
    Outlaw = 1168162263,
    BackupPlan = 1600092898,
    FieldPrep = 2869569095,
    #[num_enum(alternatives = [3551326236, ])]//huckleberry
    Rampage = 3425386926,
    OpeningShot = 47981717,
    MovingTarget = 588594999,
    AmbitiousAssassin = 2010801679,
    ClusterBomb = 1275731761,
    ExplosivePayload = 3038247973,
    FirmlyPlanted = 280464955,
    FullAutoTrigger = 2117683199,
    HeadSeeker = 460017080,
    HighImpactReserves = 2213355989,
    HipFireGrip = 1866048759,
    Snapshot = 957782887,
    TapTheTrigger = 1890422124,
    SlideWays = 2039302152,
    QuickDraw = 706527188,
    TimedPayload = 1954620775,
    ThreatDetector = 4071163871,
    SlideShot = 3161816588,
    #[num_enum(alternatives = [1409312565, ])]//cloudstrike
    TripleTap = 3400784728,
    UnderPressure = 1645158859,
    PulseMonitor = 972757866,

    //season 2 | year 1
    //lmao bozo

    //season 3 | year 1
    RangeFinder = 2846385770,
    #[num_enum(alternatives = [1683379515, ])]//Arbalest
    DisruptionBreak = 3871884143,
    #[num_enum(alternatives = [2360754333, ])]//Acrius
    TrenchBarrel = 806159697,
    Desperado = 3047969693,
    BoxBreathing = 2551157718,

    //season 4 | year 2
    ArchersTempo = 201365942,
    ExplosiveHead = 3365897133,
    #[num_enum(alternatives = [1266037485, ])]//R0
    FeedingFrenzy = 2779035018,
    #[num_enum(alternatives = [1266037486, ])]//R0
    FourthTimesTheCharm = 1354429876,
    RapidHit = 247725512,

    //season 5 | year 2
    ResevoirBurst = 1427256713,
    Surrounded = 3708227201,
    AirAssault = 3722653512,
    OverFlow = 3643424744,

    //season 6 | year 2
    FiringLine = 1771339417,
    FullCourt = 2888557110,
    #[num_enum(alternatives = [557221067, ])]// delirium
    KillingTally = 2782457288,
    Demolitionist = 3523296417,
    MultikillClip = 2458213969,
    Swashbuckler = 4082225868,

    //season 7 | year 2
    UnderDog = 205890336,
    ExplosiveLight = 3194351027,
    EyeOfTheStorm = 699525795,
    NoDistractions = 2866798147,

    //season 8 | year 3
    //TODO

    //season 9 | year 3
    ClownCartridge = 2284787283,
    ElementalCapacitor = 3511092054,
    Vorpal = 1546637391,

    //season 10 | year 3
    //bad season lmao

    //season 11 | year 3
    KillingWind = 2450788523,

    //season 12 | year 4
    DualLoader = 25606670,
    OneForAll = 4049631843,
    Recombination = 469285294,
    Reconstruction = 1523832109,
    #[num_enum(alternatives = [3967134106, ])]
    Surplus = 3436462433,

    //season 13 | year 4
    ImpulseAmplifier = 951095735,
    Frenzy = 4104185692,
    LastingImpression = 3927722942,
    KickStart = 1754714824,

    //season 14 | year 4
    Cornered = 1799762209,
    AdrenalineJunkie = 11612903,
    RewindRounds = 3418782618,
    HeatingUp = 1570042021,
    FireFly = 3824105627,
    DangerZone = 960810156,
    TunnelVision = 2946784966,

    //season 15 | year 4
    Encore = 1195158366,
    #[num_enum(alternatives = [
        615063267, //V-wing
    ])]
    Ensemble = 2621346526,
    GoldenTricorn = 2610012052,
    Harmony = 438098033,
    PerpetualMotion = 1428297954,
    Adagio = 3673922083,

    //season 16 | year 5
    BaitAndSwitch = 3078487919,
    CompulsiveReloader = 671806388,
    FocusedFury = 2896038713,
    ChillClip = 2978966579,
    SleightOfHand = 2172504645,
    StatsForAll = 1583705720,
    SteadyHands = 509074078,
    SuccesfulWarmup = 2652708987,
    UnstoppableForce = 2224838837,

    //season 17 | year 5
    FragileFocus = 2451262963,
    WellRounded = 744594675,

    //season 18 | year 5
    GutShot = 1365187766,
    Pugilist = 691659142,
    Slickdraw = 1821614984,
    OverUnder = 1870851715,

    //season 19 | year 5
    CascadePoint = 3751912585,
    CloseToMelee = 1782407750,
    OffhandStrike = 2416023159,
    PerfectFloat = 2272927194,
    ShotSwap = 2586829431,
    TargetLock = 365154968,

    //season 20 | year 6
    KeepAway = 3619207468,
    ParacausalAffinity = 3215448563,
    EnviousAssasin = 968510818,

    //subclass
    OnYourMark = 3066103999,
    Hedrons = 3469412970,
    FlowState = 4194622036,
    Frequency = 1727069361,
    HeatRises = 83039194,
    Tempering = 362132290,
    ThreadOfAscent = 4208512216,
    Amplified = 880704824,

    //kinetic exotic
    CranialSpike = 1301843770,
    AgersCall = 970163821,
    RatPack = 2121086290,
    StringofCurses = 4004944400,
    RocketTracers = 3602718766,
    ParacausalShot = 213689231,
    TimeSlip = 3556949035,
    ToM = 2724693746,
    CorruptionSpreads = 4208418110,
    HonedEdge = 1070100196,
    HakkeHeavyBurst = 2206869417,
    FateOfAllFools = 3668782036,
    ExplosiveShadow = 1791592647,
    TakenPredator = 2130042297,
    SurosLegacy = 944506345,
    SpinningUp = 1378047685,
    DualSpeedReceiver = 4012962526,
    FullStop = 2984682260,
    RideTheBull = 630329983,
    HuntersTrance = 383825919,
    NobleRounds = 2144092201,
    StormAndStress = 2238035098,
    Roadborn = 3413860062,
    MarkovChain = 2814973067,

    //energy exotic
    LagragianSight = 2881100038,
    IgnitionTrigger = 961505134,
    GuidanceRing = 2226793914,
    LooksCanKill = 3174300811,
    ConserveMomentum = 656200654,
    Broadside = 407549716,
    Impetus = 2333607307,
    Stormbringer = 3117514172,
    PerfectFith = 1000724343,
    PrismaticInferno = 571267712,
    CalculatedBalance = 838219733,
    Chimera = 924149234,
    ThinTheHerd = 2003108620,
    RavenousBeast = 2540536653,
    LordOfWolvesCatalyst = 431220296,
    ReleaseTheWolves = 299272945,
    SwoopingTalons = 2656694271,
    #[num_enum(alternatives = [
        3081173348, //borealis
    ])]
    Fundamentals = 2620589274,

    //heavy exotic
    ReignHavoc = 4148158229,
    WormsHunger = 2812324400,
    WhisperCatalyst = 1340292993,
    DarkDescent = 3333994164,
    TargetAquired = 939227542,
    SleeperCatalyst = 2142466730,

    #[num_enum(default)]
    Ignore = 69420
}

pub fn get_perk_stats(
    _perks: Vec<Perk>,
    _input_data: CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> [HashMap<u32, i32>; 2] {
    let mut dynamic_stats: HashMap<u32, i32> = HashMap::new();
    let mut static_stats: HashMap<u32, i32> = HashMap::new();
    for perk in _perks {
        let perk_stats = dyanmic_perk_stats(&perk, &_input_data, _pvp, _cached_data);
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
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let perk_enum = _perk.hash.into();
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::Roadborn => sbr_roadborn(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::OphidianAspect => {
            sbr_ophidian_aspects(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::DragonShadow => sbr_dragon_shadow(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Amplified => sbr_amplified(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Tempering => sbr_tempering(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::OnYourMark => sbr_on_your_mark(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::HeatRises => sbr_heat_rises(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Hedrons => sbr_hedrons(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ThreatDetector => {
            sbr_threat_detector(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::FieldPrep => sbr_field_prep(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FirmlyPlanted => sbr_firmly_planted(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::HipFireGrip => sbr_hip_fire_grip(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::MovingTarget => sbr_moving_target(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::OpeningShot => sbr_opening_shot(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Outlaw => sbr_outlaw(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::SlideShot => sbr_slide_shot(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::SlideWays => sbr_slide_ways(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::TapTheTrigger => sbr_tap_the_trigger(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::AirAssault => sbr_air_assault(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FeedingFrenzy => sbr_feeding_frenzy(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ElementalCapacitor => {
            sbr_elemental_capacitor(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::PulseMonitor => sbr_pulse_monitor(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::KillingWind => sbr_killing_wind(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::AdrenalineJunkie => {
            sbr_adrenaline_junkie(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::Ensemble => sbr_ensemble(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Frenzy => sbr_frenzy(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::PerpetualMotion => {
            sbr_perpetual_motion(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::PerfectFloat => sbr_perfect_float(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Pugilist => sbr_pugilist(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Encore => sbr_encore(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FragileFocus => sbr_fragile_focus(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::OffhandStrike => sbr_offhand_strike(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::StatsForAll => sbr_stats_for_all(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::SteadyHands => sbr_steady_hands(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::WellRounded => sbr_well_rounded(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Alacrity => sbr_alacrity(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Ambush => sbr_ambush(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FluidDynamics => sbr_fluid_dynamics(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::QuietMoment => sbr_quiet_moment(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::BitterSpite => sbr_bitter_spite(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::RightHook => sbr_right_hook(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::BackupPlan => sbr_backup_plan(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::DangerZone => sbr_danger_zone(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::SleightOfHand => sbr_sleight_of_hand(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Slickdraw => sbr_slickdraw(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Harmony => sbr_harmony(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::CompulsiveReloader => {
            sbr_compulsive_reloader(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::RapidHit => sbr_rapid_hit(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ExplosiveLight => {
            sbr_explosive_light(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::ReleaseTheWolves => {
            sbr_release_the_wolves(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::Fundamentals => sbr_fundamentals(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ThinTheHerd => sbr_thin_the_herd(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Chimera => sbr_chimera(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::DualSpeedReceiver => {
            sbr_dual_speed_receiver(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::Surplus => sbr_surplus(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::QuickDraw => sbr_quickdraw(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::TexBalancedStock => {
            sbr_tex_balanced_stock(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::EyeOfTheStorm => {
            sbr_eye_of_the_storm(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::HeatingUp => sbr_heating_up(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ImpulseAmplifier => {
            sbr_impulse_amplifier(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::SurosSynergy => sbr_suros_synergy(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::TunnelVision => sbr_tunnel_vision(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ShotSwap => sbr_shot_swap(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::UnderDog => sbr_underdog(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::UnderPressure => sbr_under_pressure(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Adagio => sbr_adagio(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::HuntersTrance => sbr_hunters_trance(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::KeepAway => sbr_keep_away(_input_data, val, enhanced, _pvp, _cached_data),
        // Perks::FieldTested => sbr_field_tested(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::RallyBarricade => {
            sbr_rally_barricade(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::TomeOfDawn => sbr_tome_of_dawn(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::LoaderMod => sbr_loader_mods(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::TargetingMod => sbr_targeting_mods(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::LunaFaction => sbr_lunafaction_boots(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Foetracer => sbr_foetracer(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::MechaneersTricksleeves => {
            sbr_mechaneers_tricksleeves(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::Oathkeeper => sbr_oathkeeper(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::SealedAhamkaraGrasps => {
            sbr_sealed_ahamkara_grasps(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::LuckyPants => sbr_lucky_pants(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Stompees => sbr_stompees(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::NoBackupPlans => sbr_no_backup_plans(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ActiumWarRig => sbr_actium_war_rig(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::HallowfireHeart => {
            sbr_hallowfire_heart(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::LionRampart => sbr_lion_rampants(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Peacekeepers => sbr_peacekeepers(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::PeregrineGreaves => {
            sbr_peregrine_greaves(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::EyeOfAnotherWorld => {
            sbr_eye_of_another_world(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::AstrocyteVerse => {
            sbr_astrocyte_verse(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::NecroticGrips => sbr_necrotic_grip(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::BootsOfTheAssembler => {
            sbr_boots_of_the_assembler(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::RainOfFire => sbr_rain_of_fire(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::SpeedloaderSlacks => {
            sbr_speedloader_slacks(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::ThreadOfAscent => {
            sbr_thread_of_ascent(_input_data, val, enhanced, _pvp, _cached_data)
        }
        _ => HashMap::new(),
    }
}

pub fn get_dmg_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let mut dmg_modifier = DamageModifierResponse::default();
    for perk in _perks {
        let tmp = get_perk_dmr(perk.clone(), _input_data, _pvp, _cached_data);
        dmg_modifier.impact_dmg_scale *= tmp.impact_dmg_scale;
        dmg_modifier.explosive_dmg_scale *= tmp.explosive_dmg_scale;
        dmg_modifier.crit_scale *= tmp.crit_scale;
    }
    dmg_modifier
}
fn get_perk_dmr(
    _perk: Perk,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let perk_enum = _perk.hash.into();
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::HighImpactReserves => {
            dmr_high_impact_reserves(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::BoxBreathing => dmr_box_breathing(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ExplosivePayload => {
            dmr_explosive_payload(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::TimedPayload => dmr_timed_payload(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ImpactCasing => dmr_impact_casing(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ExplosiveHead => dmr_explosive_head(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FiringLine => dmr_firing_line(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::KillingTally => dmr_killing_tally(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ResevoirBurst => dmr_resevoir_burst(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Surrounded => dmr_surrounded(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::LastingImpression => {
            dmr_lasting_impressions(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::Vorpal => dmr_vorpal(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Adagio => dmr_adagio(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::AdrenalineJunkie => {
            dmr_adrenaline_junkie(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::Frenzy => dmr_frenzy(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FocusedFury => dmr_focused_fury(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::GutShot => dmr_gutshot_straight(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::TargetLock => dmr_target_lock(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::OverUnder => dmr_over_under(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::WormsHunger => dmr_worms_hunger(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::LagragianSight => {
            dmr_lagragian_sight(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::BuiltIn => dmr_builtin(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::BossSpec => dmr_boss_spec(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::MajorSpec => dmr_major_spec(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::MinorSpec => dmr_minor_spec(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::BigOnesSpec => dmr_big_ones_spec(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::TakenSpec => dmr_taken_spec(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Rampage => dmr_rampage(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ToM => dmr_tom(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::KillClip => dmr_kill_clip(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::BackupPlan => dmr_backup_plan(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::DisruptionBreak => {
            dmr_disruption_break(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::FullCourt => dmr_full_court(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::OneForAll => dmr_one_for_all(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::GoldenTricorn => dmr_golden_tricorn(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::BaitAndSwitch => dmr_bait_and_switch(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Swashbuckler => dmr_swash_buckler(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Harmony => dmr_harmony(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::AcceleratedCoils => {
            dmr_accelerated_coils(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::ChargetimeMW => dmr_chargetime_mw(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FullChoke => dmr_full_choke(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::LiquidCoils => dmr_liquid_coils(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::HakkeHeavyBurst => {
            dmr_hakke_heavy_burst(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::MultikillClip => dmr_multi_kill_clip(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::SpikeGrenades => dmr_spike_grenades(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ExplosiveLight => {
            dmr_explosive_light(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::DisorientingGrenades => {
            dmr_disorienting_grenades(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::SwoopingTalons => {
            dmr_swooping_talons(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::IgnitionTrigger => {
            dmr_ignition_trigger(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::CalculatedBalance => {
            dmr_vex_catalyst(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::RavenousBeast => dmr_ravenous_beast(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ReleaseTheWolves => {
            dmr_release_the_wolves(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::LooksCanKill => dmr_first_glance(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FateOfAllFools => {
            dmr_fate_of_all_fools(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::HonedEdge => dmr_honed_edge(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::TakenPredator => dmr_taken_predator(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::MarkovChain => dmr_markov_chain(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::StormAndStress => {
            dmr_storm_and_stress(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::FullStop => dmr_full_stop(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ParacausalShot => {
            dmr_paracausal_shot(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::Radiant => dmr_radiant(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Weaken => dmr_weaken(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::PathOfTheBurningSteps => {
            dmr_path_of_burning_steps(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::WellOfRadiance => {
            dmr_well_of_radiance(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::MantleOfBattleHarmony => {
            dmr_mantle_of_battle_harmony(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::MaskOfBakris => dmr_mask_of_bakris(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::BallindorseWrathweavers => {
            dmr_ballidorse_wrathweavers(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::BootsOfTheAssembler => {
            dmr_blessing_of_the_sky(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::KickStart => dmr_kickstart(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::SurgeMod => dmr_surge_mods(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::MechaneersTricksleeves => {
            dmr_mechaneers_tricksleeves(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::LuckyPants => dmr_lucky_pants(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Foetracer => dmr_foetracer(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::NobleRounds => dmr_noble_rounds(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ParacausalAffinity => {
            dmr_paracausal_affinity(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::EmpRift => dmr_empowering_rift(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::WardOfDawn => dmr_ward_of_dawn(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::BannerShield => dmr_banner_shield(_input_data, val, enhanced, _pvp, _cached_data),
        _ => DamageModifierResponse::default(),
    }
}

pub fn get_reload_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    let mut reload_modifier = ReloadModifierResponse::default();
    for perk in _perks {
        let tmp = get_perk_rsmr(perk, _input_data, _pvp, _cached_data);
        reload_modifier.reload_stat_add += tmp.reload_stat_add;
        reload_modifier.reload_time_scale *= tmp.reload_time_scale;
    }
    reload_modifier
}
fn get_perk_rsmr(
    _perk: Perk,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    let perk_enum = _perk.hash.into();
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::RapidFireFrame => rsmr_rapid_fire(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::AlloyMag => rsmr_alloy_mag(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Roadborn => rsmr_roadborn(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::OphidianAspect => {
            rsmr_ophidian_aspects(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::DragonShadow => rsmr_dragon_shadow(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Frequency => rsmr_frequency(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FlowState => rsmr_flow_state(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::OnYourMark => rsmr_on_your_mark(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ThreatDetector => {
            rsmr_threat_detector(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::FieldPrep => rsmr_field_prep(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FeedingFrenzy => rsmr_feeding_frenzy(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::RapidHit => rsmr_rapid_hit(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ElementalCapacitor => {
            rsmr_elemental_capacitor(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::Ensemble => rsmr_ensemble(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Frenzy => rsmr_frenzy(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ImpulseAmplifier => {
            rsmr_impulse_amplifier(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::PerpetualMotion => {
            rsmr_perpetual_motion(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::StatsForAll => rsmr_stats_for_all(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Alacrity => rsmr_alacrity(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FluidDynamics => rsmr_fluid_dynamics(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::QuietMoment => rsmr_quiet_moment(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::BitterSpite => rsmr_bitter_spite(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::LoaderMod => rsmr_loader_mods(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Outlaw => rsmr_outlaw(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FireFly => rsmr_fire_fly(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::CompulsiveReloader => {
            rsmr_compulsive_reloader(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::SleightOfHand => {
            rsmr_sleight_of_hand(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::ReleaseTheWolves => {
            rsmr_release_the_wolves(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::Fundamentals => rsmr_fundamentals(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ThinTheHerd => rsmr_thin_the_herd(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Surplus => rsmr_surplus(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::TexBalancedStock => {
            rsmr_tex_balanced_stock(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::UnderDog => rsmr_underdog(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::HuntersTrance => rsmr_hunters_trance(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::KeepAway => rsmr_keep_away(_input_data, val, enhanced, _pvp, _cached_data),
        // Perks::FieldTested => rsmr_field_tested(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::RallyBarricade => {
            rsmr_rally_barricade(_input_data, val, enhanced, _pvp, _cached_data)
        }

        Perks::SpeedloaderSlacks => {
            rsmr_speedloader_slacks(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::LunaFaction => {
            rsmr_lunafaction_boots(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::ThreadOfAscent => {
            rsmr_thread_of_ascent(_input_data, val, enhanced, _pvp, _cached_data)
        }
        _ => ReloadModifierResponse::default(),
    }
}

pub fn get_firing_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> FiringModifierResponse {
    let mut firing_modifier = FiringModifierResponse::default();
    for perk in _perks {
        let tmp = get_perk_fmr(perk, _input_data, _pvp, _cached_data);
        firing_modifier.burst_delay_scale *= tmp.burst_delay_scale;
        firing_modifier.burst_delay_add += tmp.burst_delay_add;
        firing_modifier.inner_burst_scale *= tmp.inner_burst_scale;
        firing_modifier.burst_size_add += tmp.burst_size_add;
    }
    firing_modifier
}
fn get_perk_fmr(
    _perk: Perk,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> FiringModifierResponse {
    let perk_enum = _perk.hash.into();
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::Roadborn => fmr_roadborn(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Desperado => fmr_desperado(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ArchersTempo => fmr_archers_tempo(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Adagio => fmr_adagio(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Cornered => fmr_cornered(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::CascadePoint => fmr_cascade_point(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ReignHavoc => fmr_reign_havoc(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::BackupPlan => fmr_backup_plan(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::AcceleratedCoils => {
            fmr_accelerated_coils(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::ChargetimeMW => {
            fmr_accelerated_coils(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::LiquidCoils => fmr_liquid_coils(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::HakkeHeavyBurst => {
            fmr_hakke_heavy_burst(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::AdeptChargeTime => {
            fmr_accelerated_coils(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::RavenousBeast => fmr_ravenous_beast(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ReleaseTheWolves => {
            fmr_release_the_wolves(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::AssaultMag => fmr_assault_mag(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FullAutoTrigger => {
            fmr_full_auto_trigger(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::RatPack => fmr_rat_pack(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::SpinningUp => fmr_spinning_up(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::RideTheBull => fmr_ride_the_bull(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FasterStringT1 => {
            fmr_faster_string_t1(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::FasterStringT2 => {
            fmr_faster_string_t2(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::SlowerStringT1 => {
            fmr_slower_string_t1(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::SuccesfulWarmup => {
            fmr_succesful_warmup(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::KickStart => fmr_kickstart(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::BuiltIn => fmr_builtin(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Oathkeeper => fmr_faster_string_t1(_input_data, val, enhanced, _pvp, _cached_data),
        _ => FiringModifierResponse::default(),
    }
}

pub fn get_handling_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HandlingModifierResponse {
    let mut handling_modifier = HandlingModifierResponse::default();
    for perk in _perks {
        let tmp = get_perk_hmr(perk, _input_data, _pvp, _cached_data);
        handling_modifier.stat_add += tmp.stat_add;
        handling_modifier.stow_scale *= tmp.stow_scale;
        handling_modifier.draw_scale *= tmp.draw_scale;
        handling_modifier.ads_scale *= tmp.ads_scale;
    }
    handling_modifier
}
fn get_perk_hmr(
    _perk: Perk,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HandlingModifierResponse {
    let perk_enum = _perk.hash.into();
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::KillingWind => hmr_killing_wind(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::SwapMag => hmr_swap_mag(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::QuickAccessSling => hmr_swap_mag(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::OphidianAspect => {
            hmr_ophidian_aspects(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::DragonShadow => hmr_dragon_shadow(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Amplified => hmr_amplified(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::OnYourMark => hmr_on_your_mark(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ThreatDetector => {
            hmr_threat_detector(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::FirmlyPlanted => hmr_firmly_planted(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Snapshot => hmr_snapshot(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ElementalCapacitor => {
            hmr_elemental_capacitor(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::AdrenalineJunkie => {
            hmr_adrenaline_junkie(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::Ensemble => hmr_ensemble(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Frenzy => hmr_frenzy(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::PerpetualMotion => {
            hmr_perpetual_motion(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::Slickdraw => hmr_slickdraw(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::StatsForAll => hmr_stats_for_all(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::SteadyHands => hmr_steady_hands(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::WellRounded => hmr_well_rounded(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::HotSwap => hmr_hot_swap(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::SearchParty => hmr_search_party(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::DexterityMod => hmr_dexterity_mods(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::TargetingMod => hmr_targeting_mods(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::BackupPlan => hmr_backup_plan(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::SleightOfHand => hmr_sleight_of_hand(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Harmony => hmr_harmony(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::SlideWays => hmr_slide_ways(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Fundamentals => hmr_fundamentals(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Chimera => hmr_chimera(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Surplus => hmr_surplus(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::QuickDraw => hmr_quickdraw(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::TexBalancedStock => {
            hmr_tex_balanced_stock(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::PulseMonitor => hmr_pulse_monitor(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::EyeOfTheStorm => {
            hmr_eye_of_the_storm(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::SurosSynergy => hmr_suros_synergy(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::TunnelVision => hmr_tunnel_vision(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ShotSwap => hmr_shot_swap(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::HuntersTrance => hmr_hunters_trance(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::LuckyPants => hmr_lucky_pants(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Peacekeepers => hmr_peacekeepers(_input_data, val, enhanced, _pvp, _cached_data),
        // Perks::FieldTested => hmr_field_tested(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FreehandGrip => hmr_freehand_grip(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ThreadOfAscent => {
            hmr_thread_of_ascent(_input_data, val, enhanced, _pvp, _cached_data)
        }
        _ => HandlingModifierResponse::default(),
    }
}

pub fn get_magazine_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> MagazineModifierResponse {
    let mut magazine_modifier = MagazineModifierResponse::default();
    for perk in _perks {
        let tmp = get_perk_mmr(perk, _input_data, _pvp, _cached_data);
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
    _cached_data: &mut HashMap<String, f64>,
) -> MagazineModifierResponse {
    let perk_enum = _perk.hash.into();
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::AgersCall => mmr_agers_call(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::AmbitiousAssassin => {
            mmr_ambitious_assassin(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::OverFlow => mmr_overflow(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ClownCartridge => {
            mmr_clown_cartridge(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::Reconstruction => mmr_reconstruction(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::RunnethOver => mmr_runneth_over(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::RatPack => mmr_rat_pack(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::EnviousAssasin => {
            mmr_envious_assassin(_input_data, val, enhanced, _pvp, _cached_data)
        }
        _ => MagazineModifierResponse::default(),
    }
}

pub fn get_reserve_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> InventoryModifierResponse {
    let mut reserve_modifier = InventoryModifierResponse::default();
    for perk in _perks {
        let tmp = get_perk_imr(perk, _input_data, _pvp, _cached_data);
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
    _cached_data: &mut HashMap<String, f64>,
) -> InventoryModifierResponse {
    let perk_enum = _perk.hash.into();
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::FieldPrep => imr_field_prep(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ReserveMod => imr_reserve_mods(_input_data, val, enhanced, _pvp, _cached_data),
        _ => InventoryModifierResponse::default(),
    }
}

pub fn get_range_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> RangeModifierResponse {
    let mut range_modifier = RangeModifierResponse::default();
    for perk in _perks {
        let tmp = get_perk_rmr(perk, _input_data, _pvp, _cached_data);
        range_modifier.range_stat_add += tmp.range_stat_add;
        range_modifier.range_all_scale *= tmp.range_all_scale;
        range_modifier.range_hip_scale *= tmp.range_hip_scale;
        range_modifier.range_zoom_scale *= tmp.range_zoom_scale;
    }
    range_modifier
}
fn get_perk_rmr(
    _perk: Perk,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> RangeModifierResponse {
    let perk_enum = _perk.hash.into();
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::HipFireGrip => rmr_hip_fire_grip(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::OpeningShot => rmr_opening_shot(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::RangeFinder => rmr_range_finder(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::SlideShot => rmr_slide_shot(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::KillingWind => rmr_killing_wind(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FragileFocus => rmr_fragile_focus(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::OffhandStrike => rmr_offhand_strike(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::StatsForAll => rmr_stats_for_all(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::WellRounded => rmr_well_rounded(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Alacrity => rmr_alacrity(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::KeepAway => rmr_keep_away(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::RightHook => rmr_right_hook(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Encore => rmr_encore(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Fundamentals => rmr_fundamentals(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::DualSpeedReceiver => {
            rmr_dual_speed_receiver(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::Adagio => rmr_adagio(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::HuntersTrance => rmr_hunters_trance(_input_data, val, enhanced, _pvp, _cached_data),
        // Perks::FieldTested => rmr_field_tested(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::RallyBarricade => {
            rmr_rally_barricade(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::LunaFaction => rmr_lunafaction_boots(_input_data, val, enhanced, _pvp, _cached_data),
        _ => RangeModifierResponse::default(),
    }
}

pub fn get_refund_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> Vec<RefundResponse> {
    let mut refund_modifier = vec![];
    for perk in _perks {
        let tmp = get_perk_refund(perk, _input_data, _pvp, _cached_data);
        if tmp.requirement > 0 {
            refund_modifier.push(tmp);
        }
    }
    refund_modifier
}
fn get_perk_refund(
    _perk: Perk,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> RefundResponse {
    let perk_enum = _perk.hash.into();
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::TripleTap => rr_triple_tap(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FourthTimesTheCharm => {
            rr_fourth_times(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::VeistStinger => rr_veist_stinger(_input_data, val, enhanced, _pvp, _cached_data),
        _ => RefundResponse::default(),
    }
}

pub fn get_extra_damage(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
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
    _cached_data: &mut HashMap<String, f64>,
) -> ExtraDamageResponse {
    let perk_enum = _perk.hash.into();
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::ReignHavoc => edr_reign_havoc(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ClusterBomb => edr_cluster_bomb(_input_data, val, enhanced, _pvp, _cached_data),
        _ => ExtraDamageResponse::default(),
    }
}

pub fn get_reload_overrides(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> Vec<ReloadOverrideResponse> {
    let mut reload_overrides = vec![];
    for perk in _perks {
        let tmp = get_perk_ror(perk, _input_data, _pvp, _cached_data);
        if tmp.valid {
            reload_overrides.push(tmp);
        }
    }
    reload_overrides
}
fn get_perk_ror(
    _perk: Perk,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadOverrideResponse {
    let perk_enum = _perk.hash.into();
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::Demolitionist => ror_demolitionist(_input_data, val, enhanced, _pvp, _cached_data),
        _ => ReloadOverrideResponse::invalid(),
    }
}

pub fn get_explosion_data(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
) -> ExplosivePercentResponse {
    let mut highest_so_far = ExplosivePercentResponse::default();
    for perk in _perks {
        let tmp = get_perk_epr(perk, _input_data, _pvp);
        if tmp.percent > highest_so_far.percent {
            highest_so_far = tmp;
        }
    }
    highest_so_far
}

fn get_perk_epr(
    _perk: Perk,
    _input_data: &CalculationInput,
    _pvp: bool,
) -> ExplosivePercentResponse {
    let perk_enum = _perk.hash.into();
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::ExplosivePayload => {
            epr_explosive_payload(_input_data, val, enhanced, _pvp, &mut HashMap::new())
        }
        Perks::ExplosiveHead => {
            epr_explosive_head(_input_data, val, enhanced, _pvp, &mut HashMap::new())
        }
        Perks::TimedPayload => {
            epr_timed_payload(_input_data, val, enhanced, _pvp, &mut HashMap::new())
        }
        Perks::BuiltIn => epr_builtin(_input_data, val, enhanced, _pvp, &mut HashMap::new()),
        _ => ExplosivePercentResponse::default(),
    }
}

pub fn get_flinch_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> FlinchModifierResponse {
    let mut tmp = FlinchModifierResponse::default();
    for perk in _perks {
        tmp.flinch_scale *= get_perk_flmr(perk, _input_data, _pvp).flinch_scale;
    }
    tmp
}

fn get_perk_flmr(
    _perk: Perk,
    _input_data: &CalculationInput,
    _pvp: bool,
) -> FlinchModifierResponse {
    let perk_enum = _perk.hash.into();
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::SurosSynergy => {
            flmr_suros_synergy(_input_data, val, enhanced, _pvp, &mut HashMap::new())
        }
        Perks::NoDistractions => {
            flmr_no_distractions(_input_data, val, enhanced, _pvp, &mut HashMap::new())
        }
        Perks::UnflinchingMod => {
            flmr_unflinching_mod(_input_data, val, enhanced, _pvp, &mut HashMap::new())
        }
        Perks::RallyBarricade => {
            flmr_rally_barricade(_input_data, val, enhanced, _pvp, &mut HashMap::new())
        }
        Perks::TomeOfDawn => {
            flmr_tome_of_dawn(_input_data, val, enhanced, _pvp, &mut HashMap::new())
        }
        //Perks::PerfectFloat => todo!(), //Perfect floats flinch resist value is unknown atm
        _ => FlinchModifierResponse::default(),
    }
}

pub fn get_velocity_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> VelocityModifierResponse {
    let mut tmp = VelocityModifierResponse::default();
    for perk in _perks {
        tmp.velocity_scaler *= get_perk_vmr(perk, _input_data, _pvp).velocity_scaler;
    }
    tmp
}

fn get_perk_vmr(
    _perk: Perk,
    _input_data: &CalculationInput,
    _pvp: bool,
) -> VelocityModifierResponse {
    let perk_enum: Perks = _perk.hash.into();
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::RangeFinder => {
            vmr_range_finder(_input_data, val, enhanced, _pvp, &mut HashMap::new())
        }
        Perks::ImpulseAmplifier => {
            vmr_impulse_amplifier(_input_data, val, enhanced, _pvp, &mut HashMap::new())
        }
        _ => VelocityModifierResponse::default(),
    }
}

impl Weapon {
    pub fn get_modifier_summary(
        &self,
        _calc_input: Option<CalculationInput>,
        _pvp: bool,
        _cached_data: Option<&mut HashMap<String, f64>>,
    ) -> HashMap<BungieHash, ModifierResponseSummary> {
        let mut default_cached_data = HashMap::new();
        let cached_data = _cached_data.unwrap_or(&mut default_cached_data);
        let mut buffer: HashMap<u32, ModifierResponseSummary> = HashMap::new();
        if _calc_input.is_none() {
            return buffer;
        }

        let calc_input = _calc_input.unwrap();

        for perk in self.list_perks() {
            let mut mod_buffer = ModifierResponseSummary::default();

            let modifier = get_perk_rmr(perk.clone(), &calc_input, _pvp, cached_data);
            if modifier != RangeModifierResponse::default() {
                mod_buffer.rmr = Some(modifier);
            }

            let modifier = get_perk_dmr(perk.clone(), &calc_input, _pvp, cached_data);
            if modifier != DamageModifierResponse::default() {
                mod_buffer.dmr = Some(modifier);
            }

            let modifier = get_perk_hmr(perk.clone(), &calc_input, _pvp, cached_data);
            if modifier != HandlingModifierResponse::default() {
                mod_buffer.hmr = Some(modifier);
            }

            let modifier = get_perk_fmr(perk.clone(), &calc_input, _pvp, cached_data);
            if modifier != FiringModifierResponse::default() {
                mod_buffer.fmr = Some(modifier);
            }

            let modifier = get_perk_flmr(perk.clone(), &calc_input, _pvp);
            if modifier != FlinchModifierResponse::default() {
                mod_buffer.flmr = Some(modifier);
            }

            let modifier = get_perk_rsmr(perk.clone(), &calc_input, _pvp, cached_data);
            if modifier != ReloadModifierResponse::default() {
                mod_buffer.rsmr = Some(modifier);
            }

            let modifier = get_perk_mmr(perk.clone(), &calc_input, _pvp, cached_data);
            if modifier != MagazineModifierResponse::default() {
                mod_buffer.mmr = Some(modifier);
            }

            let modifier = get_perk_imr(perk.clone(), &calc_input, _pvp, cached_data);
            if modifier != InventoryModifierResponse::default() {
                mod_buffer.imr = Some(modifier);
            }

            let stat_mod = dyanmic_perk_stats(&perk.clone(), &calc_input, _pvp, cached_data);
            let mut stat_buffer: HashMap<BungieHash, StatBump> = HashMap::new();
            for (key, value) in stat_mod {
                stat_buffer.insert(key, value);
            }

            for (key, value) in perk.stat_buffs {
                stat_buffer
                    .entry(key)
                    .and_modify(|stat| *stat += value)
                    .or_insert(value);
            }
            mod_buffer.statbump = Some(stat_buffer);
            buffer.insert(perk.hash, mod_buffer);
        }

        buffer
    }
}
