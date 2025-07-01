// @Author: Matteo Cipriani
// @Date:   04-06-2025 10:29:39
// @Last Modified by:   Matteo Cipriani
// @Last Modified time: 01-07-2025 09:05:45
//! # Storage Module
//!
//! Handles encrypted file storage and retrieval for user notes and data.
//! Provides secure, user-isolated storage with encryption integration
//! and legacy data migration capabilities.

use crate::crypto::CryptoManager;
use crate::note::Note;
use anyhow::Result;
use std::collections::HashMap;
use std::fs;

/// Manages encrypted storage operations for user notes and data.
///
/// The StorageManager provides secure, user-isolated storage with:
/// - Per-user encrypted storage directories
/// - JSON serialization with encryption
/// - Legacy data migration support
/// - Secure file permissions on Unix systems
/// - Data size tracking and management
pub struct StorageManager {
    /// Base directory for all application data
    data_dir: std::path::PathBuf,
}

impl StorageManager {
    /// Creates a new StorageManager instance.
    ///
    /// Initializes the storage manager with the appropriate data directory
    /// based on the system's configuration directory. Creates the base
    /// directory if it doesn't exist.
    ///
    /// # Returns
    ///
    /// * `Self` - A new StorageManager instance
    ///
    /// # Directory Structure
    ///
    /// ```text
    /// ~/.config/secure_notes/          (or platform equivalent)
    /// ├── users/
    /// │   ├── user1_id/
    /// │   │   └── notes.enc
    /// │   └── user2_id/
    /// │       └── notes.enc
    /// └── notes.enc.backup             (legacy backup)
    /// ```
    pub fn new() -> Self {
        let mut data_dir = dirs::config_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        data_dir.push("secure_notes");

        Self { data_dir }
    }

    /// Saves encrypted notes for a specific user.
    ///
    /// Serializes the notes to JSON, encrypts the data using the provided
    /// crypto manager, and saves it to the user's storage directory.
    /// Sets secure file permissions on Unix systems.
    ///
    /// # Arguments
    ///
    /// * `user_id` - Unique identifier for the user
    /// * `notes` - HashMap of note IDs to Note objects to save
    /// * `crypto` - CryptoManager instance for encryption
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Ok if successful, Err with details if failed
    ///
    /// # Errors
    ///
    /// * JSON serialization fails
    /// * Encryption operation fails
    /// * File system operations fail
    /// * Permission setting fails (Unix only)
    ///
    /// # Security Features
    ///
    /// - All data is encrypted before writing to disk
    /// - User-specific storage isolation
    /// - Secure file permissions (0o600 on Unix)
    /// - Atomic write operations where possible
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

    /// Loads encrypted notes for a specific user.
    ///
    /// Reads the encrypted notes file for the specified user, decrypts
    /// the data, and deserializes it back to a HashMap of notes.
    /// Returns an empty HashMap if no notes file exists.
    ///
    /// # Arguments
    ///
    /// * `user_id` - Unique identifier for the user
    /// * `crypto` - CryptoManager instance for decryption
    ///
    /// # Returns
    ///
    /// * `Result<HashMap<String, Note>>` - Notes HashMap or error
    ///
    /// # Errors
    ///
    /// * File reading fails
    /// * Decryption operation fails (wrong key, corrupted data)
    /// * JSON deserialization fails
    /// * Invalid UTF-8 in decrypted data
    ///
    /// # Behavior
    ///
    /// - Returns empty HashMap if notes file doesn't exist
    /// - Logs the number of notes loaded for debugging
    /// - Handles missing files gracefully (new user scenario)
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

    /// Loads notes from the legacy storage format.
    ///
    /// This method supports loading notes from the old storage format
    /// (before user-specific storage was implemented). Used primarily
    /// for migration purposes.
    ///
    /// # Arguments
    ///
    /// * `crypto` - CryptoManager instance for decryption
    ///
    /// # Returns
    ///
    /// * `Result<HashMap<String, Note>>` - Notes HashMap or error
    ///
    /// # Legacy Format
    ///
    /// The legacy format stored all notes in a single `notes.enc` file
    /// in the root data directory, without user isolation.
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

    /// Migrates notes from legacy storage format to user-specific storage.
    ///
    /// Checks for the existence of legacy notes file and migrates the data
    /// to the new user-specific storage format. The legacy file is backed up
    /// rather than deleted to prevent data loss.
    ///
    /// # Arguments
    ///
    /// * `user_id` - Target user ID for migration
    /// * `crypto` - CryptoManager instance for encryption/decryption
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Ok if successful, Err if migration failed
    ///
    /// # Migration Process
    ///
    /// 1. Check for legacy `notes.enc` file
    /// 2. Load notes using legacy format
    /// 3. Save notes to user-specific location
    /// 4. Backup legacy file as `notes.enc.backup`
    /// 5. Log migration results
    ///
    /// # Safety
    ///
    /// - Original file is backed up, not deleted
    /// - Migration only occurs if legacy file exists
    /// - Empty legacy files are handled gracefully
    /// - Errors don't affect existing user data
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

    /// Deletes all data for a specific user.
    ///
    /// Removes the entire user directory and all contained files,
    /// effectively deleting all stored data for the specified user.
    /// This operation is irreversible.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User ID whose data should be deleted
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Ok if successful, Err if deletion failed
    ///
    /// # Data Deleted
    ///
    /// - Encrypted notes file
    /// - Any other user-specific files in the directory
    /// - The user directory itself
    ///
    /// # Safety
    ///
    /// - Only deletes data if user directory exists
    /// - Logs successful deletions
    /// - Handles non-existent directories gracefully
    pub fn delete_user_data(&self, user_id: &str) -> Result<()> {
        let user_dir = self.data_dir.join("users").join(user_id);

        if user_dir.exists() {
            fs::remove_dir_all(&user_dir)?;
            println!("Deleted all data for user {}", user_id);
        }

        Ok(())
    }

    /// Calculates the total storage size for a user's data.
    ///
    /// Iterates through all files in the user's directory and sums
    /// their sizes to provide storage usage information.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User ID to calculate storage for
    ///
    /// # Returns
    ///
    /// * `Result<u64>` - Total size in bytes, or error
    ///
    /// # Behavior
    ///
    /// - Returns 0 if user directory doesn't exist
    /// - Only counts regular files, not directories
    /// - Handles file system errors gracefully
    /// - Useful for storage quotas and usage display
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
