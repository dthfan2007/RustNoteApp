// @Author: Matteo Cipriani
// @Date:   20-06-2025 08:00:00
// @Last Modified by:   Matteo Cipriani
// @Last Modified time: 24-06-2025 11:37:55

use crate::auth::{AuthMode, AuthResult};
use crate::crypto::CryptoManager;
use crate::note::Note;
use crate::storage::StorageManager;
use crate::user::{User, UserManager};
use chrono::Utc;
use chrono_tz::Europe::Zurich;
use eframe::egui;
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

#[derive(Clone, Copy, PartialEq)]
pub enum TimeFormat {
    Relative, // "2 hours ago"
    Absolute, // "15.12.2024 14:30"
}

pub struct NotesApp {
    pub notes: HashMap<String, Note>,
    pub selected_note_id: Option<String>,
    pub crypto_manager: Option<CryptoManager>,
    pub storage_manager: StorageManager,
    pub user_manager: Option<UserManager>,
    pub current_user: Option<User>,

    // Authentication UI
    pub username_input: String,
    pub password_input: String,
    pub confirm_password_input: String,
    pub is_authenticated: bool,
    pub show_auth_dialog: bool,
    pub auth_mode: AuthMode,
    pub authentication_error: Option<String>,
    pub is_authenticating: bool,
    pub auth_receiver: Option<mpsc::Receiver<AuthResult>>,
    pub auth_start_time: Option<std::time::Instant>,

    // Note management
    pub new_note_title: String,
    pub last_save_time: std::time::Instant,
    pub auto_save_delay: std::time::Duration,
    pub show_new_note_dialog: bool,

    // UI state
    pub show_security_panel: bool,
    pub security_warnings: Vec<String>,
    pub show_time_format: TimeFormat,

    // Context menu
    pub context_menu_note_id: Option<String>,
    pub show_context_menu: bool,
    pub context_menu_pos: egui::Pos2,

    // User settings
    pub show_user_settings: bool,
    pub show_change_password_dialog: bool,
    pub show_delete_account_dialog: bool,
    pub old_password_input: String,
    pub new_password_input: String,
    pub confirm_new_password_input: String,
    pub delete_confirmation_input: String,

    pub status_message: Option<String>,
    pub status_message_time: Option<std::time::Instant>,
}

impl NotesApp {
    pub fn new() -> Self {
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

            status_message: None,
            status_message_time: None,
        }
    }

    pub fn start_authentication(
        &mut self,
        username: String,
        password: String,
        is_registration: bool,
    ) {
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

    pub fn check_authentication_result(&mut self) {
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

    pub fn load_notes(&mut self) {
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

    pub fn save_notes(&self) {
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

    pub fn create_new_note(&mut self, title: String) {
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

    pub fn delete_note(&mut self, note_id: &str) {
        if let Some(note) = self.notes.get(note_id) {
            println!("Deleting note: {}", note.title);
        }

        self.notes.remove(note_id);

        if self.selected_note_id.as_ref() == Some(&note_id.to_string()) {
            self.selected_note_id = None;
        }

        self.save_notes();
    }

    pub fn auto_save_if_needed(&mut self) {
        if self.last_save_time.elapsed() >= self.auto_save_delay {
            self.save_notes();
            self.last_save_time = std::time::Instant::now();
        }
    }

    pub fn get_current_time(&self) -> String {
        let now = Utc::now().with_timezone(&Zurich);
        now.format("%d.%m.%Y %H:%M:%S").to_string()
    }

    pub fn logout(&mut self) {
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

    pub fn migrate_legacy_data_if_needed(&mut self) {
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

    pub fn export_note_to_file(&self, note_id: &str) {
        if let Some(note) = self.notes.get(note_id) {
            // Create default filename from note title
            let safe_title = note
                .title
                .chars()
                .map(|c| {
                    if c.is_alphanumeric() || c == ' ' || c == '-' || c == '_' {
                        c
                    } else {
                        '_'
                    }
                })
                .collect::<String>()
                .trim()
                .to_string();

            let default_filename = if safe_title.is_empty() {
                "Untitled_Note.txt".to_string()
            } else {
                format!("{}.txt", safe_title)
            };

            // Show save dialog
            if let Some(path) = rfd::FileDialog::new()
                .set_title("Export Note")
                .set_file_name(&default_filename)
                .add_filter("Text files", &["txt"])
                .add_filter("All files", &["*"])
                .save_file()
            {
                match self.write_note_to_file(note, &path) {
                    Ok(_) => {
                        println!("Note '{}' exported successfully to: {:?}", note.title, path);
                    }
                    Err(e) => {
                        eprintln!("Failed to export note '{}': {}", note.title, e);
                    }
                }
            }
        }
    }

    fn write_note_to_file(
        &self,
        note: &Note,
        path: &std::path::Path,
    ) -> Result<(), std::io::Error> {
        use std::io::Write;

        let mut file = std::fs::File::create(path)?;

        // Write note with metadata header
        writeln!(file, "Title: {}", note.title)?;
        writeln!(file, "Created: {}", note.format_created_time())?;
        writeln!(file, "Modified: {}", note.format_modified_time())?;
        writeln!(file, "ID: {}", note.id)?;
        writeln!(file, "{}", "=".repeat(50))?;
        writeln!(file)?;
        write!(file, "{}", note.content)?;

        Ok(())
    }
}

impl eframe::App for NotesApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for authentication results
        self.check_authentication_result();

        if self.is_authenticated {
            ctx.input(|i| {
                // Ctrl+N for new note
                if i.modifiers.ctrl && i.key_pressed(egui::Key::N) {
                    self.show_new_note_dialog = true;
                    self.new_note_title.clear();
                }

                // Ctrl+S for manual save
                if i.modifiers.ctrl && i.key_pressed(egui::Key::S) {
                    self.save_notes();
                    self.status_message = Some("Note saved!".to_string());
                }

                // Escape to close dialogs
                if i.key_pressed(egui::Key::Escape) {
                    if self.show_new_note_dialog {
                        self.show_new_note_dialog = false;
                        self.new_note_title.clear();
                    }
                    if self.show_security_panel {
                        self.show_security_panel = false;
                    }
                    if self.show_user_settings {
                        self.show_user_settings = false;
                    }
                }

                // Ctrl+T for switching between modes
                if i.modifiers.ctrl && i.key_pressed(egui::Key::T) {
                    self.show_time_format = match self.show_time_format {
                        TimeFormat::Relative => {
                            self.status_message =
                                Some("Time format: Absolute (15.12.2024 14:30)".to_string());
                            TimeFormat::Absolute
                        }
                        TimeFormat::Absolute => {
                            self.status_message =
                                Some("Time format: Relative (2 hours ago)".to_string());
                            TimeFormat::Relative
                        }
                    };
                    self.status_message_time = Some(std::time::Instant::now());
                }

                // Ctrl+R for Relative time format
                if i.modifiers.ctrl && i.key_pressed(egui::Key::R) {
                    self.show_time_format = TimeFormat::Relative;
                    self.status_message =
                        Some("Time format: Relative (X [minutes | hours | days] ago)".to_string());
                    self.status_message_time = Some(std::time::Instant::now());
                }

                // Ctrl+Alt+A for Absolute time format
                if i.modifiers.ctrl && i.modifiers.alt && i.key_pressed(egui::Key::A) {
                    self.show_time_format = TimeFormat::Absolute;
                    self.status_message =
                        Some("Time format: Absolute (dd.mm.YYYY hh:mm)".to_string());
                    self.status_message_time = Some(std::time::Instant::now());
                }

                // Ctrl+E to export note
                if i.modifiers.ctrl && i.key_pressed(egui::Key::E) {
                    if let Some(ref note_id) = self.selected_note_id {
                        self.export_note_to_file(note_id);
                    }
                }
            });

            // Clear status message after 3 seconds
            if let Some(message_time) = self.status_message_time {
                if message_time.elapsed() > std::time::Duration::from_secs(3) {
                    self.status_message = None;
                    self.status_message_time = None;
                }
            }
        }

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
