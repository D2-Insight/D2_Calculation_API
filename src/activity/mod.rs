use self::damage_calc::{DifficultyOptions, rpl_mult};

pub mod damage_calc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerClass {
    Titan,
    Hunter,
    Warlock,
    Unknown,
}
impl Default for PlayerClass {
    fn default() -> Self {
        PlayerClass::Unknown
    }
}

#[derive(Debug, Clone, Default)]
pub struct Player {
    pub pl: f64,
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
            player: Player{pl: expansion_base+200.0, class: PlayerClass::default()},
        }
    }
}
impl Activity {
    pub fn get_rpl_mult(&self) -> f64 {
        rpl_mult(self.rpl)
    }
    pub fn set_difficulty(&mut self, _difficulty: DifficultyOptions) {
        self.difficulty = _difficulty;
    }
    pub fn set_player_pl(&mut self, _pl: f64) {
        self.player.pl = _pl;
    }
    pub fn set_player_class(&mut self, _class: PlayerClass) {
        self.player.class = _class;
    }
}