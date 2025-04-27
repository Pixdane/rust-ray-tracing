use crate::INFINITY;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamps(&self, x: f64) -> f64 {
        match x {
            x if x < self.min => self.min,
            x if x > self.max => self.max,
            _ => x,
        }
    }

    pub const EMPTY: Self = Interval::new(INFINITY, -INFINITY);

    pub const UNIVERSE: Self = Interval::new(-INFINITY, INFINITY);
}
