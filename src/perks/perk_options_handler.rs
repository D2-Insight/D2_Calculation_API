use std::collections::HashMap;

use serde::Serialize;

use super::{Perk, Perks};

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

#[derive(Debug, Clone, Serialize, Default)]
pub struct PerkOptionData(u32, Vec<String>, PerkValueVariant);
impl PerkOptionData {
    pub fn toggle() -> PerkOptionData {
        PerkOptionData(1, vec![], PerkValueVariant::TOGGLE)
    }
    pub fn stacking(_stacks: u32) -> PerkOptionData {
        PerkOptionData(_stacks, vec![], PerkValueVariant::SLIDER)
    }
}

fn hash_to_perk_option_data(_hash: u32) -> Option<PerkOptionData> {
    let perk: Perks = _hash.into();
    match perk {
        //Meta perks
        Perks::BuiltIn => None,
        Perks::EmpowermentBuffs => None,
        Perks::WeakenDebuffs => None,

        //intrinsics
        Perks::RapidFireFrame => Some(PerkOptionData::default()),

        //armor

        //parts
        Perks::ImpactCasing => Some(PerkOptionData::default()),
        Perks::SwapMag => Some(PerkOptionData::default()),
        Perks::FullChoke => Some(PerkOptionData::default()),
        Perks::SpikeGrenades => Some(PerkOptionData::default()),
        Perks::AlloyMag => Some(PerkOptionData::default()),

        //mods
        Perks::QuickAccessSling => Some(PerkOptionData::default()),
        Perks::BossSpec => Some(PerkOptionData::default()),
        Perks::MajorSpec => Some(PerkOptionData::default()),
        Perks::MinorSpec => Some(PerkOptionData::default()),
        Perks::BigOnesSpec => Some(PerkOptionData::default()),
        Perks::TakenSpec => Some(PerkOptionData::default()),

        //origin | year 5+
        Perks::VeistStinger => Some(PerkOptionData::default()),
        Perks::HakkeBreach => Some(PerkOptionData::default()),
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
        Perks::SearchParty => Some(PerkOptionData::default()),

        //season 1 | year 1
        Perks::KillClip => Some(PerkOptionData::toggle()),
        Perks::Outlaw => Some(PerkOptionData::toggle()),
        Perks::BackupPlan => Some(PerkOptionData::toggle()),
        Perks::FieldPrep => Some(PerkOptionData::toggle()),
        Perks::Rampage => Some(PerkOptionData::stacking(3)),
        Perks::OpeningShot => Some(PerkOptionData::toggle()),
        Perks::MovingTarget => Some(PerkOptionData::default()),
        Perks::AmbitiousAssassin => Some(PerkOptionData::toggle()),
        Perks::ClusterBomb => Some(PerkOptionData::default()),
        Perks::ExplosivePayload => Some(PerkOptionData::default()),
        Perks::FirmlyPlanted => Some(PerkOptionData::toggle()),
        Perks::FullAutoTrigger => Some(PerkOptionData::default()),
        Perks::HeadSeeker => Some(PerkOptionData::default()),
        Perks::HighImpactReserves => Some(PerkOptionData::default()),
        Perks::HipFireGrip => Some(PerkOptionData::toggle()),
        Perks::Snapshot => Some(PerkOptionData::default()),
        Perks::TapTheTrigger => Some(PerkOptionData::toggle()),
        Perks::SlideWays => Some(PerkOptionData::toggle()),
        Perks::QuickDraw => Some(PerkOptionData::default()),
        Perks::TimedPayload => Some(PerkOptionData::default()),
        Perks::ThreatDetector => Some(PerkOptionData::stacking(2)),
        Perks::SlideShot => Some(PerkOptionData::toggle()),
        Perks::TripleTap => Some(PerkOptionData::default()),
        Perks::UnderPressure => Some(PerkOptionData::toggle()),

        //season 2 | year 1
        //lmao bozo

        //season 3 | year 1
        Perks::RangeFinder => Some(PerkOptionData::default()),
        Perks::DisruptionBreak => Some(PerkOptionData::toggle()),
        Perks::TrenchBarrel => Some(PerkOptionData::toggle()),
        Perks::Desperado => Some(PerkOptionData::toggle()),
        Perks::BoxBreathing => Some(PerkOptionData::toggle()),

        //season 4 | year 2
        Perks::ArchersTempo => Some(PerkOptionData::toggle()),
        Perks::ExplosiveHead => Some(PerkOptionData::default()),
        Perks::FeedingFrenzy => Some(PerkOptionData::stacking(5)),
        Perks::FourthTimesTheCharm => Some(PerkOptionData::default()),
        Perks::RapidHit => Some(PerkOptionData::stacking(5)),

        //season 5 | year 2
        Perks::ResevoirBurst => Some(PerkOptionData::toggle()),
        Perks::Surrounded => Some(PerkOptionData::toggle()),
        Perks::AirAssault => Some(PerkOptionData::stacking(2)),

        //season 6 | year 2
        Perks::FiringLine => Some(PerkOptionData::toggle()),
        Perks::FullCourt => Some(PerkOptionData::toggle()),
        Perks::KillingTally => Some(PerkOptionData::stacking(3)),
        Perks::Demolitionist => Some(PerkOptionData(
            2,
            [
                "Disable".to_owned(),
                "Once".to_owned(),
                "every 3s".to_owned(),
            ]
            .to_vec(),
            PerkValueVariant::OPTIONS,
        )),
        Perks::MultikillClip => Some(PerkOptionData::stacking(3)),
        Perks::Swashbuckler => Some(PerkOptionData::stacking(5)),
        Perks::OverFlow => Some(PerkOptionData::toggle()),

        //season 7 | year 2
        Perks::UnderDog => Some(PerkOptionData::toggle()),
        Perks::ExplosiveLight => Some(PerkOptionData::stacking(6)),

        //season 8 | year 3
        //TODO

        //season 9 | year 3
        Perks::ClownCartridge => Some(PerkOptionData::toggle()),
        Perks::ElementalCapacitor => Some(PerkOptionData(
            4,
            [
                "None".to_owned(),
                "Void".to_owned(),
                "Solar".to_owned(),
                "Arc".to_owned(),
                "Stasis".to_owned(),
            ]
            .to_vec(),
            PerkValueVariant::OPTIONS,
        )),
        Perks::Vorpal => Some(PerkOptionData::default()),

        //season 10 | year 3
        //bad season lmao

        //season 11 | year 3
        Perks::KillingWind => Some(PerkOptionData::toggle()),

        //season 12 | year 4
        Perks::DualLoader => Some(PerkOptionData::toggle()),
        Perks::OneForAll => Some(PerkOptionData::toggle()),
        Perks::Recombination => Some(PerkOptionData::stacking(10)),
        Perks::Reconstruction => Some(PerkOptionData::toggle()),
        Perks::Surplus => Some(PerkOptionData::stacking(3)),

        //season 13 | year 4
        Perks::ImpulseAmplifier => Some(PerkOptionData::default()),
        Perks::Frenzy => Some(PerkOptionData::toggle()),
        Perks::LastingImpression => Some(PerkOptionData::default()),

        //season 14 | year 4
        Perks::Cornered => Some(PerkOptionData::toggle()),
        Perks::AdrenalineJunkie => Some(PerkOptionData::stacking(5)),
        Perks::RewindRounds => Some(PerkOptionData::default()),
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
        Perks::ChillClip => Some(PerkOptionData::default()),
        Perks::SleightOfHand => Some(PerkOptionData::stacking(3)),
        Perks::StatsForAll => Some(PerkOptionData::toggle()),
        Perks::SteadyHands => Some(PerkOptionData::toggle()),
        Perks::SuccesfulWarmup => Some(PerkOptionData::toggle()),
        Perks::UnstoppableForce => Some(PerkOptionData::toggle()),

        //season 17 | year 5
        Perks::FragileFocus => Some(PerkOptionData::toggle()),
        Perks::WellRounded => Some(PerkOptionData::stacking(2)),

        //season 18 | year 5
        Perks::GutShot => Some(PerkOptionData::default()),
        Perks::Pugilist => Some(PerkOptionData::toggle()),
        Perks::Slickdraw => Some(PerkOptionData::default()),
        Perks::OverUnder => Some(PerkOptionData::default()),

        //season 19 | year 5
        Perks::CascadePoint => Some(PerkOptionData::toggle()),
        Perks::CloseToMelee => Some(PerkOptionData::toggle()),
        Perks::OffhandStrike => Some(PerkOptionData::toggle()),
        Perks::PerfectFloat => Some(PerkOptionData::toggle()),
        Perks::ShotSwap => Some(PerkOptionData::stacking(8)),
        Perks::TargetLock => Some(PerkOptionData::default()),
        // Perks::DexterityMod,
        // Perks::ReloadMod,
        // Perks::ReserveMod,
        // Perks::LoaderMod,
        // Perks::TargetingMod,
        // Perks::QuickCharge,
        // Perks::CranialSpikeCat,
        // Perks::AgersCall,
        // Perks::LagragianSight,
        // Perks::RatPack,
        // Perks::StringofCurses,
        // Perks::WormsHunger,
        // Perks::RocketTracers,
        // Perks::ParacausalShot,
        // Perks::CorruptionSpreads,
        // Perks::TimeSlip,
        // Perks::ToM,
        // Perks::IgnitionTrigger,
        // Perks::GuidanceRing,
        // Perks::ConserveMomentum,
        // Perks::Impetus,
        // Perks::LooksCanKill,
        // Perks::PerfectFith,
        // Perks::Broadside,
        // Perks::Stormbringer,
        // Perks::PrismaticInferno,
        // Perks::ReignHavoc,
        // Perks::WhisperCatalyst,
        // Perks::OnYourMark,
        // Perks::Frequency,
        // Perks::Tempering,
        // Perks::DragonShadow,
        // Perks::OphidianAspect,
        // Perks::Hedrons,
        // Perks::HeatRises,
        // Perks::FlowState,
        _ => None,
    }
}

pub fn get_perk_options(_perks: Vec<u32>) -> HashMap<u32, PerkOptionData> {
    let mut options = HashMap::new();
    for perk in _perks {
        let data = hash_to_perk_option_data(perk);
        if data.is_some() {
            options.insert(perk, data.unwrap());
        }
    }
    options
}