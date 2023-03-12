use rand::rngs::ThreadRng;
use serde::{Deserialize, Serialize};

use crate::{
    object::HitRecord,
    primitive::{Color, Ray, Vec3},
};

use super::ScatterResult;

#[derive(Debug, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub struct Lambertian {
    pub albedo: Color,
}

pub fn scatter(
    material: &Lambertian,
    rng: &mut ThreadRng,
    _: &Ray,
    record: &HitRecord,
) -> Option<ScatterResult> {
    let mut scatter_direction = record.normal + Vec3::new_random_unit_vector(rng);
    if scatter_direction.is_near_zero() {
        scatter_direction = record.normal;
    }

    Some(ScatterResult {
        ray: Ray::new(record.point, scatter_direction),
        attenuation: material.albedo,
    })
}
