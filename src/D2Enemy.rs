

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnemyType {
    MINOR,
    ELITE,
    MINIBOSS,
    BOSS,
    VEHICLE,
    ENCLAVE,
    PLAYER,
    CHAMPION
}
impl Default for EnemyType {
    fn default() -> Self {
        EnemyType::ENCLAVE
    }
}

#[derive(Debug, Clone, Default)]
pub struct Enemy {
    health: f64,
    damage: f64,
    type_: EnemyType,
    tier: u8,
}
impl Enemy {
    pub fn set_health(&mut self, health: f64) {
        self.health = health;
    }
    pub fn set_damage(&mut self, damage: f64) {
        self.damage = damage;
    }
    pub fn set_type(&mut self, type_: EnemyType) {
        self.type_ = type_;
    }
    pub fn set_tier(&mut self, tier: u8) {
        self.tier = tier;
    }
    pub fn get_health(&self) -> f64 {
        self.health
    }
    pub fn get_damage(&self) -> f64 {
        self.damage
    }
    pub fn get_type(&self) -> EnemyType {
        self.type_
    }
    pub fn get_tier(&self) -> u8 {
        self.tier
    }
}