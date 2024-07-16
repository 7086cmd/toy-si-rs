use crate::static_unit::physics::electromagnetism::charge::Coulomb;
use crate::static_unit::physics::electromagnetism::potential::Volt;
use std::ops::{Add, Div, Mul};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Farad(f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Microfarad(f64);

pub trait ToFarad {
    fn to_farad(&self) -> Farad;
}

pub trait ToMicrofarad {
    fn to_microfarad(&self) -> Microfarad;
}

impl Farad {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Microfarad {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl ToFarad for Microfarad {
    fn to_farad(&self) -> Farad {
        Farad::new(self.0 / 1_000_000.0)
    }
}

impl ToMicrofarad for Farad {
    fn to_microfarad(&self) -> Microfarad {
        Microfarad::new(self.0 * 1_000_000.0)
    }
}

impl Add<Farad> for Farad {
    type Output = Farad;

    fn add(self, rhs: Farad) -> Self::Output {
        Farad::new(self.0 + rhs.0)
    }
}

impl<T> Add<T> for Farad
where
    T: ToFarad,
{
    type Output = Farad;

    fn add(self, rhs: T) -> Self::Output {
        Farad::new(self.0 + rhs.to_farad().value())
    }
}

impl Add<Microfarad> for Microfarad {
    type Output = Microfarad;

    fn add(self, rhs: Microfarad) -> Self::Output {
        Microfarad::new(self.0 + rhs.0)
    }
}

impl<T> Add<T> for Microfarad
where
    T: ToMicrofarad,
{
    type Output = Microfarad;

    fn add(self, rhs: T) -> Self::Output {
        Microfarad::new(self.0 + rhs.to_microfarad().value())
    }
}

impl Mul<Volt> for Farad {
    type Output = Coulomb;

    fn mul(self, rhs: Volt) -> Self::Output {
        Coulomb::new(self.0 * rhs.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let farad = Farad::new(10.0);
        assert_eq!(farad.value(), 10.0);
    }

    #[test]
    fn to_microfarad() {
        let farad = Farad::new(10.0);
        let microfarad = farad.to_microfarad();
        assert_eq!(microfarad, Microfarad::new(10_000_000.0));
    }

    #[test]
    fn add_farad() {
        let farad = Farad::new(10.0);
        let farad2 = Farad::new(20.0);
        let result = farad + farad2;
        assert_eq!(result, Farad::new(30.0));
    }

    #[test]
    fn add_microfarad() {
        let microfarad = Microfarad::new(10.0);
        let microfarad2 = Microfarad::new(20.0);
        let result = microfarad + microfarad2;
        assert_eq!(result, Microfarad::new(30.0));
    }

    #[test]
    fn add_farad_microfarad() {
        let farad = Farad::new(10.0);
        let microfarad = Microfarad::new(20.0);
        let result = farad + microfarad;
        assert_eq!(result, Farad::new(10.00002));
    }

    #[test]
    fn add_microfarad_farad() {
        let microfarad = Microfarad::new(10.0);
        let farad = Farad::new(20.0);
        let result = microfarad + farad;
        assert_eq!(result, Microfarad::new(20000010.0));
    }

    #[test]
    fn to_coulomb() {
        let farad = Farad::new(10.0);
        let volt = Volt::new(2.0);
        let coulomb = farad * volt;
        assert_eq!(coulomb, Coulomb::new(20.0));
    }
}
