use std::ops::{Add, Mul, Div};
use crate::general::mass::Kilogram;
use crate::general::time::Second;
use crate::physics::mechanics::force::Newton;
use crate::physics::velocity::MeterPerSecond;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct KilogramMeterPerSecond(f64);

pub type NewtonSecond = KilogramMeterPerSecond; // Impulse uses this more.

impl KilogramMeterPerSecond {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Add<KilogramMeterPerSecond> for KilogramMeterPerSecond {
    type Output = KilogramMeterPerSecond;

    fn add(self, rhs: KilogramMeterPerSecond) -> Self::Output {
        KilogramMeterPerSecond(self.0 + rhs.0)
    }
}

impl Div<Kilogram> for KilogramMeterPerSecond {
    type Output = MeterPerSecond;

    fn div(self, mass: Kilogram) -> Self::Output {
        MeterPerSecond::new(self.0 / mass.value())
    }
}

impl Div<Newton> for KilogramMeterPerSecond {
    type Output = MeterPerSecond;

    fn div(self, force: Newton) -> Self::Output {
        MeterPerSecond::new(self.0 / force.value())
    }
}

impl Div<Second> for KilogramMeterPerSecond {
    type Output = Newton;

    fn div(self, time: Second) -> Self::Output {
        Newton::new(self.0 / time.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let momentum = KilogramMeterPerSecond::new(10.0);
        let mass = Kilogram::new(2.0);
        let velocity = momentum / mass;
        assert_eq!(velocity, MeterPerSecond::new(5.0));
    }

    #[test]
    fn it_works_2() {
        let momentum = KilogramMeterPerSecond::new(10.0);
        let force = Newton::new(2.0);
        let time = Second::new(5.0);
        let velocity = momentum / force;
        assert_eq!(velocity, MeterPerSecond::new(5.0));
        let acceleration = momentum / time;
        assert_eq!(acceleration, Newton::new(2.0));
    }
}