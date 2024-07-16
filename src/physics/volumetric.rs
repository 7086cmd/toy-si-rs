use crate::physics::area::SquareMeter;
use crate::physics::length::ToMeter;
use std::ops::Div;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct CubicMeter(f64);

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct CubicCentimeter(f64);

pub type Milliliter = CubicCentimeter;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct CubicDecimeter(f64);

pub type Liter = CubicDecimeter;

impl CubicMeter {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl CubicCentimeter {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl CubicDecimeter {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

pub trait ToCubicMeter {
    fn to_cubic_meter(&self) -> CubicMeter;
}

pub trait ToCubicCentimeter {
    fn to_cubic_centimeter(&self) -> CubicCentimeter;
}

pub trait ToCubicDecimeter {
    fn to_cubic_decimeter(&self) -> CubicDecimeter;
}

impl ToCubicMeter for CubicCentimeter {
    fn to_cubic_meter(&self) -> CubicMeter {
        CubicMeter::new(self.value() / 1_000_000.0)
    }
}

impl ToCubicMeter for CubicDecimeter {
    fn to_cubic_meter(&self) -> CubicMeter {
        CubicMeter::new(self.value() / 1_000.0)
    }
}

impl ToCubicMeter for CubicMeter {
    fn to_cubic_meter(&self) -> CubicMeter {
        *self
    }
}

impl<T> ToCubicCentimeter for T
where
    T: ToCubicMeter,
{
    fn to_cubic_centimeter(&self) -> CubicCentimeter {
        CubicCentimeter::new(self.to_cubic_meter().value() * 1_000_000.0)
    }
}

impl<T> ToCubicDecimeter for T
where
    T: ToCubicMeter,
{
    fn to_cubic_decimeter(&self) -> CubicDecimeter {
        CubicDecimeter::new(self.to_cubic_meter().value() * 1_000.0)
    }
}

impl<T> Div<T> for CubicMeter
where
    T: ToMeter,
{
    type Output = SquareMeter;

    fn div(self, rhs: T) -> Self::Output {
        SquareMeter::new(self.value() / rhs.to_meter().value())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::physics::length::Meter;

    #[test]
    fn numeric_conversation() {
        let cubic_meter = CubicMeter::new(10.0);
        let meter = Meter::new(2.0);
        let square_meter = cubic_meter / meter;
        assert_eq!(square_meter, SquareMeter::new(5.0));
    }

    #[test]
    fn inter_unit() {
        let cubic_meter = CubicMeter::new(10.0);
        let cubic_centimeter = CubicCentimeter::new(10_000_000.0);
        let cubic_decimeter = CubicDecimeter::new(10_000.0);
        assert_eq!(cubic_meter.to_cubic_centimeter(), cubic_centimeter);
        assert_eq!(cubic_meter.to_cubic_decimeter(), cubic_decimeter);
    }
}
