use self::damage_calc::{DifficultyOptions, rpl_mult};

pub mod damage_calc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerClass {
    Unknown = 0,
    Titan = 1,
    Hunter = 2,
    Warlock = 3,
}
impl Default for PlayerClass {
    fn default() -> Self {
        PlayerClass::Unknown
    }
}

#[derive(Debug, Clone, Default)]
pub struct Player {
    pub pl: u32,
    pub class: PlayerClass,
}

#[derive(Debug, Clone)]
pub struct Activity {
    pub name: String,
    pub difficulty: DifficultyOptions,
    pub rpl: f64,
    pub cap: f64,
    pub player: Player,
}
impl Default for Activity {
    fn default() -> Self {
        let expansion_base = 1350.0;
        Activity {
            name: "Default".to_string(),
            difficulty: DifficultyOptions::default(),
            rpl: expansion_base,
            cap: 100.0,
            player: Player{pl: expansion_base as u32 +200, class: PlayerClass::default()},
        }
    }
}