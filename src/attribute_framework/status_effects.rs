use std::collections::HashMap;

use super::{AttributeKey, attributes::ModifierSet};

pub struct UpdateInput {
    pub time: f64,
    pub bullets: i32,
    pub reloads: i32,
    pub kills: i32,
    pub event: String
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatusEffectScope {
    Weapon,
    Global,
}

pub trait StatusEffect {
    fn is_active(&mut self, inp: UpdateInput) -> bool;

    fn scope(&self) -> StatusEffectScope;

    fn get_modifiers(&self, key: AttributeKey) -> ModifierSet;

    fn should_requery(&self) -> bool {
        false
    }
}


pub struct StackableStatusEffect {
    pub expiration: f64,
    pub stacks: i32,
    pub reproccable: bool,
    pub stack_cap: i32,
    pub scope: StatusEffectScope,
    pub modifiers: HashMap<AttributeKey, Box<dyn Fn(i32) -> ModifierSet>>,
}
impl StatusEffect for StackableStatusEffect {
    fn is_active(&mut self, inp: UpdateInput) -> bool {
        inp.time < self.expiration
    }

    fn scope(&self) -> StatusEffectScope {
        self.scope.clone()
    }

    fn get_modifiers(&self, key: AttributeKey) -> ModifierSet {
        if self.modifiers.contains_key(&key) {
            self.modifiers[&key](self.stacks)
        } else {
            ModifierSet::new()
        }
    }

    fn should_requery(&self) -> bool {
        self.reproccable
    }
}

pub struct UnendingStatusEffect {
    pub scope: StatusEffectScope,
    pub modifiers: HashMap<AttributeKey, Box<dyn Fn(i32) -> ModifierSet>>,
}
impl StatusEffect for UnendingStatusEffect {
    fn is_active(&mut self, _inp: UpdateInput) -> bool {
        true
    }

    fn scope(&self) -> StatusEffectScope {
        self.scope.clone()
    }

    fn get_modifiers(&self, key: AttributeKey) -> ModifierSet {
        if self.modifiers.contains_key(&key) {
            self.modifiers[&key](1)
        } else {
            ModifierSet::new()
        }
    }
}

