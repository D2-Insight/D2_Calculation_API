use serde::Serialize;

use super::{dps_simulation::DpsSimulationInstance, LoadoutDpsData};

#[derive(Debug, Clone, Copy, PartialEq, Hash, Serialize)]
pub enum WeaponSlot {
    Kinetic,
    Energy,
    Power,
}
impl From<u8> for WeaponSlot {
    fn from(slot: u8) -> Self {
        match slot {
            1 => WeaponSlot::Kinetic,
            2 => WeaponSlot::Energy,
            3 => WeaponSlot::Power,
            _ => panic!("Invalid weapon slot: {}", slot),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum AbilityTypes {
    Melee,
    UnchargedMelee,
    Grenade,
    Super,
    Class,
}
impl From<u8> for AbilityTypes {
    fn from(slot: u8) -> Self {
        match slot {
            1 => AbilityTypes::Melee,
            5 => AbilityTypes::UnchargedMelee,
            2 => AbilityTypes::Grenade,
            3 => AbilityTypes::Super,
            4 => AbilityTypes::Class,
            _ => panic!("Invalid ability slot: {}", slot),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DpsSettings {
    WhenCheckFirst(bool), //true if the when stack should be checked before the instruction|    default: false
    DpsOverTimeFormat, //the output data should be formatted for dps over time|                 default: enabled  }
    DamageOverTimeFormat, //the output data should be formatted for damage over time|           default: disabled }
    AdsTimeInSwap(f64), //% of the time spent in ads anim should be included in the swap time|  default: 0.0
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConditionSupplier {
    AbilityAvailable(AbilityTypes), //true if ability is available
    WeaponEmpty(WeaponSlot),        //true if weapon is empty
    WeaponBelow(WeaponSlot, f64),   //percentage of ammo remaining
    IsStowed(WeaponSlot),           //true if weapon is stowed
    TotalTimeHasPassed(f64),        //true if total time is greater than or equal to the given time

    //meta
    Inverse(Box<ConditionSupplier>),  //inverse of a condition
    All(Vec<Box<ConditionSupplier>>), //all conditions must be met
    Any(Vec<Box<ConditionSupplier>>), //any condition must be met
    None(Vec<Box<ConditionSupplier>>), //no conditions must be met
}
impl ConditionSupplier {
    //pass in a cloned instance of the simulation
    pub fn is_met(&self, state: &DpsSimulationInstance) -> bool {
        match self {
            ConditionSupplier::AbilityAvailable(ability) => match ability {
                AbilityTypes::Melee => {
                    if state.dps_data.melee.is_some() {
                        state.dps_data.melee.as_ref().unwrap().available
                    } else {
                        false
                    }
                }
                AbilityTypes::UnchargedMelee => true,
                AbilityTypes::Grenade => {
                    if state.dps_data.grenade.is_some() {
                        state.dps_data.grenade.as_ref().unwrap().available
                    } else {
                        false
                    }
                }
                AbilityTypes::Super => {
                    if state.dps_data._super.is_some() {
                        state.dps_data._super.as_ref().unwrap().available
                    } else {
                        false
                    }
                }
                AbilityTypes::Class => {
                    if state.dps_data.class.is_some() {
                        state.dps_data.class.as_ref().unwrap().available
                    } else {
                        false
                    }
                }
            },
            ConditionSupplier::WeaponEmpty(slot) => match slot {
                WeaponSlot::Kinetic => {
                    if state.dps_data.kinetic.is_some() {
                        state.dps_data.kinetic.as_ref().unwrap().magazine > 0.0
                    } else {
                        false
                    }
                }
                WeaponSlot::Energy => {
                    if state.dps_data.energy.is_some() {
                        state.dps_data.energy.as_ref().unwrap().magazine > 0.0
                    } else {
                        false
                    }
                }
                WeaponSlot::Power => {
                    if state.dps_data.power.is_some() {
                        state.dps_data.power.as_ref().unwrap().magazine > 0.0
                    } else {
                        false
                    }
                }
            },
            ConditionSupplier::WeaponBelow(slot, percent) => match slot {
                WeaponSlot::Kinetic => {
                    if state.dps_data.kinetic.is_some() {
                        let data = state.dps_data.kinetic.as_ref().unwrap();
                        data.magazine / data.base_magazine < *percent
                    } else {
                        false
                    }
                }
                WeaponSlot::Energy => {
                    if state.dps_data.energy.is_some() {
                        let data = state.dps_data.energy.as_ref().unwrap();
                        data.magazine / data.base_magazine < *percent
                    } else {
                        false
                    }
                }
                WeaponSlot::Power => {
                    if state.dps_data.power.is_some() {
                        let data = state.dps_data.power.as_ref().unwrap();
                        data.magazine / data.base_magazine < *percent
                    } else {
                        false
                    }
                }
            },
            ConditionSupplier::IsStowed(slot) => {
                return !(state.active_weapon == *slot);
            },
            ConditionSupplier::TotalTimeHasPassed(time) => state.total_time >= *time,
            ConditionSupplier::Inverse(condition) => !condition.is_met(state),
            ConditionSupplier::All(conditions) => {
                for condition in conditions.clone() {
                    if !condition.is_met(&state.clone()) {
                        return false;
                    }
                }
                true
            }
            ConditionSupplier::Any(conditions) => {
                for condition in conditions.clone() {
                    if condition.is_met(&state.clone()) {
                        return true;
                    }
                }
                false
            }
            ConditionSupplier::None(conditions) => {
                for condition in conditions.clone() {
                    if condition.is_met(&state.clone()) {
                        return false;
                    }
                }
                true
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionSet {
    //base actions
    SwapWeapon(WeaponSlot),   //swap to a weapon
    FireShots(u32),           //fire a number of shots
    UseAbility(AbilityTypes), //melee, grenade, super, class
    Reload(bool),             //true if short reload
    ReloadOverride(u32),      //override the reload with a trait
    GetKill(bool),            //get a kill, bool is true if shot is required

    //Waits
    WaitSeconds(f64), //will stop instruction execution for a number of simulated seconds
    WaitUntil(ConditionSupplier), //will stop instruction execution until a condition is met

    //meta instructions
    SubRoutine(Vec<Box<InstructionSet>>), //basically a function
    Loop(u16, Box<InstructionSet>),       //loop a number of times
    While(ConditionSupplier, Box<InstructionSet>), //loop until a condition is met
    If(ConditionSupplier, Box<InstructionSet>), //if a condition is met, schedule a set of instructions
    IfElse(
        //if a condition is met, schedule a set of instructions, otherwise schedule a different set of instructions
        ConditionSupplier,
        Box<InstructionSet>, // if true
        Box<InstructionSet>, // if false
    ),

    //async instructions
    When(ConditionSupplier, Box<InstructionSet>), //will put to an async stack and execute when a condition is met
    SetSetting(DpsSettings),                              //set a setting for the dps simulation
}
