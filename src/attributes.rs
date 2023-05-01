use std::{ops::Add, fmt::{Debug, Display}, pin::Pin};


//make atype alias for anything that has into float
pub type AttrResolver = Box<dyn Fn() -> f64 + 'static>;

pub trait Value: Display /*+ Into<f64>*/ {
    fn get_formula(&'static self) -> AttrResolver;

    fn getf(&'static self) -> f64 {
        (self.get_formula())()
    }

    fn geti(&'static self) -> i32 {
        self.getf() as i32
    }

    fn get_equation(&self) -> String;

    fn get_default(&self) -> f64;
}

impl Value for f64 {
    fn get_formula(&'static self) -> AttrResolver {
        Box::new(|| -> f64 { *self })
    }

    fn get_equation(&self) -> String {
        format!("{}", *self)
    }

    fn get_default(&self) -> f64 {
        *self
    }
}

impl Value for i32 {
    fn get_formula(&'static self) -> AttrResolver {
        Box::new(|| -> f64 { *self as f64 })
    }

    fn get_equation(&self) -> String {
        format!("{}", *self)
    }

    fn get_default(&self) -> f64 {
        *self as f64
    }
}

impl Value for u32 {
    fn get_formula(&'static self) -> AttrResolver {
        Box::new(|| -> f64 { *self as f64 })
    }

    fn get_equation(&self) -> String {
        format!("{}", *self)
    }

    fn get_default(&self) -> f64 {
        *self as f64
    }
}

pub trait AttributeImpl: Attribute {
    fn add<T: Value>(&'static self, other: &'static T) -> CompoundAttribute;

    fn sub<T: Value>(&'static self, other: &'static T) -> CompoundAttribute;

    fn mul<T: Value>(&'static self, other: &'static T) -> CompoundAttribute;

    fn div<T: Value>(&'static self, other: &'static T) -> CompoundAttribute;

    fn pow<T: Value>(&'static self, other: &'static T) -> CompoundAttribute;

    fn clamp<T: Value + 'static>(self, min: T, max: T) -> ClampedAttribute where Self: Sized + 'static {
        ClampedAttribute { attr: Box::new(self), min: Box::new(min), max: Box::new(max) }
    }
}
pub trait Attribute: Value {}

pub struct CompoundAttribute {
    equation: String,
    formula: AttrResolver,
    default: f64,
}
impl Into<f64> for CompoundAttribute {
    fn into(self) -> f64 {
        (self.formula)()
    }
}
impl Display for CompoundAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompoundAttribute")
            .field("equation", &self.equation)
            .field("current_value", &(self.formula)())
            .field("default", &self.default)
            .finish()
    }
}
impl Value for CompoundAttribute {
    fn get_formula(&'static self) -> AttrResolver {
        Box::new(|| -> f64 { (self.formula)() })
    }

    fn get_equation(&self) -> String {
        self.equation.clone()
    }

    fn get_default(&self) -> f64 {
        self.default
    }
}
impl AttributeImpl for CompoundAttribute {
    fn add<T: Value>(&'static self, other: &'static  T) -> CompoundAttribute {
        let equation = format!("({}) + ({})", self.get_equation(), other.get_equation());
        let formula = Box::new(|| -> f64 { (self.formula)() + (other.get_formula())() });
        CompoundAttribute { equation, formula, default: self.default }
    }

    fn sub<T: Value>(&'static self, other: &'static T) -> CompoundAttribute {
        let equation = format!("({}) - ({})", self.get_equation(), other.get_equation());
        let formula = Box::new(|| -> f64 { (self.formula)() - (other.get_formula())() });
        CompoundAttribute { equation, formula, default: self.default }
    }

    fn mul<T: Value>(&'static self, other: &'static T) -> CompoundAttribute {
        let equation = format!("({}) * ({})", self.get_equation(), other.get_equation());
        let formula = Box::new(|| -> f64 { (self.formula)() * (other.get_formula())() });
        CompoundAttribute { equation, formula, default: self.default }
    }

    fn div<T: Value>(&'static self, other: &'static T) -> CompoundAttribute {
        let equation = format!("({}) / ({})", self.get_equation(), other.get_equation());
        let formula = Box::new(|| -> f64 { (self.formula)() / (other.get_formula())() });
        CompoundAttribute { equation, formula, default: self.default }
    }

    fn pow<T: Value>(&'static self, other: &'static T) -> CompoundAttribute {
        let equation = format!("({}) ^ ({})", self.get_equation(), other.get_equation());
        let formula = Box::new(|| -> f64 { (self.formula)().powf((other.get_formula())()) });
        CompoundAttribute { equation, formula, default: self.default }
    }
}
impl Attribute for CompoundAttribute {}

pub struct PrimAttribute {
    val: f64
}
impl PrimAttribute {
    pub fn new<T: Into<f64>>(value: T) -> Self {
        Self{ val: value.into() }
    }
}
impl Into<f64> for PrimAttribute {
    fn into(self) -> f64 {
        self.val
    }
}
impl Display for PrimAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrimAttribute")
            .field("value", &self.val)
            .finish()
    }
}
impl Value for PrimAttribute {
    fn get_formula(&'static self) -> AttrResolver {
        Box::new(|| -> f64 { self.val })
    }

    fn get_equation(&self) -> String {
        format!("{}", self.val)
    }

    fn get_default(&self) -> f64 {
        self.val
    }
}
impl AttributeImpl for PrimAttribute {

    fn add<T: Value>(&'static self, other: &'static T) -> CompoundAttribute {
        let equation = format!("({}) + ({})", self.get_equation(), other.get_equation());
        let formula = Box::new(|| -> f64 { self.val + (other.get_formula())() });
        CompoundAttribute { equation, formula, default: self.val }
    }

    fn sub<T: Value>(&'static self, other: &'static T) -> CompoundAttribute {
        let equation = format!("({}) - ({})", self.get_equation(), other.get_equation());
        let formula = Box::new(|| -> f64 { self.val - (other.get_formula())() });
        CompoundAttribute { equation, formula, default: self.val }
    }

    fn mul<T: Value>(&'static self, other: &'static T) -> CompoundAttribute {
        let equation = format!("({}) * ({})", self.get_equation(), other.get_equation());
        let formula = Box::new(|| -> f64 { self.val * (other.get_formula())() });
        CompoundAttribute { equation, formula, default: self.val }
    }

    fn div<T: Value>(&'static self, other: &'static T) -> CompoundAttribute {
        let equation = format!("({}) / ({})", self.get_equation(), other.get_equation());
        let formula = Box::new(|| -> f64 { self.val / (other.get_formula())() });
        CompoundAttribute { equation, formula, default: self.val }
    }

    fn pow<T: Value>(&'static self, other: &'static T) -> CompoundAttribute {
        let equation = format!("({}) ^ ({})", self.get_equation(), other.get_equation());
        let formula = Box::new(|| -> f64 { self.val.powf((other.get_formula())()) });
        CompoundAttribute { equation, formula, default: self.val }
    }
}
impl Attribute for PrimAttribute {}

pub struct RefAttribute {
    name: String,
    val: Box<dyn Value + 'static>
}
impl RefAttribute {
    pub fn new<T: Value + 'static>(name: String, value: Box<T>) -> Self {
        Self{name, val: value }
    }
}
impl Display for RefAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.val.as_ref().fmt(f)
    }
}
// impl Into<f64> for RefAttribute {
//     fn into(self) -> f64 {
//         self.val.into().
//     }
// }
impl Value for RefAttribute {
    fn get_formula(&'static self) -> AttrResolver {
        self.val.as_ref().clone().get_formula()
    }

    fn get_equation(&self) -> String {
        // self.val.as_ref().clone().get_equation()
        format!("{{{}}}", self.name)
    }

    fn get_default(&self) -> f64 {
        self.val.as_ref().clone().get_default()
    }
}
impl AttributeImpl for RefAttribute {

    fn add<T: Value>(&'static self, other: &'static T) -> CompoundAttribute {
        let equation = format!("({}) + ({})", self.get_equation(), other.get_equation());
        let formula = Box::new(|| -> f64 { (self.val.as_ref().get_formula())() + (other.get_formula())() });
        CompoundAttribute { equation, formula, default: self.val.as_ref().get_default() }
    }

    fn sub<T: Value>(&'static self, other: &'static T) -> CompoundAttribute {
        let equation = format!("({}) - ({})", self.get_equation(), other.get_equation());
        let formula = Box::new(|| -> f64 { (self.val.as_ref().get_formula())() - (other.get_formula())() });
        CompoundAttribute { equation, formula, default: self.val.as_ref().get_default() }
    }

    fn mul<T: Value>(&'static self, other: &'static T) -> CompoundAttribute {
        let equation = format!("({}) * ({})", self.get_equation(), other.get_equation());
        let formula = Box::new(|| -> f64 { (self.val.as_ref().get_formula())() * (other.get_formula())() });
        CompoundAttribute { equation, formula, default: self.val.as_ref().get_default() }
    }

    fn div<T: Value>(&'static self, other: &'static T) -> CompoundAttribute {
        let equation = format!("({}) / ({})", self.get_equation(), other.get_equation());
        let formula = Box::new(|| -> f64 { (self.val.as_ref().get_formula())() / (other.get_formula())() });
        CompoundAttribute { equation, formula, default: self.val.as_ref().get_default() }
    }

    fn pow<T: Value>(&'static self, other: &'static T) -> CompoundAttribute {
        let equation = format!("({}) ^ ({})", self.get_equation(), other.get_equation());
        let formula = Box::new(|| -> f64 { (self.val.as_ref().get_formula())().powf((other.get_formula())()) });
        CompoundAttribute { equation, formula, default: self.val.as_ref().get_default() }
    }
}
impl Attribute for RefAttribute {}

pub struct SimpleAttribute {
    name: String,
    val: AttrResolver,
    default: f64
}
impl SimpleAttribute {
    pub fn new(name: String, value: AttrResolver, default: f64) -> Self {
        Self{name, val: value, default }
    }
}
impl Display for SimpleAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SimpleAttribute")
            .field("name", &self.name)
            .field("current_val", &(self.val)())
            .finish()
    }
}
impl Value for SimpleAttribute {
    fn get_formula(&'static self) -> AttrResolver {
        Box::new(|| -> f64 { (self.val)() })
    }

    fn get_equation(&self) -> String {
        format!("{{{}}}", self.name)
    }

    fn get_default(&self) -> f64 {
        self.default
    }
}
impl AttributeImpl for SimpleAttribute {

    fn add<T: Value>(&'static self, other: &'static T) -> CompoundAttribute {
        let equation = format!("({}) + ({})", self.get_equation(), other.get_equation());
        let formula = Box::new(|| -> f64 { (self.val)() + (other.get_formula())() });
        CompoundAttribute { equation, formula, default: self.default }
    }

    fn sub<T: Value>(&'static self, other: &'static T) -> CompoundAttribute {
        let equation = format!("({}) - ({})", self.get_equation(), other.get_equation());
        let formula = Box::new(|| -> f64 { (self.val)() - (other.get_formula())() });
        CompoundAttribute { equation, formula, default: self.default }
    }

    fn mul<T: Value>(&'static self, other: &'static T) -> CompoundAttribute {
        let equation = format!("({}) * ({})", self.get_equation(), other.get_equation());
        let formula = Box::new(|| -> f64 { (self.val)() * (other.get_formula())() });
        CompoundAttribute { equation, formula, default: self.default }
    }

    fn div<T: Value>(&'static self, other: &'static T) -> CompoundAttribute {
        let equation = format!("({}) / ({})", self.get_equation(), other.get_equation());
        let formula = Box::new(|| -> f64 { (self.val)() / (other.get_formula())() });
        CompoundAttribute { equation, formula, default: self.default }
    }

    fn pow<T: Value>(&'static self, other: &'static T) -> CompoundAttribute {
        let equation = format!("({}) ^ ({})", self.get_equation(), other.get_equation());
        let formula = Box::new(|| -> f64 { (self.val)().powf((other.get_formula())()) });
        CompoundAttribute { equation, formula, default: self.default }
    }
}
impl Attribute for SimpleAttribute {}

pub struct ClampedAttribute {
    attr: Box<dyn Attribute>,
    min: Box<dyn Value>,
    max: Box<dyn Value>
}
impl ClampedAttribute {
    pub fn new(attr: Box<dyn Attribute>, min: Box<dyn Value>, max: Box<dyn Value>) -> Self {
        Self{attr, min, max}
    }
}
impl Display for ClampedAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let equation = self.attr.as_ref().get_equation();
        let min = self.min.as_ref().get_default(); //TODO make it not do default
        let max = self.max.as_ref().get_default(); //^^^^
        f.debug_struct("ClampedAttribute")
            .field("attr", &equation)
            .field("min", &min)
            .field("max", &max)
            .finish()
    }
}
impl Value for ClampedAttribute {
    fn get_formula(&'static self) -> AttrResolver {
        let attr = self.attr.as_ref().get_formula();
        let min = self.min.as_ref().get_formula();
        let max = self.max.as_ref().get_formula();
        Box::new(move || -> f64 {
            let mut val = attr();
            if val < min() {
                val = min();
            }
            if val > max() {
                val = max();
            }
            val
        })
    }

    fn get_equation(&self) -> String {
        format!("clamp({}, {}, {})", 
            self.attr.as_ref().get_equation(),
            self.min.as_ref().get_equation(),
            self.max.as_ref().get_equation())
    }

    fn get_default(&self) -> f64 {
        self.attr.as_ref().get_default()
    }
}
impl AttributeImpl for ClampedAttribute {

    fn add<T: Value>(&'static self, other: &'static T) -> CompoundAttribute {
        let equation = format!("({}) + ({})", self.get_equation(), other.get_equation());
        let formula = Box::new(|| -> f64 { (self.get_formula())() + (other.get_formula())() });
        CompoundAttribute { equation, formula, default: self.get_default() }
    }

    fn sub<T: Value>(&'static self, other: &'static T) -> CompoundAttribute {
        let equation = format!("({}) - ({})", self.get_equation(), other.get_equation());
        let formula = Box::new(|| -> f64 { (self.get_formula())() - (other.get_formula())() });
        CompoundAttribute { equation, formula, default: self.get_default() }
    }

    fn mul<T: Value>(&'static self, other: &'static T) -> CompoundAttribute {
        let equation = format!("({}) * ({})", self.get_equation(), other.get_equation());
        let formula = Box::new(|| -> f64 { (self.get_formula())() * (other.get_formula())() });
        CompoundAttribute { equation, formula, default: self.get_default() }
    }

    fn div<T: Value>(&'static self, other: &'static T) -> CompoundAttribute {
        let equation = format!("({}) / ({})", self.get_equation(), other.get_equation());
        let formula = Box::new(|| -> f64 { (self.get_formula())() / (other.get_formula())() });
        CompoundAttribute { equation, formula, default: self.get_default() }
    }

    fn pow<T: Value>(&'static self, other: &'static T) -> CompoundAttribute {
        let equation = format!("({}) ^ ({})", self.get_equation(), other.get_equation());
        let formula = Box::new(|| -> f64 { (self.get_formula())().powf((other.get_formula())()) });
        CompoundAttribute { equation, formula, default: self.get_default() }
    }
}
impl Attribute for ClampedAttribute {}


macro_rules! attribute {
    ($name:ident, $value:expr) => {
        SimpleAttribute::new(stringify!($name).to_string(), Box::new(|| -> f64 { $value.into() }), $value.into())
    };
}

macro_rules! attribute_ref {
    ($name:ident, $value:expr) => {
        RefAttribute::new(stringify!($name).to_string(), Box::new(|| -> f64 { $value.into() }), $value.into())
    };
}