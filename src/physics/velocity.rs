use std::ops::{Div, Mul, Add};
use crate::general::phantom::PhantomUnit;
use crate::general::time::Second;
use crate::physics::length::Meter;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MeterPerSecond(f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct KilometerPerHour(f64);

impl MeterPerSecond {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn to_kilometer_per_hour(&self) -> KilometerPerHour {
        KilometerPerHour(self.0 * 3.6)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl KilometerPerHour {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn to_meter_per_second(&self) -> MeterPerSecond {
        MeterPerSecond(self.0 / 3.6)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Add<KilometerPerHour> for MeterPerSecond {
    type Output = MeterPerSecond;

    fn add(self, rhs: KilometerPerHour) -> Self::Output {
        MeterPerSecond(self.0 + rhs.to_meter_per_second().0)
    }
}

impl Add<MeterPerSecond> for KilometerPerHour {
    type Output = KilometerPerHour;

    fn add(self, rhs: MeterPerSecond) -> Self::Output {
        KilometerPerHour(self.0 + rhs.to_kilometer_per_hour().0)
    }
}

impl Mul<Second> for MeterPerSecond {
    type Output = Meter;

    fn mul(self, rhs: Second) -> Self::Output {
        Meter::new(self.0 * rhs.value())
    }
}

impl Mul<PhantomUnit> for MeterPerSecond {
    type Output = MeterPerSecond;

    fn mul(self, rhs: PhantomUnit) -> Self::Output {
        MeterPerSecond::new(self.0 * rhs.value())
    }
}

impl Div<Meter> for MeterPerSecond {
    type Output = Second;

    fn div(self, rhs: Meter) -> Self::Output {
        Second::new(self.0 / rhs.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_conversation() {
        let mps = MeterPerSecond::new(1.0);
        let kph = KilometerPerHour::new(3.6);
        assert_eq!(mps.to_kilometer_per_hour(), kph);
        assert_eq!(kph.to_meter_per_second(), mps);
    }

    #[test]
    fn add() {
        let mps = MeterPerSecond::new(1.0);
        let kph = KilometerPerHour::new(3.6);
        assert_eq!(mps + kph, MeterPerSecond::new(2.0));
    }

    #[test]
    fn mul() {
        let mps = MeterPerSecond::new(1.0);
        let second = Second::new(1.0);
        assert_eq!(mps * second, Meter::new(1.0));
    }

    #[test]
    fn div() {
        let mps = MeterPerSecond::new(1.0);
        let meter = Meter::new(1.0);
        assert_eq!(mps / meter, Second::new(1.0));
    }
}