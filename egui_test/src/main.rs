// @Author: Your name
// @Date:   02-06-2025 13:39:59
// @Last Modified by:   Your name
// @Last Modified time: 02-06-2025 18:06:23
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(rustdoc::missing_crate_level_docs)]

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use egui::{ColorImage, TextureOptions, Vec2};
use rand::{rngs::OsRng, RngCore};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 300.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Encrypted Text App",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            // Fix: Wrap the result in Ok()
            Ok(Box::new(MyApp::new()) as Box<dyn eframe::App>)
        }),
    )
}

#[derive(Serialize, Deserialize)]
struct EncryptedData {
    encrypted_name: Vec<u8>,
    nonce: Vec<u8>,
    salt: String,
    password_hash: String,
}

struct MyApp {
    name: String,
    age: u32,
    ferris_texture: Option<egui::TextureHandle>,
    password: String,
    is_unlocked: bool,
    password_input: String,
    show_password_dialog: bool,
    encryption_key: Option<[u8; 32]>,
    error_message: String,
    salt: Option<String>, // Add this field to store the salt
}

impl MyApp {
    fn new() -> Self {
        let mut app = Self {
            name: String::new(),
            age: 18,
            ferris_texture: None,
            password: String::new(),
            is_unlocked: false,
            password_input: String::new(),
            show_password_dialog: false,
            encryption_key: None,
            error_message: String::new(),
            salt: None, // Initialize salt as None
        };

        // Check if encrypted data exists
        if app.get_data_file_path().exists() {
            app.show_password_dialog = true;
        } else {
            app.show_password_dialog = true; // Show dialog to set initial password
        }

        app
    }

    fn get_data_file_path(&self) -> PathBuf {
        let mut path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        path.push("encrypted_data.json");
        path
    }

    fn derive_key_from_password(password: &str, salt: &[u8]) -> [u8; 32] {
        let argon2 = Argon2::default();
        let mut key = [0u8; 32];
        argon2
            .hash_password_into(password.as_bytes(), salt, &mut key)
            .expect("Failed to derive key");
        key
    }

    fn encrypt_text(&self, text: &str, key: &[u8; 32]) -> Result<(Vec<u8>, Vec<u8>), String> {
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, text.as_bytes())
            .map_err(|e| format!("Encryption failed: {}", e))?;

        Ok((ciphertext, nonce_bytes.to_vec()))
    }

    fn decrypt_text(
        &self,
        ciphertext: &[u8],
        nonce: &[u8],
        key: &[u8; 32],
    ) -> Result<String, String> {
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
        let nonce = Nonce::from_slice(nonce);

        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| format!("Decryption failed: {}", e))?;

        String::from_utf8(plaintext).map_err(|e| format!("Invalid UTF-8: {}", e))
    }

    fn save_encrypted_data(&self) -> Result<(), String> {
        if let (Some(key), Some(salt_str)) = (&self.encryption_key, &self.salt) {
            let (encrypted_name, nonce) = self.encrypt_text(&self.name, key)?;

            // Use the existing password hash (we don't need to rehash it every time)
            let salt =
                SaltString::from_b64(salt_str).map_err(|e| format!("Invalid salt: {}", e))?;
            let argon2 = Argon2::default();
            let password_hash = argon2
                .hash_password(self.password.as_bytes(), &salt)
                .map_err(|e| format!("Password hashing failed: {}", e))?
                .to_string();

            let encrypted_data = EncryptedData {
                encrypted_name,
                nonce,
                salt: salt_str.clone(),
                password_hash,
            };

            let json = serde_json::to_string_pretty(&encrypted_data)
                .map_err(|e| format!("JSON serialization failed: {}", e))?;

            fs::write(self.get_data_file_path(), json)
                .map_err(|e| format!("File write failed: {}", e))?;
        }
        Ok(())
    }

    fn load_encrypted_data(&mut self, password: &str) -> Result<(), String> {
        let file_path = self.get_data_file_path();
        if !file_path.exists() {
            return Err("No encrypted data file found".to_string());
        }

        let json = fs::read_to_string(file_path).map_err(|e| format!("File read failed: {}", e))?;

        let encrypted_data: EncryptedData = serde_json::from_str(&json)
            .map_err(|e| format!("JSON deserialization failed: {}", e))?;

        // Verify password
        let parsed_hash = PasswordHash::new(&encrypted_data.password_hash)
            .map_err(|e| format!("Invalid password hash: {}", e))?;

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| "Invalid password".to_string())?;

        // Derive key and decrypt using the stored salt
        let key = Self::derive_key_from_password(password, encrypted_data.salt.as_bytes());
        let decrypted_name =
            self.decrypt_text(&encrypted_data.encrypted_name, &encrypted_data.nonce, &key)?;

        self.name = decrypted_name;
        self.password = password.to_string();
        self.encryption_key = Some(key);
        self.salt = Some(encrypted_data.salt); // Store the salt
        self.is_unlocked = true;

        Ok(())
    }

    fn set_new_password(&mut self, password: &str) -> Result<(), String> {
        if password.len() < 8 {
            return Err("Password must be at least 8 characters long".to_string());
        }

        let salt = SaltString::generate(&mut rand::thread_rng());
        let salt_str = salt.as_str().to_string();
        let key = Self::derive_key_from_password(password, salt_str.as_bytes());

        self.password = password.to_string();
        self.encryption_key = Some(key);
        self.salt = Some(salt_str); // Store the salt
        self.is_unlocked = true;
        self.name = "Matteo".to_string(); // Default name for new setup

        Ok(())
    }

    fn increment(&mut self) {
        if self.age < 120 {
            self.age += 1;
        }
    }

    fn decrement(&mut self) {
        if self.age > 0 {
            self.age -= 1;
        }
    }

    // Fix: Add these helper methods to avoid borrow issues
    fn try_load_data(&mut self) {
        let password = self.password_input.clone();
        match self.load_encrypted_data(&password) {
            Ok(()) => {
                self.show_password_dialog = false;
                self.password_input.clear();
                self.error_message.clear();
            }
            Err(e) => {
                self.error_message = e;
            }
        }
    }

    fn try_set_password(&mut self) {
        let password = self.password_input.clone();
        match self.set_new_password(&password) {
            Ok(()) => {
                self.show_password_dialog = false;
                self.password_input.clear();
                self.error_message.clear();
            }
            Err(e) => {
                self.error_message = e;
            }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Load Ferris image
        if self.ferris_texture.is_none() {
            let image = include_bytes!("../assets/images/ferris.png");
            let image = image::load_from_memory(image)
                .expect("Failed to load image")
                .to_rgba8();

            let (width, height) = image.dimensions();
            let pixels = image.as_flat_samples();
            let color_image = ColorImage::from_rgba_unmultiplied(
                [width as usize, height as usize],
                pixels.as_slice(),
            );

            let texture = ctx.load_texture("ferris", color_image, TextureOptions::default());
            self.ferris_texture = Some(texture);
        }

        // Show password dialog if needed
        if self.show_password_dialog {
            egui::Window::new("Password Required")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        if self.get_data_file_path().exists() {
                            ui.label("Enter your password to unlock encrypted data:");
                        } else {
                            ui.label("Set a password to encrypt your data:");
                            ui.label("(Minimum 8 characters)");
                        }

                        ui.add_space(10.0);

                        let response = ui.add(
                            egui::TextEdit::singleline(&mut self.password_input)
                                .password(true)
                                .hint_text("Password"),
                        );

                        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            // Handle enter key
                            if self.get_data_file_path().exists() {
                                // Fix: Use the helper method
                                self.try_load_data();
                            } else {
                                // Fix: Use the helper method
                                self.try_set_password();
                            }
                        }

                        ui.add_space(10.0);

                        ui.horizontal(|ui| {
                            if ui.button("OK").clicked() {
                                if self.get_data_file_path().exists() {
                                    // Fix: Use the helper method
                                    self.try_load_data();
                                } else {
                                    // Fix: Use the helper method
                                    self.try_set_password();
                                }
                            }

                            if ui.button("Cancel").clicked() {
                                std::process::exit(0);
                            }
                        });

                        if !self.error_message.is_empty() {
                            ui.add_space(10.0);
                            ui.colored_label(egui::Color32::RED, &self.error_message);
                        }
                    });
                });
            return;
        }

        // Main application UI (only shown when unlocked)
        if self.is_unlocked {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Encrypted Text Application");

                ui.horizontal(|ui| {
                    let name_label = ui.label("Your name: ");
                    let response = ui
                        .text_edit_multiline(&mut self.name)
                        .labelled_by(name_label.id);

                    // Auto-save when text changes
                    if response.changed() {
                        if let Err(e) = self.save_encrypted_data() {
                            eprintln!("Failed to save data: {}", e);
                        }
                    }
                });

                ui.add(egui::Slider::new(&mut self.age, 0..=120).text("years old"));
                ui.label(format!(
                    "Hello {}, you are {} years old!",
                    self.name, self.age
                ));

                ui.horizontal(|ui| {
                    if ui.button("Increment").clicked() {
                        self.increment();
                    }

                    if ui.button("Decrement").clicked() {
                        self.decrement();
                    }

                    if ui.button("Save Data").clicked() {
                        match self.save_encrypted_data() {
                            Ok(()) => {
                                ui.label("✓ Data saved successfully!");
                            }
                            Err(e) => {
                                ui.colored_label(egui::Color32::RED, format!("Save failed: {}", e));
                            }
                        }
                    }
                });

                if let Some(texture) = &self.ferris_texture {
                    let custom_size = Vec2::new(125.0, 85.0);
                    ui.image((texture.id(), custom_size));
                }

                ui.separator();
                ui.small("Data is automatically encrypted and saved when you type.");
            });
        }
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // Save encrypted data when app closes
        if self.is_unlocked {
            if let Err(e) = self.save_encrypted_data() {
                eprintln!("Failed to save data on exit: {}", e);
            }
        }
    }
}
