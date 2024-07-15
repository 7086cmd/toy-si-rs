use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Second(f32);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Millisecond(f32);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Minute(f32);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Hour(f32);

impl Second {
    pub fn new(value: f32) -> Self {
        Self(value)
    }

    pub fn to_millisecond(&self) -> Millisecond {
        Millisecond(self.0 * 1000.0)
    }

    pub fn to_minute(&self) -> Minute {
        Minute(self.0 / 60.0)
    }

    pub fn to_hour(&self) -> Hour {
        Hour(self.0 / 3600.0)
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

impl Millisecond {
    pub fn new(value: f32) -> Self {
        Self(value)
    }

    pub fn to_second(&self) -> Second {
        Second(self.0 / 1000.0)
    }

    pub fn to_minute(&self) -> Minute {
        Minute(self.0 / 60000.0)
    }

    pub fn to_hour(&self) -> Hour {
        Hour(self.0 / 3600000.0)
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

impl Minute {
    pub fn new(value: f32) -> Self {
        Self(value)
    }

    pub fn to_second(&self) -> Second {
        Second(self.0 * 60.0)
    }

    pub fn to_millisecond(&self) -> Millisecond {
        Millisecond(self.0 * 60000.0)
    }

    pub fn to_hour(&self) -> Hour {
        Hour(self.0 / 60.0)
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

impl Hour {
    pub fn new(value: f32) -> Self {
        Self(value)
    }

    pub fn to_second(&self) -> Second {
        Second(self.0 * 3600.0)
    }

    pub fn to_millisecond(&self) -> Millisecond {
        Millisecond(self.0 * 3600000.0)
    }

    pub fn to_minute(&self) -> Minute {
        Minute(self.0 * 60.0)
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

impl Add for Second {
    type Output = Second;

    fn add(self, other: Second) -> Second {
        Second(self.0 + other.0)
    }
}

impl Add<Millisecond> for Second {
    type Output = Second;

    fn add(self, other: Millisecond) -> Second {
        Second(self.0 + other.to_second().0)
    }
}

impl Add<Second> for Millisecond {
    type Output = Second;

    fn add(self, other: Second) -> Second {
        Second(self.to_second().0 + other.0)
    }
}

impl Add for Millisecond {
    type Output = Millisecond;

    fn add(self, other: Millisecond) -> Millisecond {
        Millisecond(self.0 + other.0)
    }
}

impl Add for Minute {
    type Output = Minute;

    fn add(self, other: Minute) -> Minute {
        Minute(self.0 + other.0)
    }
}

impl Add for Hour {
    type Output = Hour;

    fn add(self, other: Hour) -> Hour {
        Hour(self.0 + other.0)
    }
}

impl Add<Second> for Minute {
    type Output = Minute;

    fn add(self, other: Second) -> Minute {
        Minute(self.0 + other.to_minute().0)
    }
}

impl Add<Minute> for Second {
    type Output = Minute;

    fn add(self, other: Minute) -> Minute {
        Minute(self.to_minute().0 + other.0)
    }
}

impl Add<Second> for Hour {
    type Output = Hour;

    fn add(self, other: Second) -> Hour {
        Hour(self.0 + other.to_hour().0)
    }
}

impl Add<Hour> for Second {
    type Output = Hour;

    fn add(self, other: Hour) -> Hour {
        Hour(self.to_hour().0 + other.0)
    }
}

impl Add<Minute> for Hour {
    type Output = Hour;

    fn add(self, other: Minute) -> Hour {
        Hour(self.0 + other.to_hour().0)
    }
}

impl Add<Hour> for Minute {
    type Output = Hour;

    fn add(self, other: Hour) -> Hour {
        Hour(self.to_hour().0 + other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let second = Second::new(1.0);
        let millisecond = Millisecond::new(1000.0);
        let minute = Minute::new(1.0);
        let hour = Hour::new(1.0);

        assert_eq!(second + second, Second::new(2.0));
        assert_eq!(second + millisecond, Second::new(2.0));
        assert_eq!(millisecond + second, Second::new(2.0));
        assert_eq!(millisecond + millisecond, Millisecond::new(2000.0));
        assert_eq!(minute + minute, Minute::new(2.0));
        assert_eq!(hour + hour, Hour::new(2.0));
        assert_eq!(minute + second, Minute::new(1.0166667));
        assert_eq!(second + minute, Minute::new(1.0166667));
        assert_eq!(hour + second, Hour::new(1.0002778));
        assert_eq!(second + hour, Hour::new(1.0002778));
        assert_eq!(hour + minute, Hour::new(1.0166667));
        assert_eq!(minute + hour, Hour::new(1.0166667));
    }
}