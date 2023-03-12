use egui::InputState;
use glam::Quat;
use rand::rngs::ThreadRng;

use crate::{
    config::CameraConfig,
    primitive::{Point, Ray, Vec3},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    viewport: (f32, f32),

    origin: Point,
    direction: Vec3,

    pub aperture: f32,
    pub focus_dist: f32,

    pub config: CameraConfig,
    last_delta: egui::Vec2,
}

impl Camera {
    const MOVE_SPEED: f32 = 0.1;
    const ROTATE_SPEED: f32 = 0.001;
    const UP: Vec3 = Vec3::new(0.0, 1.0, 0.0);

    pub fn new(mut config: CameraConfig, aspect_ratio: f32) -> Self {
        let CameraConfig {
            look_from: origin,
            look_to: center,
            vertical_fov,
            aperture,
            focus_dist,
        } = config;

        let viewport = {
            let viewport_height = 2.0 * (vertical_fov.to_radians() / 2.0).tan();
            let viewport_width = aspect_ratio * viewport_height;
            (viewport_width, viewport_height)
        };

        let oc = origin - center;
        let direction: Vec3 = oc.normalize().into();
        let focus_dist = focus_dist.unwrap_or_else(|| oc.length());
        config.focus_dist = Some(focus_dist);

        Self {
            viewport,
            origin,
            direction,
            aperture,
            focus_dist,
            config,
            last_delta: egui::vec2(0.0, 0.0),
        }
    }

    pub fn move_camera(&mut self, input: &InputState, skip_mouse_update: &mut bool) {
        if input.key_pressed(egui::Key::W) {
            self.origin -= self.direction * Camera::MOVE_SPEED;
        } else if input.key_pressed(egui::Key::S) {
            self.origin += self.direction * Camera::MOVE_SPEED;
        } else if input.key_pressed(egui::Key::A) {
            let right: Vec3 = self.direction.cross(*Camera::UP).into();
            self.origin += right * Camera::MOVE_SPEED;
        } else if input.key_pressed(egui::Key::D) {
            let right: Vec3 = self.direction.cross(*Camera::UP).into();
            self.origin -= right * Camera::MOVE_SPEED;
        } else if input.key_pressed(egui::Key::Space) {
            self.origin += Camera::UP * Camera::MOVE_SPEED;
        } else if input.modifiers.shift_only() {
            self.origin -= Camera::UP * Camera::MOVE_SPEED;
        }

        let delta = if *skip_mouse_update {
            *skip_mouse_update = false;
            self.last_delta
        } else {
            let delta = input.pointer.delta();
            self.last_delta = delta;
            delta
        };
        if delta.x != 0.0 || delta.y != 0.0 {
            let right = self.direction.cross(*Camera::UP);
            let pitch = Quat::from_axis_angle(right, delta.y * Camera::ROTATE_SPEED);
            let yaw = Quat::from_axis_angle(*Camera::UP, -delta.x * Camera::ROTATE_SPEED);

            let rotation = pitch.mul_quat(yaw).normalize();
            self.direction = rotation.mul_vec3(*self.direction).into();
        }
    }

    pub fn get_ray(&self, rng: &mut ThreadRng, s: f32, t: f32) -> Ray {
        let u = Camera::UP.cross(*self.direction).normalize();
        let v = self.direction.cross(u);
        let offset = {
            let rd = Vec3::new_random_in_unit_disk(rng) * self.aperture / 2.0;
            (u * rd.x + v * rd.y).into()
        };

        let horizontal: Vec3 = (u * self.focus_dist * self.viewport.0).into();
        let vertical: Vec3 = (v * self.focus_dist * self.viewport.1).into();

        let lower_left =
            self.origin - horizontal / 2.0 - vertical / 2.0 - self.direction * self.focus_dist;

        Ray::new(
            self.origin + offset,
            lower_left + horizontal * s + vertical * t - self.origin - offset,
        )
    }
}
