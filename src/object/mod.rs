use derive_more::From;
use serde::{Deserialize, Serialize};

mod aabb;
mod sphere;

pub use sphere::Sphere;

use crate::{
    material::Material,
    primitive::{Point, Ray, Vec3},
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub t: f32,
    pub is_front_face: bool,
    pub material: Material,
}

impl HitRecord {
    pub fn new(
        point: Point,
        normal: Vec3,
        t: f32,
        is_front_face: bool,
        material: Material,
    ) -> Self {
        Self {
            point,
            normal,
            t,
            is_front_face,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[derive(Debug, PartialEq, Deserialize, Serialize, From)]
pub enum Object {
    Sphere(Sphere),
}

impl Hittable for &[Object] {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_hit = None;
        let mut closest_so_far = t_max;

        for hittable in self.iter() {
            let hit = match hittable {
                Object::Sphere(s) => sphere::hit(s, ray, t_min, closest_so_far),
            };

            // If we hit something
            if let Some(h) = hit {
                closest_hit = Some(h);
                closest_so_far = h.t;
            }
        }
        closest_hit
    }
}
