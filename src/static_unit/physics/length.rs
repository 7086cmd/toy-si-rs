use crate::static_unit::general::time::{Hour, Second};
use crate::static_unit::physics::velocity::{KilometerPerHour, MeterPerSecond};
use std::ops::Add;
use std::ops::Div;
use crate::BasicUnit;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Meter(f64);

impl BasicUnit for Meter {}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Kilometer(f64);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Centimeter(f64);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Millimeter(f64);

impl Meter {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Kilometer {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Centimeter {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Millimeter {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

pub trait ToMeter {
    fn to_meter(&self) -> Meter;
}

pub trait ToKilometer {
    fn to_kilometer(&self) -> Kilometer;
}

pub trait ToCentimeter {
    fn to_centimeter(&self) -> Centimeter;
}

pub trait ToMillimeter {
    fn to_millimeter(&self) -> Millimeter;
}

impl ToMeter for Meter {
    fn to_meter(&self) -> Meter {
        *self
    }
}

impl ToMeter for Kilometer {
    fn to_meter(&self) -> Meter {
        Meter(self.0 * 1000.0)
    }
}

impl ToKilometer for Meter {
    fn to_kilometer(&self) -> Kilometer {
        Kilometer(self.0 / 1000.0)
    }
}

impl ToMeter for Centimeter {
    fn to_meter(&self) -> Meter {
        Meter(self.0 / 100.0)
    }
}

impl ToCentimeter for Meter {
    fn to_centimeter(&self) -> Centimeter {
        Centimeter(self.0 * 100.0)
    }
}

impl ToMeter for Millimeter {
    fn to_meter(&self) -> Meter {
        Meter(self.0 / 1000.0)
    }
}

impl ToMillimeter for Meter {
    fn to_millimeter(&self) -> Millimeter {
        Millimeter(self.0 * 1000.0)
    }
}

impl<T> Add<T> for Meter
where
    T: ToMeter,
{
    type Output = Meter;

    fn add(self, rhs: T) -> Self::Output {
        Meter(self.0 + rhs.to_meter().0)
    }
}

impl Add<Kilometer> for Kilometer {
    type Output = Kilometer;

    fn add(self, rhs: Kilometer) -> Self::Output {
        Kilometer(self.0 + rhs.0)
    }
}

impl<T> Add<T> for Kilometer
where
    T: ToKilometer,
{
    type Output = Kilometer;

    fn add(self, rhs: T) -> Self::Output {
        Kilometer(self.0 + rhs.to_kilometer().0)
    }
}

impl Add<Centimeter> for Centimeter {
    type Output = Centimeter;

    fn add(self, rhs: Centimeter) -> Self::Output {
        Centimeter(self.0 + rhs.0)
    }
}

impl<T> Add<T> for Centimeter
where
    T: ToCentimeter,
{
    type Output = Centimeter;

    fn add(self, rhs: T) -> Self::Output {
        Centimeter(self.0 + rhs.to_centimeter().0)
    }
}

impl Add<Millimeter> for Millimeter {
    type Output = Millimeter;

    fn add(self, rhs: Millimeter) -> Self::Output {
        Millimeter(self.0 + rhs.0)
    }
}

impl<T> Add<T> for Millimeter
where
    T: ToMillimeter,
{
    type Output = Millimeter;

    fn add(self, rhs: T) -> Self::Output {
        Millimeter(self.0 + rhs.to_millimeter().0)
    }
}

impl Div<Second> for Meter {
    type Output = MeterPerSecond;

    fn div(self, rhs: Second) -> Self::Output {
        MeterPerSecond::new(self.0 / rhs.value())
    }
}

impl Div<Hour> for Kilometer {
    type Output = KilometerPerHour;

    fn div(self, rhs: Hour) -> Self::Output {
        KilometerPerHour::new(self.0 / rhs.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let distance = Meter::new(10.0);
        let time = Second::new(2.0);
        let velocity = distance / time;
        assert_eq!(velocity, MeterPerSecond::new(5.0));
    }

    #[test]
    fn it_works_2() {
        let distance = Kilometer::new(10.0);
        let time = Hour::new(2.0);
        let velocity = distance / time;
        assert_eq!(velocity, KilometerPerHour::new(5.0));
    }
}
