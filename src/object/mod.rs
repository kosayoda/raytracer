use enum_dispatch::enum_dispatch;

mod sphere;
use serde::Deserialize;
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

#[enum_dispatch]
pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[enum_dispatch(Hittable)]
#[derive(Debug, PartialEq, Deserialize)]
pub enum Object {
    Sphere,
}

impl Hittable for Vec<Object> {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_hit = None;
        let mut closest_so_far = t_max;

        for hittable in self {
            // If we hit something
            if let Some(h) = hittable.hit(ray, t_min, closest_so_far) {
                closest_hit = Some(h);
                closest_so_far = h.t;
            }
        }
        closest_hit
    }
}

impl Hittable for [Object] {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_hit = None;
        let mut closest_so_far = t_max;

        for hittable in self {
            // If we hit something
            if let Some(h) = hittable.hit(ray, t_min, closest_so_far) {
                closest_hit = Some(h);
                closest_so_far = h.t;
            }
        }
        closest_hit
    }
}
