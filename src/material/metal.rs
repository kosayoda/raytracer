use rand::rngs::ThreadRng;
use serde::Deserialize;

use crate::{
    object::HitRecord,
    primitive::{Color, Ray, Vec3},
};

use super::{ScatterResult, Scatterable};

#[derive(Debug, PartialEq, Clone, Copy, Deserialize)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Scatterable for Metal {
    fn scatter(
        &self,
        rng: &mut ThreadRng,
        r_in: &Ray,
        record: &HitRecord,
    ) -> Option<ScatterResult> {
        let reflected = Vec3::from(r_in.direction.normalize()).reflect(&record.normal);
        let scattered = Ray::new(
            record.point,
            Vec3::new_random_in_unit_sphere(rng) * self.fuzz + reflected.into(),
        );

        if scattered.direction.dot(*record.normal) > 0. {
            Some(ScatterResult {
                ray: scattered,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
