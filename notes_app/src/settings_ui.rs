// @Author: Matteo Cipriani
// @Date:   20-06-2025 08:00:00
// @Last Modified by:   Matteo Cipriani
// @Last Modified time: 20-06-2025 16:40:45

use crate::app::NotesApp;
use eframe::egui;

impl NotesApp {
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

    pub fn handle_account_deletion(&mut self) {
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
}
