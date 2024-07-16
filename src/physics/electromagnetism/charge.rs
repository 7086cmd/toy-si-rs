use crate::general::time::{Second, ToSecond};
use crate::physics::electromagnetism::current::Ampere;
use crate::physics::electromagnetism::current_element::AmpereMeter;
use crate::physics::velocity::ToMeterPerSecond;
use std::ops::{Add, Div, Mul};

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

impl<T> Div<T> for Coulomb
where
    T: ToSecond,
{
    type Output = Ampere;

    fn div(self, rhs: T) -> Self::Output {
        Ampere::new(self.0 / rhs.to_second().value())
    }
}

impl<T> Mul<T> for Coulomb
where
    T: ToMeterPerSecond,
{
    type Output = AmpereMeter;

    fn mul(self, rhs: T) -> Self::Output {
        AmpereMeter::new(self.0 * rhs.to_meter_per_second().value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coulomb_new() {
        let value = 1.0;
        let coulomb = Coulomb::new(value);
        assert_eq!(value, coulomb.value());
    }

    #[test]
    fn coulomb_add_coulomb() {
        let value = 1.0;
        let coulomb = Coulomb::new(value);
        assert_eq!(Coulomb::new(value + value), coulomb + coulomb);
    }

    #[test]
    fn coulomb_div_second() {
        let coulomb = Coulomb::new(120.0);
        let time = Second::new(60.0);
        assert_eq!(Ampere::new(2.0), coulomb / time);
    }
}
