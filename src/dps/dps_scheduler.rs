use crate::logging::{extern_log, LogLevel};

use super::{dps_simulation::*, instruction_set::*};

pub struct DpsScheduler<'a> {
    instance: &'a mut DpsSimulationInstance,
    when_stack: Vec<InstructionSet>,
    when_first: bool,
    time_passed: f64,
    output_format: DpsSettings,
}
impl<'a> DpsScheduler<'a> {
    pub fn new(instance: &'a mut DpsSimulationInstance) -> DpsScheduler<'a> {
        DpsScheduler {
            instance,
            when_stack: Vec::new(),
            when_first: true,
            time_passed: 0.0,
            output_format: DpsSettings::DpsOverTimeFormat,
        }
    }
    pub fn step(&mut self) {
        //pass
    }
    pub fn schedule(&mut self, instruction: InstructionSet) {
        // match for each type of instruction
        match instruction {
            InstructionSet::When(_, _) => {
                self.when_stack.push(instruction);
            }
            InstructionSet::If(cond, instruction) => {
                if cond.is_met(self.instance) {
                    self.schedule(*instruction);
                }
            }
            InstructionSet::SubRoutine(instruction_vec) => {
                for instruct in instruction_vec {
                    self.schedule(*instruct);
                }
            }
            InstructionSet::Loop(count, instruction) => {
                for _ in 0..count {
                    self.schedule((*instruction).clone());
                }
            }
            InstructionSet::IfElse(cond, instruction_true, instruction_false) => {
                if cond.is_met(self.instance) {
                    self.schedule(*instruction_true);
                } else {
                    self.schedule(*instruction_false);
                }
            }
            InstructionSet::While(cond, instruction) => {
                let mut count = 0;
                while cond.is_met(self.instance) {
                    self.schedule((*instruction).clone());
                    count += 1;
                    if count > 1000 {
                        extern_log("While loop exceeded 1000 iterations", LogLevel::Error);
                        break;
                    }
                }
            }
            InstructionSet::SwapWeapon(slot) => {
                self.instance.swap_weapon(slot);
            }
            InstructionSet::FireShots(count) => {
                for _ in 0..count {
                    self.instance.fire();
                }
            }
            InstructionSet::UseAbility(ability) => {
                if ability == AbilityTypes::Grenade {
                    self.instance.use_grenade();
                } else if ability == AbilityTypes::Melee {
                    self.instance.use_melee();
                } else if ability == AbilityTypes::Super {
                    self.instance.use_super();
                } else if ability == AbilityTypes::Class {
                    self.instance.use_class_ability();
                } else if ability == AbilityTypes::UnchargedMelee {
                    self.instance.use_uncharged_melee();
                }
            }
            InstructionSet::Reload(short_reload) => {
                if short_reload {
                    self.instance.reload_cancel();
                } else {
                    self.instance.reload();
                }
            }
            InstructionSet::WaitSeconds(time) => {
                self.time_passed += time;
            }

            _ => {
                //pass
            }
        }
    }
    fn set_setting(&mut self, setting: DpsSettings) {
        match setting {
            DpsSettings::AdsTimeInSwap(ads_time_in_swap) => {
                self.instance.ads_time_in_swap = ads_time_in_swap;
            }
            DpsSettings::WhenCheckFirst(when_first) => {
                self.when_first = when_first;
            }
            DpsSettings::DamageOverTimeFormat => {
                self.output_format = DpsSettings::DamageOverTimeFormat;
            }
            DpsSettings::DpsOverTimeFormat => {
                self.output_format = DpsSettings::DpsOverTimeFormat;
            }
        }
    }
}
