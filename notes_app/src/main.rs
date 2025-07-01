// @Author: Matteo Cipriani
// @Date:   04-06-2025 10:24:58
// @Last Modified by:   Matteo Cipriani
// @Last Modified time: 01-07-2025 11:20:24
#![windows_subsystem = "windows"]

//! # Secure Notes Application
//!
//! A secure, encrypted note-taking application built with eframe/egui.
//!
//! ## Features
//!
//! - **Multi-user Support**: Individual user accounts with secure authentication
//! - **End-to-End Encryption**: All notes are encrypted using ChaCha20Poly1305
//! - **Hardware Binding**: Cryptographic keys are bound to hardware fingerprints
//! - **Secure Key Derivation**: Uses Argon2id for password-based key derivation
//! - **Auto-save**: Automatic saving with configurable intervals
//! - **Export Functionality**: Export notes to plain text files
//! - **Time Display Options**: Relative ("2 hours ago") or absolute timestamps
//! - **Security Auditing**: Built-in security monitoring and warnings
//!
//! ## Security Architecture
//!
//! The application implements multiple layers of security:
//!
//! 1. **User Authentication**: Argon2id password hashing with individual salts
//! 2. **Data Encryption**: ChaCha20Poly1305 authenticated encryption
//! 3. **Key Derivation**: Hardware-bound Argon2id key derivation (128MB memory, 3 iterations)
//! 4. **Hardware Fingerprinting**: Device binding to prevent unauthorized access
//! 5. **Secure Storage**: User-isolated encrypted storage with secure file permissions
//!
//! ## Usage
//!
//! Run the application with:
//! ```bash
//! cargo run
//! ```
//!
//! On first launch, create a user account. The application will:
//! - Generate a hardware fingerprint for your system
//! - Derive encryption keys from your password
//! - Create encrypted storage for your notes
//!
//! ## File Structure
//!
//! ```text
//! ~/.config/secure_notes/          (Linux/macOS) or %APPDATA%/secure_notes/ (Windows)
//! ├── users.json                   # User account database (encrypted hashes)
//! └── users/
//!     └── <user_id>/
//!         ├── auth.hash            # Password verification hash
//!         ├── security.meta        # Hardware fingerprint and security metadata
//!         └── notes.enc            # Encrypted notes data
//! ```
//!
//! ## Dependencies
//!
//! - `eframe/egui`: Cross-platform GUI framework
//! - `chacha20poly1305`: Authenticated encryption
//! - `argon2`: Password hashing and key derivation
//! - `uuid`: Unique identifier generation
//! - `chrono`: Date and time handling
//! - `serde`: Serialization framework

use eframe::egui;
use egui::IconData;

mod app;
mod auth;
mod crypto;
mod note;
mod notes_ui;
mod settings_ui;
mod storage;
mod user;

use app::NotesApp;

/// Loads the application icon from embedded PNG data.
///
/// This function loads the application icon from the assets directory at compile time,
/// converts it from PNG format to RGBA8, and returns it as IconData suitable for
/// use in the application window title bar and taskbar.
///
/// The icon is embedded in the binary using `include_bytes!` macro, which means
/// the application is self-contained and doesn't require external icon files.
///
/// # Returns
///
/// * `IconData` - The icon data structure containing:
///   - `rgba`: Raw RGBA pixel data as a `Vec<u8>`
///   - `width`: Icon width in pixels
///   - `height`: Icon height in pixels
///
/// # Panics
///
/// This function will panic if:
/// - The icon file `../assets/icons/icon.png` cannot be found at compile time
/// - The PNG data is corrupted or cannot be decoded
/// - The image cannot be converted to RGBA8 format
///
/// # Examples
///
/// ```rust
/// let icon = load_icon();
/// println!("Icon size: {}x{}", icon.width, icon.height);
/// println!("Icon data length: {}", icon.rgba.len());
/// ```
///
/// # Icon Requirements
///
/// The icon file should be:
/// - PNG format for best compatibility
/// - Square aspect ratio (e.g., 32x32, 64x64, 128x128)
/// - Reasonable size (under 1MB) to keep binary size manageable
/// - High contrast for visibility in different themes
fn load_icon() -> IconData {
    let (icon_rgba, icon_width, icon_height) = {
        // Load the icon from embedded bytes at compile time
        let icon = include_bytes!("../assets/icons/icon.png");

        // Decode PNG and convert to RGBA8 format
        let image = image::load_from_memory(icon)
            .expect("Failed to open icon path")
            .into_rgba8();

        // Extract dimensions and raw pixel data
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}

/// Main entry point for the Secure Notes application.
///
/// Initializes and runs the eframe/egui application with appropriate window settings
/// and configuration. This function sets up the native window with security-focused
/// defaults and launches the main application loop.
///
/// # Window Configuration
///
/// The application window is configured with:
/// - **Minimum Size**: 650x465 pixels (ensures UI elements are properly visible)
/// - **Title**: "Secure Notes" (displayed in title bar and taskbar)
/// - **Maximized**: Starts maximized for better user experience
/// - **Decorations**: Standard window decorations (title bar, borders, controls)
/// - **Custom Icon**: Application-specific icon loaded from assets
///
/// # Application Lifecycle
///
/// 1. **Initialization**: Creates NativeOptions with window configuration
/// 2. **Icon Loading**: Loads and sets the application icon
/// 3. **App Creation**: Instantiates NotesApp with default state
/// 4. **Event Loop**: Starts the main GUI event loop
/// 5. **Cleanup**: Automatic cleanup when application exits
///
/// # Returns
///
/// * `Result<(), eframe::Error>` - Returns:
///   - `Ok(())` if the application runs and exits successfully
///   - `Err(eframe::Error)` if there's an error during initialization or runtime
///
/// # Errors
///
/// This function can fail due to:
/// - **Graphics Initialization**: GPU/graphics driver issues
/// - **Window Creation**: OS-level window system problems
/// - **Icon Loading**: Icon file corruption or format issues
/// - **Memory Allocation**: Insufficient system resources
/// - **Platform Compatibility**: Unsupported OS or hardware
///
/// # Platform Support
///
/// The application supports:
/// - **Windows**: Windows 10/11 with DirectX or OpenGL
/// - **macOS**: macOS 10.15+ with Metal or OpenGL
/// - **Linux**: X11 or Wayland with OpenGL or Vulkan
///
/// # Security Considerations
///
/// The main function doesn't handle sensitive data directly, but it:
/// - Sets up the secure application environment
/// - Ensures proper window isolation
/// - Initializes the GUI framework for secure user interaction
///
/// # Examples
///
/// Run the application:
/// ```bash
/// cargo run
/// ```
///
/// Run with debug logging:
/// ```bash
/// RUST_LOG=debug cargo run
/// ```
///
/// # Performance Notes
///
/// - The application uses hardware acceleration when available
/// - GUI rendering is optimized for 60 FPS
/// - Memory usage scales with the number of notes
/// - Startup time includes key derivation (5-10 seconds for security)

fn main() -> Result<(), eframe::Error> {
    // Configure the native window options
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            // Set minimum window size to ensure UI is usable
            .with_min_inner_size([650.0, 465.0])
            // Set window title
            .with_title("Secure Notes")
            // Start maximized for better user experience
            .with_maximized(true)
            // Enable standard window decorations
            .with_decorations(true)
            // Set custom application icon
            .with_icon(load_icon()),

        // Use default values for other options
        ..Default::default()
    };

    // Run the native application
    eframe::run_native(
        "Secure Notes", // Application name for the window manager
        options,        // Window configuration
        Box::new(|_cc| {
            // App creation closure
            // Create and return the main application instance
            // The _cc parameter contains creation context (currently unused)
            Ok(Box::new(NotesApp::new()))
        }),
    )
}
