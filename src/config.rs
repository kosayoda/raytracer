use std::num::{NonZeroU32, NonZeroUsize};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub width: NonZeroU32,
    pub height: NonZeroU32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub image: Image,
}
