// @Author: Matteo Cipriani
// @Date:   04-06-2025 10:29:39
// @Last Modified by:   Matteo Cipriani
// @Last Modified time: 20-06-2025 08:10:41
use crate::crypto::CryptoManager;
use crate::note::Note;
use anyhow::Result;
use serde_json;
use std::collections::HashMap;
use std::fs;

pub struct StorageManager {
    data_dir: std::path::PathBuf,
}

impl StorageManager {
    pub fn new() -> Self {
        let mut data_dir = dirs::config_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        data_dir.push("secure_notes");

        Self { data_dir }
    }

    pub fn save_user_notes(
        &self,
        user_id: &str,
        notes: &HashMap<String, Note>,
        crypto: &CryptoManager,
    ) -> Result<()> {
        let json_data = serde_json::to_string(notes)?;
        let encrypted_data = crypto.encrypt(json_data.as_bytes())?;

        // Create user-specific directory
        let user_dir = self.data_dir.join("users").join(user_id);
        fs::create_dir_all(&user_dir)?;

        let notes_file = user_dir.join("notes.enc");
        fs::write(&notes_file, encrypted_data)?;

        // Set secure file permissions on Unix systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&notes_file)?.permissions();
            perms.set_mode(0o600); // Read/write for owner only
            fs::set_permissions(&notes_file, perms)?;
        }

        println!("Saved {} notes for user {}", notes.len(), user_id);
        Ok(())
    }

    pub fn load_user_notes(
        &self,
        user_id: &str,
        crypto: &CryptoManager,
    ) -> Result<HashMap<String, Note>> {
        let notes_file = self.data_dir.join("users").join(user_id).join("notes.enc");

        if !notes_file.exists() {
            println!(
                "No notes file found for user {}, starting with empty notes",
                user_id
            );
            return Ok(HashMap::new());
        }

        let encrypted_data = fs::read(&notes_file)?;
        let decrypted_data = crypto.decrypt(&encrypted_data)?;
        let json_str = String::from_utf8(decrypted_data)?;
        let notes: HashMap<String, Note> = serde_json::from_str(&json_str)?;

        println!("Loaded {} notes for user {}", notes.len(), user_id);
        Ok(notes)
    }

    pub fn load_notes(&self, crypto: &CryptoManager) -> Result<HashMap<String, Note>> {
        let notes_file = self.data_dir.join("notes.enc");

        if !notes_file.exists() {
            return Ok(HashMap::new());
        }

        let encrypted_data = fs::read(&notes_file)?;
        let decrypted_data = crypto.decrypt(&encrypted_data)?;
        let json_str = String::from_utf8(decrypted_data)?;
        let notes: HashMap<String, Note> = serde_json::from_str(&json_str)?;

        Ok(notes)
    }

    pub fn migrate_legacy_notes(&self, user_id: &str, crypto: &CryptoManager) -> Result<()> {
        let legacy_file = self.data_dir.join("notes.enc");

        if legacy_file.exists() {
            println!("Found legacy notes file, migrating to user-specific storage...");

            // Load legacy notes
            let legacy_notes = self.load_notes(crypto)?;

            if !legacy_notes.is_empty() {
                // Save to user-specific location
                self.save_user_notes(user_id, &legacy_notes, crypto)?;

                // Backup the legacy file instead of deleting it
                let backup_file = self.data_dir.join("notes.enc.backup");
                fs::rename(&legacy_file, &backup_file)?;

                println!(
                    "Migrated {} notes to user-specific storage",
                    legacy_notes.len()
                );
                println!("Legacy file backed up as notes.enc.backup");
            }
        }

        Ok(())
    }

    pub fn delete_user_data(&self, user_id: &str) -> Result<()> {
        let user_dir = self.data_dir.join("users").join(user_id);

        if user_dir.exists() {
            fs::remove_dir_all(&user_dir)?;
            println!("Deleted all data for user {}", user_id);
        }

        Ok(())
    }

    pub fn get_user_data_size(&self, user_id: &str) -> Result<u64> {
        let user_dir = self.data_dir.join("users").join(user_id);

        if !user_dir.exists() {
            return Ok(0);
        }

        let mut total_size = 0u64;

        for entry in fs::read_dir(&user_dir)? {
            let entry = entry?;
            let metadata = entry.metadata()?;
            if metadata.is_file() {
                total_size += metadata.len();
            }
        }

        Ok(total_size)
    }
}
