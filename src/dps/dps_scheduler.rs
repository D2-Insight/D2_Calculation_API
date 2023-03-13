use super::{instruction_set::*, dps_simulation::*};

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
            time_passed: 0.0,
            when_first: true,
            output_format: DpsSettings::DpsOverTimeFormat,
        }
    }
    pub fn tick(&mut self) {
        //pass
        self.time_passed += 1.0 / 60.0;
    }
    pub fn schedule(&mut self, instruction: InstructionSet) {
        // match for each type of instruction
        match instruction {
            InstructionSet::When(_, _) => {
                self.when_stack.push(instruction);
            }
            _ => {
                //pass
            }
        }
    }
    pub fn is_done(&self) -> bool {
        true
    }
    fn set_setting(&mut self, setting: DpsSettings) {
        match setting {
            DpsSettings::AdsTimeInSwap(ads_time_in_swap) => {
                self.instance.ads_time_in_swap = ads_time_in_swap;
            }
            DpsSettings::InferReloadOverride(infer_reload_overrides) => {
                self.instance.infer_reload_overrides = infer_reload_overrides;
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
