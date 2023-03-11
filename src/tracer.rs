use image::RgbImage;
use rand::{rngs::SmallRng, Rng, SeedableRng};

use crate::{
    camera::Camera,
    config::ImageConfig,
    object::{Hittable, Object},
    primitive::{Color, Ray, Vec3},
};

pub struct Tracer {
    config: ImageConfig,
    camera: Camera,
    rng: SmallRng,
    pub pixels: RgbImage,
}

impl Tracer {
    pub fn new(seed: Option<u64>, config: ImageConfig) -> Self {
        let width = config.width.get();
        let height = config.height.get();

        let camera = {
            let aspect_ratio: f32 = width as f32 / height as f32;
            tracing::debug!(aspect_ratio, "Aspect ratio");
            Camera::new(aspect_ratio)
        };

        let rng = if let Some(seed) = seed {
            SmallRng::seed_from_u64(seed)
        } else {
            SmallRng::from_entropy()
        };

        Self {
            config,
            camera,
            pixels: RgbImage::new(width, height),
            rng,
        }
    }

    pub fn render(&mut self, world: &[Object]) {
        let width = self.config.width.get();
        let height = self.config.height.get();

        for j in 0..height {
            for i in 0..width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.config.samples_per_pixel {
                    let u = (i as f32 + self.rng.gen::<f32>()) / (width - 1) as f32;
                    let v = (j as f32 + self.rng.gen::<f32>()) / (height - 1) as f32;

                    let ray = self.camera.get_ray(u, v);
                    pixel_color += Tracer::ray_color(ray, world);
                }
                let pixel_color = {
                    let scale = 1.0 / self.config.samples_per_pixel as f32;
                    pixel_color * scale
                };
                self.pixels.put_pixel(i, height - j - 1, pixel_color.into());
            }
        }
    }

    fn ray_color(ray: Ray, world: &[Object]) -> Color {
        if let Some(record) = world.hit(ray, 0.0, f32::MAX) {
            return ((Vec3::new(1.0, 1.0, 1.0) + record.normal) * 0.5).into();
        }

        let unit_direction = ray.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);

        let blue = Color::new(0.5, 0.7, 1.0) * t;
        let white = Color::new(1.0, 1.0, 1.0) * (1.0 - t);

        white + blue
    }
}
