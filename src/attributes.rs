use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    ops::Add,
    pin::Pin,
    rc::Rc,
};

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
    None,
}
impl Debug for Attribute<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Attribute::PrimF(fl) => write!(f, "PrimF({})", *fl),
            Attribute::PrimI(int) => write!(f, "PrimI({})", *int),
            // Attribute::Lambda(_, p) => write!(f, "Lambda: {}", p),
            Attribute::Ref(_) => write!(f, "Ref"),
            Attribute::Lambda(_) => write!(f, "SideEffect"),
            Attribute::Compound(_) => write!(f, "Compound"),
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
}

impl Attribute<'_> {
    pub fn val(&self) -> f64 {
        match self {
            Attribute::PrimF(f) => *f,
            Attribute::PrimI(i) => *i as f64,
            Attribute::Ref(r) => *r.borrow(),
            Attribute::Compound(resolver) => resolver.resolve(),
            Attribute::Lambda(f) => f(),
            Attribute::None => 0.0,
        }
    }

    pub fn add<'a>(&'a self, other: &'a Attribute) -> Attribute {
        Attribute::Compound(Box::new(Resolver {
            attribute1: self,
            attribute2: other,
            operator: Operator::Add,
        }))
    }

    // pub fn sub<'a>(&self, other: &Attribute) -> Attribute {
    //     Attribute::Compound(Box::new(Resolver {
    //         attribute1: self,
    //         attribute2: other,
    //         operator: Operator::Sub
    //     }))
    // }

    // pub fn mul<'a>(&self, other: &Attribute) -> Attribute {
    //     Attribute::Compound(Box::new(Resolver {
    //         attribute1: self,
    //         attribute2: other,
    //         operator: Operator::Mul
    //     }))
    // }

    // pub fn div<'a>(&self, other: &Attribute) -> Attribute {
    //     Attribute::Compound(Box::new(Resolver {
    //         attribute1: self,
    //         attribute2: other,
    //         operator: Operator::Div
    //     }))
    // }

    // pub fn modu<'a>(&self, other: &Attribute) -> Attribute {
    //     Attribute::Compound(Box::new(Resolver {
    //         attribute1: self,
    //         attribute2: other,
    //         operator: Operator::Mod
    //     }))
    // }

    // pub fn pow<'a>(&'a self, other: &'a Attribute) -> Attribute<'a> {
    //     Attribute::Compound(Box::new(Resolver {
    //         attribute1: self,
    //         attribute2: other,
    //         operator: Operator::Pow
    //     }))
    // }

    // pub fn abs(&self) -> Attribute {
    //     Attribute::Compound(Box::new(Resolver {
    //         attribute1: self,
    //         attribute2: &Attribute::None,
    //         operator: Operator::Abs
    //     }))
    // }

    // pub fn clamp(&self, min: f64, max: f64) -> Attribute {
    //     Attribute::Compound(Box::new(Resolver {
    //         attribute1: self,
    //         attribute2: &Attribute::None,
    //         operator: Operator::CLP(min, max)
    //     }))
    // }

    pub fn inner(&self) -> Option<&RefCell<f64>> {
        match self {
            Attribute::Ref(cell) => Some(cell),
            _ => None,
        }
    }
}
impl From<Attribute<'_>> for f64 {
    fn from(attr: Attribute) -> Self {
        attr.val()
    }
}
