use crate::physics::electromagnetism::capacitance::{Farad, ToFarad};
use crate::physics::electromagnetism::charge::Coulomb;
use crate::physics::electromagnetism::electronic_field_strength::NewtonPerCoulomb;
use crate::physics::length::{Meter, ToMeter};
use crate::physics::mechanics::energy::Joule;
use std::ops::{Add, Div, Mul};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Volt(f64);

impl Volt {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Add<Volt> for Volt {
    type Output = Volt;

    fn add(self, rhs: Volt) -> Self::Output {
        Volt(self.0 + rhs.0)
    }
}

impl Mul<Coulomb> for Volt {
    type Output = Joule;

    fn mul(self, rhs: Coulomb) -> Self::Output {
        Joule::new(self.0 * rhs.value())
    }
}

impl Div<NewtonPerCoulomb> for Volt {
    type Output = Meter;

    fn div(self, rhs: NewtonPerCoulomb) -> Self::Output {
        Meter::new(self.0 / rhs.value())
    }
}

impl<T> Div<T> for Volt
where
    T: ToMeter,
{
    type Output = NewtonPerCoulomb;

    fn div(self, rhs: T) -> Self::Output {
        NewtonPerCoulomb::new(self.0 / rhs.to_meter().value())
    }
}

impl<T> Mul<T> for Volt
where
    T: ToFarad,
{
    type Output = Farad;

    fn mul(self, rhs: T) -> Self::Output {
        Farad::new(self.0 * rhs.to_farad().value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let volt = Volt::new(10.0);
        assert_eq!(volt.value(), 10.0);
    }

    #[test]
    fn to_joule() {
        let volt = Volt::new(10.0);
        let coulomb = Coulomb::new(2.0);
        let joule = volt * coulomb;
        assert_eq!(joule, Joule::new(20.0));
    }

    #[test]
    fn to_meter() {
        let volt = Volt::new(10.0);
        let newton_per_coulomb = NewtonPerCoulomb::new(2.0);
        let meter = volt / newton_per_coulomb;
        assert_eq!(meter, Meter::new(5.0));
    }
}
