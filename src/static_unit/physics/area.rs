use crate::static_unit::physics::length::{Meter, ToMeter};
use std::ops::{Div, Mul};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct SquareMeter(f64);

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct SquareKilometer(f64);

impl SquareMeter {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl SquareKilometer {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

pub trait ToSquareMeter {
    fn to_square_meter(&self) -> SquareMeter;
}

pub trait ToSquareKilometer {
    fn to_square_kilometer(&self) -> SquareKilometer;
}

impl ToSquareMeter for SquareKilometer {
    fn to_square_meter(&self) -> SquareMeter {
        SquareMeter::new(self.value() * 1_000_000.0)
    }
}

impl ToSquareKilometer for SquareMeter {
    fn to_square_kilometer(&self) -> SquareKilometer {
        SquareKilometer::new(self.value() / 1_000_000.0)
    }
}

impl ToSquareMeter for SquareMeter {
    fn to_square_meter(&self) -> SquareMeter {
        *self
    }
}

impl ToSquareKilometer for SquareKilometer {
    fn to_square_kilometer(&self) -> SquareKilometer {
        *self
    }
}

impl<T> Div<T> for SquareMeter
where
    T: ToMeter,
{
    type Output = Meter;

    fn div(self, rhs: T) -> Meter {
        Meter::new(self.value() / rhs.to_meter().value())
    }
}

impl<T> Mul<T> for SquareMeter
where
    T: ToMeter,
{
    type Output = Meter;

    fn mul(self, rhs: T) -> Meter {
        Meter::new(self.value() * rhs.to_meter().value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numerical_conversion() {
        let square_meter = SquareMeter::new(1.0);
        let square_kilometer = SquareKilometer::new(1.0);

        assert_eq!(square_meter.to_square_kilometer().value(), 0.000001);
        assert_eq!(square_kilometer.to_square_meter().value(), 1_000_000.0);
    }
}
