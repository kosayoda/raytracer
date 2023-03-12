use derive_more::From;
use rand::rngs::ThreadRng;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, PartialEq, Clone, Copy, Deserialize, Serialize, From)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material {
    pub fn scatter(
        &self,
        rng: &mut ThreadRng,
        r_in: &Ray,
        record: &HitRecord,
    ) -> Option<ScatterResult> {
        match self {
            Material::Lambertian(l) => lambertian::scatter(l, rng, r_in, record),
            Material::Metal(m) => metal::scatter(m, rng, r_in, record),
            Material::Dielectric(d) => dielectric::scatter(d, rng, r_in, record),
        }
    }
}
