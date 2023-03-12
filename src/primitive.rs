use std::ops::{Mul, MulAssign};

use derive_more::{
    Add, AddAssign, Deref, DerefMut, Div, From, Into, Mul, MulAssign, Neg, Sub, SubAssign,
};
use image::Rgb;
use rand::{rngs::ThreadRng, Rng};
use serde::{Deserialize, Serialize};

#[derive(
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Neg,
    Mul,
    MulAssign,
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
    Serialize,
)]
pub struct Vec3(glam::Vec3);

impl Vec3 {
    pub const ZERO: Vec3 = Self(glam::Vec3::ZERO);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
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

    pub fn new_random_in_unit_disk(rng: &mut ThreadRng) -> Self {
        let mut p;
        loop {
            p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn new_random_unit_vector(rng: &mut ThreadRng) -> Self {
        Self(Self::new_random_in_unit_sphere(rng).normalize())
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

    pub fn is_near_zero(&self) -> bool {
        self.x.abs() < 1e-8_f32 && self.y.abs() < 1e-8_f32 && self.z.abs() < 1e-8_f32
    }

    pub fn to_rgb(&self, scale: f32) -> [u8; 3] {
        let f = self.0;
        [
            ((f.x * scale).sqrt().clamp(0.0, 0.999) * 255.999) as u8,
            ((f.y * scale).sqrt().clamp(0.0, 0.999) * 255.999) as u8,
            ((f.z * scale).sqrt().clamp(0.0, 0.999) * 255.999) as u8,
        ]
    }

    pub fn reflect(self, normal: &Vec3) -> Self {
        self - *normal * 2.0 * self.dot(**normal)
    }

    pub fn refract(self, normal: &Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = ((-self).dot(**normal)).min(1.0);
        let r_out_perpendicular = (self + *normal * cos_theta) * etai_over_etat;
        let r_out_parallel =
            **normal * -f32::sqrt(f32::abs(1.0 - r_out_perpendicular.length_squared()));
        r_out_perpendicular + r_out_parallel.into()
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        *self = *self * rhs;
    }
}

pub type Point = Vec3;
pub type Color = Vec3;

#[derive(Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    /// Get the point along the vector at a certain param t
    pub fn at(self, t: f32) -> Point {
        self.origin + self.direction * t
    }
}
