use crate::physics::electromagnetism::charge::Coulomb;
use crate::physics::electromagnetism::potential::Volt;
use crate::physics::length::{Meter, ToMeter};
use crate::physics::mechanics::force::Newton;
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct NewtonPerCoulomb(f64);

pub type VoltPerMeter = NewtonPerCoulomb;

impl NewtonPerCoulomb {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Add<NewtonPerCoulomb> for NewtonPerCoulomb {
    type Output = NewtonPerCoulomb;

    fn add(self, rhs: NewtonPerCoulomb) -> Self::Output {
        NewtonPerCoulomb(self.0 + rhs.0)
    }
}

impl Mul<Coulomb> for NewtonPerCoulomb {
    type Output = Newton;

    fn mul(self, rhs: Coulomb) -> Self::Output {
        Newton::new(self.0 * rhs.value())
    }
}

impl<T> Mul<T> for NewtonPerCoulomb
where
    T: ToMeter,
{
    type Output = Volt;

    fn mul(self, rhs: T) -> Self::Output {
        Volt::new(self.0 * rhs.to_meter().value())
    }
}
