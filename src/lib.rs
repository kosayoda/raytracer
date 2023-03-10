use egui::ColorImage;

#[derive(Default)]
pub struct Raytracer {
    image: ColorImage,
}

impl eframe::App for Raytracer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let texture =
                ui.ctx()
                    .load_texture("raytracer output", self.image.clone(), Default::default());
            ui.image(&texture, ui.available_size());
        });
    }
}
