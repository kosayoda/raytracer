use std::{num::NonZeroU32, time::Duration};

use egui::{ColorImage, Slider, TextureHandle};
use fast_image_resize as fr;

mod camera;
mod config;
mod object;
mod primitive;
mod tracer;

pub use config::Config;
use object::{Object, Sphere};
use tracer::Tracer;

pub struct App {
    frame: Option<TextureHandle>,
    frame_size: egui::Vec2,
    last_render_time: Duration,
    tracer: Tracer,
    config: Config,
}

impl App {
    pub fn new(config: Config) -> Self {
        Self {
            frame: None,
            frame_size: egui::Vec2::default(),
            last_render_time: Duration::ZERO,
            tracer: Tracer::new(config.seed, config.image),
            config,
        }
    }

    pub fn render(&mut self) {
        self.last_render_time = {
            let now = std::time::Instant::now();
            self.tracer.render(&self.config.world);
            now.elapsed()
        };
    }

    pub fn resize_to_frame(&mut self, ui: &mut egui::Ui) -> TextureHandle {
        let new_frame_size = ui.available_size();

        // If we have a texture already...
        if let Some(texture) = &self.frame {
            // And the size hasn't changed...
            if new_frame_size == self.frame_size {
                // Used the already rendered texture
                return texture.clone();
            }
        }

        // Original sized image
        let image = fr::Image::from_slice_u8(
            self.config.image.width,
            self.config.image.height,
            &mut *self.tracer.pixels,
            fr::PixelType::U8x3,
        )
        .unwrap();

        // Get new image size fitting image aspect ratio
        let (new_width, new_height) = {
            let old_width = self.config.image.width.get() as f32;
            let old_height = self.config.image.height.get() as f32;
            let screen_ratio = (new_frame_size.x / old_width).min(new_frame_size.y / old_height);

            (old_width * screen_ratio, old_height * screen_ratio)
        };

        // Resize image
        let image_size = [new_width as _, new_height as _];
        let mut resized = fr::Image::new(
            NonZeroU32::new(new_width as u32).unwrap(),
            NonZeroU32::new(new_height as u32).unwrap(),
            fr::PixelType::U8x3,
        );

        let mut resizer = fr::Resizer::new(fr::ResizeAlg::Nearest);
        resizer
            .resize(&image.view(), &mut resized.view_mut())
            .unwrap();

        let image = ColorImage::from_rgb(image_size, &resized.buffer());
        let handle = ui
            .ctx()
            .load_texture("raytracer output", image, Default::default());
        self.frame = Some(handle.clone());
        handle
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("info").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.label(format!(
                    "Render time: {:.2}ms",
                    self.last_render_time.as_micros() as f32 / 1000.0
                ));
            })
        });
        egui::SidePanel::right("Settings").show(ctx, |ui| {
            ui.heading("Settings");
            ui.separator();

            egui::CollapsingHeader::new("Objects")
                .default_open(true)
                .show(ui, |ui| {
                    for (idx, obj) in self.config.world.iter_mut().enumerate() {
                        match obj {
                            Object::Sphere(s) => {
                                egui::Grid::new(idx.to_string()).show(ui, |ui| {
                                    ui.label("Center");
                                    ui.horizontal(|ui| {
                                        ui.add(egui::DragValue::new(&mut s.center.x).speed(0.01))
                                            .on_hover_text("x");
                                        ui.add(egui::DragValue::new(&mut s.center.y).speed(0.01))
                                            .on_hover_text("y");
                                        ui.add(egui::DragValue::new(&mut s.center.z).speed(0.01))
                                            .on_hover_text("z");
                                    });
                                    ui.end_row();

                                    ui.spacing();

                                    ui.label("Radius");
                                    ui.add(
                                        egui::Slider::new(&mut s.radius, 0.0..=100.0)
                                            .drag_value_speed(0.1),
                                    );
                                    ui.end_row();
                                });
                            }
                        }

                        ui.separator();
                    }
                });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Render image
            self.render();

            // Display resized image to egui frame
            let frame = self.resize_to_frame(ui);
            ui.centered_and_justified(|ui| ui.image(&frame, frame.size_vec2()));
        });
    }
}
