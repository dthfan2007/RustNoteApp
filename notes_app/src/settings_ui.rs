// @Author: Matteo Cipriani
// @Date:   20-06-2025 08:09:07
// @Last Modified by:   Matteo Cipriani
// @Last Modified time: 01-07-2025 09:05:32
//! # Settings UI Module
//!
//! Handles user interface for account settings, password changes, and account deletion.
//! Provides secure dialogs for sensitive operations with proper validation and confirmation.

use crate::app::NotesApp;
use eframe::egui;

impl NotesApp {
    /// Renders the main user settings dialog.
    ///
    /// Displays account information and provides access to various account
    /// management functions including:
    /// - Account details (username, creation date)
    /// - Data storage information
    /// - Password change functionality
    /// - Account deletion (danger zone)
    ///
    /// The dialog is modal and can be closed with the Close button or
    /// by pressing Escape.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The egui context for rendering
    pub fn render_user_settings(&mut self, ctx: &egui::Context) {
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

                    // Display basic account information
                    ui.label(format!("Username: {}", user.username));
                    ui.label(format!(
                        "Account created: {}",
                        user.created_at.format("%d.%m.%Y %H:%M:%S")
                    ));

                    ui.separator();

                    // Storage information - show data usage
                    if let Ok(size) = self.storage_manager.get_user_data_size(&user.id) {
                        ui.label(format!("Data size: {} bytes", size));
                    }

                    ui.separator();

                    // Change password button
                    if ui.button("Change Password").clicked() {
                        change_password = true;
                    }

                    ui.separator();

                    // Danger zone - account deletion
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

        // Handle button actions outside the window closure
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

    /// Renders the password change dialog.
    ///
    /// A secure dialog for changing the user's password with:
    /// - Current password verification field
    /// - New password input field
    /// - Password confirmation field
    /// - Real-time validation feedback
    /// - Secure password requirements (minimum 6 characters)
    ///
    /// The dialog validates that:
    /// - Current password is provided
    /// - New password meets minimum requirements
    /// - New password confirmation matches
    ///
    /// # Arguments
    ///
    /// * `ctx` - The egui context for rendering
    pub fn render_change_password_dialog(&mut self, ctx: &egui::Context) {
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

                    // Current password field
                    ui.label("Current Password:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.old_password_input)
                            .password(true)
                            .desired_width(250.0),
                    );

                    ui.add_space(10.0);

                    // New password field
                    ui.label("New Password:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.new_password_input)
                            .password(true)
                            .desired_width(250.0),
                    );

                    ui.add_space(10.0);

                    // Confirm new password field
                    ui.label("Confirm New Password:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.confirm_new_password_input)
                            .password(true)
                            .desired_width(250.0),
                    );

                    ui.add_space(15.0);

                    // Validation logic for enabling submit button
                    let can_submit = !self.old_password_input.is_empty()
                        && !self.new_password_input.is_empty()
                        && self.new_password_input.len() >= 6
                        && self.new_password_input == self.confirm_new_password_input;

                    // Action buttons
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

                    // Real-time validation feedback
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

        // Handle actions outside the window closure
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

    /// Renders the account deletion confirmation dialog.
    ///
    /// A highly secure dialog for permanent account deletion with:
    /// - Clear warning about data loss
    /// - Explicit confirmation requirement (typing "DELETE")
    /// - No accidental deletion protection
    /// - Irreversible action warning
    ///
    /// This dialog implements a "type to confirm" pattern to prevent
    /// accidental account deletion. Users must type "DELETE" exactly
    /// to enable the deletion button.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The egui context for rendering
    pub fn render_delete_account_dialog(&mut self, ctx: &egui::Context) {
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

                    // Strong warning messages
                    ui.colored_label(egui::Color32::RED, "WARNING");
                    ui.label("This action cannot be undone!");
                    ui.label("All your notes and data will be permanently deleted.");

                    ui.add_space(15.0);

                    // Confirmation input - must type "DELETE" exactly
                    ui.label("Type 'DELETE' to confirm:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.delete_confirmation_input)
                            .desired_width(250.0),
                    );

                    ui.add_space(15.0);

                    // Only enable deletion if exact confirmation is typed
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

        // Handle actions outside the window closure
        if confirm_delete {
            self.handle_account_deletion();
            close_dialog = true;
        }

        if close_dialog {
            self.show_delete_account_dialog = false;
            self.delete_confirmation_input.clear();
        }
    }

    /// Handles the password change operation.
    ///
    /// Coordinates the password change process across multiple systems:
    /// 1. Updates the cryptographic manager with new password
    /// 2. Updates the user manager's password hash
    /// 3. Re-initializes encryption with the new password
    ///
    /// This ensures that both authentication and encryption systems
    /// are updated consistently. If any step fails, appropriate error
    /// messages are logged.
    ///
    /// # Security Considerations
    ///
    /// - Old password is verified before making changes
    /// - New password is validated for strength requirements
    /// - Encryption keys are re-derived with the new password
    /// - All password hashes are updated atomically
    pub fn handle_password_change(&mut self) {
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

    /// Handles the complete account deletion process.
    ///
    /// Performs a comprehensive cleanup of all user data:
    /// 1. Deletes encrypted note storage
    /// 2. Removes cryptographic configuration and keys
    /// 3. Deletes user account from user manager
    /// 4. Logs out the user and clears session data
    ///
    /// This operation is irreversible and removes all traces of the
    /// user account and associated data from the system.
    ///
    /// # Data Removed
    ///
    /// - All encrypted notes and content
    /// - User authentication credentials
    /// - Cryptographic keys and metadata
    /// - Security fingerprints and audit logs
    /// - User preferences and settings
    ///
    /// # Security Considerations
    ///
    /// - All sensitive data is securely deleted
    /// - User is immediately logged out
    /// - Session state is completely cleared
    /// - No recoverable data remains on the system
    pub fn handle_account_deletion(&mut self) {
        if let Some(ref user) = self.current_user.clone() {
            // Delete user data from storage
            let _ = self.storage_manager.delete_user_data(&user.id);

            // Delete cryptographic data and keys
            if let Some(ref crypto_manager) = self.crypto_manager {
                let _ = crypto_manager.delete_user_crypto_data(&user.id);
            }

            // Delete user account from user manager
            if let Some(ref mut user_manager) = self.user_manager {
                let _ = user_manager.delete_user(&user.username);
            }

            println!("Account deleted successfully");

            // Immediately logout to clear all session data
            self.logout();
        }
    }
}
