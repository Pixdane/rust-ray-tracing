use nalgebra::ComplexField;

use crate::interval::Interval;

extern crate nalgebra as na;

// #[derive(Debug, PartialEq, PartialOrd)]
// pub struct Color(pub na::Vector3<f64>);

pub type Color = na::Vector3<f64>;

pub trait Color3 {
    const INTENSITY: Interval;
    fn gamma_correction(_: f64) -> f64;
    fn r(&self) -> f64;
    fn g(&self) -> f64;
    fn b(&self) -> f64;
    fn r_byte(&self) -> u8;
    fn g_byte(&self) -> u8;
    fn b_byte(&self) -> u8;
    fn write(&self) -> String;
}

impl Color3 for Color {
    const INTENSITY: Interval = Interval::new(0., 0.999);

    fn gamma_correction(color: f64) -> f64 {
        color.try_sqrt().unwrap_or(0.)
    }

    fn r(&self) -> f64 {
        self.x
    }

    fn g(&self) -> f64 {
        self.y
    }

    fn b(&self) -> f64 {
        self.z
    }

    fn r_byte(&self) -> u8 {
        (255.999 * Color::INTENSITY.clamps(Self::gamma_correction(self.r()))).trunc() as u8
    }

    fn g_byte(&self) -> u8 {
        (255.999 * Color::INTENSITY.clamps(Self::gamma_correction(self.g()))).trunc() as u8
    }

    fn b_byte(&self) -> u8 {
        (255.999 * Color::INTENSITY.clamps(Self::gamma_correction(self.b()))).trunc() as u8
    }

    fn write(&self) -> String {
        format!("{} {} {}", self.r_byte(), self.g_byte(), self.b_byte())
    }
}
