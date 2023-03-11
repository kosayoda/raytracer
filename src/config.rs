use std::num::NonZeroU32;

use serde::Deserialize;

use crate::object::Object;

#[derive(Deserialize, Debug)]
pub struct Image {
    pub width: NonZeroU32,
    pub height: NonZeroU32,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub image: Image,
    pub world: Vec<Object>,
}
