use std::collections::HashMap;

use serde::Serialize;

use super::{enhanced_check, Perk, Perks};

#[derive(Debug, Clone, Serialize)]
pub enum PerkValueVariant {
    STATIC,
    TOGGLE,
    SLIDER,
    OPTIONS,
}
impl Default for PerkValueVariant {
    fn default() -> Self {
        PerkValueVariant::STATIC
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PerkOptionData {
    stacks: (u32, u32),
    options: Vec<String>,
    #[serde(rename = "optionType")]
    option_type: PerkValueVariant,
}
impl PerkOptionData {
    pub fn passive() -> PerkOptionData {
        PerkOptionData {
            stacks: (0, 0),
            options: vec![],
            option_type: PerkValueVariant::STATIC,
        }
    }
    pub fn toggle() -> PerkOptionData {
        PerkOptionData {
            stacks: (0, 1),
            options: vec![],
            option_type: PerkValueVariant::TOGGLE,
        }
    }
    pub fn stacking(_stacks: u32) -> PerkOptionData {
        PerkOptionData {
            stacks: (0, _stacks),
            options: vec![],
            option_type: PerkValueVariant::SLIDER,
        }
    }
    pub fn stacking_min(_stacks: u32, _min_stacks: u32) -> PerkOptionData {
        PerkOptionData {
            stacks: (_min_stacks, _stacks),
            options: vec![],
            option_type: PerkValueVariant::SLIDER,
        }
    }
    pub fn options(_options: Vec<&str>) -> PerkOptionData {
        let mut options = vec!["None".to_string()];
        for option in _options {
            options.push(option.to_string());
        }
        PerkOptionData {
            stacks: (0, options.len() as u32 - 1),
            options,
            option_type: PerkValueVariant::OPTIONS,
        }
    }
}

fn hash_to_perk_option_data(_hash: u32) -> Option<PerkOptionData> {
    let perk: Perks = enhanced_check(_hash).0.into();
    match perk {
        //Meta perks
        Perks::BuiltIn => None,
        Perks::RallyBarricade => Some(PerkOptionData::passive()),
        Perks::EmpRift => Some(PerkOptionData::passive()),

        //intrinsics
        Perks::RapidFireFrame => Some(PerkOptionData::toggle()),

        //armor

        //parts
        Perks::ImpactCasing => Some(PerkOptionData::passive()),
        Perks::SwapMag => Some(PerkOptionData::passive()),
        Perks::FullChoke => Some(PerkOptionData::passive()),
        Perks::SpikeGrenades => Some(PerkOptionData::passive()),
        Perks::AlloyMag => Some(PerkOptionData::toggle()),
        Perks::LiquidCoils => Some(PerkOptionData::passive()),
        Perks::AcceleratedCoils => Some(PerkOptionData::passive()),
        Perks::ChargetimeMW => Some(PerkOptionData::passive()),
        Perks::DisorientingGrenades => Some(PerkOptionData::passive()),
        Perks::AssaultMag => Some(PerkOptionData::passive()),
        Perks::AdeptChargeTime => Some(PerkOptionData::passive()),
        //bow strings
        Perks::SlowerStringT1 => Some(PerkOptionData::passive()),
        Perks::FasterStringT1 => Some(PerkOptionData::passive()),
        Perks::FasterStringT2 => Some(PerkOptionData::passive()),

        //mods
        Perks::QuickAccessSling => Some(PerkOptionData::passive()),
        Perks::BossSpec => Some(PerkOptionData::passive()),
        Perks::MajorSpec => Some(PerkOptionData::passive()),
        Perks::MinorSpec => Some(PerkOptionData::passive()),
        Perks::BigOnesSpec => Some(PerkOptionData::passive()),
        Perks::TakenSpec => Some(PerkOptionData::toggle()),
        Perks::FreehandGrip => Some(PerkOptionData::passive()),

        //origin | year 5+
        Perks::VeistStinger => Some(PerkOptionData::toggle()),
        Perks::HakkeBreach => Some(PerkOptionData::toggle()),
        Perks::Alacrity => Some(PerkOptionData::toggle()),
        Perks::FluidDynamics => Some(PerkOptionData::toggle()),
        Perks::QuietMoment => Some(PerkOptionData::toggle()),
        Perks::SurosSynergy => Some(PerkOptionData::toggle()),
        Perks::BitterSpite => Some(PerkOptionData::stacking(5)),
        Perks::RunnethOver => Some(PerkOptionData::stacking(5)),
        Perks::HotSwap => Some(PerkOptionData::toggle()),
        Perks::RightHook => Some(PerkOptionData::toggle()),
        Perks::Ambush => Some(PerkOptionData::toggle()),
        Perks::TexBalancedStock => Some(PerkOptionData::toggle()),
        Perks::SearchParty => Some(PerkOptionData::passive()),

        //season 1 | year 1
        Perks::KillClip => Some(PerkOptionData::toggle()),
        Perks::Outlaw => Some(PerkOptionData::toggle()),
        Perks::BackupPlan => Some(PerkOptionData::toggle()),
        Perks::FieldPrep => Some(PerkOptionData::toggle()),
        Perks::Rampage => Some(PerkOptionData::stacking(3)),
        Perks::OpeningShot => Some(PerkOptionData::toggle()),
        Perks::MovingTarget => Some(PerkOptionData::toggle()),
        Perks::AmbitiousAssassin => Some(PerkOptionData::stacking(15)),
        Perks::ClusterBomb => Some(PerkOptionData::passive()),
        Perks::ExplosivePayload => Some(PerkOptionData::passive()),
        Perks::FirmlyPlanted => Some(PerkOptionData::toggle()),
        Perks::FullAutoTrigger => Some(PerkOptionData::passive()),
        Perks::HeadSeeker => Some(PerkOptionData::passive()),
        Perks::HighImpactReserves => Some(PerkOptionData::passive()),
        Perks::HipFireGrip => Some(PerkOptionData::toggle()),
        Perks::Snapshot => Some(PerkOptionData::passive()),
        Perks::TapTheTrigger => Some(PerkOptionData::toggle()),
        Perks::SlideWays => Some(PerkOptionData::toggle()),
        Perks::QuickDraw => Some(PerkOptionData::toggle()),
        Perks::TimedPayload => Some(PerkOptionData::passive()),
        Perks::ThreatDetector => Some(PerkOptionData::stacking(2)),
        Perks::SlideShot => Some(PerkOptionData::toggle()),
        Perks::TripleTap => Some(PerkOptionData::passive()),
        Perks::UnderPressure => Some(PerkOptionData::toggle()),
        Perks::PulseMonitor => Some(PerkOptionData::toggle()),

        //season 2 | year 1
        //lmao bozo

        //season 3 | year 1
        Perks::RangeFinder => Some(PerkOptionData::passive()),
        Perks::DisruptionBreak => Some(PerkOptionData::toggle()),
        Perks::TrenchBarrel => Some(PerkOptionData::toggle()),
        Perks::Desperado => Some(PerkOptionData::toggle()),
        Perks::BoxBreathing => Some(PerkOptionData::toggle()),

        //season 4 | year 2
        Perks::ArchersTempo => Some(PerkOptionData::toggle()),
        Perks::ExplosiveHead => Some(PerkOptionData::passive()),
        Perks::FeedingFrenzy => Some(PerkOptionData::stacking(5)),
        Perks::FourthTimesTheCharm => Some(PerkOptionData::passive()),
        Perks::RapidHit => Some(PerkOptionData::stacking(5)),

        //season 5 | year 2
        Perks::ResevoirBurst => Some(PerkOptionData::toggle()),
        Perks::Surrounded => Some(PerkOptionData::toggle()),
        Perks::AirAssault => Some(PerkOptionData::stacking(2)),

        //season 6 | year 2
        Perks::FiringLine => Some(PerkOptionData::toggle()),
        Perks::FullCourt => Some(PerkOptionData::toggle()),
        Perks::KillingTally => Some(PerkOptionData::stacking(3)),
        // Perks::Demolitionist => Some(PerkOptionData::options(vec!["Once", "Every 3s"])),
        Perks::MultikillClip => Some(PerkOptionData::stacking(3)),
        Perks::Swashbuckler => Some(PerkOptionData::stacking(5)),
        Perks::OverFlow => Some(PerkOptionData::toggle()),

        //season 7 | year 2
        Perks::UnderDog => Some(PerkOptionData::toggle()),
        Perks::ExplosiveLight => Some(PerkOptionData::toggle()),
        Perks::EyeOfTheStorm => Some(PerkOptionData::toggle()),
        Perks::NoDistractions => Some(PerkOptionData::toggle()),

        //season 8 | year 3
        //TODO

        //season 9 | year 3
        Perks::ClownCartridge => Some(PerkOptionData::passive()),
        Perks::ElementalCapacitor => Some(PerkOptionData::options(
            ["Void", "Solar", "Arc", "Stasis", "Strand"].to_vec(),
        )),
        Perks::Vorpal => Some(PerkOptionData::passive()),

        //season 10 | year 3
        //bad season lmao

        //season 11 | year 3
        Perks::KillingWind => Some(PerkOptionData::toggle()),

        //season 12 | year 4
        Perks::DualLoader => Some(PerkOptionData::toggle()),
        Perks::OneForAll => Some(PerkOptionData::toggle()),
        Perks::Recombination => Some(PerkOptionData::toggle()),
        Perks::Reconstruction => Some(PerkOptionData::toggle()),
        Perks::Surplus => Some(PerkOptionData::stacking(3)),

        //season 13 | year 4
        Perks::ImpulseAmplifier => Some(PerkOptionData::passive()),
        Perks::Frenzy => Some(PerkOptionData::toggle()),
        Perks::LastingImpression => Some(PerkOptionData::passive()),
        Perks::KickStart => Some(PerkOptionData::toggle()),

        //season 14 | year 4
        Perks::Cornered => Some(PerkOptionData::toggle()),
        Perks::AdrenalineJunkie => Some(PerkOptionData::stacking(5)),
        Perks::RewindRounds => Some(PerkOptionData::passive()),
        Perks::HeatingUp => Some(PerkOptionData::stacking(2)),
        Perks::FireFly => Some(PerkOptionData::toggle()),
        Perks::DangerZone => Some(PerkOptionData::toggle()),
        Perks::TunnelVision => Some(PerkOptionData::toggle()),

        //season 15 | year 4
        Perks::Encore => Some(PerkOptionData::stacking(4)),
        Perks::Ensemble => Some(PerkOptionData::toggle()),
        Perks::GoldenTricorn => Some(PerkOptionData::stacking(2)),
        Perks::Harmony => Some(PerkOptionData::toggle()),
        Perks::PerpetualMotion => Some(PerkOptionData::stacking(2)),
        Perks::Adagio => Some(PerkOptionData::toggle()),

        //season 16 | year 5
        Perks::BaitAndSwitch => Some(PerkOptionData::toggle()),
        Perks::CompulsiveReloader => Some(PerkOptionData::toggle()),
        Perks::FocusedFury => Some(PerkOptionData::toggle()),
        Perks::ChillClip => Some(PerkOptionData::passive()),
        Perks::SleightOfHand => Some(PerkOptionData::stacking(3)),
        Perks::StatsForAll => Some(PerkOptionData::toggle()),
        Perks::SteadyHands => Some(PerkOptionData::toggle()),
        Perks::SuccesfulWarmup => Some(PerkOptionData::toggle()),
        Perks::UnstoppableForce => Some(PerkOptionData::toggle()),

        //season 17 | year 5
        Perks::FragileFocus => Some(PerkOptionData::toggle()),
        Perks::WellRounded => Some(PerkOptionData::stacking(2)),

        //season 18 | year 5
        Perks::GutShot => Some(PerkOptionData::passive()),
        Perks::Pugilist => Some(PerkOptionData::toggle()),
        Perks::Slickdraw => Some(PerkOptionData::passive()),
        Perks::OverUnder => Some(PerkOptionData::passive()),

        //season 19 | year 5
        Perks::CascadePoint => Some(PerkOptionData::toggle()),
        Perks::CloseToMelee => Some(PerkOptionData::toggle()),
        Perks::OffhandStrike => Some(PerkOptionData::toggle()),
        Perks::PerfectFloat => Some(PerkOptionData::toggle()),
        Perks::ShotSwap => Some(PerkOptionData::toggle()),
        Perks::TargetLock => Some(PerkOptionData::passive()),

        //season 20 | year 6
        Perks::KeepAway => Some(PerkOptionData::toggle()),
        Perks::ParacausalAffinity => Some(PerkOptionData::toggle()),
        Perks::EnviousAssasin => Some(PerkOptionData::stacking(15)),
        // Perks::FieldTested => Some(PerkOptionData::stacking(5)),

        //exotics
        Perks::CranialSpike => Some(PerkOptionData::stacking(5)),
        Perks::AgersCall => Some(PerkOptionData::toggle()),
        Perks::LagragianSight => Some(PerkOptionData::toggle()),
        Perks::StringofCurses => Some(PerkOptionData::stacking(5)),
        Perks::WormsHunger => Some(PerkOptionData::stacking(20)),
        Perks::RocketTracers => Some(PerkOptionData::passive()),
        Perks::ParacausalShot => Some(PerkOptionData::stacking(7)),
        Perks::CorruptionSpreads => Some(PerkOptionData::passive()),
        Perks::TimeSlip => Some(PerkOptionData::toggle()),
        Perks::ToM => Some(PerkOptionData::toggle()),
        Perks::IgnitionTrigger => Some(PerkOptionData::toggle()),
        Perks::GuidanceRing => Some(PerkOptionData::toggle()),
        Perks::ConserveMomentum => Some(PerkOptionData::stacking(15)),
        Perks::Impetus => Some(PerkOptionData::toggle()),
        Perks::FirstGlance => Some(PerkOptionData::toggle()),
        Perks::PerfectFith => Some(PerkOptionData::passive()),
        Perks::Broadside => Some(PerkOptionData::stacking(4)),
        Perks::Stormbringer => Some(PerkOptionData::passive()),
        Perks::PrismaticInferno => Some(PerkOptionData::passive()),
        Perks::ReignHavoc => Some(PerkOptionData::toggle()),
        Perks::WhisperCatalyst => Some(PerkOptionData::toggle()),
        Perks::Roadborn => Some(PerkOptionData::toggle()),
        Perks::SwoopingTalons => Some(PerkOptionData::toggle()),
        Perks::CalculatedBalance => Some(PerkOptionData::toggle()),
        Perks::RavenousBeast => Some(PerkOptionData::toggle()),
        Perks::LordOfWolvesCatalyst => Some(PerkOptionData::passive()),
        Perks::ReleaseTheWolves => Some(PerkOptionData::toggle()),
        Perks::Fundamentals => Some(PerkOptionData::options(["Void", "Solar", "Arc"].to_vec())),
        Perks::ThinTheHerd => Some(PerkOptionData::toggle()),
        Perks::Chimera => Some(PerkOptionData::toggle()),
        Perks::FateOfAllFools => Some(PerkOptionData::stacking(3)),
        Perks::HonedEdge => Some(PerkOptionData::stacking_min(4, 1)),
        Perks::TakenPredator => Some(PerkOptionData::options(
            ["Taken", "Witherhoard", "Both"].to_vec(),
        )),
        Perks::MarkovChain => Some(PerkOptionData::stacking(5)),
        Perks::StormAndStress => Some(PerkOptionData::toggle()),
        Perks::DualSpeedReceiver => Some(PerkOptionData::toggle()),
        Perks::ExplosiveShadow => Some(PerkOptionData::passive()),
        Perks::SurosLegacy => Some(PerkOptionData::passive()),
        Perks::SpinningUp => Some(PerkOptionData::stacking(2)),
        Perks::DarkDescent => Some(PerkOptionData::toggle()),
        Perks::SleeperCatalyst => Some(PerkOptionData::passive()),
        Perks::TargetAquired => Some(PerkOptionData::toggle()),
        Perks::RatPack => Some(PerkOptionData::stacking_min(5, 1)),
        Perks::HuntersTrance => Some(PerkOptionData::passive()),
        Perks::RideTheBull => Some(PerkOptionData::stacking(2)),
        Perks::NobleRounds => Some(PerkOptionData::toggle()),

        Perks::DexterityMod => Some(PerkOptionData::stacking(3)),
        Perks::ReserveMod => Some(PerkOptionData::stacking(3)),
        Perks::LoaderMod => Some(PerkOptionData::stacking(3)),
        Perks::TargetingMod => Some(PerkOptionData::stacking(3)),
        Perks::UnflinchingMod => Some(PerkOptionData::stacking(3)),
        Perks::SurgeMod => Some(PerkOptionData::stacking(3)),
        Perks::OnYourMark => Some(PerkOptionData::stacking(3)),
        Perks::Frequency => Some(PerkOptionData::toggle()),
        Perks::Tempering => Some(PerkOptionData::toggle()),
        Perks::DragonShadow => Some(PerkOptionData::toggle()),
        Perks::OphidianAspect => Some(PerkOptionData::passive()),
        Perks::Hedrons => Some(PerkOptionData::toggle()),
        Perks::HeatRises => Some(PerkOptionData::toggle()),
        Perks::FlowState => Some(PerkOptionData::toggle()),
        Perks::TomeOfDawn => Some(PerkOptionData::toggle()),
        Perks::ThreadOfAscent => Some(PerkOptionData::toggle()),
        Perks::WellOfRadiance => Some(PerkOptionData::passive()),
        Perks::Amplified => Some(PerkOptionData::passive()),
        Perks::Radiant => Some(PerkOptionData::passive()),
        Perks::Weaken => Some(PerkOptionData::passive()),
        Perks::WardOfDawn => Some(PerkOptionData::passive()),
        Perks::BannerShield => Some(PerkOptionData::passive()),

        Perks::PathOfTheBurningSteps => Some(PerkOptionData::stacking(4)),
        Perks::MantleOfBattleHarmony => Some(PerkOptionData::passive()),
        Perks::MaskOfBakris => Some(PerkOptionData::options(["one buff", "both buffs"].to_vec())),
        Perks::BallindorseWrathweavers => Some(PerkOptionData::toggle()),
        Perks::LunaFaction => Some(PerkOptionData::options(
            ["Heal Rift", "Empowering Rift / Well"].to_vec(),
        )),
        Perks::Foetracer => Some(PerkOptionData::toggle()),
        Perks::MechaneersTricksleeves => Some(PerkOptionData::toggle()),
        Perks::Oathkeeper => Some(PerkOptionData::passive()),
        Perks::SealedAhamkaraGrasps => Some(PerkOptionData::toggle()),
        Perks::LuckyPants => Some(PerkOptionData::toggle()),
        Perks::Stompees => Some(PerkOptionData::passive()),
        Perks::NoBackupPlans => Some(PerkOptionData::passive()),
        Perks::ActiumWarRig => Some(PerkOptionData::passive()),
        Perks::HallowfireHeart => Some(PerkOptionData::passive()),
        Perks::LionRampart => Some(PerkOptionData::toggle()),
        Perks::Peacekeepers => Some(PerkOptionData::passive()),
        Perks::EyeOfAnotherWorld => Some(PerkOptionData::passive()),
        Perks::AstrocyteVerse => Some(PerkOptionData::passive()),
        Perks::NecroticGrips => Some(PerkOptionData::passive()),
        Perks::BootsOfTheAssembler => Some(PerkOptionData::passive()),
        Perks::RainOfFire => Some(PerkOptionData::passive()),
        Perks::SpeedloaderSlacks => Some(PerkOptionData::stacking(5)),
        Perks::PeregrineGreaves => Some(PerkOptionData::passive()),

        _ => None,
    }
}

pub fn enh_hash_to_perk_option_data(_hash: u32) -> Option<PerkOptionData> {
    let perk: Perks = enhanced_check(_hash).0.into();
    match perk {
        Perks::Recombination => Some(PerkOptionData::stacking(8)),
        Perks::ExplosiveLight => Some(PerkOptionData::stacking(7)),
        _ => hash_to_perk_option_data(_hash),
    }
}

pub fn get_perk_options(_perks: Vec<u32>) -> HashMap<u32, PerkOptionData> {
    let mut options = HashMap::new();
    for perk in _perks {
        // let data = if  _input._is_enhanced {enh_hash_to_perk_option_data(perk)} else {hash_to_perk_option_data(perk)};
        let data = hash_to_perk_option_data(perk);
        if data.is_some() {
            options.insert(perk, data.unwrap());
        }
    }
    options
}
