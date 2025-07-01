// @Author: Matteo Cipriani
// @Date:   20-06-2025 08:08:29
// @Last Modified by:   Matteo Cipriani
// @Last Modified time: 01-07-2025 09:05:13
//! # Application Module
//!
//! Main application state and logic for the Secure Notes application.
//! Handles authentication, note management, UI state, and application lifecycle.

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

/// Time display format options for the UI.
#[derive(Clone, Copy, PartialEq)]
pub enum TimeFormat {
    /// Show relative time like "2 hours ago"
    Relative,
    /// Show absolute time like "15.12.2024 14:30"
    Absolute,
}

/// Main application state structure.
///
/// Contains all the state needed for the secure notes application including
/// user authentication, note storage, UI state, and various dialog states.
pub struct NotesApp {
    // Core data
    /// Map of note IDs to Note objects
    pub notes: HashMap<String, Note>,
    /// Currently selected note ID for editing
    pub selected_note_id: Option<String>,
    /// Cryptographic manager for encryption/decryption
    pub crypto_manager: Option<CryptoManager>,
    /// Storage manager for file operations
    pub storage_manager: StorageManager,
    /// User management system
    pub user_manager: Option<UserManager>,
    /// Currently authenticated user
    pub current_user: Option<User>,

    // Authentication UI state
    /// Username input field content
    pub username_input: String,
    /// Password input field content
    pub password_input: String,
    /// Confirm password input field content
    pub confirm_password_input: String,
    /// Whether user is currently authenticated
    pub is_authenticated: bool,
    /// Whether to show the authentication dialog
    pub show_auth_dialog: bool,
    /// Current authentication mode (Login/Register)
    pub auth_mode: AuthMode,
    /// Current authentication error message
    pub authentication_error: Option<String>,
    /// Whether authentication is in progress
    pub is_authenticating: bool,
    /// Channel receiver for authentication results
    pub auth_receiver: Option<mpsc::Receiver<AuthResult>>,
    /// Start time of current authentication attempt
    pub auth_start_time: Option<std::time::Instant>,

    // Note management state
    /// Input field for new note title
    pub new_note_title: String,
    /// Last time notes were saved
    pub last_save_time: std::time::Instant,
    /// Delay before auto-saving
    pub auto_save_delay: std::time::Duration,
    /// Whether to show the new note dialog
    pub show_new_note_dialog: bool,

    // UI state
    /// Whether to show the security information panel
    pub show_security_panel: bool,
    /// List of current security warnings
    pub security_warnings: Vec<String>,
    /// Current time display format
    pub show_time_format: TimeFormat,

    // Context menu state
    /// Note ID for which context menu is shown
    pub context_menu_note_id: Option<String>,
    /// Whether context menu is visible
    pub show_context_menu: bool,
    /// Position of the context menu
    pub context_menu_pos: egui::Pos2,

    // User settings state
    /// Whether to show user settings dialog
    pub show_user_settings: bool,
    /// Whether to show change password dialog
    pub show_change_password_dialog: bool,
    /// Whether to show delete account dialog
    pub show_delete_account_dialog: bool,
    /// Old password input for password change
    pub old_password_input: String,
    /// New password input for password change
    pub new_password_input: String,
    /// Confirm new password input for password change
    pub confirm_new_password_input: String,
    /// Confirmation input for account deletion
    pub delete_confirmation_input: String,

    // Status and messaging
    /// Current status message to display
    pub status_message: Option<String>,
    /// Time when status message was set
    pub status_message_time: Option<std::time::Instant>,
}

impl NotesApp {
    /// Creates a new instance of the NotesApp.
    ///
    /// Initializes all state with default values and attempts to create
    /// a UserManager. If UserManager creation fails, the app will still
    /// function but without user management capabilities.
    ///
    /// # Returns
    ///
    /// * `Self` - A new NotesApp instance
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

    /// Starts the authentication process in a background thread.
    ///
    /// This method spawns a background thread to handle the potentially
    /// time-consuming authentication process (especially key derivation)
    /// without blocking the UI. Results are communicated back via a channel.
    ///
    /// # Arguments
    ///
    /// * `username` - The username to authenticate
    /// * `password` - The password to authenticate with
    /// * `is_registration` - Whether this is a registration (true) or login (false)
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

    /// Checks for authentication results from the background thread.
    ///
    /// This method should be called regularly (e.g., in the update loop)
    /// to check if the background authentication process has completed
    /// and handle the results appropriately.
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

    /// Loads notes from storage for the current user.
    ///
    /// Attempts to load encrypted notes from the user's storage directory.
    /// If loading fails, an error is logged but the application continues
    /// with an empty note set.
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

    /// Saves all notes to encrypted storage.
    ///
    /// Encrypts and saves all current notes to the user's storage directory.
    /// If saving fails, an error is logged but the application continues.
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

    /// Creates a new note with the given title.
    ///
    /// Creates a new note, adds it to the notes collection, selects it
    /// for editing, and saves the updated notes to storage.
    ///
    /// # Arguments
    ///
    /// * `title` - The title for the new note. If empty, defaults to "Untitled Note"
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

    /// Deletes a note by its ID.
    ///
    /// Removes the note from the collection, deselects it if it was selected,
    /// and saves the updated notes to storage.
    ///
    /// # Arguments
    ///
    /// * `note_id` - The ID of the note to delete
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

    /// Performs auto-save if enough time has elapsed since the last save.
    ///
    /// Checks if the auto-save delay has passed and saves notes if needed.
    /// This helps prevent data loss without constantly writing to disk.
    pub fn auto_save_if_needed(&mut self) {
        if self.last_save_time.elapsed() >= self.auto_save_delay {
            self.save_notes();
            self.last_save_time = std::time::Instant::now();
        }
    }

    /// Gets the current time formatted for display in Swiss timezone.
    ///
    /// # Returns
    ///
    /// * `String` - Current time in "DD.MM.YYYY HH:MM:SS" format
    pub fn get_current_time(&self) -> String {
        let now = Utc::now().with_timezone(&Zurich);
        now.format("%d.%m.%Y %H:%M:%S").to_string()
    }

    /// Logs out the current user and resets application state.
    ///
    /// Clears all user-specific data, resets UI state, and returns
    /// to the authentication dialog. This ensures no sensitive data
    /// remains in memory after logout.
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

    /// Migrates legacy data from old storage format if needed.
    ///
    /// Checks for notes stored in the old format (before user-specific storage)
    /// and migrates them to the current user's storage directory.
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

    /// Exports a note to a text file.
    ///
    /// Opens a file dialog for the user to choose where to save the note,
    /// then writes the note content along with metadata to the selected file.
    ///
    /// # Arguments
    ///
    /// * `note_id` - The ID of the note to export
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

    /// Writes a note to a file with metadata header.
    ///
    /// # Arguments
    ///
    /// * `note` - The note to write
    /// * `path` - The file path to write to
    ///
    /// # Returns
    ///
    /// * `Result<(), std::io::Error>` - Ok if successful, Err if file operation failed
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
    /// Main update loop for the application.
    ///
    /// This method is called by eframe on every frame and handles:
    /// - Authentication result checking
    /// - Keyboard shortcuts
    /// - UI rendering
    /// - Auto-save functionality
    /// - Status message management
    ///
    /// # Arguments
    ///
    /// * `ctx` - The egui context
    /// * `_frame` - The eframe frame (unused)
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

                // Ctrl+T for switching between time display modes
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
