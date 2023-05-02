pub mod attributes;
pub mod status_effects;
pub mod resource_pool;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttributeKey {
    //weapon stats
    Range,
    Stability,
    Handling,
    Reload,
    BurstDelay,
    BurstCount,
    InnerBurstDelay,
    Magazine,
    InventorySize,
    Zoom,
    BlastRadius,
    Velocity,
    Damage,
    CritMult,

    //armor stats
    Mobility,
    Resilience,
    Recovery,
    Intellect,
    Discipline,
    Strength,

    //Combatant scales
    Minor,
    Elite,
    Major,
    Champion,
    Boss,
    Vehicle,

    //None
    None,
}