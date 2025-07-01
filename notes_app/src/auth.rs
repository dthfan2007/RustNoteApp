// @Author: Matteo Cipriani
// @Date:   20-06-2025 08:08:38
// @Last Modified by:   Matteo Cipriani
// @Last Modified time: 01-07-2025 09:04:55
//! # Authentication Module
//!
//! Handles user authentication UI and related functionality including
//! login, registration, and authentication state management.

use crate::app::NotesApp;
use crate::crypto::CryptoManager;
use crate::user::User;
use eframe::egui;

/// Represents the current authentication mode in the UI.
#[derive(Clone, Copy, PartialEq)]
pub enum AuthMode {
    /// User is attempting to log in with existing credentials
    Login,
    /// User is creating a new account
    Register,
}

/// Result of an authentication attempt.
///
/// Contains either successful authentication data (crypto manager and user)
/// or an error message describing what went wrong.
pub enum AuthResult {
    /// Authentication succeeded with crypto manager and user data
    Success(CryptoManager, User),
    /// Authentication failed with error message
    Error(String),
}

impl NotesApp {
    /// Renders the authentication dialog UI.
    ///
    /// This method displays the login/registration form including:
    /// - Mode selection (Login/Register)
    /// - Username and password input fields
    /// - Confirm password field for registration
    /// - Submit button with validation
    /// - Error messages and loading states
    /// - User count and current time display
    ///
    /// The dialog handles keyboard shortcuts (Enter to submit) and
    /// provides real-time validation feedback.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The egui context for rendering UI elements
    pub fn render_auth_dialog(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(75.0);
                ui.heading("Secure Notes");
                ui.add_space(20.0);

                if self.is_authenticating {
                    // Show loading state with progress information
                    ui.label("Processing... Please wait");
                    ui.spinner();

                    // Show elapsed time for user feedback
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

                        // Show error state and cancel option for very long waits
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
                    let screen_width = ui.available_width();

                    // Mode selection - calculate actual widget width and center it
                    ui.horizontal(|ui| {
                        // Calculate actual text widths for proper centering
                        let login_text_size = ui
                            .fonts(|f| {
                                f.layout_no_wrap(
                                    "Login".to_string(),
                                    egui::FontId::default(),
                                    egui::Color32::WHITE,
                                )
                            })
                            .size();

                        let register_text_size = ui
                            .fonts(|f| {
                                f.layout_no_wrap(
                                    "Register".to_string(),
                                    egui::FontId::default(),
                                    egui::Color32::WHITE,
                                )
                            })
                            .size();

                        let spacing = ui.spacing().item_spacing.x;
                        let total_width = login_text_size.x + register_text_size.x + spacing;

                        let padding = (screen_width - total_width) / 2.0;

                        ui.add_space(padding.max(0.0));
                        ui.selectable_value(&mut self.auth_mode, AuthMode::Login, "Login");
                        ui.selectable_value(&mut self.auth_mode, AuthMode::Register, "Register");
                    });

                    ui.add_space(20.0);

                    // Username input field
                    ui.label("Username:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.username_input).desired_width(200.0),
                    );

                    ui.add_space(10.0);

                    // Password input field
                    ui.label("Password:");
                    let password_response = ui.add(
                        egui::TextEdit::singleline(&mut self.password_input)
                            .password(true)
                            .desired_width(200.0),
                    );

                    // Confirm password for registration mode
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

                    // Submit button with validation
                    let button_text = match self.auth_mode {
                        AuthMode::Login => "Login",
                        AuthMode::Register => "Register",
                    };

                    let can_submit = !self.username_input.trim().is_empty()
                        && !self.password_input.is_empty()
                        && self.password_input.len() >= 6
                        && (self.auth_mode == AuthMode::Login
                            || self.password_input == self.confirm_password_input);

                    // Handle button click or Enter key press
                    if ui
                        .add_enabled(can_submit, egui::Button::new(button_text))
                        .clicked()
                        || (password_response.lost_focus()
                            && ui.input(|i| i.key_pressed(egui::Key::Enter))
                            && can_submit)
                    {
                        // Validate input before starting authentication
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

                    // Show real-time validation errors
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

                    // Show authentication error messages
                    if let Some(error) = &self.authentication_error {
                        ui.add_space(10.0);
                        ui.colored_label(egui::Color32::RED, error);
                    }

                    // Show user count and current time for context
                    if let Some(ref user_manager) = self.user_manager {
                        let screen_height = ui.available_height();
                        ui.add_space(screen_height - 45.0);
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
}
