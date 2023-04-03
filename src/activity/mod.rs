use self::damage_calc::{gpl_delta, rpl_mult, DifficultyOptions};

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
    pub rpl: u32,
    pub cap: i32,
    pub player: Player,
}
impl Default for Activity {
    fn default() -> Self {
        let expansion_base = 1350;
        Activity {
            name: "Default".to_string(),
            difficulty: DifficultyOptions::default(),
            rpl: expansion_base,
            cap: 100,
            player: Player {
                pl: expansion_base + 200,
                class: PlayerClass::default(),
            },
        }
    }
}
impl Activity {
    pub fn get_pl_delta(&self) -> f64 {
        let gpl = self.player.pl;
        gpl_delta(self.clone(), gpl)
    }
    pub fn get_rpl_mult(&self) -> f64 {
        rpl_mult(self.rpl as f64)
    }
}
