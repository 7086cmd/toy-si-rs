use crate::physics::electromagnetism::current_element::AmpereMeter;
use crate::physics::mechanics::force::Newton;
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Tesla(f64);

impl Tesla {
    pub fn new(tesla: f64) -> Self {
        Tesla(tesla)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Mul<AmpereMeter> for Tesla {
    type Output = Newton;

    fn mul(self, current_element: AmpereMeter) -> Newton {
        Newton::new(self.0 * current_element.value())
    }
}
