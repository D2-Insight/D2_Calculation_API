use crate::activity::Activity;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum EnemyType {
    MINOR,
    ELITE,
    MINIBOSS,
    BOSS,
    VEHICLE,
    ENCLAVE,
    PLAYER,
    CHAMPION,
}
impl Default for EnemyType {
    fn default() -> Self {
        EnemyType::ENCLAVE
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Enemy {
    pub health: f64,
    pub damage: f64,
    pub damage_resistance: f64,
    pub type_: EnemyType,
    pub tier: u8,
}
impl Enemy {
    pub fn get_adjusted_health(&self, _activity: Activity) -> f64 {
        self.health * (1.0 - self.damage_resistance)
    }
}
