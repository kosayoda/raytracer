use rand::rngs::ThreadRng;

use crate::{
    config::CameraConfig,
    primitive::{Point, Ray, Vec3},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CameraBasis {
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    origin: Point,
    lower_left: Point,

    horizontal: Vec3,
    vertical: Vec3,

    basis: CameraBasis,
    lens_radius: f32,

    config: CameraConfig,
}

impl Camera {
    pub fn new(config: CameraConfig, aspect_ratio: f32) -> Self {
        let viewport_height = 2.0 * (config.vfov.to_radians() / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let basis = {
            let vup = config.vup;
            let w = (config.look_from - config.look_to).normalize();
            let u = vup.cross(w).normalize();
            let v = w.cross(u);
            CameraBasis {
                u: u.into(),
                v: v.into(),
                w: w.into(),
            }
        };

        let focus_dist = (config.look_from - config.look_to).length();

        let origin = config.look_from;
        let horizontal = basis.u * focus_dist * viewport_width;
        let vertical = basis.v * focus_dist * viewport_height;

        let lower_left = origin - horizontal / 2. - vertical / 2. - basis.w * focus_dist;
        let lens_radius = config.aperture / 2.0;

        Self {
            origin,
            lower_left,
            horizontal,
            vertical,
            basis,
            lens_radius,
            config,
        }
    }

    pub fn get_ray(self, rng: &mut ThreadRng, u: f32, v: f32) -> Ray {
        let rd = Vec3::new_random_in_unit_disk(rng) * self.lens_radius;
        let offset = self.basis.u * rd.x + self.basis.v * rd.y;

        let horizontal_offset = self.horizontal * u;
        let vertical_offset = self.vertical * v;
        Ray::new(
            self.origin + offset,
            self.lower_left + horizontal_offset + vertical_offset - self.origin - offset,
        )
    }
}
