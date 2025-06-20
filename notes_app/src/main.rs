// @Author: Matteo Cipriani
// @Date:   04-06-2025 10:24:58
// @Last Modified by:   Matteo Cipriani
// @Last Modified time: 20-06-2025 07:59:24

use chrono::Utc; // Get current UTC time
use chrono_tz::Europe::Zurich; // Timezone conversion
use eframe::egui; // Lightweight GUI
use std::collections::HashMap; // Create key-value pairs that can be saved
use std::sync::mpsc; // Thread-safe communication between threads.
use std::thread; // Enables concurrent execution

mod crypto; // Import crypto.rs
mod note; // Import note.rs
mod storage; // Import storage.rs
mod user; // Import user.rs

use crypto::CryptoManager; // Use CryptoManager struct from crypto.rs
use note::Note; // Use Note struct from note.rs
use storage::StorageManager; // Use StorageManager struct from storage.rs
use user::{User, UserManager}; // Use User and UserManager structs from user.rs

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
    Success(CryptoManager, User),
    Error(String),
}

#[derive(Clone, Copy, PartialEq)]
enum AuthMode {
    Login,
    Register,
}

struct NotesApp {
    notes: HashMap<String, Note>,
    selected_note_id: Option<String>,
    crypto_manager: Option<CryptoManager>,
    storage_manager: StorageManager,
    user_manager: Option<UserManager>,
    current_user: Option<User>,

    // Authentication UI
    username_input: String,
    password_input: String,
    confirm_password_input: String,
    is_authenticated: bool,
    show_auth_dialog: bool,
    auth_mode: AuthMode,
    authentication_error: Option<String>,
    is_authenticating: bool,
    auth_receiver: Option<mpsc::Receiver<AuthResult>>,
    auth_start_time: Option<std::time::Instant>,

    // Note management
    new_note_title: String,
    last_save_time: std::time::Instant,
    auto_save_delay: std::time::Duration,
    show_new_note_dialog: bool,

    // UI state
    show_security_panel: bool,
    security_warnings: Vec<String>,
    show_time_format: TimeFormat,

    // Context menu
    context_menu_note_id: Option<String>,
    show_context_menu: bool,
    context_menu_pos: egui::Pos2,

    // User settings
    show_user_settings: bool,
    show_change_password_dialog: bool,
    show_delete_account_dialog: bool,
    old_password_input: String,
    new_password_input: String,
    confirm_new_password_input: String,
    delete_confirmation_input: String,
}

#[derive(Clone, Copy, PartialEq)]
enum TimeFormat {
    Relative, // "2 hours ago"
    Absolute, // "15.12.2024 14:30"
}

impl NotesApp {
    fn new() -> Self {
        let user_manager = UserManager::new().ok();

        Self {
            notes: HashMap::new(),
            selected_note_id: None,
            crypto_manager: None,
            storage_manager: StorageManager::new(),
            user_manager,
            current_user: None,

            username_input: String::new(),
            password_input: String::new(),
            confirm_password_input: String::new(),
            is_authenticated: false,
            show_auth_dialog: true,
            auth_mode: AuthMode::Login,
            authentication_error: None,
            is_authenticating: false,
            auth_receiver: None,
            auth_start_time: None,

            new_note_title: String::new(),
            last_save_time: std::time::Instant::now(),
            auto_save_delay: std::time::Duration::from_secs(2),
            show_new_note_dialog: false,

            show_security_panel: false,
            security_warnings: Vec::new(),
            show_time_format: TimeFormat::Relative,

            context_menu_note_id: None,
            show_context_menu: false,
            context_menu_pos: egui::Pos2::ZERO,

            show_user_settings: false,
            show_change_password_dialog: false,
            show_delete_account_dialog: false,
            old_password_input: String::new(),
            new_password_input: String::new(),
            confirm_new_password_input: String::new(),
            delete_confirmation_input: String::new(),
        }
    }

    fn start_authentication(&mut self, username: String, password: String, is_registration: bool) {
        if self.is_authenticating {
            return; // Already authenticating
        }

        self.is_authenticating = true;
        self.authentication_error = None;
        self.auth_start_time = Some(std::time::Instant::now());

        let (sender, receiver) = mpsc::channel();
        self.auth_receiver = Some(receiver);

        let user_manager = self.user_manager.clone();

        // Spawn background thread for authentication
        thread::spawn(move || {
            println!("Starting authentication in background thread...");

            if let Some(mut user_manager) = user_manager {
                let result = if is_registration {
                    // Registration flow
                    match user_manager.create_user(username.clone(), &password) {
                        Ok(_) => {
                            println!("User created successfully, now authenticating...");
                            // After successful registration, authenticate the user
                            match user_manager.authenticate(&username, &password) {
                                Ok(user) => {
                                    let mut crypto_manager = CryptoManager::new();
                                    match crypto_manager.initialize_for_user(&user.id, &password) {
                                        Ok(_) => {
                                            println!("Registration and authentication successful!");
                                            AuthResult::Success(crypto_manager, user)
                                        }
                                        Err(e) => {
                                            println!("Crypto initialization failed: {}", e);
                                            AuthResult::Error(format!(
                                                "Crypto initialization failed: {}",
                                                e
                                            ))
                                        }
                                    }
                                }
                                Err(e) => {
                                    println!("Authentication after registration failed: {}", e);
                                    AuthResult::Error(format!(
                                        "Authentication after registration failed: {}",
                                        e
                                    ))
                                }
                            }
                        }
                        Err(e) => {
                            println!("Registration failed: {}", e);
                            AuthResult::Error(format!("Registration failed: {}", e))
                        }
                    }
                } else {
                    // Login flow
                    match user_manager.authenticate(&username, &password) {
                        Ok(user) => {
                            println!("User authenticated, initializing crypto...");
                            let mut crypto_manager = CryptoManager::new();
                            match crypto_manager.initialize_for_user(&user.id, &password) {
                                Ok(_) => {
                                    println!("Login successful!");
                                    AuthResult::Success(crypto_manager, user)
                                }
                                Err(e) => {
                                    println!("Crypto initialization failed: {}", e);
                                    AuthResult::Error(format!("Authentication failed: {}", e))
                                }
                            }
                        }
                        Err(e) => {
                            println!("Login failed: {}", e);
                            AuthResult::Error(format!("Login failed: {}", e))
                        }
                    }
                };

                if let Err(_) = sender.send(result) {
                    println!("Failed to send authentication result - UI may have closed");
                }
            } else {
                let _ = sender.send(AuthResult::Error("User manager not available".to_string()));
            }
        });
    }

    fn check_authentication_result(&mut self) {
        if let Some(receiver) = &self.auth_receiver {
            match receiver.try_recv() {
                Ok(AuthResult::Success(crypto_manager, user)) => {
                    if let Some(start_time) = self.auth_start_time {
                        println!(
                            "Authentication completed in {:.2}s",
                            start_time.elapsed().as_secs_f64()
                        );
                    }

                    self.crypto_manager = Some(crypto_manager);
                    self.current_user = Some(user);
                    self.load_notes();
                    self.migrate_legacy_data_if_needed();

                    // Perform security audit
                    if let Some(ref crypto) = self.crypto_manager {
                        if let Ok(warnings) = crypto.security_audit() {
                            self.security_warnings = warnings;
                        }
                    }

                    self.is_authenticated = true;
                    self.show_auth_dialog = false;
                    self.is_authenticating = false;
                    self.auth_receiver = None;
                    self.auth_start_time = None;

                    // Clear input fields
                    self.username_input.clear();
                    self.password_input.clear();
                    self.confirm_password_input.clear();
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
        if let (Some(ref crypto_manager), Some(ref user)) =
            (&self.crypto_manager, &self.current_user)
        {
            match self
                .storage_manager
                .load_user_notes(&user.id, crypto_manager)
            {
                Ok(notes) => {
                    self.notes = notes;
                    println!(
                        "Loaded {} notes for user {}",
                        self.notes.len(),
                        user.username
                    );
                }
                Err(e) => {
                    eprintln!("Failed to load notes: {}", e);
                }
            }
        }
    }

    fn save_notes(&self) {
        if let (Some(ref crypto_manager), Some(ref user)) =
            (&self.crypto_manager, &self.current_user)
        {
            if let Err(e) =
                self.storage_manager
                    .save_user_notes(&user.id, &self.notes, crypto_manager)
            {
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

    fn delete_note(&mut self, note_id: &str) {
        if let Some(note) = self.notes.get(note_id) {
            println!("Deleting note: {}", note.title);
        }

        self.notes.remove(note_id);

        if self.selected_note_id.as_ref() == Some(&note_id.to_string()) {
            self.selected_note_id = None;
        }

        self.save_notes();
    }

    fn auto_save_if_needed(&mut self) {
        if self.last_save_time.elapsed() >= self.auto_save_delay {
            self.save_notes();
            self.last_save_time = std::time::Instant::now();
        }
    }

    fn get_current_time(&self) -> String {
        let now = Utc::now().with_timezone(&Zurich);
        now.format("%d.%m.%Y %H:%M:%S").to_string()
    }

    fn logout(&mut self) {
        println!("User logging out");
        self.is_authenticated = false;
        self.show_auth_dialog = true;
        self.crypto_manager = None;
        self.current_user = None;
        self.notes.clear();
        self.selected_note_id = None;
        self.username_input.clear();
        self.password_input.clear();
        self.confirm_password_input.clear();
        self.authentication_error = None;
        self.auth_mode = AuthMode::Login;
        self.security_warnings.clear();

        // Clear settings dialogs
        self.show_user_settings = false;
        self.show_change_password_dialog = false;
        self.show_delete_account_dialog = false;
        self.old_password_input.clear();
        self.new_password_input.clear();
        self.confirm_new_password_input.clear();
        self.delete_confirmation_input.clear();
    }

    fn handle_password_change(&mut self) {
        if let (Some(ref mut crypto_manager), Some(ref user)) =
            (&mut self.crypto_manager, &self.current_user)
        {
            match crypto_manager.change_password(
                &self.old_password_input,
                &self.new_password_input,
                &user.id,
            ) {
                Ok(_) => {
                    // Also update the user manager
                    if let Some(ref mut user_manager) = self.user_manager {
                        let _ = user_manager.change_password(
                            &user.username,
                            &self.old_password_input,
                            &self.new_password_input,
                        );
                    }
                    println!("Password changed successfully!");
                }
                Err(e) => {
                    eprintln!("Failed to change password: {}", e);
                }
            }
        }
    }

    fn handle_account_deletion(&mut self) {
        if let Some(ref user) = self.current_user.clone() {
            // Delete user data
            let _ = self.storage_manager.delete_user_data(&user.id);

            // Delete crypto data
            if let Some(ref crypto_manager) = self.crypto_manager {
                let _ = crypto_manager.delete_user_crypto_data(&user.id);
            }

            // Delete user account
            if let Some(ref mut user_manager) = self.user_manager {
                let _ = user_manager.delete_user(&user.username);
            }

            println!("Account deleted successfully");

            // Logout after deletion
            self.logout();
        }
    }

    fn render_auth_dialog(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(150.0);
                ui.heading("Secure Notes");
                ui.add_space(20.0);

                if self.is_authenticating {
                    ui.label("Processing... Please wait");
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
                    // Mode selection
                    ui.horizontal(|ui| {
                        ui.add_space(345.0);
                        ui.selectable_value(&mut self.auth_mode, AuthMode::Login, "Login");
                        ui.selectable_value(&mut self.auth_mode, AuthMode::Register, "Register");
                    });

                    ui.add_space(20.0);

                    // Username input
                    ui.label("Username:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.username_input).desired_width(200.0),
                    );

                    ui.add_space(10.0);

                    // Password input
                    ui.label("Password:");
                    let password_response = ui.add(
                        egui::TextEdit::singleline(&mut self.password_input)
                            .password(true)
                            .desired_width(200.0),
                    );

                    // Confirm password for registration
                    if self.auth_mode == AuthMode::Register {
                        ui.add_space(10.0);
                        ui.label("Confirm Password:");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.confirm_password_input)
                                .password(true)
                                .desired_width(200.0),
                        );
                    }

                    ui.add_space(20.0);

                    // Submit button
                    let button_text = match self.auth_mode {
                        AuthMode::Login => "Login",
                        AuthMode::Register => "Register",
                    };

                    let can_submit = !self.username_input.trim().is_empty()
                        && !self.password_input.is_empty()
                        && self.password_input.len() >= 6
                        && (self.auth_mode == AuthMode::Login
                            || self.password_input == self.confirm_password_input);

                    if ui
                        .add_enabled(can_submit, egui::Button::new(button_text))
                        .clicked()
                        || (password_response.lost_focus()
                            && ui.input(|i| i.key_pressed(egui::Key::Enter))
                            && can_submit)
                    {
                        if self.auth_mode == AuthMode::Register
                            && self.password_input != self.confirm_password_input
                        {
                            self.authentication_error = Some("Passwords do not match".to_string());
                        } else if self.password_input.len() < 6 {
                            self.authentication_error =
                                Some("Password must be at least 6 characters long".to_string());
                        } else {
                            let username = self.username_input.clone();
                            let password = self.password_input.clone();
                            let is_registration = self.auth_mode == AuthMode::Register;
                            self.start_authentication(username, password, is_registration);
                        }
                    }

                    // Show validation errors
                    if self.auth_mode == AuthMode::Register
                        && !self.password_input.is_empty()
                        && !self.confirm_password_input.is_empty()
                        && self.password_input != self.confirm_password_input
                    {
                        ui.add_space(10.0);
                        ui.colored_label(egui::Color32::YELLOW, "Passwords do not match");
                    }

                    if !self.password_input.is_empty() && self.password_input.len() < 6 {
                        ui.add_space(10.0);
                        ui.colored_label(
                            egui::Color32::YELLOW,
                            "Password must be at least 6 characters",
                        );
                    }

                    // Show authentication error
                    if let Some(error) = &self.authentication_error {
                        ui.add_space(10.0);
                        ui.colored_label(egui::Color32::RED, error);
                    }

                    // Show user count for context
                    if let Some(ref user_manager) = self.user_manager {
                        ui.add_space(20.0);
                        ui.separator();
                        ui.small(format!(
                            "Registered users: {}",
                            user_manager.get_user_count()
                        ));
                        ui.small(format!("Current time: {}", self.get_current_time()));
                    }
                }
            });
        });
    }

    fn migrate_legacy_data_if_needed(&mut self) {
        if let (Some(ref user), Some(ref crypto_manager)) =
            (&self.current_user, &self.crypto_manager)
        {
            if let Err(e) = self
                .storage_manager
                .migrate_legacy_notes(&user.id, crypto_manager)
            {
                eprintln!("Failed to migrate legacy notes: {}", e);
            }
        }
    }

    fn render_notes_sidebar(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("notes_list").show(ctx, |ui| {
            // Header with user info - Fix borrowing issue
            let username = self.current_user.as_ref().map(|u| u.username.clone());

            if let Some(username) = username {
                ui.horizontal(|ui| {
                    ui.heading("Notes");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.small_button("Logout").clicked() {
                            self.logout();
                        }
                        ui.small(format!("User: {}", username));
                    });
                });
            } else {
                ui.heading("Notes");
            }

            ui.separator();

            // Action buttons at the top
            ui.horizontal(|ui| {
                if ui.button("📝 New Note").clicked() {
                    self.show_new_note_dialog = true;
                    self.new_note_title.clear();
                }

                if ui.button("⚙️ Settings").clicked() {
                    self.show_user_settings = true;
                }
            });

            ui.separator();

            // Time format toggle
            ui.horizontal(|ui| {
                ui.label("Time format:");
                ui.selectable_value(&mut self.show_time_format, TimeFormat::Relative, "Relative");
                ui.selectable_value(&mut self.show_time_format, TimeFormat::Absolute, "Absolute");
            });

            ui.separator();

            // Calculate available height for notes list
            let available_height = ui.available_height();
            let bottom_section_height = 80.0; // Reserve space for bottom buttons
            let notes_list_height = (available_height - bottom_section_height).max(200.0);

            // Notes list with fixed height and proper clipping
            ui.allocate_ui_with_layout(
                [ui.available_width(), notes_list_height].into(),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    egui::ScrollArea::vertical()
                        .max_height(notes_list_height)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            let mut notes_vec: Vec<_> = self.notes.iter().collect();
                            notes_vec.sort_by(|a, b| b.1.modified_at.cmp(&a.1.modified_at));

                            if notes_vec.is_empty() {
                                ui.vertical_centered(|ui| {
                                    ui.add_space(50.0);
                                    ui.label("No notes yet");
                                    ui.small("Create your first note!");
                                });
                            } else {
                                for (note_id, note) in notes_vec {
                                    let is_selected =
                                        self.selected_note_id.as_ref() == Some(note_id);

                                    // Use a simple button approach but with better text handling
                                    let response = ui.add_sized(
                                        [ui.available_width(), 60.0],
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

                                    // Handle interactions
                                    if response.secondary_clicked() {
                                        self.context_menu_note_id = Some(note_id.clone());
                                        self.show_context_menu = true;
                                        self.context_menu_pos =
                                            ui.input(|i| i.pointer.hover_pos().unwrap_or_default());
                                    }

                                    if response.clicked() {
                                        self.selected_note_id = Some(note_id.clone());
                                    }

                                    // Draw text on top of the button, but properly clipped
                                    let button_rect = response.rect;
                                    let text_rect = button_rect.shrink(8.0);

                                    // Use the painter to draw text with proper clipping
                                    let painter = ui.painter_at(text_rect);

                                    // Title text
                                    let title_color = if is_selected {
                                        egui::Color32::WHITE
                                    } else {
                                        egui::Color32::LIGHT_GRAY
                                    };

                                    let title_pos = text_rect.left_top() + egui::vec2(0.0, 8.0);
                                    painter.text(
                                        title_pos,
                                        egui::Align2::LEFT_TOP,
                                        &note.title,
                                        egui::FontId::proportional(14.0),
                                        title_color,
                                    );

                                    // Time text
                                    let time_text = match self.show_time_format {
                                        TimeFormat::Relative => note.relative_time(),
                                        TimeFormat::Absolute => note.format_modified_time(),
                                    };

                                    let time_color = if is_selected {
                                        egui::Color32::from_rgb(200, 200, 200)
                                    } else {
                                        egui::Color32::GRAY
                                    };

                                    let time_pos = text_rect.left_top() + egui::vec2(0.0, 32.0);
                                    painter.text(
                                        time_pos,
                                        egui::Align2::LEFT_TOP,
                                        &time_text,
                                        egui::FontId::proportional(11.0),
                                        time_color,
                                    );

                                    ui.add_space(4.0); // Space between notes
                                }
                            }
                        });
                },
            );

            // Bottom section with fixed position
            ui.separator();

            // Security button and warnings at the bottom
            if ui.button("🔒 Security Info").clicked() {
                self.show_security_panel = !self.show_security_panel;
            }

            // Display security warnings if any (but limit the space they take)
            if !self.security_warnings.is_empty() {
                ui.separator();
                ui.colored_label(egui::Color32::from_rgb(255, 165, 0), "⚠ Security Warnings:");

                // Limit the number of warnings shown to prevent overflow
                let max_warnings = 2;
                for (i, warning) in self.security_warnings.iter().enumerate() {
                    if i >= max_warnings {
                        ui.colored_label(
                            egui::Color32::from_rgb(255, 100, 100),
                            format!(
                                "• ... and {} more warnings",
                                self.security_warnings.len() - max_warnings
                            ),
                        );
                        break;
                    }
                    ui.colored_label(
                        egui::Color32::from_rgb(255, 100, 100),
                        format!("• {}", warning),
                    );
                }
            }
        });

        // Render context menu
        self.render_context_menu(ctx);
    }

    fn render_context_menu(&mut self, ctx: &egui::Context) {
        if !self.show_context_menu {
            return;
        }

        let mut close_menu = false;
        let mut delete_note_id = None;

        egui::Area::new("context_menu".into()) // Fix: Add .into() to convert &str to Id
            .fixed_pos(self.context_menu_pos)
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                egui::Frame::popup(ui.style()).show(ui, |ui| {
                    ui.set_min_width(120.0);

                    if let Some(ref note_id) = self.context_menu_note_id {
                        if let Some(note) = self.notes.get(note_id) {
                            ui.label(format!("Note: {}", note.title));
                            ui.separator();
                        }
                    }

                    if ui.button("Delete Note").clicked() {
                        delete_note_id = self.context_menu_note_id.clone();
                        close_menu = true;
                    }

                    if ui.button("Cancel").clicked() {
                        close_menu = true;
                    }
                });
            });

        // Handle actions
        if let Some(note_id) = delete_note_id {
            self.delete_note(&note_id);
        }

        if close_menu {
            self.show_context_menu = false;
            self.context_menu_note_id = None;
        }

        // Close menu if clicked elsewhere
        if ctx.input(|i| i.pointer.any_click()) && !self.show_context_menu {
            self.show_context_menu = false;
            self.context_menu_note_id = None;
        }
    }

    fn render_user_settings(&mut self, ctx: &egui::Context) {
        if !self.show_user_settings {
            return;
        }

        let mut close_settings = false;
        let mut change_password = false;
        let mut delete_account = false;

        egui::Window::new("Settings")
            .open(&mut self.show_user_settings)
            .default_width(400.0)
            .show(ctx, |ui| {
                if let Some(ref user) = self.current_user {
                    ui.heading("Account Settings");
                    ui.separator();

                    ui.label(format!("Username: {}", user.username));
                    ui.label(format!(
                        "Account created: {}",
                        user.created_at.format("%d.%m.%Y %H:%M:%S")
                    ));

                    ui.separator();

                    // Storage information
                    if let Ok(size) = self.storage_manager.get_user_data_size(&user.id) {
                        ui.label(format!("Data size: {} bytes", size));
                    }

                    ui.separator();

                    // Change password button
                    if ui.button("Change Password").clicked() {
                        change_password = true;
                    }

                    ui.separator();

                    // Danger zone
                    ui.colored_label(egui::Color32::RED, "⚠ Danger Zone");
                    if ui.button("Delete Account").clicked() {
                        delete_account = true;
                    }

                    ui.separator();

                    if ui.button("Close").clicked() {
                        close_settings = true;
                    }
                }
            });

        if close_settings {
            self.show_user_settings = false;
        }

        if change_password {
            self.show_change_password_dialog = true;
        }

        if delete_account {
            self.show_delete_account_dialog = true;
        }
    }

    fn render_change_password_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_change_password_dialog {
            return;
        }

        let mut close_dialog = false;
        let mut submit_change = false;

        egui::Window::new("🔑 Change Password")
            .open(&mut self.show_change_password_dialog)
            .default_width(300.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);

                    ui.label("Current Password:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.old_password_input)
                            .password(true)
                            .desired_width(250.0),
                    );

                    ui.add_space(10.0);

                    ui.label("New Password:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.new_password_input)
                            .password(true)
                            .desired_width(250.0),
                    );

                    ui.add_space(10.0);

                    ui.label("Confirm New Password:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.confirm_new_password_input)
                            .password(true)
                            .desired_width(250.0),
                    );

                    ui.add_space(15.0);

                    let can_submit = !self.old_password_input.is_empty()
                        && !self.new_password_input.is_empty()
                        && self.new_password_input.len() >= 6
                        && self.new_password_input == self.confirm_new_password_input;

                    ui.horizontal(|ui| {
                        if ui
                            .add_enabled(can_submit, egui::Button::new("Change Password"))
                            .clicked()
                        {
                            submit_change = true;
                        }

                        if ui.button("Cancel").clicked() {
                            close_dialog = true;
                        }
                    });

                    // Show validation errors
                    if !self.new_password_input.is_empty() && self.new_password_input.len() < 6 {
                        ui.add_space(10.0);
                        ui.colored_label(
                            egui::Color32::YELLOW,
                            "New password must be at least 6 characters",
                        );
                    }

                    if !self.new_password_input.is_empty()
                        && !self.confirm_new_password_input.is_empty()
                        && self.new_password_input != self.confirm_new_password_input
                    {
                        ui.add_space(10.0);
                        ui.colored_label(egui::Color32::YELLOW, "Passwords do not match");
                    }

                    ui.add_space(10.0);
                });
            });

        if submit_change {
            self.handle_password_change();
            close_dialog = true;
        }

        if close_dialog {
            self.show_change_password_dialog = false;
            self.old_password_input.clear();
            self.new_password_input.clear();
            self.confirm_new_password_input.clear();
        }
    }

    fn render_delete_account_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_delete_account_dialog {
            return;
        }

        let mut close_dialog = false;
        let mut confirm_delete = false;

        egui::Window::new("Delete Account")
            .open(&mut self.show_delete_account_dialog)
            .default_width(350.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);

                    ui.colored_label(egui::Color32::RED, "WARNING");
                    ui.label("This action cannot be undone!");
                    ui.label("All your notes and data will be permanently deleted.");

                    ui.add_space(15.0);

                    ui.label("Type 'DELETE' to confirm:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.delete_confirmation_input)
                            .desired_width(250.0),
                    );

                    ui.add_space(15.0);

                    let can_delete = self.delete_confirmation_input == "DELETE";

                    ui.horizontal(|ui| {
                        if ui
                            .add_enabled(can_delete, egui::Button::new("Delete Account"))
                            .clicked()
                        {
                            confirm_delete = true;
                        }

                        if ui.button("Cancel").clicked() {
                            close_dialog = true;
                        }
                    });

                    ui.add_space(10.0);
                });
            });

        if confirm_delete {
            self.handle_account_deletion();
            close_dialog = true;
        }

        if close_dialog {
            self.show_delete_account_dialog = false;
            self.delete_confirmation_input.clear();
        }
    }

    fn render_new_note_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_new_note_dialog {
            return;
        }

        // Extract the current title to avoid borrowing issues
        let mut current_title = self.new_note_title.clone();
        let mut create_note = false;
        let mut cancel_dialog = false;

        egui::Window::new("Create New Note")
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
                        if ui.button("Create").clicked() {
                            create_note = true;
                        }

                        if ui.button("Cancel").clicked() {
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
                    ui.small(format!("Current time: {}", self.get_current_time()));
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

        let current_time = self.get_current_time();
        let has_crypto_manager = self.crypto_manager.is_some();
        let security_warnings = self.security_warnings.clone();
        let user_info = self
            .current_user
            .as_ref()
            .map(|u| (u.username.clone(), u.created_at));

        // Track if we need to run a security audit
        let mut run_audit = false;

        egui::Window::new("Security Information")
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

                if let Some((username, created_at)) = user_info {
                    ui.separator();
                    ui.heading("User Information");
                    ui.label(format!("Username: {}", username));
                    ui.label(format!(
                        "Account created: {}",
                        created_at.format("%d.%m.%Y %H:%M:%S")
                    ));
                }

                ui.separator();
                ui.heading("Security Audit");

                if has_crypto_manager && ui.button("🔍 Run Security Audit").clicked() {
                    run_audit = true;
                }

                if security_warnings.is_empty() {
                    ui.colored_label(
                        egui::Color32::from_rgb(0, 200, 0),
                        "No security issues detected",
                    );
                } else {
                    ui.colored_label(
                        egui::Color32::from_rgb(255, 100, 100),
                        "⚠ Security warnings:",
                    );
                    for warning in &security_warnings {
                        ui.colored_label(
                            egui::Color32::from_rgb(255, 150, 150),
                            format!("• {}", warning),
                        );
                    }
                }

                ui.separator();
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

        if self.show_auth_dialog {
            self.render_auth_dialog(ctx);
            return;
        }

        // Render the main application UI
        self.render_notes_sidebar(ctx);
        self.render_main_content(ctx);
        self.render_security_panel(ctx);
        self.render_new_note_dialog(ctx);
        self.render_user_settings(ctx);
        self.render_change_password_dialog(ctx);
        self.render_delete_account_dialog(ctx);

        // Auto-save functionality
        self.auto_save_if_needed();

        // Request repaint for auto-save timing and relative time updates
        ctx.request_repaint_after(std::time::Duration::from_millis(500));
    }
}
