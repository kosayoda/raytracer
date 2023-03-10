use crate::primitive::{Point, Ray, Vec3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    origin: Point,
    lower_left: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point::ZERO;
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);

        let lower_left =
            origin - horizontal / 2. - vertical / 2. - Vec3::new(0.0, 0.0, focal_length);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left,
        }
    }

    pub fn get_ray(self, u: f32, v: f32) -> Ray {
        let horizontal_offset = u * self.horizontal;
        let vertical_offset = v * self.vertical;
        Ray::new(
            self.origin,
            self.lower_left + horizontal_offset + vertical_offset - self.origin,
        )
    }
}
