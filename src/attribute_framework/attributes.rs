use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    ops::Add, collections::HashMap,
};

use super::AttributeKey;


impl From<f64> for Attribute<'_> {
    fn from(f: f64) -> Self {
        Attribute::PrimF(f)
    }
}

impl From<i32> for Attribute<'_> {
    fn from(i: i32) -> Self {
        Attribute::PrimI(i)
    }
}

impl From<Attribute<'_>> for f64 {
    fn from(a: Attribute) -> Self {
        a.float()
    }
}

impl From<Attribute<'_>> for i32 {
    fn from(a: Attribute) -> Self {
        a.int()
    }
}

impl From<Attribute<'_>> for bool {
    fn from(a: Attribute) -> Self {
        a.bool()
    }
}

impl From<Attribute<'_>> for u32 {
    fn from(a: Attribute) -> Self {
        a.uint()
    }
}


#[derive(Debug)]
pub struct Resolver<'a> {
    attribute1: &'a Attribute<'a>,
    attribute2: &'a Attribute<'a>,
    operator: Operator,
}
impl Resolver<'_> {
    pub fn resolve(&self) -> f64 {
        match self.operator {
            Operator::Add => self.attribute1.val() + self.attribute2.val(),
            Operator::Sub => self.attribute1.val() - self.attribute2.val(),
            Operator::Mul => self.attribute1.val() * self.attribute2.val(),
            Operator::Div => self.attribute1.val() / self.attribute2.val(),
            Operator::Mod => self.attribute1.val() % self.attribute2.val(),
            Operator::Pow => self.attribute1.val().powf(self.attribute2.val()),
            Operator::Abs => self.attribute1.val().abs(),
            Operator::CLP(min, max) => self.attribute1.val().max(max).min(min),
            Operator::RND(precision, multiple_of) => {
                //round to the nearest multiple of multiple_of
                let mut val = self.attribute1.val() / multiple_of as f64;
                val = val.round() * multiple_of as f64;
                //round to the nearest precision
                val = (val * 10_f64.powi(precision as i32)).round() / 10_f64.powi(precision as i32);
                val
            }
        }
    }
}

#[derive()]
pub enum Attribute<'a> {
    PrimF(f64),
    PrimI(i32),
    Ref(RefCell<f64>),
    Lambda(Box<dyn Fn() -> f64 + 'a>),
    Compound(Box<Resolver<'a>>),
    Bool(bool),
    None,
}
impl Debug for Attribute<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Attribute::PrimF(fl) => write!(f, "PrimF({})", *fl),
            Attribute::PrimI(int) => write!(f, "PrimI({})", *int),
            Attribute::Ref(rf) => write!(f, "Ref({})", rf.borrow().to_owned()),
            Attribute::Lambda(fun) => write!(f, "SideEffect({})", fun()),
            Attribute::Compound(res) => write!(f, "Compound({})", res.resolve()),
            Attribute::Bool(b) => write!(f, "Bool({})", *b),
            Attribute::None => write!(f, "None"),
        }
    }
}
impl PartialEq for Attribute<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.val() == other.val()
    }
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Abs,
    CLP(f64, f64),
    RND(u32, u32),
}

impl Attribute<'_> {
    pub fn val(&self) -> f64 {
        match self {
            Attribute::PrimF(f) => *f,
            Attribute::PrimI(i) => *i as f64,
            Attribute::Ref(r) => *r.borrow(),
            Attribute::Compound(resolver) => resolver.resolve(),
            Attribute::Lambda(f) => f(),
            Attribute::Bool(b) => *b as i32 as f64,
            Attribute::None => 0.0,
        }
    }

    pub fn bool(&self) -> bool {
        match self {
            Attribute::Bool(b) => *b,
            _ => self.val() != 0.0,
        }
    }

    pub fn int(&self) -> i32 {
        match self {
            Attribute::PrimI(i) => *i,
            _ => self.val() as i32,
        }
    }

    pub fn float(&self) -> f64 {
        self.val()
    }

    pub fn uint(&self) -> u32 {
        self.val() as u32
    }

    pub fn add<'a>(&'a self, other: &'a Attribute) -> Attribute {
        Attribute::Compound(Box::new(Resolver {
            attribute1: self,
            attribute2: other,
            operator: Operator::Add,
        }))
    }

    pub fn sub<'a>(&'a self, other: &'a Attribute) -> Attribute {
        Attribute::Compound(Box::new(Resolver {
            attribute1: self,
            attribute2: other,
            operator: Operator::Sub
        }))
    }

    pub fn mul<'a>(&'a self, other: &'a Attribute) -> Attribute {
        Attribute::Compound(Box::new(Resolver {
            attribute1: self,
            attribute2: other,
            operator: Operator::Mul
        }))
    }

    pub fn div<'a>(&'a self, other: &'a Attribute) -> Attribute {
        Attribute::Compound(Box::new(Resolver {
            attribute1: self,
            attribute2: other,
            operator: Operator::Div
        }))
    }

    pub fn modu<'a>(&'a self, other: &'a Attribute) -> Attribute {
        Attribute::Compound(Box::new(Resolver {
            attribute1: self,
            attribute2: other,
            operator: Operator::Mod
        }))
    }

    pub fn pow<'a>(&'a self, other: &'a Attribute) -> Attribute<'a> {
        Attribute::Compound(Box::new(Resolver {
            attribute1: self,
            attribute2: other,
            operator: Operator::Pow
        }))
    }

    pub fn abs(&self) -> Attribute {
        Attribute::Compound(Box::new(Resolver {
            attribute1: self,
            attribute2: &Attribute::None,
            operator: Operator::Abs
        }))
    }

    pub fn clamp(&self, min: f64, max: f64) -> Attribute {
        Attribute::Compound(Box::new(Resolver {
            attribute1: self,
            attribute2: &Attribute::None,
            operator: Operator::CLP(min, max)
        }))
    }

    pub fn round(&self, precision: u32, multiple_of: u32) -> Attribute {
        Attribute::Compound(Box::new(Resolver {
            attribute1: self,
            attribute2: &Attribute::None,
            operator: Operator::RND(precision, multiple_of)
        }))
    }

    pub fn inner(&self) -> Option<&RefCell<f64>> {
        match self {
            Attribute::Ref(cell) => Some(cell),
            _ => None,
        }
    }

    pub fn freeze(&self) -> Attribute {
        Attribute::PrimF(self.val())
    }
}

#[derive(Debug)]
pub enum ModifierType {
    PreAdd,
    Scale,
    PostAdd,
    Override,
}

#[derive(Debug)]
pub struct Modifier {
    pub modifier_type: ModifierType,
    pub value: f64,
}
impl Modifier {
    pub fn pre_add(value: f64) -> Modifier {
        Modifier {
            modifier_type: ModifierType::PreAdd,
            value,
        }
    }

    pub fn scale(value: f64) -> Modifier {
        Modifier {
            modifier_type: ModifierType::Scale,
            value,
        }
    }

    pub fn post_add(value: f64) -> Modifier {
        Modifier {
            modifier_type: ModifierType::PostAdd,
            value,
        }
    }

    pub fn override_val(value: f64) -> Modifier {
        Modifier {
            modifier_type: ModifierType::Override,
            value,
        }
    }
    pub fn filler() -> Modifier {
        Modifier {
            modifier_type: ModifierType::Scale,
            value: 1.0,
        }
    }
}

#[derive(Debug)]
pub struct AttributeStub<'a> {
    pub attribute: Attribute<'a>,
    pre_add: f64,
    scale: f64,
    post_add: f64,
    override_val: Option<f64>,
}
impl AttributeStub<'_> {
    pub fn modify(&mut self, modifier: Modifier) {
        match modifier.modifier_type {
            ModifierType::PreAdd => self.pre_add += modifier.value,
            ModifierType::Scale => self.scale *= modifier.value,
            ModifierType::PostAdd => self.post_add += modifier.value,
            ModifierType::Override => self.override_val = Some(modifier.value),
        }
    }

    pub fn get(&self) -> f64 {
        let mut val = self.attribute.val();
        val += self.pre_add;
        val *= self.scale;
        val += self.post_add;
        if let Some(override_val) = self.override_val {
            val = override_val;
        }
        val
    }

    pub fn clear(&mut self) {
        self.pre_add = 0.0;
        self.scale = 1.0;
        self.post_add = 0.0;
        self.override_val = None;
    }

    pub fn get_and_clear(&mut self) -> f64 {
        let val = self.get();
        self.clear();
        val
    }
}
impl From<AttributeStub<'_>> for f64 {
    fn from(attr: AttributeStub) -> Self {
        attr.get()
    }
}

pub struct ModifierSet {
    pub modifiers: Vec<Modifier>,
}
impl ModifierSet {
    pub fn new() -> Self {
        ModifierSet {
            modifiers: Vec::new(),
        }
    }
}

pub struct AttributeManager<'a> {
    attrs: RefCell<HashMap<AttributeKey, AttributeStub<'a>>>
}
impl<'a> AttributeManager<'_> {
    pub fn new() -> Self {
        AttributeManager {
            attrs: RefCell::new(HashMap::new()),
        }
    }

    pub fn get_value(&self, key: AttributeKey) -> Option<f64> {
        self.attrs.borrow().get(&key).map(|attr| attr.get())
    }

    pub fn get_value_and_clear(&self, key: AttributeKey) -> Option<f64> {
        self.attrs.borrow_mut().get_mut(&key).map(|attr| attr.get_and_clear())
    }

    pub fn modify_attribute(&self, key: AttributeKey, modifier: Modifier) {
        if let Some(attr) = self.attrs.borrow_mut().get_mut(&key) {
            attr.modify(modifier);
        }
    }

    pub fn modify_attribute_many(&self, key: AttributeKey, modifiers: ModifierSet) {
        if let Some(attr) = self.attrs.borrow_mut().get_mut(&key) {
            for modifier in modifiers.modifiers {
                attr.modify(modifier);
            }
        }
    }
}