use std::num::NonZeroU32;

use serde::Deserialize;

use crate::object::Object;

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct ImageConfig {
    pub width: NonZeroU32,
    pub height: NonZeroU32,
    pub samples_per_pixel: usize,
    pub max_ray_depth: i32,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub image: ImageConfig,
    pub world: Vec<Object>,
}
