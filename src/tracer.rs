use std::path::Path;

use image::{buffer::EnumeratePixelsMut, ImageResult, Rgb, RgbImage};
use rand::{rngs::ThreadRng, thread_rng, Rng};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::{
    camera::Camera,
    config::ImageConfig,
    object::{Hittable, Object},
    primitive::{Color, Ray, Vec3},
};

pub struct Tracer {
    camera: Camera,
    pixels: RgbImage,
    pub config: ImageConfig,
}

impl Tracer {
    pub fn new(config: ImageConfig) -> Self {
        let width = config.width.get();
        let height = config.height.get();

        let camera = {
            let aspect_ratio: f32 = width as f32 / height as f32;
            tracing::debug!(aspect_ratio, "Aspect ratio");
            Camera::new(aspect_ratio)
        };

        Self {
            config,
            camera,
            pixels: RgbImage::new(width, height),
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

        let rows: Vec<(u32, EnumeratePixelsMut<Rgb<u8>>)> =
            self.pixels.enumerate_rows_mut().collect();
        rows.into_par_iter().for_each(|(j, row)| {
            let j = height - j - 1;
            let mut rng = thread_rng();
            row.enumerate().for_each(|(i, pixel)| {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.config.samples_per_pixel {
                    let u = (i as f32 + rng.gen::<f32>()) / (width - 1) as f32;
                    let v = (j as f32 + rng.gen::<f32>()) / (height - 1) as f32;

                    let ray = self.camera.get_ray(u, v);
                    pixel_color +=
                        Tracer::ray_color(&mut rng, ray, world, self.config.max_ray_depth);
                }

                let color = {
                    let scale = 1.0 / self.config.samples_per_pixel as f32;
                    pixel_color.to_rgb(scale)
                };
                *pixel.2 = color;
            });
        });
    }

    fn ray_color(rng: &mut ThreadRng, ray: Ray, world: &[Object], depth: i32) -> Color {
        if depth < 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(record) = world.hit(ray, 0.001, f32::MAX) {
            let target = record.point + Vec3::new_random_in_hemisphere(rng, record.normal);
            let ray = Ray::new(record.point, target - record.point);
            return Tracer::ray_color(rng, ray, world, depth - 1) * 0.5;
        }

        let unit_direction = ray.direction().normalize_or_zero();
        let t = 0.5 * (unit_direction.y + 1.0);

        let blue = Color::new(0.5, 0.7, 1.0) * t;
        let white = Color::new(1.0, 1.0, 1.0) * (1.0 - t);

        white + blue
    }
}
