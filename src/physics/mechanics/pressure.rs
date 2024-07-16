use crate::physics::area::{SquareMeter, ToSquareMeter};
use crate::physics::mechanics::force::Newton;
use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Pascal(f64);

impl Pascal {
    pub fn new(value: f64) -> Self {
        Pascal(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl<T> Mul<T> for Pascal
where
    T: ToSquareMeter,
{
    type Output = Newton;

    fn mul(self, rhs: T) -> Self::Output {
        Newton::new(self.0 * rhs.to_square_meter().value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::area::SquareKilometer;

    #[test]
    fn numeric_conversion() {
        let pascal = Pascal::new(10.0);
        let square_kilometer = SquareKilometer::new(2.0);
        let newton = pascal * square_kilometer;
        assert_eq!(newton.value(), 20_000_000.0);
    }

    #[test]
    fn trans() {
        let pascal = Pascal::new(10.0);
        let square_meter = SquareMeter::new(2.0);
        let newton = pascal * square_meter;
        assert_eq!(newton.value(), 20.0);
    }
}
