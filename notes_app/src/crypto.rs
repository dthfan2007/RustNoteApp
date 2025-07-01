// @Author: Matteo Cipriani
// @Date:   04-06-2025 10:29:20
// @Last Modified by:   Matteo Cipriani
// @Last Modified time: 01-07-2025 09:06:33
//! # Cryptographic Module
//!
//! Provides secure encryption, decryption, and key management functionality.
//! Uses ChaCha20Poly1305 for encryption and Argon2 for key derivation.
//! Implements hardware fingerprinting for additional security.

use anyhow::{anyhow, Result};
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fs;
use std::hash::{Hash, Hasher};

/// Security metadata stored alongside encrypted data.
///
/// Contains version information, creation timestamp, and hardware fingerprint
/// data used to verify that the encrypted data is being accessed from the
/// same system where it was created.
#[derive(Serialize, Deserialize)]
struct SecurityMetadata {
    /// Version of the security metadata format
    version: u32,
    /// Unix timestamp when the metadata was created
    created_timestamp: u64,
    /// Hash of the hardware fingerprint components
    hardware_fingerprint_hash: u64,
    /// List of hardware components used for fingerprinting
    #[serde(default)] // This makes the field optional for backward compatibility
    hardware_components: Vec<String>,
}

/// Main cryptographic manager for the application.
///
/// Handles all cryptographic operations including:
/// - Key derivation from passwords using Argon2
/// - Encryption/decryption using ChaCha20Poly1305
/// - Hardware fingerprinting for device binding
/// - Security metadata management
/// - Password verification and changes
pub struct CryptoManager {
    /// The encryption cipher instance
    cipher: Option<ChaCha20Poly1305>,
    /// Path to the configuration directory
    config_path: std::path::PathBuf,
    /// Security metadata for the current session
    security_metadata: Option<SecurityMetadata>,
}

impl CryptoManager {
    /// Creates a new CryptoManager instance.
    ///
    /// Initializes the configuration directory path and creates it if it doesn't exist.
    /// The cipher is not initialized until `initialize_for_user` is called.
    ///
    /// # Returns
    ///
    /// * `Self` - A new CryptoManager instance
    pub fn new() -> Self {
        let mut config_path = config_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        config_path.push("secure_notes");

        if !config_path.exists() {
            fs::create_dir_all(&config_path).expect("Failed to create config directory");
        }

        Self {
            cipher: None,
            config_path,
            security_metadata: None,
        }
    }

    /// Initializes the crypto manager for a specific user.
    ///
    /// This method performs several critical operations:
    /// 1. Creates user-specific configuration directory
    /// 2. Loads or creates security metadata with hardware fingerprinting
    /// 3. Verifies password against stored hash (for existing users)
    /// 4. Derives encryption key using Argon2
    /// 5. Initializes the ChaCha20Poly1305 cipher
    ///
    /// The process is designed to be secure but may take several seconds due to
    /// the intentionally expensive key derivation process.
    ///
    /// # Arguments
    ///
    /// * `user_id` - Unique identifier for the user
    /// * `password` - User's password for key derivation
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Ok if initialization succeeds, Err with details if it fails
    ///
    /// # Errors
    ///
    /// * Password verification fails for existing users
    /// * Hardware fingerprint doesn't match (potential security breach)
    /// * File system operations fail
    /// * Key derivation fails
    pub fn initialize_for_user(&mut self, user_id: &str, password: &str) -> Result<()> {
        println!("Starting crypto initialization for user: {}", user_id);
        let start_time = std::time::Instant::now();

        // Create user-specific config directory
        let mut user_config_path = self.config_path.clone();
        user_config_path.push("users");
        user_config_path.push(user_id);

        if !user_config_path.exists() {
            fs::create_dir_all(&user_config_path)?;
        }

        let key_file = user_config_path.join("auth.hash");
        let metadata_file = user_config_path.join("security.meta");

        let key = if key_file.exists() && metadata_file.exists() {
            println!("Loading existing user configuration...");

            // Load existing setup
            let stored_hash = fs::read_to_string(&key_file)?;
            let parsed_hash = PasswordHash::new(&stored_hash)
                .map_err(|e| anyhow!("Failed to parse password hash: {}", e))?;

            println!("Verifying password...");
            // Verify password (this should be fast with default Argon2)
            Argon2::default()
                .verify_password(password.as_bytes(), &parsed_hash)
                .map_err(|e| anyhow!("Password verification failed: {}", e))?;

            println!("Loading metadata...");
            // Load metadata
            let metadata_content = fs::read_to_string(&metadata_file)?;
            let mut metadata: SecurityMetadata = serde_json::from_str(&metadata_content)
                .map_err(|e| anyhow!("Failed to parse security metadata: {}", e))?;

            // Handle backward compatibility - if hardware_components is empty, regenerate it
            if metadata.hardware_components.is_empty() {
                println!("Upgrading old metadata format...");
                let (current_hash, current_components) =
                    self.generate_stable_hardware_fingerprint()?;

                // Update the metadata with current components
                metadata.hardware_components = current_components;
                metadata.hardware_fingerprint_hash = current_hash;

                // Save updated metadata
                fs::write(&metadata_file, serde_json::to_string_pretty(&metadata)?)?;
                println!("Metadata upgraded successfully");
            } else {
                println!("Checking hardware fingerprint...");
                // Get current hardware components
                let (current_hash, current_components) =
                    self.generate_stable_hardware_fingerprint()?;

                // Debug output
                println!(
                    "Stored hardware components: {:?}",
                    metadata.hardware_components
                );
                println!("Current hardware components: {:?}", current_components);
                println!("Stored hash: {}", metadata.hardware_fingerprint_hash);
                println!("Current hash: {}", current_hash);

                // Check if hardware fingerprint matches
                if metadata.hardware_fingerprint_hash != current_hash {
                    // Try to identify what changed
                    let mut changed_components = Vec::new();
                    for (i, (stored, current)) in metadata
                        .hardware_components
                        .iter()
                        .zip(current_components.iter())
                        .enumerate()
                    {
                        if stored != current {
                            changed_components
                                .push(format!("Component {}: '{}' -> '{}'", i, stored, current));
                        }
                    }

                    if !changed_components.is_empty() {
                        println!("Hardware changes detected:");
                        for change in &changed_components {
                            println!("  {}", change);
                        }

                        // For now, let's be more lenient and only fail if critical components changed
                        if self.is_critical_hardware_change(
                            &metadata.hardware_components,
                            &current_components,
                        ) {
                            return Err(anyhow!(
                                "Critical hardware components changed: {}",
                                changed_components.join(", ")
                            ));
                        } else {
                            println!("Non-critical hardware changes detected, allowing access...");
                            // Update the stored fingerprint
                            metadata.hardware_fingerprint_hash = current_hash;
                            metadata.hardware_components = current_components;

                            // Save updated metadata
                            fs::write(&metadata_file, serde_json::to_string_pretty(&metadata)?)?;
                        }
                    }
                } else {
                    println!("Hardware fingerprint matches!");
                }
            }

            self.security_metadata = Some(metadata);

            println!("Deriving encryption key...");
            // Use standard security key derivation
            self.derive_secure_key(password)
        } else {
            println!("First time setup for user...");

            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            let (hardware_hash, hardware_components) =
                self.generate_stable_hardware_fingerprint()?;

            println!("Initial hardware components: {:?}", hardware_components);
            println!("Initial hardware hash: {}", hardware_hash);

            let metadata = SecurityMetadata {
                version: 1,
                created_timestamp: current_time,
                hardware_fingerprint_hash: hardware_hash,
                hardware_components,
            };

            let key = self.derive_secure_key(password);

            println!("Storing password hash...");
            // Store password hash
            let verification_salt = SaltString::generate(&mut OsRng);
            let argon2 = Argon2::default();
            let password_hash = argon2
                .hash_password(password.as_bytes(), &verification_salt)
                .map_err(|e| anyhow!("Failed to hash password: {}", e))?;

            fs::write(&key_file, password_hash.to_string())?;
            fs::write(&metadata_file, serde_json::to_string_pretty(&metadata)?)?;

            self.secure_file_permissions(&key_file)?;
            self.secure_file_permissions(&metadata_file)?;

            self.security_metadata = Some(metadata);
            key
        };

        self.cipher = Some(ChaCha20Poly1305::new(&key));

        let elapsed = start_time.elapsed();
        println!(
            "Crypto initialization completed in {:.2}s",
            elapsed.as_secs_f64()
        );

        Ok(())
    }

    /// Generates a stable hardware fingerprint for device binding.
    ///
    /// Creates a fingerprint based on stable system characteristics that
    /// should remain consistent across reboots but change if the software
    /// is moved to a different system. Uses only relatively stable components
    /// to avoid false positives from minor system changes.
    ///
    /// # Returns
    ///
    /// * `Result<(u64, Vec<String>)>` - Tuple of (hash, components) if successful
    ///
    /// # Components Used
    ///
    /// * Username - Very stable
    /// * Home directory - Usually stable
    /// * OS and architecture - Very stable
    /// * Computer name - Usually stable but can change
    fn generate_stable_hardware_fingerprint(&self) -> Result<(u64, Vec<String>)> {
        println!("Generating stable hardware fingerprint...");

        // Use only the most stable components
        let mut components = Vec::new();

        // 1. Username (very stable)
        let username = env::var("USER")
            .or_else(|_| env::var("USERNAME"))
            .unwrap_or_else(|_| "unknown_user".to_string());
        components.push(format!("user:{}", username));

        // 2. Home directory (stable, but can change)
        if let Ok(home) = env::var("HOME").or_else(|_| env::var("USERPROFILE")) {
            components.push(format!("home:{}", home));
        } else {
            components.push("home:unknown".to_string());
        }

        // 3. OS and Architecture (very stable)
        components.push(format!("os:{}", env::consts::OS));
        components.push(format!("arch:{}", env::consts::ARCH));

        // 4. Computer name (can change but usually stable)
        let computer_name = env::var("COMPUTERNAME")
            .or_else(|_| env::var("HOSTNAME"))
            .or_else(|_| env::var("NAME"))
            .unwrap_or_else(|_| "unknown_computer".to_string());
        components.push(format!("computer:{}", computer_name));

        // Sort components for consistency
        components.sort();

        // Generate hash
        let combined = components.join("||");
        let mut hasher = DefaultHasher::new();
        combined.hash(&mut hasher);
        let hash = hasher.finish();

        println!("Hardware fingerprint components: {:?}", components);
        println!("Generated hash: {}", hash);

        Ok((hash, components))
    }

    /// Determines if hardware changes are critical enough to deny access.
    ///
    /// Only considers changes to username, OS, or architecture as critical,
    /// as these should never change on the same physical machine.
    ///
    /// # Arguments
    ///
    /// * `stored` - Previously stored hardware components
    /// * `current` - Current hardware components
    ///
    /// # Returns
    ///
    /// * `bool` - true if critical components changed, false otherwise
    fn is_critical_hardware_change(&self, stored: &[String], current: &[String]) -> bool {
        // Only consider it critical if the username or OS changed
        // These should never change on the same machine

        let stored_critical: Vec<_> = stored
            .iter()
            .filter(|c| c.starts_with("user:") || c.starts_with("os:") || c.starts_with("arch:"))
            .collect();

        let current_critical: Vec<_> = current
            .iter()
            .filter(|c| c.starts_with("user:") || c.starts_with("os:") || c.starts_with("arch:"))
            .collect();

        stored_critical != current_critical
    }

    /// Derives a secure encryption key from a password using Argon2.
    ///
    /// Uses production-grade parameters that balance security and performance:
    /// - Memory cost: 128 MB
    /// - Iterations: 3
    /// - Parallelism: 4 threads
    ///
    /// The process is intentionally expensive (5-10 seconds) to make
    /// brute force attacks impractical.
    ///
    /// # Arguments
    ///
    /// * `password` - The user's password
    ///
    /// # Returns
    ///
    /// * `chacha20poly1305::Key` - 32-byte encryption key
    fn derive_secure_key(&self, password: &str) -> chacha20poly1305::Key {
        println!("Using standard security key derivation...");

        // Standard security parameters - should take ~5-10 seconds on most hardware
        let memory_cost = 131072; // 128 MB
        let iterations = 3; // 3 iterations
        let parallelism = 4; // 4 parallel threads

        let params = argon2::Params::new(memory_cost, iterations, parallelism, Some(32))
            .expect("Invalid Argon2 parameters");

        let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);

        // Generate a hardware-bound salt
        let hardware_salt = self.generate_hardware_salt();

        let mut key = [0u8; 32];
        argon2
            .hash_password_into(password.as_bytes(), &hardware_salt, &mut key)
            .expect("Failed to derive key");

        key.into()
    }

    /// Generates a deterministic salt based on hardware fingerprint.
    ///
    /// Creates a 32-byte salt that is consistent for the same hardware
    /// but different across different systems. This adds an additional
    /// layer of hardware binding to the encryption.
    ///
    /// # Returns
    ///
    /// * `[u8; 32]` - 32-byte salt array
    fn generate_hardware_salt(&self) -> [u8; 32] {
        // Create a deterministic salt based on hardware fingerprint
        if let Ok((hardware_hash, _)) = self.generate_stable_hardware_fingerprint() {
            let mut salt = [0u8; 32];

            // Use the hardware hash to seed the salt
            let hash_bytes = hardware_hash.to_le_bytes();

            // Fill the salt array with a pattern based on the hardware hash
            // Use wrapping_mul to avoid overflow
            for i in 0..32 {
                // Use wrapping_mul to avoid overflow
                let factor = (i as u8).wrapping_mul(17);
                salt[i] = hash_bytes[i % 8] ^ factor ^ 0xAA;
            }

            salt
        } else {
            // Fallback salt if hardware fingerprinting fails
            *b"fallback_salt_for_key_derivation"
        }
    }

    /// Sets secure file permissions on Unix systems.
    ///
    /// Sets file permissions to 0o600 (read/write for owner only) on Unix systems.
    /// On other systems, this is a no-op.
    ///
    /// # Arguments
    ///
    /// * `_file_path` - Path to the file to secure
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Ok if successful, Err if permission setting failed
    fn secure_file_permissions(&self, _file_path: &std::path::Path) -> Result<()> {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(_file_path)?.permissions();
            perms.set_mode(0o600);
            fs::set_permissions(_file_path, perms)?;
        }
        Ok(())
    }

    /// Encrypts data using ChaCha20Poly1305.
    ///
    /// Generates a random nonce and encrypts the data, prepending the nonce
    /// to the ciphertext for later decryption.
    ///
    /// # Arguments
    ///
    /// * `data` - The plaintext data to encrypt
    ///
    /// # Returns
    ///
    /// * `Result<Vec<u8>>` - Encrypted data with nonce prepended, or error
    ///
    /// # Errors
    ///
    /// * Cipher not initialized (call `initialize_for_user` first)
    /// * Encryption operation fails
    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let cipher = self
            .cipher
            .as_ref()
            .ok_or_else(|| anyhow!("Cipher not initialized"))?;
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let ciphertext = cipher
            .encrypt(&nonce, data)
            .map_err(|e| anyhow!("Encryption failed: {}", e))?;

        let mut result = Vec::new();
        result.extend_from_slice(&nonce);
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }

    /// Decrypts data using ChaCha20Poly1305.
    ///
    /// Extracts the nonce from the beginning of the data and decrypts
    /// the remaining ciphertext.
    ///
    /// # Arguments
    ///
    /// * `data` - The encrypted data with nonce prepended
    ///
    /// # Returns
    ///
    /// * `Result<Vec<u8>>` - Decrypted plaintext data, or error
    ///
    /// # Errors
    ///
    /// * Cipher not initialized
    /// * Invalid data format (too short or corrupted)
    /// * Decryption operation fails (wrong key, tampered data, etc.)
    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let cipher = self
            .cipher
            .as_ref()
            .ok_or_else(|| anyhow!("Cipher not initialized"))?;

        if data.len() < 12 {
            return Err(anyhow!("Invalid encrypted data"));
        }

        let (nonce_bytes, ciphertext) = data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| anyhow!("Decryption failed: {}", e))?;
        Ok(plaintext)
    }

    /// Performs a security audit of the current session.
    ///
    /// Checks for potential security issues such as hardware fingerprint
    /// changes that might indicate the data is being accessed from a
    /// different system.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<String>>` - List of security warnings, or error
    pub fn security_audit(&self) -> Result<Vec<String>> {
        let mut warnings = Vec::new();

        if let Some(metadata) = &self.security_metadata {
            let (current_hash, current_components) = self.generate_stable_hardware_fingerprint()?;
            if metadata.hardware_fingerprint_hash != current_hash {
                warnings.push("Hardware fingerprint has changed since last login".to_string());

                // Show what changed
                for (stored, current) in metadata
                    .hardware_components
                    .iter()
                    .zip(current_components.iter())
                {
                    if stored != current {
                        warnings.push(format!("Changed: {} -> {}", stored, current));
                    }
                }
            }
        }

        Ok(warnings)
    }

    /// Gets detailed security information for display.
    ///
    /// Returns a formatted string containing security configuration details
    /// including encryption parameters, hardware binding status, and metadata.
    ///
    /// # Returns
    ///
    /// * `Option<String>` - Formatted security information, or None if not available
    pub fn get_security_info(&self) -> Option<String> {
        self.security_metadata.as_ref().map(|metadata| {
            let components_str = if metadata.hardware_components.is_empty() {
                "Legacy format (upgraded)".to_string()
            } else {
                metadata.hardware_components.join(", ")
            };

            format!(
                "Security Level: Standard (Production)\nVersion: {}\nCreated: {}\nHardware Bound: Yes\nMemory Cost: 128 MB\nIterations: 3\nParallelism: 4\nHardware Components: {}",
                metadata.version,
                chrono::DateTime::from_timestamp(metadata.created_timestamp as i64, 0)
                    .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
                    .unwrap_or_else(|| "Unknown".to_string()),
                components_str
            )
        })
    }

    /// Changes the user's password and re-initializes encryption.
    ///
    /// Verifies the old password, generates a new password hash, saves it,
    /// and re-initializes the crypto manager with the new password.
    ///
    /// # Arguments
    ///
    /// * `old_password` - Current password for verification
    /// * `new_password` - New password to set
    /// * `user_id` - User ID for file operations
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Ok if successful, Err with details if failed
    ///
    /// # Errors
    ///
    /// * Old password verification fails
    /// * File operations fail
    /// * Re-initialization with new password fails
    pub fn change_password(
        &mut self,
        old_password: &str,
        new_password: &str,
        user_id: &str,
    ) -> Result<()> {
        // Verify old password first
        let user_config_path = self.config_path.join("users").join(user_id);
        let key_file = user_config_path.join("auth.hash");

        if !key_file.exists() {
            return Err(anyhow!("User configuration not found"));
        }

        let stored_hash = fs::read_to_string(&key_file)?;
        let parsed_hash = PasswordHash::new(&stored_hash)
            .map_err(|e| anyhow!("Failed to parse password hash: {}", e))?;

        // Verify old password
        Argon2::default()
            .verify_password(old_password.as_bytes(), &parsed_hash)
            .map_err(|_| anyhow!("Current password is incorrect"))?;

        // Generate new password hash
        let verification_salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let new_password_hash = argon2
            .hash_password(new_password.as_bytes(), &verification_salt)
            .map_err(|e| anyhow!("Failed to hash new password: {}", e))?;

        // Save new password hash
        fs::write(&key_file, new_password_hash.to_string())?;
        self.secure_file_permissions(&key_file)?;

        // Re-initialize with new password
        self.initialize_for_user(user_id, new_password)?;

        println!("Password changed successfully for user {}", user_id);
        Ok(())
    }

    /// Deletes all cryptographic data for a user.
    ///
    /// Removes the user's entire cryptographic configuration directory,
    /// including password hashes, security metadata, and any other
    /// crypto-related files.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User ID whose crypto data should be deleted
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Ok if successful, Err if deletion failed
    pub fn delete_user_crypto_data(&self, user_id: &str) -> Result<()> {
        let user_config_path = self.config_path.join("users").join(user_id);

        if user_config_path.exists() {
            fs::remove_dir_all(&user_config_path)?;
            println!("Deleted crypto data for user {}", user_id);
        }

        Ok(())
    }
}
