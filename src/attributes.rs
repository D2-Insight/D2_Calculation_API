use std::{ops::Add, fmt::{Debug, Display}, pin::Pin, rc::Rc, cell::RefCell};

#[derive(Debug)]
pub struct Resolver {
    attribute1: Attribute,
    attribute2: Attribute,
    operator: Operator
}
impl Resolver {
    pub fn resolve(&self) -> f64{
        match self.operator {
            Operator::Add => self.attribute1.val() + self.attribute2.val(),
            Operator::Sub => self.attribute1.val() - self.attribute2.val(),
            Operator::Mul => self.attribute1.val() * self.attribute2.val(),
            Operator::Div => self.attribute1.val() / self.attribute2.val(),
            Operator::Mod => self.attribute1.val() % self.attribute2.val(),
            Operator::Pow => self.attribute1.val().powf(self.attribute2.val()),
            Operator::Abs => self.attribute1.val().abs(),
            Operator::CLP(min, max) => self.attribute1.val().max(max).min(min)
        }
    }
}

pub enum Attribute {
    PrimF(f64),
    PrimI(i32),
    Lambda(Box<dyn Fn() -> f64>),
    Compound(Box<Resolver>),
    None
}
impl Debug for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Attribute::PrimF(fl) => write!(f, "PrimF({})", *fl),
            Attribute::PrimI(int) => write!(f, "PrimI({})", *int),
            Attribute::Lambda(_) => write!(f, "Lambda"),
            Attribute::Compound(_) => write!(f, "Compound"),
            Attribute::None => write!(f, "None")
        }
    }
}
impl PartialEq for Attribute {
    fn eq(&self, other: &Self) -> bool {
        self.val() == other.val()
    }
}
impl Eq for Attribute {}

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Abs,
    CLP(f64, f64)
}

impl Attribute {
    pub fn val(&self) -> f64{
        match self {
            Attribute::PrimF(f) => *f,
            Attribute::PrimI(i) => *i as f64,
            Attribute::Compound(resolver) => resolver.resolve(),
            Attribute::Lambda(func) => func(),
            Attribute::None => 0.0
        }
    }

    pub fn add(self, other: Attribute) -> Attribute {
        Attribute::Compound(Box::new(Resolver {
            attribute1: self,
            attribute2: other,
            operator: Operator::Add
        }))
    }

    pub fn sub(self, other: Attribute) -> Attribute {
        Attribute::Compound(Box::new(Resolver {
            attribute1: self,
            attribute2: other,
            operator: Operator::Sub
        }))
    }

    pub fn mul(self, other: Attribute) -> Attribute {
        Attribute::Compound(Box::new(Resolver {
            attribute1: self,
            attribute2: other,
            operator: Operator::Mul
        }))
    }

    pub fn div(self, other: Attribute) -> Attribute {
        Attribute::Compound(Box::new(Resolver {
            attribute1: self,
            attribute2: other,
            operator: Operator::Div
        }))
    }

    pub fn modu(self, other: Attribute) -> Attribute {
        Attribute::Compound(Box::new(Resolver {
            attribute1: self,
            attribute2: other,
            operator: Operator::Mod
        }))
    }

    pub fn pow(self, other: Attribute) -> Attribute {
        Attribute::Compound(Box::new(Resolver {
            attribute1: self,
            attribute2: other,
            operator: Operator::Pow
        }))
    }

    pub fn abs(self) -> Attribute {
        Attribute::Compound(Box::new(Resolver {
            attribute1: self,
            attribute2: Attribute::None,
            operator: Operator::Abs
        }))
    }

    pub fn clamp(self, min: f64, max: f64) -> Attribute {
        Attribute::Compound(Box::new(Resolver {
            attribute1: self,
            attribute2: Attribute::None,
            operator: Operator::CLP(min, max)
        }))
    }
}
impl From<Attribute> for f64 {
    fn from(attr: Attribute) -> Self {
        attr.val()
    }
}