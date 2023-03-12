use std::num::NonZeroU32;

use serde::{Deserialize, Serialize};

use crate::{object::Object, primitive::Point};

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct ImageConfig {
    pub width: NonZeroU32,
    pub height: NonZeroU32,
    pub samples_per_pixel: usize,
    pub max_ray_depth: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
pub struct CameraConfig {
    pub look_from: Point,
    pub look_to: Point,
    pub vertical_fov: f32,
    pub aperture: f32,
    pub focus_dist: Option<f32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub camera: CameraConfig,
    pub image: ImageConfig,
    pub world: Vec<Object>,
}
