extern crate nalgebra as na;

pub type Point = na::Point3<f64>;
pub type Vector = na::Vector3<f64>;

pub trait R3<T: Copy> {
    fn x(&self) -> T;
    fn y(&self) -> T;
    fn z(&self) -> T;
}

impl<T: Copy> R3<T> for na::Vector3<T> {
    fn x(&self) -> T {
        self[0]
    }

    fn y(&self) -> T {
        self[1]
    }

    fn z(&self) -> T {
        self[2]
    }
}
