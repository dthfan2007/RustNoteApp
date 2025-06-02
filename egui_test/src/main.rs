// @Author: Your name
// @Date:   02-06-2025 13:39:59
// @Last Modified by:   Your name
// @Last Modified time: 02-06-2025 15:47:39
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(rustdoc::missing_crate_level_docs)]

use egui::{ColorImage, TextureOptions, Vec2};

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "My first egui App",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(MyApp::default()) as Box<dyn eframe::App>
        }),
    )
}

struct MyApp {
    name: String,
    age: u32,
    ferris_texture: Option<egui::TextureHandle>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Matteo".to_owned(),
            age: 18,
            ferris_texture: None,
        }
    }
}

impl MyApp {
    fn increment(&mut self) {
        if self.age < 120 {
            self.age += 1;
        }
    }

    fn decrement(&mut self) {
        if self.age > 0 {
            self.age -= 1;
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let image = include_bytes!("../assets/images/ferris.png");
        let image = image::load_from_memory(image)
            .expect("Failed to load image")
            .to_rgba8();

        let (width, height) = image.dimensions();
        let pixels = image.as_flat_samples();
        let color_image = ColorImage::from_rgba_unmultiplied(
            [width as usize, height as usize],
            pixels.as_slice(),
        );

        let texture = ctx.load_texture("ferris", color_image, TextureOptions::default());
        self.ferris_texture = Some(texture);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My first egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_multiline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("years old"));
            ui.label(format!(
                "Hello {}, you are {} years old!",
                self.name, self.age
            ));

            if ui.button("Increment").clicked() {
                self.increment();
            }

            if ui.button("Decrement").clicked() {
                self.decrement();
            }

            if let Some(texture) = &self.ferris_texture {
                let custom_size = Vec2::new(125.0, 85.0);
                ui.image((texture.id(), custom_size));
            }
        });
    }
}
