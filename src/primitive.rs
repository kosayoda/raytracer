use derive_more::{Add, AddAssign, From, Mul};
use image::Rgb;
use serde::Deserialize;

pub type Vec3 = glam::Vec3;
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
        self.origin + t * self.direction
    }
}
