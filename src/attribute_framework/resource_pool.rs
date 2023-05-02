use std::cell::RefCell;

use super::attributes::Attribute;


pub struct ResourcePool<'a> {
    pub current: Attribute<'a>,
    pub max: f64,
    pub min: f64,
    // pub regen_rate: f64,
    // pub regen_delay: f64,
    // pub regen_delay_timer: f64,
}
impl ResourcePool<'_> {
    pub fn attr(&self) -> &Attribute {
        &self.current
    }

    //if the amount has pushed current above max or below min will return false, else true
    pub fn add(&self, amount: f64) -> bool {
        let old = self.current.val();
        let new = (old + amount)
                        .clamp(self.min, self.max);
        self.current.inner().unwrap().replace(new);
        if (old + amount) != new {
            return false;
        } else {
            return true;
        }
    }

    pub fn sub(&mut self, amount: f64) -> bool {
        self.add(amount * -1.0)
    }

    pub fn new<'a>(current: f64, max: f64, min: f64) -> ResourcePool<'a> {
        ResourcePool {
            current: Attribute::Ref(RefCell::new(current)),
            max,
            min,
        }
    }

    // pub fn update(&mut self, dt: f64) {
    //     if self.current.val() < self.max {
    //         self.regen_delay_timer += dt;
    //         if self.regen_delay_timer >= self.regen_delay {
    //             self.add(self.regen_rate * dt);
    //         }
    //     }
    // }

    // pub fn new<'a>(current: f64, max: f64, min: f64, regen_rate: f64, regen_delay: f64) -> ResourcePool<'a> {
    //     ResourcePool {
    //         current: Attribute::Ref(RefCell::new(current)),
    //         max,
    //         min,
    //         regen_rate,
    //         regen_delay,
    //         regen_delay_timer: 0.0,
    //     }
    // }
}