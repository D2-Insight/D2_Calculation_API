use super::{Loadout, LoadoutDpsData, instruction_set::InstructionSet};



#[derive(Debug, Clone)]
pub struct DamageEvent(f64, f64);
impl DamageEvent {
    pub fn new(time: f64, damage: f64) -> DamageEvent {
        DamageEvent(time, damage)
    }
    pub fn time(&self) -> f64{self.0}
    pub fn damage(&self) -> f64{self.1}
}

#[derive(Debug, Clone)]
pub struct DpsSimulationInstance {
    pub loadout: Loadout,
    pub dps_data: LoadoutDpsData,
    pub active_weapon: u8,
    pub total_time: f64,
    pub time_damage_data: Vec<(f64, f64)>,
    //% of ads time to add to swap time
    pub ads_time_in_swap: f64,
    pub infer_reload_overrides: bool,
}
impl DpsSimulationInstance {
    pub fn new(loadout: Loadout, active_weapon: u8) -> DpsSimulationInstance {
        DpsSimulationInstance {
            loadout,
            dps_data: LoadoutDpsData::new(),
            active_weapon,
            total_time: 0.0,
            time_damage_data: Vec::new(),
            ads_time_in_swap: 0.0,
            infer_reload_overrides: false,
        }
    }
    pub fn fire(&mut self) -> Option<InstructionSet> {
        //pass
        None
    }
    pub fn swap_weapon(&mut self, slot: u8) -> Option<InstructionSet> {
        self.active_weapon = slot;
        None
    }
    pub fn use_granade(&mut self) -> Option<InstructionSet> {
        if !self.loadout.has_grenade_ability() { return None; }
        None
    }
    pub fn use_super(&mut self) -> Option<InstructionSet> {
        if !self.loadout.has_super_ability() { return None; }
        None
    }
    pub fn use_melee(&mut self) -> Option<InstructionSet> {
        if !self.loadout.has_melee_ability() { return None; }
        None
    }
    pub fn use_uncharged_melee(&mut self) -> Option<InstructionSet> {
        None
    }
    pub fn use_class_ability(&mut self) -> Option<InstructionSet> {
        if !self.loadout.has_class_ability() { return None; }
        None
    }
    pub fn reload(&mut self) -> Option<InstructionSet> {
        None
    }
    pub fn reload_cancel(&mut self) -> Option<InstructionSet> {
        None
    }
}