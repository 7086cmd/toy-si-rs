use std::ops::Add;
use crate::BasicUnit;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Kilogram(f64);

impl BasicUnit for Kilogram {}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Gram(f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Ton(f64);

impl Kilogram {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn to_gram(&self) -> Gram {
        Gram(self.0 * 1000.0)
    }

    pub fn to_ton(&self) -> Ton {
        Ton(self.0 / 1000.0)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Gram {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn to_kilogram(&self) -> Kilogram {
        Kilogram(self.0 / 1000.0)
    }

    pub fn to_ton(&self) -> Ton {
        Ton(self.0 / 1000000.0)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Ton {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn to_kilogram(&self) -> Kilogram {
        Kilogram(self.0 * 1000.0)
    }

    pub fn to_gram(&self) -> Gram {
        Gram(self.0 * 1000000.0)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

pub trait ToKilogram {
    fn to_kilogram(&self) -> Kilogram;
}

pub trait ToGram {
    fn to_gram(&self) -> Gram;
}

pub trait ToTon {
    fn to_ton(&self) -> Ton;
}

impl ToKilogram for Gram {
    fn to_kilogram(&self) -> Kilogram {
        Kilogram::new(self.0 / 1000.0)
    }
}

impl ToKilogram for Ton {
    fn to_kilogram(&self) -> Kilogram {
        Kilogram::new(self.0 * 1000.0)
    }
}

impl ToGram for Kilogram {
    fn to_gram(&self) -> Gram {
        Gram::new(self.0 * 1000.0)
    }
}

impl ToGram for Ton {
    fn to_gram(&self) -> Gram {
        Gram::new(self.0 * 1000000.0)
    }
}

impl ToTon for Kilogram {
    fn to_ton(&self) -> Ton {
        Ton::new(self.0 / 1000.0)
    }
}

impl ToTon for Gram {
    fn to_ton(&self) -> Ton {
        Ton::new(self.0 / 1000000.0)
    }
}

impl<T> Add<T> for Kilogram
where
    T: ToKilogram,
{
    type Output = Kilogram;

    fn add(self, rhs: T) -> Self::Output {
        Kilogram(self.0 + rhs.to_kilogram().0)
    }
}

impl<T> Add<T> for Gram
where
    T: ToGram,
{
    type Output = Gram;

    fn add(self, rhs: T) -> Self::Output {
        Gram(self.0 + rhs.to_gram().0)
    }
}

impl<T> Add<T> for Ton
where
    T: ToTon,
{
    type Output = Ton;

    fn add(self, rhs: T) -> Self::Output {
        Ton(self.0 + rhs.to_ton().0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let kilogram = Kilogram::new(1.0);
        let gram = Gram::new(1000.0);
        let sum = kilogram + gram;
        assert_eq!(sum, Kilogram::new(2.0));
    }
}
