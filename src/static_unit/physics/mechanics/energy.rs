use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Joule(f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Kilojoule(f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct KilowattHour(f64);

impl Joule {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn to_kilojoule(&self) -> Kilojoule {
        Kilojoule(self.0 / 1000.0)
    }

    pub fn to_kilowatt_hour(&self) -> KilowattHour {
        KilowattHour(self.0 / 3600000.0)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Kilojoule {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn to_joule(&self) -> Joule {
        Joule(self.0 * 1000.0)
    }

    pub fn to_kilowatt_hour(&self) -> KilowattHour {
        KilowattHour(self.0 / 3.6)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl KilowattHour {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn to_joule(&self) -> Joule {
        Joule(self.0 * 3600000.0)
    }

    pub fn to_kilojoule(&self) -> Kilojoule {
        Kilojoule(self.0 * 3.6)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

pub trait ToJoule {
    fn to_joule(&self) -> Joule;
}

pub trait ToKilojoule {
    fn to_kilojoule(&self) -> Kilojoule;
}

pub trait ToKilowattHour {
    fn to_kilowatt_hour(&self) -> KilowattHour;
}

impl ToJoule for Kilojoule {
    fn to_joule(&self) -> Joule {
        Joule::new(self.0 * 1000.0)
    }
}

impl ToJoule for KilowattHour {
    fn to_joule(&self) -> Joule {
        Joule::new(self.0 * 3600000.0)
    }
}

impl ToKilojoule for Joule {
    fn to_kilojoule(&self) -> Kilojoule {
        Kilojoule::new(self.0 / 1000.0)
    }
}

impl ToKilojoule for KilowattHour {
    fn to_kilojoule(&self) -> Kilojoule {
        Kilojoule::new(self.0 / 3.6)
    }
}

impl ToKilowattHour for Joule {
    fn to_kilowatt_hour(&self) -> KilowattHour {
        KilowattHour::new(self.0 / 3600000.0)
    }
}

impl ToKilowattHour for Kilojoule {
    fn to_kilowatt_hour(&self) -> KilowattHour {
        KilowattHour::new(self.0 * 3.6)
    }
}

impl Add<Joule> for Joule {
    type Output = Joule;

    fn add(self, rhs: Joule) -> Self::Output {
        Joule(self.0 + rhs.0)
    }
}

// Support types that implement the `ToJoule` trait
impl<T> Add<T> for Joule
where
    T: ToJoule,
{
    type Output = Joule;

    fn add(self, rhs: T) -> Self::Output {
        Joule(self.0 + rhs.to_joule().0)
    }
}

impl Add<Kilojoule> for Kilojoule {
    type Output = Kilojoule;

    fn add(self, rhs: Kilojoule) -> Self::Output {
        Kilojoule(self.0 + rhs.0)
    }
}

impl Add<KilowattHour> for KilowattHour {
    type Output = KilowattHour;

    fn add(self, rhs: KilowattHour) -> Self::Output {
        KilowattHour(self.0 + rhs.0)
    }
}

impl<T> Add<T> for Kilojoule
where
    T: ToKilojoule,
{
    type Output = Kilojoule;

    fn add(self, rhs: T) -> Self::Output {
        Kilojoule(self.0 + rhs.to_kilojoule().0)
    }
}

impl<T> Add<T> for KilowattHour
where
    T: ToKilowattHour,
{
    type Output = KilowattHour;

    fn add(self, rhs: T) -> Self::Output {
        KilowattHour(self.0 + rhs.to_kilowatt_hour().0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let energy = Joule::new(10.0);
        let energy_2 = Joule::new(20.0);
        let total_energy = energy + energy_2;
        assert_eq!(total_energy, Joule::new(30.0));
    }

    #[test]
    fn it_works_2() {
        let energy = Joule::new(10.0);
        let energy_2 = Kilojoule::new(0.02);
        let total_energy = energy + energy_2;
        assert_eq!(total_energy, Joule::new(30.0));
    }

    #[test]
    fn it_works_3() {
        let energy = Joule::new(10.0);
        let energy_2 = KilowattHour::new(0.0000027777777777777776);
        let total_energy = energy + energy_2;
        assert_eq!(total_energy, Joule::new(20.0));
    }

    #[test]
    fn it_works_4() {
        let energy = Kilojoule::new(0.01);
        let energy_2 = Kilojoule::new(0.01);
        let total_energy = energy + energy_2;
        assert_eq!(total_energy, Kilojoule::new(0.02));
    }
}
