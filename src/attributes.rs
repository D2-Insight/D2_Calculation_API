use std::{ops::Add, fmt::{Debug, Display}, pin::Pin, rc::Rc};


//make atype alias for anything that has into float
pub type AttrResolverBox = Box<dyn Fn() -> f64>;
pub type AttrResolver = dyn Fn() -> f64;


pub trait Value: Display /*+ Into<f64>*/ {
    fn get_formula<'a>(&'a self) -> &'a AttrResolver;

    fn getf(&self) -> f64 {
        (self.get_formula())()
    }

    fn geti(&self) -> i32 {
        self.getf() as i32
    }

    fn get_equation(&self) -> String;

    fn get_default(&self) -> f64;
}

// impl Value for f64 {
//     fn get_formula<'a>(&'a self) -> &'a AttrResolver {
//         &|| -> f64 { *self }
//     }

//     fn get_equation(&self) -> String {
//         format!("{}", *self)
//     }

//     fn get_default(&self) -> f64 {
//         *self
//     }
// }

// impl Value for i32 {
//     fn get_formula<'a>(&'a self) -> &'a AttrResolver {
//         &|| -> f64 { *self as f64 }
//     }

//     fn get_equation(&self) -> String {
//         format!("{}", *self)
//     }

//     fn get_default(&self) -> f64 {
//         *self as f64
//     }
// }

// impl Value for u32 {
//     fn get_formula<'a>(&'a self) -> &'a AttrResolver {
//         // Rc::new(|| -> f64 { *self as f64 })
//     }

//     fn get_equation(&self) -> String {
//         format!("{}", *self)
//     }

//     fn get_default(&self) -> f64 {
//         *self as f64
//     }
// }

pub trait AttributeImpl: Attribute {
    fn add<'a, T: Value>(&'a self, other: &'a T) -> Box<CompoundAttribute>;

    fn sub<'a, T: Value>(&'a self, other: &'a T) -> Box<CompoundAttribute>;

    fn mul<'a, T: Value>(&'a self, other: &'a T) -> Box<CompoundAttribute>;

    fn div<'a, T: Value>(&'a self, other: &'a T) -> Box<CompoundAttribute>;

    fn pow<'a, T: Value>(&'a self, other: &'a T) -> Box<CompoundAttribute>;

    // fn clamp<T: Value + 'a>(self, min: T, max: T) -> ClampedAttribute where Self: Sized + 'a {
    //     ClampedAttribute::new(Box::new(self), Box::new(min), Box::new(max))
    // }
}
pub trait Attribute: Value {}

pub struct CompoundAttribute {
    equation: String,
    formula: Rc<AttrResolver>,
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
    fn get_formula<'a>(&'a self) -> &'a AttrResolver {
        &*self.formula
    }

    fn get_equation(&self) -> String {
        self.equation.clone()
    }

    fn get_default(&self) -> f64 {
        self.default
    }
}
impl AttributeImpl for CompoundAttribute {
    fn add<'a, T: Value>(&'a self, other: &'a T) -> Box<CompoundAttribute> {
        let equation = format!("({}) + ({})", self.get_equation(), other.get_equation());
        let formula = Rc::new(|| -> f64 { (self.formula)() + (other.get_formula())() });
        Box::new(CompoundAttribute { equation, formula, default: self.default })
    }

    fn sub<'a, T: Value>(&'a self, other: &'a T) -> Box<CompoundAttribute> {
        let equation = format!("({}) - ({})", self.get_equation(), other.get_equation());
        let formula = Rc::new(|| -> f64 { (self.formula)() - (other.get_formula())() });
        Box::new(CompoundAttribute { equation, formula, default: self.default })
    }

    fn mul<'a, T: Value>(&'a self, other: &'a T) -> Box<CompoundAttribute> {
        let equation = format!("({}) * ({})", self.get_equation(), other.get_equation());
        let formula = Rc::new(|| -> f64 { (self.formula)() * (other.get_formula())() });
        Box::new(CompoundAttribute { equation, formula, default: self.default })
    }

    fn div<'a, T: Value>(&'a self, other: &'a T) -> Box<CompoundAttribute> {
        let equation = format!("({}) / ({})", self.get_equation(), other.get_equation());
        let formula = Rc::new(|| -> f64 { (self.formula)() / (other.get_formula())() });
        Box::new(CompoundAttribute { equation, formula, default: self.default })
    }

    fn pow<'a, T: Value>(&'a self, other: &'a T) -> Box<CompoundAttribute> {
        let equation = format!("({}) ^ ({})", self.get_equation(), other.get_equation());
        let formula = Rc::new(|| -> f64 { (self.formula)().powf((other.get_formula())()) });
        Box::new(CompoundAttribute { equation, formula, default: self.default })
    }
}
impl Attribute for CompoundAttribute {}

pub struct PrimAttribute<'a> {
    val: f64,
    formula: &'a AttrResolver,
}
impl PrimAttribute<'_> {
    pub fn new<T: Into<f64> + Clone>(value: T) -> Self {
        Self{ val: value.clone().into(), formula: &|| -> f64 { value.clone().into() }}
    }
}
impl Into<f64> for PrimAttribute<'_> {
    fn into(self) -> f64 {
        self.val
    }
}
impl Display for PrimAttribute<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrimAttribute")
            .field("value", &self.val)
            .finish()
    }
}
impl Value for PrimAttribute<'_> {
    fn get_formula<'a>(&'a self) -> &'a AttrResolver {
        &*self.formula
    }

    fn get_equation(&self) -> String {
        format!("{}", self.val)
    }

    fn get_default(&self) -> f64 {
        self.val
    }
}
impl AttributeImpl for PrimAttribute<'_> {

    fn add<'a, T: Value>(&'a self, other: &'a T) -> Box<CompoundAttribute> {
        let equation = format!("({}) + ({})", self.get_equation(), other.get_equation());
        let formula = Rc::new(|| -> f64 { self.val + (other.get_formula())() });
        Box::new(CompoundAttribute { equation, formula, default: self.val })
    }

    fn sub<'a, T: Value>(&'a self, other: &'a T) -> Box<CompoundAttribute> {
        let equation = format!("({}) - ({})", self.get_equation(), other.get_equation());
        let formula = Rc::new(|| -> f64 { self.val - (other.get_formula())() });
        Box::new(CompoundAttribute { equation, formula, default: self.val })
    }

    fn mul<'a, T: Value>(&'a self, other: &'a T) -> Box<CompoundAttribute> {
        let equation = format!("({}) * ({})", self.get_equation(), other.get_equation());
        let formula = Rc::new(|| -> f64 { self.val * (other.get_formula())() });
        Box::new(CompoundAttribute { equation, formula, default: self.val })
    }

    fn div<'a, T: Value>(&'a self, other: &'a T) -> Box<CompoundAttribute> {
        let equation = format!("({}) / ({})", self.get_equation(), other.get_equation());
        let formula = Rc::new(|| -> f64 { self.val / (other.get_formula())() });
        Box::new(CompoundAttribute { equation, formula, default: self.val })
    }

    fn pow<'a, T: Value>(&'a self, other: &'a T) -> Box<CompoundAttribute> {
        let equation = format!("({}) ^ ({})", self.get_equation(), other.get_equation());
        let formula = Rc::new(|| -> f64 { self.val.powf((other.get_formula())()) });
        Box::new(CompoundAttribute { equation, formula, default: self.val })
    }
}
impl Attribute for PrimAttribute<'_> {}

pub struct RefAttribute {
    name: String,
    val: Box<dyn Value>
}
impl RefAttribute {
    pub fn new<T: Value>(name: String, value: Box<T>) -> Self {
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
    fn get_formula<'a>(&'a self) -> &'a AttrResolver {
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

    fn add<'a, T: Value>(&'a self, other: &'a T) -> Box<CompoundAttribute> {
        let equation = format!("({}) + ({})", self.get_equation(), other.get_equation());
        let formula = Rc::new(|| -> f64 { (self.val.as_ref().get_formula())() + (other.get_formula())() });
        Box::new(CompoundAttribute { equation, formula, default: self.val.as_ref().get_default() })
    }

    fn sub<'a, T: Value>(&'a self, other: &'a T) -> Box<CompoundAttribute> {
        let equation = format!("({}) - ({})", self.get_equation(), other.get_equation());
        let formula = Rc::new(|| -> f64 { (self.val.as_ref().get_formula())() - (other.get_formula())() });
        Box::new(CompoundAttribute { equation, formula, default: self.val.as_ref().get_default() })
    }

    fn mul<'a, T: Value>(&'a self, other: &'a T) -> Box<CompoundAttribute> {
        let equation = format!("({}) * ({})", self.get_equation(), other.get_equation());
        let formula = Rc::new(|| -> f64 { (self.val.as_ref().get_formula())() * (other.get_formula())() });
        Box::new(CompoundAttribute { equation, formula, default: self.val.as_ref().get_default() })
    }

    fn div<'a, T: Value>(&'a self, other: &'a T) -> Box<CompoundAttribute> {
        let equation = format!("({}) / ({})", self.get_equation(), other.get_equation());
        let formula = Rc::new(|| -> f64 { (self.val.as_ref().get_formula())() / (other.get_formula())() });
        Box::new(CompoundAttribute { equation, formula, default: self.val.as_ref().get_default() })
    }

    fn pow<'a, T: Value>(&'a self, other: &'a T) -> Box<CompoundAttribute> {
        let equation = format!("({}) ^ ({})", self.get_equation(), other.get_equation());
        let formula = Rc::new(|| -> f64 { (self.val.as_ref().get_formula())().powf((other.get_formula())()) });
        Box::new(CompoundAttribute { equation, formula, default: self.val.as_ref().get_default() })
    }
}
impl Attribute for RefAttribute {}

pub struct SimpleAttribute {
    name: String,
    val: AttrResolverBox,
    default: f64
}
impl SimpleAttribute {
    pub fn new(name: String, value: AttrResolverBox, default: f64) -> Self {
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
    fn get_formula<'a>(&'a self) -> &'a AttrResolver {
        &|| -> f64 { (self.val)() }
    }

    fn get_equation(&self) -> String {
        format!("{{{}}}", self.name)
    }

    fn get_default(&self) -> f64 {
        self.default
    }
}
impl AttributeImpl for SimpleAttribute {

    fn add<'a, T: Value>(&'a self, other: &'a T) -> Box<CompoundAttribute> {
        let equation = format!("({}) + ({})", self.get_equation(), other.get_equation());
        let formula = Rc::new(|| -> f64 { (self.val)() + (other.get_formula())() });
        Box::new(CompoundAttribute { equation, formula, default: self.default })
    }

    fn sub<'a, T: Value>(&'a self, other: &'a T) -> Box<CompoundAttribute> {
        let equation = format!("({}) - ({})", self.get_equation(), other.get_equation());
        let formula = Rc::new(|| -> f64 { (self.val)() - (other.get_formula())() });
        Box::new(CompoundAttribute { equation, formula, default: self.default })
    }

    fn mul<'a, T: Value>(&'a self, other: &'a T) -> Box<CompoundAttribute> {
        let equation = format!("({}) * ({})", self.get_equation(), other.get_equation());
        let formula = Rc::new(|| -> f64 { (self.val)() * (other.get_formula())() });
        Box::new(CompoundAttribute { equation, formula, default: self.default })
    }

    fn div<'a, T: Value>(&'a self, other: &'a T) -> Box<CompoundAttribute> {
        let equation = format!("({}) / ({})", self.get_equation(), other.get_equation());
        let formula = Rc::new(|| -> f64 { (self.val)() / (other.get_formula())() });
        Box::new(CompoundAttribute { equation, formula, default: self.default })
    }

    fn pow<'a, T: Value>(&'a self, other: &'a T) -> Box<CompoundAttribute> {
        let equation = format!("({}) ^ ({})", self.get_equation(), other.get_equation());
        let formula = Rc::new(|| -> f64 { (self.val)().powf((other.get_formula())()) });
        Box::new(CompoundAttribute { equation, formula, default: self.default })
    }
}
impl Attribute for SimpleAttribute {}

pub struct ClampedAttribute {
    attr: Box<dyn Attribute>,
    min: Box<dyn Value>,
    max: Box<dyn Value>,
    formula: Box<dyn Fn() -> f64>
}
impl ClampedAttribute {
    pub fn new(attr: Box<dyn Attribute>, min: Box<dyn Value>, max: Box<dyn Value>) -> Self {
        let fmin = min.as_ref().get_formula();
        let fmax = max.as_ref().get_formula();
        let fattr = attr.as_ref().get_formula();
        let func = (move || -> f64 {
            let mut val = fattr();
            if val < fmin() {
                val = fmin();
            }
            if val > fmax() {
                val = fmax();
            }
            val
        });
        Self{attr, min, max, formula: Box::new(func)}
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
    fn get_formula<'a>(&'a self) -> &'a AttrResolver {
        &self.formula
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

    fn add<'a, T: Value>(&'a self, other: &'a T) -> Box<CompoundAttribute> {
        let equation = format!("({}) + ({})", self.get_equation(), other.get_equation());
        let formula = Rc::new(|| -> f64 { (self.get_formula())() + (other.get_formula())() });
        Box::new(CompoundAttribute { equation, formula, default: self.get_default() })
    }

    fn sub<'a, T: Value>(&'a self, other: &'a T) -> Box<CompoundAttribute> {
        let equation = format!("({}) - ({})", self.get_equation(), other.get_equation());
        let formula = Rc::new(|| -> f64 { (self.get_formula())() - (other.get_formula())() });
        Box::new(CompoundAttribute { equation, formula, default: self.get_default() })
    }

    fn mul<'a, T: Value>(&'a self, other: &'a T) -> Box<CompoundAttribute> {
        let equation = format!("({}) * ({})", self.get_equation(), other.get_equation());
        let formula = Rc::new(|| -> f64 { (self.get_formula())() * (other.get_formula())() });
        Box::new(CompoundAttribute { equation, formula, default: self.get_default() })
    }

    fn div<'a, T: Value>(&'a self, other: &'a T) -> Box<CompoundAttribute> {
        let equation = format!("({}) / ({})", self.get_equation(), other.get_equation());
        let formula = Rc::new(|| -> f64 { (self.get_formula())() / (other.get_formula())() });
        Box::new(CompoundAttribute { equation, formula, default: self.get_default() })
    }

    fn pow<'a, T: Value>(&'a self, other: &'a T) -> Box<CompoundAttribute> {
        let equation = format!("({}) ^ ({})", self.get_equation(), other.get_equation());
        let formula = Rc::new(|| -> f64 { (self.get_formula())().powf((other.get_formula())()) });
        Box::new(CompoundAttribute { equation, formula, default: self.get_default() })
    }
}
impl Attribute for ClampedAttribute {}


#[macro_export]
macro_rules! attribute {
    // ($name:ident, $value:expr) => {
    //     SimpleAttribute::new(stringify!($name).to_string(), Box::new(|| -> f64 { $value.into() }), $value.into())
    // };
    // take in one arg of a variable and derive name and value from it
    ($name:ident) => {
        SimpleAttribute::new(stringify!($name).to_string(), Box::new(|| -> f64 { $name.into() }), $name.into())
    };
}

#[macro_export]
macro_rules! attribute_ref {
    ($name:ident, $value:expr) => {
        RefAttribute::new(stringify!($name).to_string(), Box::new(|| -> f64 { $value.into() }), $value.into())
    };
}