use std::num::NonZeroU32;

use egui::ColorImage;
use fast_image_resize as fr;
use glam::Vec3;
use image::RgbImage;

mod camera;
mod config;
mod object;
mod primitive;

use camera::Camera;
pub use config::Config;
use object::{Hittable, Object};
use primitive::{Color, Ray};
use rand::{rngs::SmallRng, Rng, SeedableRng};

pub struct Raytracer {
    config: Config,
    camera: Camera,
    pixels: RgbImage,
    rng: SmallRng,
}

impl Raytracer {
    pub fn new(config: Config) -> Self {
        let width = config.image.width.get();
        let height = config.image.height.get();

        let camera = {
            let aspect_ratio: f32 = width as f32 / height as f32;
            tracing::debug!(aspect_ratio, "Aspect ratio");
            Camera::new(aspect_ratio)
        };

        let rng = if let Some(seed) = config.seed {
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

    pub fn render(&mut self) {
        let width = self.config.image.width.get();
        let height = self.config.image.height.get();

        for j in 0..height {
            for i in 0..width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.config.image.samples_per_pixel {
                    let u = (i as f32 + self.rng.gen::<f32>()) / (width - 1) as f32;
                    let v = (j as f32 + self.rng.gen::<f32>()) / (height - 1) as f32;

                    let ray = self.camera.get_ray(u, v);
                    pixel_color += Raytracer::ray_color(ray, &self.config.world);
                }
                let pixel_color = {
                    let scale = 1.0 / self.config.image.samples_per_pixel as f32;
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

impl eframe::App for Raytracer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let render_time = {
            let now = std::time::Instant::now();
            self.render();
            now.elapsed()
        };

        let image = fr::Image::from_slice_u8(
            self.config.image.width,
            self.config.image.height,
            &mut *self.pixels,
            fr::PixelType::U8x3,
        )
        .unwrap();

        egui::TopBottomPanel::top("info").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.label(format!(
                    "Render time: {:.2}ms",
                    render_time.as_micros() as f32 / 1000.0
                ));
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Get available size fitting image aspect ratio
            let screen_size = ui.available_size();
            let screen_ratio = (screen_size.x / image.width().get() as f32)
                .min(screen_size.y / image.height().get() as f32);
            let width = image.width().get() as f32 * screen_ratio;
            let height = image.height().get() as f32 * screen_ratio;

            // Resize image to output window
            let image = {
                let mut resized = fr::Image::new(
                    NonZeroU32::new(width as u32).unwrap(),
                    NonZeroU32::new(height as u32).unwrap(),
                    fr::PixelType::U8x3,
                );

                let mut resizer = fr::Resizer::new(fr::ResizeAlg::Nearest);
                resizer
                    .resize(&image.view(), &mut resized.view_mut())
                    .unwrap();
                ColorImage::from_rgb([width as usize, height as usize], resized.buffer())
            };

            // Render resized image to egui
            let texture = ui
                .ctx()
                .load_texture("raytracer output", image, Default::default());
            ui.centered_and_justified(|ui| ui.add(egui::Image::new(&texture, texture.size_vec2())));
        });
    }
}
