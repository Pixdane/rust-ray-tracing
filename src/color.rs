extern crate nalgebra as na;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Color(pub na::Vector3<f64>);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color(na::Vector3::new(r, g, b))
    }

    pub fn r(&self) -> f64 {
        self.x
    }

    pub fn g(&self) -> f64 {
        self.y
    }

    pub fn b(&self) -> f64 {
        self.z
    }

    pub fn r_byte(&self) -> u8 {
        (255.999 * self.r()).trunc() as u8
    }

    pub fn g_byte(&self) -> u8 {
        (255.999 * self.g()).trunc() as u8
    }

    pub fn b_byte(&self) -> u8 {
        (255.999 * self.b()).trunc() as u8
    }
}

impl std::ops::Deref for Color {
    type Target = na::Vector3<f64>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.r_byte(), self.g_byte(), self.b_byte())
    }
}
