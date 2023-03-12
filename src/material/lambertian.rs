use rand::rngs::ThreadRng;
use serde::Deserialize;

use crate::{
    object::HitRecord,
    primitive::{Color, Ray, Vec3},
};

use super::{ScatterResult, Scatterable};

#[derive(Debug, PartialEq, Clone, Copy, Deserialize)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Scatterable for Lambertian {
    fn scatter(&self, rng: &mut ThreadRng, _: &Ray, record: &HitRecord) -> Option<ScatterResult> {
        let mut scatter_direction = record.normal + Vec3::new_random_unit_vector(rng);
        if scatter_direction.is_near_zero() {
            scatter_direction = record.normal;
        }

        Some(ScatterResult {
            ray: Ray::new(record.point, scatter_direction),
            attenuation: self.albedo,
        })
    }
}