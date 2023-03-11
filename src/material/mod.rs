use enum_dispatch::enum_dispatch;
use rand::rngs::ThreadRng;
use serde::Deserialize;

use crate::{
    object::HitRecord,
    primitive::{Color, Ray},
};

mod dielectric;
mod lambertian;
mod metal;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

pub struct ScatterResult {
    pub attenuation: Color,
    pub ray: Ray,
}

#[enum_dispatch]
pub trait Scatterable {
    fn scatter(&self, rng: &mut ThreadRng, r_in: &Ray, record: &HitRecord)
        -> Option<ScatterResult>;
}

#[enum_dispatch(Scatterable)]
#[derive(Debug, PartialEq, Clone, Copy, Deserialize)]
pub enum Material {
    Lambertian,
    Metal,
    Dielectric,
}
