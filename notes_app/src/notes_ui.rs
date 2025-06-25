// @Author: Matteo Cipriani
// @Date:   20-06-2025 08:00:00
// @Last Modified by:   Matteo Cipriani
// @Last Modified time: 24-06-2025 16:35:19

use crate::app::{NotesApp, TimeFormat};
use eframe::egui;

impl NotesApp {
    pub fn render_notes_sidebar(&mut self, ctx: &egui::Context) {
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
                if ui.button("New Note").on_hover_text("Ctrl + N").clicked() {
                    self.show_new_note_dialog = true;
                    self.new_note_title.clear();
                }

                if ui.button("Settings").clicked() {
                    self.show_user_settings = true;
                }
            });

            ui.separator();

            // Time format toggle
            ui.horizontal(|ui| {
                ui.label("Time format:");
                ui.selectable_value(&mut self.show_time_format, TimeFormat::Relative, "Relative")
                    .on_hover_text("Ctrl + R");
                ui.selectable_value(&mut self.show_time_format, TimeFormat::Absolute, "Absolute")
                    .on_hover_text("Ctrl + Alt + A");
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
            if ui.button("Security Info").clicked() {
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

    pub fn render_context_menu(&mut self, ctx: &egui::Context) {
        if !self.show_context_menu {
            return;
        }

        let mut close_menu = false;
        let mut delete_note_id = None;
        let mut export_note_id = None;

        egui::Area::new("context_menu".into())
            .fixed_pos(self.context_menu_pos)
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                egui::Frame::popup(ui.style()).show(ui, |ui| {
                    ui.set_min_width(150.0);

                    if let Some(ref note_id) = self.context_menu_note_id {
                        if let Some(note) = self.notes.get(note_id) {
                            ui.label(format!("Note: {}", note.title));
                            ui.separator();
                        }

                        // Export option
                        if ui.button("Export to file").clicked() {
                            export_note_id = Some(note_id.clone());
                            close_menu = true;
                        }

                        ui.separator();

                        // Delete option
                        if ui.button("Delete Note").clicked() {
                            delete_note_id = Some(note_id.clone());
                            close_menu = true;
                        }

                        ui.separator();
                    }

                    if ui.button("Cancel").clicked() {
                        close_menu = true;
                    }
                });
            });

        // Handle actions
        if let Some(note_id) = export_note_id {
            self.export_note_to_file(&note_id);
        }

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

    pub fn render_main_content(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Show status message at the top if present
            if let Some(ref message) = self.status_message {
                ui.horizontal(|ui| {
                    ui.colored_label(egui::Color32::from_rgb(100, 200, 100), "ℹ");
                    ui.label(message);
                });
                ui.separator();
            }

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

                // Display the header with note info and export button
                ui.horizontal(|ui| {
                    ui.heading(&note_title);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Export button
                        if ui
                            .button("Export (Ctrl + E)")
                            .on_hover_text("Export note to .txt file")
                            .clicked()
                        {
                            self.export_note_to_file(&note_id);
                        }

                        ui.separator();

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

    pub fn render_new_note_dialog(&mut self, ctx: &egui::Context) {
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

    pub fn render_security_panel(&mut self, ctx: &egui::Context) {
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

                if has_crypto_manager && ui.button("Run Security Audit").clicked() {
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
