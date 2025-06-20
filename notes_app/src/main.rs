// @Author: Matteo Cipriani
// @Date:   04-06-2025 10:24:58
// @Last Modified by:   Matteo Cipriani
// @Last Modified time: 20-06-2025 08:35:31

use eframe::egui;

mod app;
mod auth;
mod crypto;
mod note;
mod notes_ui;
mod settings_ui;
mod storage;
mod user;

use app::NotesApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([650.0, 465.0])
            .with_title("Secure Notes"),
        ..Default::default()
    };

    eframe::run_native(
        "Secure Notes",
        options,
        Box::new(|_cc| Ok(Box::new(NotesApp::new()))),
    )
}
