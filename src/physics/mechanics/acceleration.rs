use crate::general::time::{Second, ToSecond};
use crate::physics::velocity::MeterPerSecond;
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MeterPerSquareSecond(f64);

impl MeterPerSquareSecond {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Add<MeterPerSquareSecond> for MeterPerSquareSecond {
    type Output = MeterPerSquareSecond;

    fn add(self, rhs: MeterPerSquareSecond) -> Self::Output {
        MeterPerSquareSecond(self.0 + rhs.0)
    }
}

impl<T> Mul<T> for MeterPerSquareSecond
where
    T: ToSecond,
{
    type Output = MeterPerSecond;

    fn mul(self, rhs: T) -> Self::Output {
        MeterPerSecond::new(self.0 * rhs.to_second().value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let acceleration = MeterPerSquareSecond::new(10.0);
        let time = Second::new(2.0);
        let velocity = acceleration * time;
        assert_eq!(velocity, MeterPerSecond::new(20.0));
    }
}
