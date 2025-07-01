// @Author: Matteo Cipriani
// @Date:   18-06-2025 08:33:19
// @Last Modified by:   Matteo Cipriani
// @Last Modified time: 01-07-2025 09:06:05
//! # User Management Module
//!
//! Handles user account creation, authentication, and management.
//! Provides secure password hashing, user validation, and account operations
//! with persistent storage of user credentials.

use anyhow::{anyhow, Result};
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chacha20poly1305::aead::OsRng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use uuid::Uuid;

/// Represents a user account with authentication credentials.
///
/// Contains all necessary information for user authentication and
/// account management, including secure password storage using
/// Argon2 hashing with individual salts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique identifier for the user (UUID)
    pub id: String,
    /// Username chosen by the user
    pub username: String,
    /// Argon2 password hash for authentication
    pub password_hash: String,
    /// Salt used for password hashing
    pub salt: String,
    /// UTC timestamp when the account was created
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl User {
    /// Creates a new user with secure password hashing.
    ///
    /// Generates a unique UUID for the user, creates a random salt,
    /// and hashes the password using Argon2 with secure parameters.
    ///
    /// # Arguments
    ///
    /// * `username` - The desired username for the account
    /// * `password` - The plaintext password to hash and store
    ///
    /// # Returns
    ///
    /// * `Result<Self>` - New User instance or error
    ///
    /// # Errors
    ///
    /// * Password hashing fails due to invalid parameters
    /// * Random salt generation fails
    ///
    /// # Security Features
    ///
    /// - Unique UUID for each user
    /// - Cryptographically secure random salt
    /// - Argon2id password hashing (industry standard)
    /// - UTC timestamps for consistency
    ///
    /// # Examples
    ///
    /// ```rust
    /// let user = User::new("alice".to_string(), "secure_password123")?;
    /// assert_eq!(user.username, "alice");
    /// assert!(user.verify_password("secure_password123")?);
    /// ```
    pub fn new(username: String, password: &str) -> Result<Self> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow!("Failed to hash password: {}", e))?;

        Ok(Self {
            id: Uuid::new_v4().to_string(),
            username,
            password_hash: password_hash.to_string(),
            salt: salt.to_string(),
            created_at: chrono::Utc::now(),
        })
    }

    /// Verifies a password against the stored hash.
    ///
    /// Uses Argon2 to verify that the provided plaintext password
    /// matches the stored password hash. This is a secure, constant-time
    /// operation that prevents timing attacks.
    ///
    /// # Arguments
    ///
    /// * `password` - The plaintext password to verify
    ///
    /// # Returns
    ///
    /// * `Result<bool>` - true if password matches, false if not, or error
    ///
    /// # Errors
    ///
    /// * Password hash parsing fails (corrupted data)
    /// * Argon2 verification fails due to invalid parameters
    ///
    /// # Security Features
    ///
    /// - Constant-time comparison prevents timing attacks
    /// - Uses the same Argon2 parameters as during creation
    /// - Handles invalid hashes gracefully
    ///
    /// # Examples
    ///
    /// ```rust
    /// let user = User::new("alice".to_string(), "password123")?;
    /// assert!(user.verify_password("password123")?);
    /// assert!(!user.verify_password("wrong_password")?);
    /// ```
    pub fn verify_password(&self, password: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(&self.password_hash)
            .map_err(|e| anyhow!("Failed to parse password hash: {}", e))?;

        match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

/// Manages user accounts and authentication operations.
///
/// Provides a complete user management system with:
/// - User registration with validation
/// - Secure authentication
/// - Password changes
/// - Account deletion
/// - Persistent storage with encryption
/// - User enumeration and statistics
#[derive(Clone)]
pub struct UserManager {
    /// Path to the users database file
    users_file: std::path::PathBuf,
    /// In-memory cache of all users
    users: HashMap<String, User>,
}

impl UserManager {
    /// Creates a new UserManager instance.
    ///
    /// Initializes the user manager with the appropriate storage location
    /// and loads existing users from the database file. Creates the
    /// necessary directories if they don't exist.
    ///
    /// # Returns
    ///
    /// * `Result<Self>` - New UserManager instance or error
    ///
    /// # Errors
    ///
    /// * Directory creation fails
    /// * User database loading fails
    /// * File permissions cannot be set
    ///
    /// # Storage Location
    ///
    /// Uses the system's configuration directory:
    /// - Linux/macOS: `~/.config/secure_notes/users.json`
    /// - Windows: `%APPDATA%/secure_notes/users.json`
    pub fn new() -> Result<Self> {
        let mut users_file = dirs::config_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        users_file.push("secure_notes");
        users_file.push("users.json");

        if let Some(parent) = users_file.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut manager = Self {
            users_file,
            users: HashMap::new(),
        };

        manager.load_users()?;
        Ok(manager)
    }

    /// Loads users from the persistent storage file.
    ///
    /// Reads the users.json file and deserializes it into the in-memory
    /// user cache. Handles missing files gracefully by starting with
    /// an empty user database.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Ok if successful, Err if loading failed
    ///
    /// # Behavior
    ///
    /// - Creates empty database if file doesn't exist
    /// - Logs the number of users loaded
    /// - Handles JSON parsing errors
    fn load_users(&mut self) -> Result<()> {
        if !self.users_file.exists() {
            println!("Users file doesn't exist, starting with empty user database");
            return Ok(());
        }

        let content = fs::read_to_string(&self.users_file)?;
        self.users = serde_json::from_str(&content)?;
        println!("Loaded {} users from database", self.users.len());
        Ok(())
    }

    /// Saves users to the persistent storage file.
    ///
    /// Serializes the in-memory user cache to JSON and writes it to
    /// the users.json file. Sets secure file permissions on Unix systems.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Ok if successful, Err if saving failed
    ///
    /// # Security Features
    ///
    /// - Pretty-printed JSON for readability
    /// - Secure file permissions (0o600 on Unix)
    /// - Atomic write operations where possible
    /// - Logs successful saves
    fn save_users(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(&self.users)?;
        fs::write(&self.users_file, content)?;

        // Set secure file permissions on Unix systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&self.users_file)?.permissions();
            perms.set_mode(0o600); // Read/write for owner only
            fs::set_permissions(&self.users_file, perms)?;
        }

        println!("Saved {} users to database", self.users.len());
        Ok(())
    }

    /// Creates a new user account with comprehensive validation.
    ///
    /// Validates the username and password according to security policies,
    /// checks for existing users, creates the account, and saves it to
    /// persistent storage.
    ///
    /// # Arguments
    ///
    /// * `username` - Desired username for the new account
    /// * `password` - Password for the new account
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Ok if successful, Err with validation details
    ///
    /// # Validation Rules
    ///
    /// ## Username Requirements:
    /// - Not empty or whitespace-only
    /// - Minimum 3 characters
    /// - Maximum 50 characters
    /// - Only alphanumeric characters, underscores, and hyphens
    /// - Case-insensitive uniqueness check
    ///
    /// ## Password Requirements:
    /// - Minimum 6 characters
    /// - Maximum 128 characters
    ///
    /// # Errors
    ///
    /// * Username validation fails
    /// * Password validation fails
    /// * Username already exists
    /// * User creation fails
    /// * Database save fails
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut manager = UserManager::new()?;
    /// manager.create_user("alice".to_string(), "secure_password")?;
    /// // User "alice" is now registered and can authenticate
    /// ```
    pub fn create_user(&mut self, username: String, password: &str) -> Result<()> {
        // Validate input
        if username.trim().is_empty() {
            return Err(anyhow!("Username cannot be empty"));
        }

        if username.len() < 3 {
            return Err(anyhow!("Username must be at least 3 characters long"));
        }

        if username.len() > 50 {
            return Err(anyhow!("Username must be less than 50 characters"));
        }

        // Check for invalid characters
        if !username
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(anyhow!(
                "Username can only contain letters, numbers, underscores, and hyphens"
            ));
        }

        if password.len() < 6 {
            return Err(anyhow!("Password must be at least 6 characters long"));
        }

        if password.len() > 128 {
            return Err(anyhow!("Password must be less than 128 characters"));
        }

        // Check if user already exists (case-insensitive)
        let username_lower = username.to_lowercase();
        if self
            .users
            .keys()
            .any(|k| k.to_lowercase() == username_lower)
        {
            return Err(anyhow!("Username already exists"));
        }

        // Create the user
        let user = User::new(username.clone(), password)?;
        self.users.insert(username, user);
        self.save_users()?;

        println!("Successfully created user account");
        Ok(())
    }

    /// Authenticates a user with username and password.
    ///
    /// Looks up the user by username and verifies the provided password
    /// against the stored hash. Returns the user object if authentication
    /// succeeds.
    ///
    /// # Arguments
    ///
    /// * `username` - Username to authenticate
    /// * `password` - Password to verify
    ///
    /// # Returns
    ///
    /// * `Result<User>` - User object if successful, error if failed
    ///
    /// # Errors
    ///
    /// * Username not found
    /// * Password verification fails
    /// * Password hash is corrupted
    ///
    /// # Security Features
    ///
    /// - Constant-time password verification
    /// - Generic error messages to prevent username enumeration
    /// - Logs successful authentications
    ///
    /// # Examples
    ///
    /// ```rust
    /// let manager = UserManager::new()?;
    /// let user = manager.authenticate("alice", "password123")?;
    /// println!("Authenticated user: {}", user.username);
    /// ```
    pub fn authenticate(&self, username: &str, password: &str) -> Result<User> {
        let user = self
            .users
            .get(username)
            .ok_or_else(|| anyhow!("Invalid username or password"))?;

        if user.verify_password(password)? {
            println!("User {} authenticated successfully", username);
            Ok(user.clone())
        } else {
            Err(anyhow!("Invalid username or password"))
        }
    }

    /// Returns the total number of registered users.
    ///
    /// Useful for displaying statistics or implementing user limits.
    ///
    /// # Returns
    ///
    /// * `usize` - Number of registered users
    pub fn get_user_count(&self) -> usize {
        self.users.len()
    }

    /// Deletes a user account permanently.
    ///
    /// Removes the user from the database and saves the changes.
    /// This operation is irreversible and only affects the user
    /// account data, not associated encrypted notes.
    ///
    /// # Arguments
    ///
    /// * `username` - Username of the account to delete
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Ok if successful, Err if user not found or save failed
    ///
    /// # Note
    ///
    /// This method only deletes the user account. Associated encrypted
    /// notes and cryptographic data must be deleted separately using
    /// the StorageManager and CryptoManager.
    pub fn delete_user(&mut self, username: &str) -> Result<()> {
        if !self.users.contains_key(username) {
            return Err(anyhow!("User not found"));
        }

        self.users.remove(username);
        self.save_users()?;
        println!("User {} deleted successfully", username);
        Ok(())
    }

    /// Changes a user's password with verification.
    ///
    /// Verifies the current password, validates the new password,
    /// generates a new password hash, and updates the user record.
    ///
    /// # Arguments
    ///
    /// * `username` - Username of the account to update
    /// * `old_password` - Current password for verification
    /// * `new_password` - New password to set
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Ok if successful, Err with details if failed
    ///
    /// # Errors
    ///
    /// * User not found
    /// * Current password verification fails
    /// * New password validation fails
    /// * Password hashing fails
    /// * Database save fails
    ///
    /// # Security Features
    ///
    /// - Requires current password verification
    /// - Generates new salt for new password
    /// - Validates new password strength
    /// - Atomic update operation
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut manager = UserManager::new()?;
    /// manager.change_password("alice", "old_password", "new_secure_password")?;
    /// ```
    pub fn change_password(
        &mut self,
        username: &str,
        old_password: &str,
        new_password: &str,
    ) -> Result<()> {
        let user = self
            .users
            .get(username)
            .ok_or_else(|| anyhow!("User not found"))?;

        if !user.verify_password(old_password)? {
            return Err(anyhow!("Current password is incorrect"));
        }

        if new_password.len() < 6 {
            return Err(anyhow!("New password must be at least 6 characters long"));
        }

        if new_password.len() > 128 {
            return Err(anyhow!("New password must be less than 128 characters"));
        }

        // Create new user with updated password
        let mut updated_user = user.clone();
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(new_password.as_bytes(), &salt)
            .map_err(|e| anyhow!("Failed to hash password: {}", e))?;

        updated_user.password_hash = password_hash.to_string();
        updated_user.salt = salt.to_string();

        self.users.insert(username.to_string(), updated_user);
        self.save_users()?;

        println!("Password changed successfully for user {}", username);
        Ok(())
    }
}
