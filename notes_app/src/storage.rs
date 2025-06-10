// @Author: Matteo Cipriani
// @Date:   04-06-2025 10:29:39
// @Last Modified by:   Matteo Cipriani
// @Last Modified time: 04-06-2025 10:37:06
use crate::{CryptoManager, Note};
use anyhow::Result;
use serde_json;
use std::collections::HashMap;
use std::fs;

pub struct StorageManager {
    data_path: std::path::PathBuf,
}

impl StorageManager {
    pub fn new() -> Self {
        let mut data_path = dirs::config_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        data_path.push("secure_notes");
        data_path.push("notes.enc");

        Self { data_path }
    }

    pub fn save_notes(&self, notes: &HashMap<String, Note>, crypto: &CryptoManager) -> Result<()> {
        let json_data = serde_json::to_string(notes)?;
        let encrypted_data = crypto.encrypt(json_data.as_bytes())?;

        if let Some(parent) = self.data_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&self.data_path, encrypted_data)?;
        Ok(())
    }

    pub fn load_notes(&self, crypto: &CryptoManager) -> Result<HashMap<String, Note>> {
        if !self.data_path.exists() {
            return Ok(HashMap::new());
        }

        let encrypted_data = fs::read(&self.data_path)?;
        let decrypted_data = crypto.decrypt(&encrypted_data)?;
        let json_str = String::from_utf8(decrypted_data)?;
        let notes: HashMap<String, Note> = serde_json::from_str(&json_str)?;

        Ok(notes)
    }
}
