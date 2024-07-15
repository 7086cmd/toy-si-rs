use std::ops::{Add, Div};
use crate::general::time::{Second, ToSecond};
use crate::physics::electromagnetism::current::Ampere;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Coulomb(f64);

impl Coulomb {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Add<Coulomb> for Coulomb {
    type Output = Coulomb;

    fn add(self, rhs: Coulomb) -> Self::Output {
        Coulomb(self.0 + rhs.0)
    }
}

impl<T> Div<T> for Coulomb where T: ToSecond {
    type Output = Ampere;

    fn div(self, rhs: T) -> Self::Output {
        Ampere::new(self.0 / rhs.to_second().value())
    }
}