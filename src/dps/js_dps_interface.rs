#![cfg(feature = "wasm")]

use std::cell::RefCell;
use std::rc::Rc;
use std::{clone, collections::HashMap};

use dyn_clone::{clone_trait_object, DynClone};
use wasm_bindgen::prelude::*;

use super::instruction_set::{ConditionSupplier, InstructionSet};

thread_local! {
    static RUNTIME: RefCell<ScriptRuntime> = RefCell::new(ScriptRuntime::new());
}

pub trait Instruction: DynClone {
    fn get_instruction(&self) -> InstructionSet;
}
clone_trait_object!(Instruction);

pub trait Condition: DynClone {
    fn get_condition(&self) -> ConditionSupplier;
}
clone_trait_object!(Condition);

#[derive(Default, Clone)]
pub struct ScriptRuntime {
    pub instructions: Vec<Box<dyn Instruction>>,
    pub conditions: Vec<Box<dyn Condition>>,
    pub abstract_script: Vec<usize>,
}
impl ScriptRuntime {
    pub fn new() -> ScriptRuntime {
        ScriptRuntime {
            instructions: Vec::new(),
            conditions: Vec::new(),
            abstract_script: Vec::new(),
        }
    }
    pub fn add_instruction(&mut self, instruction: Box<dyn Instruction>) -> usize {
        self.instructions.push(instruction);
        self.instructions.len() - 1
    }
    pub fn add_condition(&mut self, condition: Box<dyn Condition>) -> usize {
        self.conditions.push(condition);
        self.conditions.len() - 1
    }
    pub fn get_instructions(&self, ptr: usize) -> InstructionSet {
        self.instructions[ptr].get_instruction()
    }
    pub fn get_condition(&self, ptr: usize) -> ConditionSupplier {
        self.conditions[ptr].get_condition()
    }
    pub fn next(&mut self, ptr: usize) {
        if ptr > self.conditions.len() - 1 {
            return;
        }
        self.abstract_script.push(ptr);
    }
    pub fn reset(&mut self) {
        self.instructions = Vec::new();
        self.conditions = Vec::new();
        self.abstract_script = Vec::new();
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct FireInstuction {
    #[wasm_bindgen(skip)]
    pub count: u32,
}
impl Instruction for FireInstuction {
    fn get_instruction(&self) -> InstructionSet {
        InstructionSet::FireShots(self.count)
    }
}
#[wasm_bindgen]
impl FireInstuction {
    #[wasm_bindgen(constructor)]
    pub fn new(count: u32) -> usize {
        RUNTIME.with(|runtime| {
            let ptr = runtime
                .borrow_mut()
                .add_instruction(Box::new(FireInstuction { count }));
            ptr
        })
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct SwapWeaponInstruction {
    #[wasm_bindgen(skip)]
    pub slot: u8,
}
impl Instruction for SwapWeaponInstruction {
    fn get_instruction(&self) -> InstructionSet {
        InstructionSet::SwapWeapon(self.slot.into())
    }
}
#[wasm_bindgen]
impl SwapWeaponInstruction {
    #[wasm_bindgen(constructor)]
    pub fn new(slot: u8) -> usize {
        RUNTIME.with(|runtime| {
            let ptr = runtime
                .borrow_mut()
                .add_instruction(Box::new(SwapWeaponInstruction { slot }));
            ptr
        })
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct UseAbilityInstruction {
    #[wasm_bindgen(skip)]
    pub slot: u8,
}
impl Instruction for UseAbilityInstruction {
    fn get_instruction(&self) -> InstructionSet {
        InstructionSet::UseAbility(self.slot.into())
    }
}
#[wasm_bindgen]
impl UseAbilityInstruction {
    #[wasm_bindgen(constructor)]
    pub fn new(slot: u8) -> usize {
        RUNTIME.with(|runtime| {
            let ptr = runtime
                .borrow_mut()
                .add_instruction(Box::new(UseAbilityInstruction { slot }));
            ptr
        })
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct ReloadInstruction {
    #[wasm_bindgen(skip)]
    pub short: bool,
}
impl Instruction for ReloadInstruction {
    fn get_instruction(&self) -> InstructionSet {
        InstructionSet::Reload(self.short)
    }
}
#[wasm_bindgen]
impl ReloadInstruction {
    #[wasm_bindgen(constructor)]
    pub fn new(short: bool) -> usize {
        RUNTIME.with(|runtime| {
            let ptr = runtime
                .borrow_mut()
                .add_instruction(Box::new(ReloadInstruction { short }));
            ptr
        })
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct ReloadOverrideInstruction {
    #[wasm_bindgen(skip)]
    pub x: u32,
}
impl Instruction for ReloadOverrideInstruction {
    fn get_instruction(&self) -> InstructionSet {
        InstructionSet::ReloadOverride(self.x)
    }
}
#[wasm_bindgen]
impl ReloadOverrideInstruction {
    #[wasm_bindgen(constructor)]
    pub fn new(x: u32) -> usize {
        RUNTIME.with(|runtime| {
            let ptr = runtime
                .borrow_mut()
                .add_instruction(Box::new(ReloadOverrideInstruction { x }));
            ptr
        })
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct GetKillInstruction {
    #[wasm_bindgen(skip)]
    pub require_shot: bool,
}
impl Instruction for GetKillInstruction {
    fn get_instruction(&self) -> InstructionSet {
        InstructionSet::GetKill(self.require_shot)
    }
}
#[wasm_bindgen]
impl GetKillInstruction {
    #[wasm_bindgen(constructor)]
    pub fn new(require_shot: bool) -> usize {
        RUNTIME.with(|runtime| {
            let ptr = runtime
                .borrow_mut()
                .add_instruction(Box::new(GetKillInstruction { require_shot }));
            ptr
        })
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct WaitSecondsInstruction {
    #[wasm_bindgen(skip)]
    pub seconds: f64,
}
impl Instruction for WaitSecondsInstruction {
    fn get_instruction(&self) -> InstructionSet {
        InstructionSet::WaitSeconds(self.seconds)
    }
}
#[wasm_bindgen]
impl WaitSecondsInstruction {
    #[wasm_bindgen(constructor)]
    pub fn new(seconds: f64) -> usize {
        RUNTIME.with(|runtime| {
            let ptr = runtime
                .borrow_mut()
                .add_instruction(Box::new(WaitSecondsInstruction { seconds }));
            ptr
        })
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct WaitUntilInstruction {
    #[wasm_bindgen(skip)]
    pub condition_ptr: usize,
}
impl Instruction for WaitUntilInstruction {
    fn get_instruction(&self) -> InstructionSet {
        RUNTIME.with(|runtime| {
            InstructionSet::WaitUntil(runtime.borrow().get_condition(self.condition_ptr))
        })
    }
}
#[wasm_bindgen]
impl WaitUntilInstruction {
    #[wasm_bindgen(constructor)]
    pub fn new(condition_ptr: usize) -> usize {
        RUNTIME.with(|runtime| {
            let ptr = runtime
                .borrow_mut()
                .add_instruction(Box::new(WaitUntilInstruction { condition_ptr }));
            ptr
        })
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct SubRoutineInstruction {
    #[wasm_bindgen(skip)]
    pub instruction_ptrs: Vec<usize>,
}
impl Instruction for SubRoutineInstruction {
    fn get_instruction(&self) -> InstructionSet {
        let mut instructions = Vec::new();
        RUNTIME.with(|runtime| {
            for ptr in self.instruction_ptrs.iter() {
                instructions.push(Box::new(runtime.borrow().get_instructions(*ptr)));
            }
        });
        InstructionSet::SubRoutine(instructions)
    }
}
#[wasm_bindgen]
impl SubRoutineInstruction {
    #[wasm_bindgen(constructor)]
    pub fn new(instruction_ptrs: Vec<usize>) -> usize {
        RUNTIME.with(|runtime| {
            let ptr = runtime
                .borrow_mut()
                .add_instruction(Box::new(SubRoutineInstruction { instruction_ptrs }));
            ptr
        })
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct LoopInstruction {
    #[wasm_bindgen(skip)]
    pub instruction_ptr: usize,
    #[wasm_bindgen(skip)]
    pub count: u16,
}
impl Instruction for LoopInstruction {
    fn get_instruction(&self) -> InstructionSet {
        RUNTIME.with(|runtime| {
            InstructionSet::Loop(
                self.count,
                Box::new(runtime.borrow().get_instructions(self.instruction_ptr)),
            )
        })
    }
}
#[wasm_bindgen]
impl LoopInstruction {
    #[wasm_bindgen(constructor)]
    pub fn new(instruction_ptr: usize, count: u16) -> usize {
        RUNTIME.with(|runtime| {
            let ptr = runtime
                .borrow_mut()
                .add_instruction(Box::new(LoopInstruction {
                    instruction_ptr,
                    count,
                }));
            ptr
        })
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct WhileInstruction {
    #[wasm_bindgen(skip)]
    pub instruction_ptr: usize,
    #[wasm_bindgen(skip)]
    pub condition_ptr: usize,
}
impl Instruction for WhileInstruction {
    fn get_instruction(&self) -> InstructionSet {
        RUNTIME.with(|runtime| {
            InstructionSet::While(
                runtime.borrow().get_condition(self.condition_ptr),
                Box::new(runtime.borrow().get_instructions(self.instruction_ptr)),
            )
        })
    }
}
#[wasm_bindgen]
impl WhileInstruction {
    #[wasm_bindgen(constructor)]
    pub fn new(instruction_ptr: usize, condition_ptr: usize) -> usize {
        RUNTIME.with(|runtime| {
            let ptr = runtime
                .borrow_mut()
                .add_instruction(Box::new(WhileInstruction {
                    instruction_ptr,
                    condition_ptr,
                }));
            ptr
        })
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct IfInstruction {
    #[wasm_bindgen(skip)]
    pub condition_ptr: usize,
    #[wasm_bindgen(skip)]
    pub instruction_ptr: usize,
}
impl Instruction for IfInstruction {
    fn get_instruction(&self) -> InstructionSet {
        RUNTIME.with(|runtime| {
            InstructionSet::If(
                runtime.borrow().get_condition(self.condition_ptr),
                Box::new(runtime.borrow().get_instructions(self.instruction_ptr)),
            )
        })
    }
}
#[wasm_bindgen]
impl IfInstruction {
    #[wasm_bindgen(constructor)]
    pub fn new(condition_ptr: usize, instruction_ptr: usize) -> usize {
        RUNTIME.with(|runtime| {
            let ptr = runtime
                .borrow_mut()
                .add_instruction(Box::new(IfInstruction {
                    condition_ptr,
                    instruction_ptr,
                }));
            ptr
        })
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct IfElseInstruction {
    #[wasm_bindgen(skip)]
    pub condition_ptr: usize,
    #[wasm_bindgen(skip)]
    pub instruction_ptr: usize,
    #[wasm_bindgen(skip)]
    pub else_instruction_ptr: usize,
}
impl Instruction for IfElseInstruction {
    fn get_instruction(&self) -> InstructionSet {
        RUNTIME.with(|runtime| {
            InstructionSet::IfElse(
                runtime.borrow().get_condition(self.condition_ptr),
                Box::new(runtime.borrow().get_instructions(self.instruction_ptr)),
                Box::new(runtime.borrow().get_instructions(self.else_instruction_ptr)),
            )
        })
    }
}
#[wasm_bindgen]
impl IfElseInstruction {
    #[wasm_bindgen(constructor)]
    pub fn new(condition_ptr: usize, instruction_ptr: usize, else_instruction_ptr: usize) -> usize {
        RUNTIME.with(|runtime| {
            let ptr = runtime
                .borrow_mut()
                .add_instruction(Box::new(IfElseInstruction {
                    condition_ptr,
                    instruction_ptr,
                    else_instruction_ptr,
                }));
            ptr
        })
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct WhenInstruction {
    #[wasm_bindgen(skip)]
    pub condition_ptr: usize,
    #[wasm_bindgen(skip)]
    pub instruction_ptr: usize,
}
impl Instruction for WhenInstruction {
    fn get_instruction(&self) -> InstructionSet {
        RUNTIME.with(|runtime| {
            InstructionSet::When(
                runtime.borrow().get_condition(self.condition_ptr),
                Box::new(runtime.borrow().get_instructions(self.instruction_ptr)),
            )
        })
    }
}
#[wasm_bindgen]
impl WhenInstruction {
    #[wasm_bindgen(constructor)]
    pub fn new(condition_ptr: usize, instruction_ptr: usize) -> usize {
        RUNTIME.with(|runtime| {
            let ptr = runtime
                .borrow_mut()
                .add_instruction(Box::new(WhenInstruction {
                    condition_ptr,
                    instruction_ptr,
                }));
            ptr
        })
    }
}

//conditions
#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct AbilityAvailableCondition {
    #[wasm_bindgen(skip)]
    pub ability: u8,
}
impl Condition for AbilityAvailableCondition {
    fn get_condition(&self) -> ConditionSupplier {
        ConditionSupplier::AbilityAvailable(self.ability.into())
    }
}
#[wasm_bindgen]
impl AbilityAvailableCondition {
    #[wasm_bindgen(constructor)]
    pub fn new(ability: u8) -> usize {
        RUNTIME.with(|runtime| {
            let ptr = runtime
                .borrow_mut()
                .add_condition(Box::new(AbilityAvailableCondition { ability }));
            ptr
        })
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct WeaponEmptyCondition {
    #[wasm_bindgen(skip)]
    pub slot: u8,
}
impl Condition for WeaponEmptyCondition {
    fn get_condition(&self) -> ConditionSupplier {
        ConditionSupplier::WeaponEmpty(self.slot.into())
    }
}
#[wasm_bindgen]
impl WeaponEmptyCondition {
    #[wasm_bindgen(constructor)]
    pub fn new(slot: u8) -> usize {
        RUNTIME.with(|runtime| {
            let ptr = runtime
                .borrow_mut()
                .add_condition(Box::new(WeaponEmptyCondition { slot }));
            ptr
        })
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct WeaponBelowCondition {
    #[wasm_bindgen(skip)]
    pub slot: u8,
    #[wasm_bindgen(skip)]
    pub mag_percent: f64,
}
impl Condition for WeaponBelowCondition {
    fn get_condition(&self) -> ConditionSupplier {
        ConditionSupplier::WeaponBelow(self.slot.into(), self.mag_percent)
    }
}
#[wasm_bindgen]
impl WeaponBelowCondition {
    #[wasm_bindgen(constructor)]
    pub fn new(slot: u8, mag_percent: f64) -> usize {
        RUNTIME.with(|runtime| {
            let ptr = runtime
                .borrow_mut()
                .add_condition(Box::new(WeaponBelowCondition { slot, mag_percent }));
            ptr
        })
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct IsStowedCondition {
    #[wasm_bindgen(skip)]
    pub slot: u8,
}
impl Condition for IsStowedCondition {
    fn get_condition(&self) -> ConditionSupplier {
        ConditionSupplier::IsStowed(self.slot.into())
    }
}
#[wasm_bindgen]
impl IsStowedCondition {
    #[wasm_bindgen(constructor)]
    pub fn new(slot: u8) -> usize {
        RUNTIME.with(|runtime| {
            let ptr = runtime
                .borrow_mut()
                .add_condition(Box::new(IsStowedCondition { slot }));
            ptr
        })
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct TotalTimeHasPassed {
    #[wasm_bindgen(skip)]
    pub time: f64,
}
impl Condition for TotalTimeHasPassed {
    fn get_condition(&self) -> ConditionSupplier {
        ConditionSupplier::TotalTimeHasPassed(self.time)
    }
}
#[wasm_bindgen]
impl TotalTimeHasPassed {
    #[wasm_bindgen(constructor)]
    pub fn new(time: f64) -> usize {
        RUNTIME.with(|runtime| {
            let ptr = runtime
                .borrow_mut()
                .add_condition(Box::new(TotalTimeHasPassed { time }));
            ptr
        })
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct InverseCondition {
    #[wasm_bindgen(skip)]
    pub condition_ptr: usize,
}
impl Condition for InverseCondition {
    fn get_condition(&self) -> ConditionSupplier {
        RUNTIME.with(|runtime| {
            ConditionSupplier::Inverse(Box::new(runtime.borrow().get_condition(self.condition_ptr)))
        })
    }
}
#[wasm_bindgen]
impl InverseCondition {
    #[wasm_bindgen(constructor)]
    pub fn new(condition_ptr: usize) -> usize {
        RUNTIME.with(|runtime| {
            let ptr = runtime
                .borrow_mut()
                .add_condition(Box::new(InverseCondition { condition_ptr }));
            ptr
        })
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct AllCondition {
    #[wasm_bindgen(skip)]
    pub condition_ptrs: Vec<usize>,
}
impl Condition for AllCondition {
    fn get_condition(&self) -> ConditionSupplier {
        RUNTIME.with(|runtime| {
            let mut conditions = Vec::new();
            for ptr in self.condition_ptrs.iter() {
                conditions.push(Box::new(runtime.borrow().get_condition(*ptr)));
            }
            ConditionSupplier::All(conditions)
        })
    }
}
#[wasm_bindgen]
impl AllCondition {
    #[wasm_bindgen(constructor)]
    pub fn new(condition_ptrs: Vec<usize>) -> usize {
        RUNTIME.with(|runtime| {
            let ptr = runtime
                .borrow_mut()
                .add_condition(Box::new(AllCondition { condition_ptrs }));
            ptr
        })
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct AnyCondition {
    #[wasm_bindgen(skip)]
    pub condition_ptrs: Vec<usize>,
}
impl Condition for AnyCondition {
    fn get_condition(&self) -> ConditionSupplier {
        RUNTIME.with(|runtime| {
            let mut conditions = Vec::new();
            for ptr in self.condition_ptrs.iter() {
                conditions.push(Box::new(runtime.borrow().get_condition(*ptr)));
            }
            ConditionSupplier::Any(conditions)
        })
    }
}
#[wasm_bindgen]
impl AnyCondition {
    #[wasm_bindgen(constructor)]
    pub fn new(condition_ptrs: Vec<usize>) -> usize {
        RUNTIME.with(|runtime| {
            let ptr = runtime
                .borrow_mut()
                .add_condition(Box::new(AnyCondition { condition_ptrs }));
            ptr
        })
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct NoneCondition {
    #[wasm_bindgen(skip)]
    pub condition_ptrs: Vec<usize>,
}
impl Condition for NoneCondition {
    fn get_condition(&self) -> ConditionSupplier {
        RUNTIME.with(|runtime| {
            let mut conditions = Vec::new();
            for ptr in self.condition_ptrs.iter() {
                conditions.push(Box::new(runtime.borrow().get_condition(*ptr)));
            }
            ConditionSupplier::None(conditions)
        })
    }
}
#[wasm_bindgen]
impl NoneCondition {
    #[wasm_bindgen(constructor)]
    pub fn new(condition_ptrs: Vec<usize>) -> usize {
        RUNTIME.with(|runtime| {
            let ptr = runtime
                .borrow_mut()
                .add_condition(Box::new(NoneCondition { condition_ptrs }));
            ptr
        })
    }
}


#[wasm_bindgen]
pub fn start() {
    RUNTIME.with(|runtime| {
        runtime.borrow_mut().reset();
    });
}

pub fn next(instruction_ptr: usize) {
    RUNTIME.with(|runtime| {
        runtime.borrow_mut().next(instruction_ptr);
    });
}