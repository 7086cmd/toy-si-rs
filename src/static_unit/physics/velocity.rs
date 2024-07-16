use crate::static_unit::general::phantom::PhantomUnit;
use crate::static_unit::general::time::{Second, ToSecond};
use crate::static_unit::physics::length::Meter;
use crate::static_unit::physics::mechanics::acceleration::MeterPerSquareSecond;
use std::ops::{Add, Div, Mul};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MeterPerSecond(f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct KilometerPerHour(f64);

impl MeterPerSecond {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl KilometerPerHour {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

pub trait ToMeterPerSecond {
    fn to_meter_per_second(&self) -> MeterPerSecond;
}

pub trait ToKilometerPerHour {
    fn to_kilometer_per_hour(&self) -> KilometerPerHour;
}

impl ToMeterPerSecond for KilometerPerHour {
    fn to_meter_per_second(&self) -> MeterPerSecond {
        MeterPerSecond::new(self.0 / 3.6)
    }
}

impl ToKilometerPerHour for MeterPerSecond {
    fn to_kilometer_per_hour(&self) -> KilometerPerHour {
        KilometerPerHour::new(self.0 * 3.6)
    }
}

impl<T> Add<T> for MeterPerSecond
where
    T: ToMeterPerSecond,
{
    type Output = MeterPerSecond;

    fn add(self, rhs: T) -> Self::Output {
        MeterPerSecond(self.0 + rhs.to_meter_per_second().0)
    }
}

impl<T> Add<T> for KilometerPerHour
where
    T: ToKilometerPerHour,
{
    type Output = KilometerPerHour;

    fn add(self, rhs: T) -> Self::Output {
        KilometerPerHour(self.0 + rhs.to_kilometer_per_hour().0)
    }
}

impl<T> Mul<T> for MeterPerSecond
where
    T: ToSecond,
{
    type Output = Meter;

    fn mul(self, rhs: T) -> Self::Output {
        Meter::new(self.0 * rhs.to_second().value())
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

impl<T> Div<T> for MeterPerSecond
where
    T: ToSecond,
{
    type Output = MeterPerSquareSecond;

    fn div(self, rhs: T) -> Self::Output {
        MeterPerSquareSecond::new(self.0 / rhs.to_second().value())
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
