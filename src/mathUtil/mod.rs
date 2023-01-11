pub mod stat_calc;
pub mod ttk_calc;
pub mod damage_calc;
pub mod dps_calc;

pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

pub fn ceil(a: f64) -> f64 {
    a.ceil()
}
