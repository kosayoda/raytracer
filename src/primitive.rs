use derive_more::{Add, AddAssign, Deref, DerefMut, Div, From, Mul, Neg, Sub};
use image::Rgb;
use serde::Deserialize;

#[derive(
    Add,
    AddAssign,
    Sub,
    Neg,
    Mul,
    Div,
    Deref,
    DerefMut,
    From,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Deserialize,
)]
pub struct Vec3(glam::Vec3);

impl Vec3 {
    pub const ZERO: Vec3 = Self(glam::Vec3::ZERO);

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(glam::Vec3::new(x, y, z))
    }
}

pub type Point = Vec3;

#[derive(Add, AddAssign, Mul, From, Clone, Copy, Debug, Deserialize)]
pub struct Color(Vec3);

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self(Vec3::new(r, g, b))
    }
}

impl From<Color> for Rgb<u8> {
    fn from(value: Color) -> Self {
        let f = value.0;
        Rgb([
            (f.x.clamp(0.0, 0.999) * 256.0) as u8,
            (f.y.clamp(0.0, 0.999) * 256.0) as u8,
            (f.z.clamp(0.0, 0.999) * 256.0) as u8,
        ])
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ray {
    origin: Point,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(self) -> Point {
        self.origin
    }

    pub fn direction(self) -> Vec3 {
        self.direction
    }

    /// Get the point along the vector at a certain param t
    pub fn at(self, t: f32) -> Point {
        self.origin + self.direction * t
    }
}
