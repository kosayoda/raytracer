use std::{num::NonZeroU32, time::Duration};

use egui::{Align2, ColorImage, TextureHandle};
use fast_image_resize as fr;

mod camera;
mod config;
mod material;
mod object;
mod primitive;
mod tracer;

pub use config::Config;
use object::Object;
use tracer::Tracer;

use crate::material::Material;

#[derive(Debug, PartialEq, Eq)]
enum AppState {
    JustStarted,
    Running,
    Paused,
}

impl AppState {
    fn to_button_str(&self) -> &'static str {
        match self {
            AppState::JustStarted => "Run",
            AppState::Running => "Pause",
            AppState::Paused => "Run",
        }
    }
}

pub struct App {
    frame: Option<TextureHandle>,
    frame_size: egui::Vec2,
    last_render_time: Duration,
    tracer: Tracer,
    world: Vec<Object>,
    state: AppState,
}

impl App {
    pub fn new(config: Config) -> Self {
        Self {
            frame: None,
            frame_size: egui::Vec2::default(),
            last_render_time: Duration::ZERO,
            tracer: Tracer::new(config.image, config.camera),
            state: AppState::JustStarted,
            world: config.world,
        }
    }

    pub fn render(&mut self) {
        self.last_render_time = {
            let now = std::time::Instant::now();
            self.tracer.render(&self.world);
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

        // Calculate new image size fitting image aspect ratio
        let (new_width, new_height) = {
            let old_width = self.tracer.config.width.get() as f32;
            let old_height = self.tracer.config.height.get() as f32;
            let screen_ratio = (new_frame_size.x / old_width).min(new_frame_size.y / old_height);

            (old_width * screen_ratio, old_height * screen_ratio)
        };

        // Original sized image
        let image = fr::Image::from_slice_u8(
            self.tracer.config.width,
            self.tracer.config.height,
            self.tracer.buffer_mut(),
            fr::PixelType::U8x3,
        )
        .unwrap();

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
        const SPACING: f32 = 10.0;

        egui::SidePanel::right("Settings").show(ctx, |ui| {
            ui.add_space(SPACING);
            ui.heading("Settings");
            ui.separator();

            egui::CollapsingHeader::new("Image").show(ui, |ui| {
                egui::Grid::new("ImageGrid").show(ui, |ui| {
                    ui.label("Samples Per Pixel");
                    ui.add(egui::Slider::new(
                        &mut self.tracer.config.samples_per_pixel,
                        1..=100,
                    ));
                    ui.end_row();

                    ui.label("Max Ray Depth");
                    ui.add(egui::Slider::new(
                        &mut self.tracer.config.max_ray_depth,
                        1..=50,
                    ));
                    ui.end_row();
                });
            });

            egui::CollapsingHeader::new("Objects").show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .max_height(ui.available_height() * 0.8)
                    .show(ui, |ui| {
                        for (idx, obj) in self.world.iter_mut().enumerate() {
                            match obj {
                                Object::Sphere(s) => {
                                    egui::Grid::new(idx.to_string()).show(ui, |ui| {
                                        ui.label("Center");
                                        ui.horizontal(|ui| {
                                            ui.add(
                                                egui::DragValue::new(&mut s.center.x).speed(0.01),
                                            )
                                            .on_hover_text("x");
                                            ui.add(
                                                egui::DragValue::new(&mut s.center.y).speed(0.01),
                                            )
                                            .on_hover_text("y");
                                            ui.add(
                                                egui::DragValue::new(&mut s.center.z).speed(0.01),
                                            )
                                            .on_hover_text("z");
                                        });
                                        ui.end_row();

                                        ui.label("Radius");
                                        ui.add(
                                            egui::Slider::new(&mut s.radius, 0.0..=100.0)
                                                .drag_value_speed(0.1),
                                        );
                                        ui.end_row();

                                        match &mut s.material {
                                            Material::Lambertian(l) => {
                                                ui.label("Albedo");
                                                ui.color_edit_button_rgb(&mut l.albedo.as_mut());
                                                ui.end_row();
                                            }
                                            Material::Metal(m) => {
                                                ui.label("Albedo");
                                                ui.color_edit_button_rgb(&mut m.albedo.as_mut());
                                                ui.end_row();
                                                ui.label("Fuzz");
                                                ui.add(
                                                    egui::DragValue::new(&mut m.fuzz)
                                                        .speed(0.01)
                                                        .clamp_range(0.0..=1.0),
                                                );
                                                ui.end_row();
                                            }
                                            Material::Dielectric(d) => {
                                                ui.label("Refractive index");
                                                ui.add(
                                                    egui::DragValue::new(&mut d.refractive_index)
                                                        .speed(0.01)
                                                        .clamp_range(0.0..=2.0),
                                                );
                                                ui.end_row();
                                            }
                                        }
                                    });
                                }
                            }

                            ui.separator();
                        }
                    });
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
                ui.add_space(SPACING);
                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        let mut dialog = rfd::FileDialog::new()
                            .set_title("Save render to")
                            .set_file_name("image.png");
                        if let Ok(current_dir) = std::env::current_dir() {
                            dialog = dialog.set_directory(current_dir);
                        }
                        if let Some(path) = dialog.save_file() {
                            if let Err(e) = self.tracer.save(path) {
                                egui::Window::new("SaveFileError")
                                    .anchor(Align2::LEFT_TOP, [SPACING, SPACING])
                                    .show(ctx, |ui| {
                                        ui.heading("Error");
                                        ui.label(format!("{e}"));
                                    });
                            }
                        }
                    }

                    let render_button = ui.add_enabled(
                        matches!(self.state, AppState::Paused),
                        egui::Button::new("Render"),
                    );
                    if render_button.clicked() {
                        self.render();
                    }

                    if ui.button(self.state.to_button_str()).clicked() {
                        match self.state {
                            AppState::JustStarted | AppState::Paused => {
                                self.state = AppState::Running
                            }
                            AppState::Running => self.state = AppState::Paused,
                        }
                    }
                });
            });
        });

        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                ui.label(format!(
                    "Render time: {:.2}ms",
                    self.last_render_time.as_micros() as f32 / 1000.0
                ));
                ui.label(format!("State: {:?}", self.state));
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Render image
            match self.state {
                AppState::JustStarted => {
                    self.render();
                    self.state = AppState::Paused;
                }
                AppState::Running => {
                    self.render();
                }
                AppState::Paused => (),
            }

            // Display resized image to egui frame
            let frame = self.resize_to_frame(ui);
            ui.centered_and_justified(|ui| ui.image(&frame, frame.size_vec2()));
        });
    }
}
