use rand::rngs::ThreadRng;
use serde::{Deserialize, Serialize};

use crate::{
    object::HitRecord,
    primitive::{Color, Ray, Vec3},
};

use super::ScatterResult;

#[derive(Debug, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

pub fn scatter(
    material: &Metal,
    rng: &mut ThreadRng,
    r_in: &Ray,
    record: &HitRecord,
) -> Option<ScatterResult> {
    let reflected = Vec3::from(r_in.direction.normalize()).reflect(&record.normal);
    let scattered = Ray::new(
        record.point,
        Vec3::new_random_in_unit_sphere(rng) * material.fuzz + reflected.into(),
    );

    if scattered.direction.dot(*record.normal) > 0. {
        Some(ScatterResult {
            ray: scattered,
            attenuation: material.albedo,
        })
    } else {
        None
    }
}
