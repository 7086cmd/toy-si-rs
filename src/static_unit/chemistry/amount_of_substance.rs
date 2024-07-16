use crate::BasicUnit;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Mole(f64);

impl BasicUnit for Mole {}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Millimole(f64);

impl Mole {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Millimole {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

pub trait ToMole {
    fn to_mole(&self) -> Mole;
}

impl ToMole for Millimole {
    fn to_mole(&self) -> Mole {
        Mole::new(self.value() / 1000.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let millimole = Millimole::new(1.0);
        let mole = millimole.to_mole();
        assert_eq!(mole.value(), 0.001);
    }
}
