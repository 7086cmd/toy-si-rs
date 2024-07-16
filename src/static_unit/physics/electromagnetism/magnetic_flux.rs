use std::ops::{Mul, Div};
use crate::static_unit::general::time::ToSecond;
use crate::static_unit::physics::area::{SquareMeter, ToSquareMeter};
use crate::static_unit::physics::electromagnetism::potential::Volt;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Weber(f64);

impl Weber {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl<T> Mul<T> for Weber where T: ToSquareMeter {
    type Output = Weber;

    fn mul(self, rhs: T) -> Self::Output {
        Weber(self.0 * rhs.to_square_meter().value())
    }
}

impl<T> Div<T> for Weber where T: ToSecond {
    type Output = Volt;

    fn div(self, rhs: T) -> Self::Output {
        Volt::new(self.0 / rhs.to_second().value())
    }
}