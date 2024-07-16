use crate::static_unit::physics::electromagnetism::magnetic_flux_density::Tesla;
use crate::static_unit::physics::mechanics::force::Newton;
use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct AmpereMeter(f64);

impl AmpereMeter {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

pub trait ToAmpereMeter {
    fn to_ampere_meter(&self) -> AmpereMeter;
}

impl ToAmpereMeter for AmpereMeter {
    fn to_ampere_meter(&self) -> AmpereMeter {
        *self
    }
}

impl Mul<Tesla> for AmpereMeter {
    type Output = Newton;

    fn mul(self, magnetic_flux_density: Tesla) -> Newton {
        Newton::new(self.0 * magnetic_flux_density.value())
    }
}
