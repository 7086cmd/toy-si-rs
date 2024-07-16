use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Second(f64);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Millisecond(f64);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Minute(f64);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Hour(f64);

impl Second {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Millisecond {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Minute {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Hour {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

pub trait ToSecond {
    fn to_second(&self) -> Second;
}

trait ToMillisecond {
    fn to_millisecond(&self) -> Millisecond;
}

pub trait ToMinute {
    fn to_minute(&self) -> Minute;
}

pub trait ToHour {
    fn to_hour(&self) -> Hour;
}

impl ToSecond for Second {
    fn to_second(&self) -> Second {
        *self
    }
}

impl ToSecond for Millisecond {
    fn to_second(&self) -> Second {
        Second(self.0 / 1000.0)
    }
}

impl ToSecond for Minute {
    fn to_second(&self) -> Second {
        Second(self.0 * 60.0)
    }
}

impl ToSecond for Hour {
    fn to_second(&self) -> Second {
        Second(self.0 * 3600.0)
    }
}

impl ToMillisecond for Second {
    fn to_millisecond(&self) -> Millisecond {
        Millisecond(self.0 * 1000.0)
    }
}

impl ToMillisecond for Minute {
    fn to_millisecond(&self) -> Millisecond {
        Millisecond(self.0 * 60.0 * 1000.0)
    }
}

impl ToMillisecond for Hour {
    fn to_millisecond(&self) -> Millisecond {
        Millisecond(self.0 * 3600.0 * 1000.0)
    }
}

impl ToMinute for Second {
    fn to_minute(&self) -> Minute {
        Minute(self.0 / 60.0)
    }
}

impl ToMinute for Millisecond {
    fn to_minute(&self) -> Minute {
        Minute(self.0 / 1000.0 / 60.0)
    }
}

impl ToMinute for Hour {
    fn to_minute(&self) -> Minute {
        Minute(self.0 * 60.0)
    }
}

impl ToHour for Second {
    fn to_hour(&self) -> Hour {
        Hour(self.0 / 3600.0)
    }
}

impl ToHour for Millisecond {
    fn to_hour(&self) -> Hour {
        Hour(self.0 / 1000.0 / 3600.0)
    }
}

impl ToHour for Minute {
    fn to_hour(&self) -> Hour {
        Hour(self.0 / 60.0)
    }
}

impl<T> Add<T> for Second
where
    T: ToSecond,
{
    type Output = Second;

    fn add(self, rhs: T) -> Self::Output {
        Second(self.0 + rhs.to_second().0)
    }
}

impl<T> Add<T> for Millisecond
where
    T: ToMillisecond,
{
    type Output = Millisecond;

    fn add(self, rhs: T) -> Self::Output {
        Millisecond(self.0 + rhs.to_millisecond().0)
    }
}

impl<T> Add<T> for Minute
where
    T: ToMinute,
{
    type Output = Minute;

    fn add(self, rhs: T) -> Self::Output {
        Minute(self.0 + rhs.to_minute().0)
    }
}

impl<T> Add<T> for Hour
where
    T: ToHour,
{
    type Output = Hour;

    fn add(self, rhs: T) -> Self::Output {
        Hour(self.0 + rhs.to_hour().0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let second = Second(10.0);
        let millisecond = Millisecond(200.0);
        let sum = second + millisecond;
        assert_eq!(sum, Second(10.2));
    }

    #[test]
    fn it_works_2() {
        let minute = Minute(1.0);
        let hour = Hour(1.0);
        let sum = minute + hour;
        assert_eq!(sum, Minute(61.0));
    }
}
