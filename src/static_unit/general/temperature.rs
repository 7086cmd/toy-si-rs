use crate::BasicUnit;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Kelvin(f64);

impl BasicUnit for Kelvin {}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Celsius(f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Fahrenheit(f64);

impl Kelvin {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Celsius {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Fahrenheit {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

pub trait ToKelvin {
    fn to_kelvin(&self) -> Kelvin;
}

impl ToKelvin for Celsius {
    fn to_kelvin(&self) -> Kelvin {
        Kelvin::new(self.0 + 273.15)
    }
}

impl ToKelvin for Fahrenheit {
    fn to_kelvin(&self) -> Kelvin {
        Kelvin::new((self.0 - 32.0) * 5.0 / 9.0 + 273.15)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn celsius_to_kelvin() {
        let value = 0.0;
        let celsius = Celsius::new(value);
        let kelvin = celsius.to_kelvin();
        assert_eq!(Kelvin::new(273.15), kelvin);
    }

    #[test]
    fn fahrenheit_to_kelvin() {
        let value = 32.0;
        let fahrenheit = Fahrenheit::new(value);
        let kelvin = fahrenheit.to_kelvin();
        assert_eq!(Kelvin::new(273.15), kelvin);
    }

    #[test]
    fn fahrenheit_to_celsius() {
        let fahrenheit = Fahrenheit::new(212.0);
        let celsius = Celsius::new(100.0);
        assert_eq!(celsius.to_kelvin(), fahrenheit.to_kelvin());
    }
}