use std::ops::Mul;

pub struct PhantomUnit(f64);

impl PhantomUnit {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}
