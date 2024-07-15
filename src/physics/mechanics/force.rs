use std::ops::{Add, Div, Mul};
use crate::general::mass::Kilogram;
use crate::general::time::Second;
use crate::physics::length::Meter;
use crate::physics::mechanics::acceleration::MeterPerSquareSecond;
use crate::physics::mechanics::energy::Joule;
use crate::physics::mechanics::momentum::KilogramMeterPerSecond;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Newton(f64);

impl Newton {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Add<Newton> for Newton {
    type Output = Newton;

    fn add(self, rhs: Newton) -> Self::Output {
        Newton(self.0 + rhs.0)
    }
}

impl Div<Kilogram> for Newton {
    type Output = MeterPerSquareSecond;

    fn div(self, rhs: Kilogram) -> Self::Output {
        MeterPerSquareSecond::new(self.0 / rhs.value())
    }
}

impl Mul<Meter> for Newton {
    type Output = Joule;

    fn mul(self, rhs: Meter) -> Self::Output {
        Joule::new(self.0 * rhs.value())
    }
}

impl Mul<Second> for Newton {
    type Output = KilogramMeterPerSecond;

    fn mul(self, rhs: Second) -> Self::Output {
        KilogramMeterPerSecond::new(self.0 * rhs.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let force = Newton::new(10.0);
        let mass = Kilogram::new(2.0);
        let acceleration = force / mass;
        assert_eq!(acceleration, MeterPerSquareSecond::new(5.0));
    }

    #[test]
    fn it_works_2() {
        let force = Newton::new(10.0);
        let distance = Meter::new(2.0);
        let work = force * distance;
        assert_eq!(work, Joule::new(20.0));
    }
}
