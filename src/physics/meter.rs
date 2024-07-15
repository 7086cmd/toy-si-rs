use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Meter(f32);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Kilometer(f32);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Centimeter(f32);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Millimeter(f32);

impl Meter {
    pub fn new(value: f32) -> Self {
        Self(value)
    }

    pub fn to_kilometer(&self) -> Kilometer {
        Kilometer(self.0 / 1000.0)
    }

    pub fn to_centimeter(&self) -> Centimeter {
        Centimeter(self.0 * 100.0)
    }

    pub fn to_millimeter(&self) -> Millimeter {
        Millimeter(self.0 * 1000.0)
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

impl Kilometer {
    pub fn new(value: f32) -> Self {
        Self(value)
    }

    pub fn to_meter(&self) -> Meter {
        Meter(self.0 * 1000.0)
    }

    pub fn to_centimeter(&self) -> Centimeter {
        Centimeter(self.0 * 100000.0)
    }

    pub fn to_millimeter(&self) -> Millimeter {
        Millimeter(self.0 * 1000000.0)
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

impl Centimeter {
    pub fn new(value: f32) -> Self {
        Self(value)
    }

    pub fn to_meter(&self) -> Meter {
        Meter(self.0 / 100.0)
    }

    pub fn to_kilometer(&self) -> Kilometer {
        Kilometer(self.0 / 100000.0)
    }

    pub fn to_millimeter(&self) -> Millimeter {
        Millimeter(self.0 * 10.0)
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

impl Millimeter {
    pub fn new(value: f32) -> Self {
        Self(value)
    }

    pub fn to_meter(&self) -> Meter {
        Meter(self.0 / 1000.0)
    }

    pub fn to_kilometer(&self) -> Kilometer {
        Kilometer(self.0 / 1000000.0)
    }

    pub fn to_centimeter(&self) -> Centimeter {
        Centimeter(self.0 / 10.0)
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

impl Add<Millimeter> for Meter {
    type Output = Meter;

    fn add(self, rhs: Millimeter) -> Self::Output {
        Meter(self.0 + rhs.to_meter().value())
    }
}

impl Add<Centimeter> for Meter {
    type Output = Meter;

    fn add(self, rhs: Centimeter) -> Self::Output {
        Meter(self.0 + rhs.to_meter().value())
    }
}

impl Add<Kilometer> for Meter {
    type Output = Meter;

    fn add(self, rhs: Kilometer) -> Self::Output {
        Meter(self.0 + rhs.to_meter().value())
    }
}

impl Add<Meter> for Millimeter {
    type Output = Millimeter;

    fn add(self, rhs: Meter) -> Self::Output {
        Millimeter(self.0 + rhs.to_millimeter().value())
    }
}

impl Add<Centimeter> for Millimeter {
    type Output = Millimeter;

    fn add(self, rhs: Centimeter) -> Self::Output {
        Millimeter(self.0 + rhs.to_millimeter().value())
    }
}

impl Add<Kilometer> for Millimeter {
    type Output = Millimeter;

    fn add(self, rhs: Kilometer) -> Self::Output {
        Millimeter(self.0 + rhs.to_millimeter().value())
    }
}

impl Add<Meter> for Centimeter {
    type Output = Centimeter;

    fn add(self, rhs: Meter) -> Self::Output {
        Centimeter(self.0 + rhs.to_centimeter().value())
    }
}

impl Add<Millimeter> for Centimeter {
    type Output = Centimeter;

    fn add(self, rhs: Millimeter) -> Self::Output {
        Centimeter(self.0 + rhs.to_centimeter().value())
    }
}

impl Add<Kilometer> for Centimeter {
    type Output = Centimeter;

    fn add(self, rhs: Kilometer) -> Self::Output {
        Centimeter(self.0 + rhs.to_centimeter().value())
    }
}

impl Add<Meter> for Kilometer {
    type Output = Kilometer;

    fn add(self, rhs: Meter) -> Self::Output {
        Kilometer(self.0 + rhs.to_kilometer().value())
    }
}

impl Add<Millimeter> for Kilometer {
    type Output = Kilometer;

    fn add(self, rhs: Millimeter) -> Self::Output {
        Kilometer(self.0 + rhs.to_kilometer().value())
    }
}

impl Add<Centimeter> for Kilometer {
    type Output = Kilometer;

    fn add(self, rhs: Centimeter) -> Self::Output {
        Kilometer(self.0 + rhs.to_kilometer().value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let meter = Meter::new(1.0);
        let kilometer = Kilometer::new(1.0);
        let centimeter = Centimeter::new(1.0);
        let millimeter = Millimeter::new(1.0);

        assert_eq!(meter + millimeter, Meter::new(1.001));
        assert_eq!(meter + centimeter, Meter::new(1.01));
        assert_eq!(meter + kilometer, Meter::new(1001.0));

        assert_eq!(millimeter + meter, Millimeter::new(1001.0));
        assert_eq!(millimeter + centimeter, Millimeter::new(11.0));
        assert_eq!(millimeter + kilometer, Millimeter::new(1000001.0));

        assert_eq!(centimeter + meter, Centimeter::new(101.0));
        assert_eq!(centimeter + millimeter, Centimeter::new(1.1));
        assert_eq!(centimeter + kilometer, Centimeter::new(100001.0));

        assert_eq!(kilometer + meter, Kilometer::new(1.001));
        assert_eq!(kilometer + millimeter, Kilometer::new(1.000001));
        assert_eq!(kilometer + centimeter, Kilometer::new(1.00001));
    }
}
