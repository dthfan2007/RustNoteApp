// @Author: Matteo Cipriani
// @Date:   04-06-2025 10:24:58
// @Last Modified by:   Matteo Cipriani
// @Last Modified time: 25-06-2025 09:40:07

use eframe::egui;
use egui::IconData;
use image;

mod app;
mod auth;
mod crypto;
mod note;
mod notes_ui;
mod settings_ui;
mod storage;
mod user;

use app::NotesApp;

fn load_icon() -> IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let icon = include_bytes!("../assets/icons/icon.png");
        let image = image::load_from_memory(icon)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size([650.0, 465.0])
            .with_title("Secure Notes")
            .with_maximized(true)
            .with_decorations(true)
            .with_icon(load_icon()),
        ..Default::default()
    };

    eframe::run_native(
        "Secure Notes",
        options,
        Box::new(|_cc| Ok(Box::new(NotesApp::new()))),
    )
}
