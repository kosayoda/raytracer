use std::path::Path;

use image::{ImageResult, RgbImage};
use rand::{rngs::ThreadRng, thread_rng, Rng};
use rayon::prelude::{ParallelBridge, ParallelIterator};

use crate::{
    camera::Camera,
    config::{CameraConfig, ImageConfig},
    material::Scatterable,
    object::{Hittable, Object},
    primitive::{Color, Ray},
};

pub struct Tracer {
    pixels: RgbImage,
    pub camera: Camera,
    pub config: ImageConfig,

    pub spp: usize,
}

impl Tracer {
    pub fn new(config: ImageConfig, camera: CameraConfig) -> Self {
        let width = config.width.get();
        let height = config.height.get();

        let camera = {
            let aspect_ratio: f32 = width as f32 / height as f32;
            tracing::debug!(aspect_ratio, "Aspect ratio");
            Camera::new(camera, aspect_ratio)
        };

        Self {
            config,
            camera,
            pixels: RgbImage::new(width, height),
            spp: config.samples_per_pixel,
        }
    }

    pub fn save<P: AsRef<Path>>(&mut self, path: P) -> ImageResult<()> {
        self.pixels.save(path)
    }

    pub fn buffer_mut(&mut self) -> &mut [u8] {
        &mut self.pixels
    }

    pub fn render(&mut self, world: &[Object]) {
        let width = self.config.width.get();
        let height = self.config.height.get();

        self.pixels
            .enumerate_rows_mut()
            .into_iter()
            .par_bridge()
            .for_each(|(j, row)| {
                let j = height - j - 1;
                let mut rng = thread_rng();
                row.enumerate().for_each(|(i, pixel)| {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for _ in 0..self.spp {
                        let u = (i as f32 + rng.gen::<f32>()) / (width - 1) as f32;
                        let v = (j as f32 + rng.gen::<f32>()) / (height - 1) as f32;

                        let ray = self.camera.get_ray(&mut rng, u, v);
                        pixel_color +=
                            Tracer::ray_color(&mut rng, ray, world, self.config.max_ray_depth);
                    }

                    let color = {
                        let scale = 1.0 / self.spp as f32;
                        pixel_color.to_rgb(scale)
                    };
                    *pixel.2 = color;
                });
            });
    }

    fn ray_color(rng: &mut ThreadRng, ray: Ray, world: &[Object], depth: i32) -> Color {
        let mut result = Color::new(0.0, 0.0, 0.0);
        let mut global_attenuation = Color::new(1.0, 1.0, 1.0);

        let mut current_ray = ray;

        let white = Color::new(1.0, 1.0, 1.0);
        let blue = Color::new(0.5, 0.7, 1.0);

        for _ in 0..depth {
            if let Some(record) = world.hit(current_ray, 0.001, f32::MAX) {
                if let Some(res) = record.material.scatter(rng, &current_ray, &record) {
                    global_attenuation *= res.attenuation;
                    current_ray = res.ray;
                } else {
                    break;
                }
            } else {
                let unit_direction = current_ray.direction.normalize();
                let t = 0.5 * (unit_direction.y + 1.0);

                let color = white * (1.0 - t) + blue * t;
                result += global_attenuation * color;
                break;
            }
        }

        result
    }
}
