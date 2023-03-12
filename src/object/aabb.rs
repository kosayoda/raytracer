use crate::{
    material::Material,
    object::{HitRecord, Hittable},
    primitive::{Point, Ray, Vec3},
};
use serde::{Deserialize, Serialize};

macro_rules! zip {
    ($x: expr) => ($x);
    ($x: expr, $($y: expr), +) => (
        $x.iter().zip(
            zip!($($y), +))
    )
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Aabb {
    minimum: Point,
    maximum: Point,
}

impl Aabb {
    pub fn new(minimum: Point, maximum: Point) -> Self {
        Self { minimum, maximum }
    }

    #[inline]
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        for (origin, (direction, (_min, _max))) in zip!(
            ray.origin.to_array(),
            ray.direction.to_array(),
            self.minimum.to_array(),
            self.maximum.to_array()
        ) {
            let inv_d = 1.0 / direction;
        }
        return true;
    }
}
