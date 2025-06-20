// @Author: Matteo Cipriani
// @Date:   18-06-2025 08:30:00
// @Last Modified by:   Matteo Cipriani
// @Last Modified time: 18-06-2025 09:27:21

use anyhow::{anyhow, Result};
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chacha20poly1305::aead::OsRng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub salt: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl User {
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

    pub fn verify_password(&self, password: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(&self.password_hash)
            .map_err(|e| anyhow!("Failed to parse password hash: {}", e))?;

        match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

#[derive(Clone)]
pub struct UserManager {
    users_file: std::path::PathBuf,
    users: HashMap<String, User>,
}

impl UserManager {
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

    pub fn get_user_count(&self) -> usize {
        self.users.len()
    }

    pub fn delete_user(&mut self, username: &str) -> Result<()> {
        if !self.users.contains_key(username) {
            return Err(anyhow!("User not found"));
        }

        self.users.remove(username);
        self.save_users()?;
        println!("User {} deleted successfully", username);
        Ok(())
    }

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
