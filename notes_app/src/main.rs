// @Author: Matteo Cipriani
// @Date:   04-06-2025 10:24:58
// @Last Modified by:   Matteo Cipriani
// @Last Modified time: 06-06-2025 21:12:15
use chrono::Utc;
use chrono_tz::Europe::Zurich;
use eframe::egui;
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

mod crypto;
mod note;
mod storage;

use crypto::CryptoManager;
use note::Note;
use storage::StorageManager;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Secure Notes"),
        ..Default::default()
    };

    eframe::run_native(
        "Secure Notes",
        options,
        Box::new(|_cc| Ok(Box::new(NotesApp::new()))),
    )
}

enum AuthResult {
    Success(CryptoManager),
    Error(String),
}

struct NotesApp {
    notes: HashMap<String, Note>,
    selected_note_id: Option<String>,
    crypto_manager: Option<CryptoManager>,
    storage_manager: StorageManager,
    password_input: String,
    is_authenticated: bool,
    show_password_dialog: bool,
    new_note_title: String,
    last_save_time: std::time::Instant,
    auto_save_delay: std::time::Duration,
    show_security_panel: bool,
    security_warnings: Vec<String>,
    authentication_error: Option<String>,
    is_authenticating: bool,
    auth_receiver: Option<mpsc::Receiver<AuthResult>>,
    auth_start_time: Option<std::time::Instant>,
    show_time_format: TimeFormat,
    show_new_note_dialog: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum TimeFormat {
    Relative, // "2 hours ago"
    Absolute, // "15.12.2024 14:30"
}

impl NotesApp {
    fn new() -> Self {
        Self {
            notes: HashMap::new(),
            selected_note_id: None,
            crypto_manager: None,
            storage_manager: StorageManager::new(),
            password_input: String::new(),
            is_authenticated: false,
            show_password_dialog: true,
            new_note_title: String::new(),
            last_save_time: std::time::Instant::now(),
            auto_save_delay: std::time::Duration::from_secs(2),
            show_security_panel: false,
            security_warnings: Vec::new(),
            authentication_error: None,
            is_authenticating: false,
            auth_receiver: None,
            auth_start_time: None,
            show_time_format: TimeFormat::Relative,
            show_new_note_dialog: false,
        }
    }

    fn start_authentication(&mut self, password: String) {
        if self.is_authenticating {
            return; // Already authenticating
        }

        self.is_authenticating = true;
        self.authentication_error = None;
        self.auth_start_time = Some(std::time::Instant::now());

        let (sender, receiver) = mpsc::channel();
        self.auth_receiver = Some(receiver);

        // Spawn background thread for authentication
        thread::spawn(move || {
            println!("Starting authentication in background thread...");

            let mut crypto_manager = CryptoManager::new();

            let result = match crypto_manager.initialize(&password) {
                Ok(_) => {
                    println!("Authentication successful!");
                    AuthResult::Success(crypto_manager)
                }
                Err(e) => {
                    println!("Authentication failed: {}", e);
                    AuthResult::Error(format!("Authentication failed: {}", e))
                }
            };

            if let Err(_) = sender.send(result) {
                println!("Failed to send authentication result - UI may have closed");
            }
        });
    }

    fn check_authentication_result(&mut self) {
        if let Some(receiver) = &self.auth_receiver {
            match receiver.try_recv() {
                Ok(AuthResult::Success(crypto_manager)) => {
                    if let Some(start_time) = self.auth_start_time {
                        println!(
                            "Authentication completed in {:.2}s",
                            start_time.elapsed().as_secs_f64()
                        );
                    }

                    self.crypto_manager = Some(crypto_manager);
                    self.load_notes();

                    // Perform security audit
                    if let Some(ref crypto) = self.crypto_manager {
                        if let Ok(warnings) = crypto.security_audit() {
                            self.security_warnings = warnings;
                        }
                    }

                    self.is_authenticated = true;
                    self.show_password_dialog = false;
                    self.is_authenticating = false;
                    self.auth_receiver = None;
                    self.auth_start_time = None;
                }
                Ok(AuthResult::Error(error)) => {
                    self.authentication_error = Some(error);
                    self.is_authenticating = false;
                    self.auth_receiver = None;
                    self.auth_start_time = None;
                }
                Err(mpsc::TryRecvError::Empty) => {
                    // Still waiting for result
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.authentication_error = Some("Authentication process failed".to_string());
                    self.is_authenticating = false;
                    self.auth_receiver = None;
                    self.auth_start_time = None;
                }
            }
        }
    }

    fn load_notes(&mut self) {
        if let Some(ref crypto_manager) = self.crypto_manager {
            match self.storage_manager.load_notes(crypto_manager) {
                Ok(notes) => {
                    self.notes = notes;
                }
                Err(e) => {
                    eprintln!("Failed to load notes: {}", e);
                }
            }
        }
    }

    fn save_notes(&self) {
        if let Some(ref crypto_manager) = self.crypto_manager {
            if let Err(e) = self.storage_manager.save_notes(&self.notes, crypto_manager) {
                eprintln!("Failed to save notes: {}", e);
            }
        }
    }

    fn create_new_note(&mut self, title: String) {
        let final_title = if title.trim().is_empty() {
            "Untitled Note".to_string()
        } else {
            title
        };

        let note = Note::new(final_title);
        let note_id = note.id.clone();
        self.notes.insert(note_id.clone(), note);
        self.selected_note_id = Some(note_id);
        self.save_notes();
    }

    fn delete_selected_note(&mut self) {
        if let Some(note_id) = &self.selected_note_id {
            self.notes.remove(note_id);
            self.selected_note_id = None;
            self.save_notes();
        }
    }

    fn auto_save_if_needed(&mut self) {
        if self.last_save_time.elapsed() >= self.auto_save_delay {
            self.save_notes();
            self.last_save_time = std::time::Instant::now();
        }
    }

    fn get_current_swiss_time(&self) -> String {
        let now = Utc::now().with_timezone(&Zurich);
        now.format("%d.%m.%Y %H:%M:%S").to_string()
    }

    fn render_password_dialog(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(200.0);
                ui.heading("Secure Notes");
                ui.add_space(20.0);

                if self.is_authenticating {
                    ui.label("Authenticating... Please wait");
                    ui.spinner();

                    // Show elapsed time
                    if let Some(start_time) = self.auth_start_time {
                        let elapsed = start_time.elapsed().as_secs_f64();
                        ui.label(format!("Elapsed: {:.1}s", elapsed));

                        // Show warning if taking too long
                        if elapsed > 10.0 {
                            ui.colored_label(
                                egui::Color32::YELLOW,
                                "This is taking longer than expected...",
                            );
                        }

                        if elapsed > 30.0 {
                            ui.colored_label(
                                egui::Color32::RED,
                                "Something may be wrong. Try restarting the application.",
                            );
                            if ui.button("Cancel Authentication").clicked() {
                                self.is_authenticating = false;
                                self.auth_receiver = None;
                                self.auth_start_time = None;
                            }
                        }
                    }

                    // Request repaint to update timer
                    ctx.request_repaint_after(std::time::Duration::from_millis(100));
                } else {
                    ui.label("Enter your master password:");
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut self.password_input)
                            .password(true)
                            .desired_width(200.0),
                    );

                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        if !self.password_input.is_empty() {
                            let password = self.password_input.clone();
                            self.password_input.clear();
                            self.start_authentication(password);
                        }
                    }

                    ui.add_space(10.0);
                    if ui.button("Unlock").clicked() {
                        if !self.password_input.is_empty() {
                            let password = self.password_input.clone();
                            self.password_input.clear();
                            self.start_authentication(password);
                        }
                    }

                    // Show authentication error if any
                    if let Some(error) = &self.authentication_error {
                        ui.add_space(10.0);
                        ui.colored_label(egui::Color32::RED, error);
                    }

                    // Debug info
                    ui.add_space(20.0);
                    ui.separator();
                    ui.small("Debug: Using standard security mode");
                    ui.small(format!("Current time: {}", self.get_current_swiss_time()));
                }
            });
        });
    }

    fn render_notes_sidebar(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("notes_list").show(ctx, |ui| {
            ui.heading("Notes");
            ui.separator();

            // New Note button that opens a dialog
            if ui.button("📝 New Note").clicked() {
                self.show_new_note_dialog = true;
                self.new_note_title.clear(); // Clear any previous input
            }

            ui.separator();

            // Time format toggle
            ui.horizontal(|ui| {
                ui.label("Time format:");
                ui.selectable_value(&mut self.show_time_format, TimeFormat::Relative, "Relative");
                ui.selectable_value(&mut self.show_time_format, TimeFormat::Absolute, "Absolute");
            });

            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut notes_vec: Vec<_> = self.notes.iter().collect();
                notes_vec.sort_by(|a, b| b.1.modified_at.cmp(&a.1.modified_at));

                for (note_id, note) in notes_vec {
                    let is_selected = self.selected_note_id.as_ref() == Some(note_id);

                    // Make the entire note area clickable
                    let response = ui.add_sized(
                        [ui.available_width(), 60.0], // Fixed height for consistent appearance
                        egui::Button::new("")
                            .fill(if is_selected {
                                egui::Color32::from_rgb(70, 130, 180)
                            } else {
                                egui::Color32::from_rgb(45, 45, 45)
                            })
                            .stroke(egui::Stroke::new(
                                1.0,
                                if is_selected {
                                    egui::Color32::from_rgb(100, 150, 200)
                                } else {
                                    egui::Color32::from_rgb(80, 80, 80)
                                },
                            )),
                    );

                    // Add the content on top of the button using the modern approach
                    let button_rect = response.rect;
                    ui.scope(|ui| {
                        ui.set_clip_rect(button_rect);
                        // Position the content within the button area
                        ui.allocate_ui_at_rect(button_rect, |ui| {
                            ui.vertical(|ui| {
                                ui.add_space(8.0);
                                ui.horizontal(|ui| {
                                    ui.add_space(8.0);
                                    ui.vertical(|ui| {
                                        // Note title
                                        ui.label(
                                            egui::RichText::new(&note.title).size(14.0).color(
                                                if is_selected {
                                                    egui::Color32::WHITE
                                                } else {
                                                    egui::Color32::LIGHT_GRAY
                                                },
                                            ),
                                        );

                                        // Time information
                                        let time_text = match self.show_time_format {
                                            TimeFormat::Relative => note.relative_time(),
                                            TimeFormat::Absolute => note.format_modified_time(),
                                        };

                                        ui.label(egui::RichText::new(time_text).size(11.0).color(
                                            if is_selected {
                                                egui::Color32::from_rgb(200, 200, 200)
                                            } else {
                                                egui::Color32::GRAY
                                            },
                                        ));
                                    });
                                });
                            });
                        });
                    });

                    if response.clicked() {
                        self.selected_note_id = Some(note_id.clone());
                    }

                    ui.add_space(4.0); // Space between notes
                }
            });

            ui.separator();
            if ui.button("🗑 Delete Selected").clicked() && self.selected_note_id.is_some() {
                self.delete_selected_note();
            }

            ui.separator();
            if ui.button("🔒 Security Info").clicked() {
                self.show_security_panel = !self.show_security_panel;
            }

            // Display security warnings if any
            if !self.security_warnings.is_empty() {
                ui.separator();
                ui.colored_label(egui::Color32::from_rgb(255, 165, 0), "⚠ Security Warnings:");
                for warning in &self.security_warnings {
                    ui.colored_label(
                        egui::Color32::from_rgb(255, 100, 100),
                        format!("• {}", warning),
                    );
                }
            }
        });
    }

    fn render_new_note_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_new_note_dialog {
            return;
        }

        // Extract the current title to avoid borrowing issues
        let mut current_title = self.new_note_title.clone();
        let mut create_note = false;
        let mut cancel_dialog = false;

        egui::Window::new("📝 Create New Note")
            .open(&mut self.show_new_note_dialog)
            .default_width(300.0)
            .resizable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    ui.label("Enter note title:");
                    ui.add_space(10.0);

                    let response = ui.add_sized(
                        [250.0, 25.0],
                        egui::TextEdit::singleline(&mut current_title).hint_text("My new note..."),
                    );

                    // Auto-focus the text field when dialog opens
                    response.request_focus();

                    // Handle Enter key
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        create_note = true;
                    }

                    ui.add_space(15.0);

                    ui.horizontal(|ui| {
                        if ui.button("✅ Create").clicked() {
                            create_note = true;
                        }

                        if ui.button("❌ Cancel").clicked() {
                            cancel_dialog = true;
                        }
                    });

                    ui.add_space(10.0);
                });
            });

        // Update the title back to self
        self.new_note_title = current_title;

        // Handle actions outside the window closure
        if create_note {
            let title = self.new_note_title.clone();
            self.create_new_note(title);
            self.show_new_note_dialog = false;
            self.new_note_title.clear();
        }

        if cancel_dialog {
            self.show_new_note_dialog = false;
            self.new_note_title.clear();
        }
    }

    fn render_main_content(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Clone the selected note ID to avoid borrowing issues
            if let Some(note_id) = self.selected_note_id.clone() {
                // Get the note data we need for display (immutable borrow)
                let (note_title, note_created_time, note_modified_time) = {
                    if let Some(note) = self.notes.get(&note_id) {
                        (
                            note.title.clone(),
                            note.format_created_time(),
                            note.format_modified_time(),
                        )
                    } else {
                        return; // Note doesn't exist anymore
                    }
                };

                // Display the header with note info
                ui.horizontal(|ui| {
                    ui.heading(&note_title);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Show both created and modified times
                        ui.vertical(|ui| {
                            ui.small(format!("Modified: {}", note_modified_time));
                            ui.small(format!("Created: {}", note_created_time));
                        });
                    });
                });
                ui.separator();

                // Calculate available space for the text editor
                let available_height = ui.available_height();
                let header_height = 80.0; // Approximate height for header and separator
                let text_area_height = (available_height - header_height).max(200.0);

                // Create a scrollable text area with fixed height
                egui::ScrollArea::vertical()
                    .max_height(text_area_height)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        // Now get mutable access to the note content
                        if let Some(note) = self.notes.get_mut(&note_id) {
                            let response = ui.add_sized(
                                [
                                    ui.available_width(),
                                    ui.available_height().max(text_area_height),
                                ],
                                egui::TextEdit::multiline(&mut note.content)
                                    .desired_width(f32::INFINITY)
                                    .desired_rows(20), // Minimum number of visible rows
                            );

                            if response.changed() {
                                note.update_modified_time();
                                self.last_save_time = std::time::Instant::now();
                            }
                        }
                    });
            } else {
                ui.vertical_centered(|ui| {
                    ui.add_space(200.0);
                    ui.heading("Select a note to edit");
                    ui.label("Or create a new note using the sidebar");
                    ui.add_space(20.0);
                    ui.small(format!("Current time: {}", self.get_current_swiss_time()));
                });
            }
        });
    }

    fn render_security_panel(&mut self, ctx: &egui::Context) {
        if !self.show_security_panel {
            return;
        }

        // Extract the data we need before the window closure
        let security_info = self
            .crypto_manager
            .as_ref()
            .and_then(|crypto| crypto.get_security_info());

        let current_time = self.get_current_swiss_time();
        let has_crypto_manager = self.crypto_manager.is_some();
        let security_warnings = self.security_warnings.clone();

        // Track if we need to run a security audit
        let mut run_audit = false;

        egui::Window::new("🔒 Security Information")
            .open(&mut self.show_security_panel)
            .default_width(400.0)
            .show(ctx, |ui| {
                ui.heading("Security Status");
                ui.separator();

                if let Some(info) = security_info {
                    ui.label(info);
                } else {
                    ui.label("Security information not available");
                }

                ui.separator();
                ui.heading("Security Audit");

                if has_crypto_manager && ui.button("🔍 Run Security Audit").clicked() {
                    run_audit = true;
                }

                if security_warnings.is_empty() {
                    ui.colored_label(egui::Color32::from_rgb(0, 200, 0), "✅ No security issues detected");
                } else {
                    ui.colored_label(egui::Color32::from_rgb(255, 100, 100), "⚠ Security warnings:");
                    for warning in &security_warnings {
                        ui.colored_label(egui::Color32::from_rgb(255, 150, 150), format!("• {}", warning));
                    }
                }

                ui.separator();
                ui.small("This application uses military-grade encryption with hardware binding for maximum security.");
                ui.small(format!("Local time: {}", current_time));
            });

        // Run the security audit outside the window closure
        if run_audit {
            if let Some(ref crypto_manager) = self.crypto_manager {
                if let Ok(warnings) = crypto_manager.security_audit() {
                    self.security_warnings = warnings;
                }
            }
        }
    }
}

impl eframe::App for NotesApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for authentication results
        self.check_authentication_result();

        if self.show_password_dialog {
            self.render_password_dialog(ctx);
            return;
        }

        // Render the main application UI
        self.render_notes_sidebar(ctx);
        self.render_main_content(ctx);
        self.render_security_panel(ctx);
        self.render_new_note_dialog(ctx);

        // Auto-save functionality
        self.auto_save_if_needed();

        // Request repaint for auto-save timing and relative time updates
        ctx.request_repaint_after(std::time::Duration::from_millis(500));
    }
}
