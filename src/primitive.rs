use derive_more::{Add, AddAssign, Deref, DerefMut, Div, From, Into, Mul, Neg, Sub};
use image::Rgb;
use rand::{rngs::ThreadRng, Rng};
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
    Into,
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

    pub fn new_random_range(rng: &mut ThreadRng, _min: f32, _max: f32) -> Self {
        Self(glam::Vec3::new(
            rng.gen_range(_min.._max),
            rng.gen_range(_min.._max),
            rng.gen_range(_min.._max),
        ))
    }

    pub fn new_random(rng: &mut ThreadRng) -> Self {
        Self(glam::Vec3::new(
            rng.gen::<f32>(),
            rng.gen::<f32>(),
            rng.gen::<f32>(),
        ))
    }

    pub fn new_random_in_unit_sphere(rng: &mut ThreadRng) -> Self {
        let mut p;
        loop {
            p = Self::new_random_range(rng, -1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn new_random_unit_vector(rng: &mut ThreadRng) -> Self {
        Self(Self::new_random_in_unit_sphere(rng).normalize_or_zero())
    }

    pub fn new_random_in_hemisphere(rng: &mut ThreadRng, normal: Vec3) -> Self {
        let in_unit_sphere = Self::new_random_in_unit_sphere(rng);
        if in_unit_sphere.dot(*normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn sqrt(&self) -> Self {
        Self(glam::Vec3::new(self.x.sqrt(), self.y.sqrt(), self.z.sqrt()))
    }
}

pub type Point = Vec3;

#[derive(Add, AddAssign, Mul, Deref, DerefMut, From, Into, Clone, Copy, Debug, Deserialize)]
pub struct Color(Vec3);

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self(Vec3::new(r, g, b))
    }

    pub fn to_rgb(&self, scale: f32) -> Rgb<u8> {
        let f = self.0;
        Rgb([
            ((f.x * scale).sqrt().clamp(0.0, 0.999) * 256.0) as u8,
            ((f.y * scale).sqrt().clamp(0.0, 0.999) * 256.0) as u8,
            ((f.z * scale).sqrt().clamp(0.0, 0.999) * 256.0) as u8,
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
