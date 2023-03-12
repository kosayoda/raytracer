use crate::{
    material::Material,
    object::HitRecord,
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

pub fn hit(sphere: &Sphere, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let oc = ray.origin - sphere.center;
    let a = ray.direction.length_squared();
    let h = oc.dot(*ray.direction);
    let c = oc.length_squared() - sphere.radius.powi(2);

    let discriminant = h.powi(2) - a * c;
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
    let outward_normal = (point - sphere.center) / sphere.radius;
    let front_face = ray.direction.dot(*outward_normal) < 0.0;
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
        sphere.material,
    ))
}
