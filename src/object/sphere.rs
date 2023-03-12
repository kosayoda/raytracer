use crate::{
    material::Material,
    object::{HitRecord, Hittable},
    primitive::{Point, Ray, Vec3},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Point, radius: f32, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let h = oc.dot(*ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let mut t = (-h - sqrt_d) / a;
        if t < t_min || t_max < t {
            t = (-h + sqrt_d) / a;
            if t < t_min || t_max < t {
                return None;
            }
        }

        let point = ray.at(t);
        let outward_normal = (point - self.center) / self.radius;
        let front_face = is_front_face(&ray, &outward_normal);
        let outward_normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Some(HitRecord::new(
            point,
            outward_normal,
            t,
            front_face,
            self.material,
        ))
    }
}

#[inline]
fn is_front_face(ray: &Ray, outward_normal: &Vec3) -> bool {
    ray.direction.dot(**outward_normal) < 0.0
}
