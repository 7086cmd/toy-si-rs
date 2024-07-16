use crate::physics::electromagnetism::current_element::AmpereMeter;
use crate::physics::length::ToMeter;
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Ampere(f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Milliampere(f64);

pub trait ToAmpere {
    fn to_ampere(&self) -> Ampere;
}

pub trait ToMilliampere {
    fn to_milliampere(&self) -> Milliampere;
}

impl Ampere {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Milliampere {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl ToAmpere for Milliampere {
    fn to_ampere(&self) -> Ampere {
        Ampere::new(self.0 / 1000.0)
    }
}

impl ToMilliampere for Ampere {
    fn to_milliampere(&self) -> Milliampere {
        Milliampere::new(self.0 * 1000.0)
    }
}

impl Add<Ampere> for Ampere {
    type Output = Ampere;

    fn add(self, rhs: Ampere) -> Self::Output {
        Ampere::new(self.0 + rhs.0)
    }
}

impl Add<Milliampere> for Milliampere {
    type Output = Milliampere;

    fn add(self, rhs: Milliampere) -> Self::Output {
        Milliampere::new(self.0 + rhs.0)
    }
}

impl<T> Add<T> for Ampere
where
    T: ToAmpere,
{
    type Output = Ampere;

    fn add(self, rhs: T) -> Self::Output {
        Ampere::new(self.0 + rhs.to_ampere().0)
    }
}

impl<T> Add<T> for Milliampere
where
    T: ToMilliampere,
{
    type Output = Milliampere;

    fn add(self, rhs: T) -> Self::Output {
        Milliampere::new(self.0 + rhs.to_milliampere().0)
    }
}

impl<T> Mul<T> for Ampere
where
    T: ToMeter,
{
    type Output = AmpereMeter;

    fn mul(self, rhs: T) -> Self::Output {
        AmpereMeter::new(self.0 * rhs.to_meter().value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::length::{Centimeter, Meter};

    #[test]
    fn it_works() {
        let current = Ampere::new(10.0);
        let current_2 = Milliampere::new(200.0);
        let current_3 = current + current_2;
        assert_eq!(current_3, Ampere::new(10.2));
    }

    #[test]
    fn it_works_2() {
        let current = Ampere::new(10.0);
        let current_2 = Milliampere::new(200.0);
        let current_3 = current + current_2;
        assert_eq!(current_3, Ampere::new(10.2));
    }

    #[test]
    fn it_works_3() {
        let current = Ampere::new(10.0);
        let distance = Centimeter::new(200.0);
        let current_element = current * distance;
        assert_eq!(current_element, AmpereMeter::new(20.0));
    }
}
