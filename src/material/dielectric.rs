use serde::{Deserialize, Serialize};

use crate::{object::HitRecord, ray::Ray, vec3::Color};
use rand::{prelude::SmallRng, Rng, SeedableRng};

use super::{ScatterResult, Scatterable};

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    // Shlick's approximation
    let r0 = (1. - ref_idx) / (1. + ref_idx);
    let r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cosine).powi(5)
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub struct Dielectric {
    pub refractive_index: f32,
}

impl Scatterable for Dielectric {
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<ScatterResult> {
        let refraction_ratio = if record.is_front_face() {
            1. / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = r_in.direction().unit_vector();

        let cos_theta = (-unit_direction).dot(record.normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = (refraction_ratio * sin_theta) > 1.;
        let mut rng = SmallRng::from_entropy();
        let should_reflect = reflectance(cos_theta, refraction_ratio) > rng.gen::<f32>();

        // Cannot refract
        let direction = if cannot_refract || should_reflect {
            unit_direction.reflect(record.normal())
        } else {
            unit_direction.refract(record.normal(), refraction_ratio)
        };

        Some(ScatterResult {
            ray: Ray::new(record.point(), direction),
            attenuation: Color::new(1., 1., 1.),
        })
    }
}
