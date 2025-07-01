# Secure Notes App

A secure, encrypted notes application built entirely in Rust with a focus on data safety, user privacy, and multi-user support.

## 📋 Project Overview

This project is an individual final project for the ZLI BLJ, demonstrating advanced Rust programming concepts including GUI development, cryptography, secure data storage, and user authentication. The application provides a comprehensive solution for creating, storing, and managing encrypted notes with enterprise-grade security features.

- [Secure Notes App](#secure-notes-app)
  - [📋 Project Overview](#-project-overview)
  - [✨ Features](#-features)
    - [Core Functionality](#core-functionality)
    - [Security Features](#security-features)
    - [User Experience](#user-experience)
    - [Advanced Features](#advanced-features)
  - [🛡️ Security Architecture](#️-security-architecture)
    - [Encryption Process](#encryption-process)
    - [Hardware Binding](#hardware-binding)
    - [User Isolation](#user-isolation)
  - [🏗️ Technical Architecture](#️-technical-architecture)
    - [Application Modules](#application-modules)
    - [Tech Stack](#tech-stack)
    - [First Run](#first-run)
  - [📖 Documentation](#-documentation)
  - [🤝 Contributing](#-contributing)
    - [Development Guidelines](#development-guidelines)
  - [📄 License](#-license)
  - [🎯 Project Goals Achieved](#-project-goals-achieved)

## ✨ Features

### Core Functionality

- 👥 **Multi-User Support** - Multiple isolated user accounts on single device
- 🔐 **Secure Authentication** - Argon2id password hashing with hardware binding
- 📝 **Note Management** - Create, edit, delete, and organize notes
- 💾 **Auto-Save** - Automatic saving every 2 seconds with manual save option
- 🔍 **Note Search** - Quick navigation through note list
- 📤 **Export Notes** - Export individual notes to plain text files

### Security Features

- 🛡️ **Military-Grade Encryption** - ChaCha20Poly1305 encryption algorithm
- 🔑 **Hardware Binding** - Account tied to specific hardware fingerprint
- 🏠 **Local Storage Only** - No cloud sync, all data stays on your device
- 🔒 **User Data Isolation** - Complete separation between user accounts
- 🔐 **Secure Key Derivation** - Argon2id with configurable security levels
- 🚨 **Security Auditing** - Built-in security monitoring and warnings

### User Experience

- 🎨 **Modern Native GUI** - Clean, responsive interface built with egui
- ⌨️ **Keyboard Shortcuts** - Efficient navigation and quick actions
- ⏰ **Smart Timestamps** - Relative and absolute time display options
- 🌍 **Timezone Support** - Swiss timezone with proper formatting
- 📱 **Responsive Design** - Adapts to different window sizes
- 🎯 **Context Menus** - Right-click actions for enhanced productivity

### Advanced Features

- 🔄 **Legacy Migration** - Automatic migration from older data formats
- 🛠️ **Settings Management** - Comprehensive user settings and preferences
- 📊 **Security Information** - Detailed security status and audit reports
- 🔧 **Password Management** - Secure password change functionality
- 🗑️ **Account Deletion** - Complete data removal with confirmation
- 📈 **Performance Optimization** - Efficient handling of large notes

## 🛡️ Security Architecture

### Encryption Process

1. **User Authentication**
   - Password verification using stored Argon2id hash
   - Hardware fingerprint validation
   - Session establishment with crypto manager
2. **Key Derivation**

   - Argon2id with hardware-bound salt (128MB memory, 3 iterations, 4 threads)
   - 32-byte encryption key generation
   - Hardware fingerprint integration for device binding
3. **Data Encryption**
   - ChaCha20Poly1305 authenticated encryption
   - Random 12-byte nonce per encryption operation
   - Authenticated encryption with integrity verification
4. **Storage Security**
   - Encrypted file storage with secure permissions
   - User-specific data directories
   - Tamper detection and integrity verification

### Hardware Binding

The application implements hardware fingerprinting for enhanced security:

- **Components Tracked**: Username, home directory, OS, architecture, computer name
- **Stability**: Designed to handle minor system changes
- **Security**: Prevents unauthorized access from different devices
- **Flexibility**: Allows non-critical hardware modifications

### User Isolation

- **Separate Encryption Keys**: Each user has unique encryption keys
- **Isolated Storage**: User data stored in separate encrypted directories
- **Access Control**: No cross-user data access possible
- **Independent Sessions**: Complete session isolation between users

## 🏗️ Technical Architecture

### Application Modules

- **`app.rs`** - Main application state, UI coordination, and business logic
- **`auth.rs`** - Authentication UI components and user interaction
- **`crypto.rs`** - Cryptographic operations, key management, and security
- **`user.rs`** - User account management and authentication backend
- **`storage.rs`** - Encrypted file operations and data persistence
- **`note.rs`** - Note data structures and timestamp management
- **`notes_ui.rs`** - Note editing interface and sidebar components
- **`settings_ui.rs`** - User settings and account management interface

### Tech Stack

**Core Framework:**

- **GUI**: `egui` 0.24+ with `eframe` for native desktop applications
- **Runtime**: Native Rust with cross-platform support

**Cryptography:**

- **Password Hashing**: `argon2` for secure password storage
- **Encryption**: `chacha20poly1305` for authenticated encryption
- **Key Derivation**: Custom Argon2id implementation with hardware binding

**Data Management:**

- **Serialization**: `serde` with `serde_json` for data structures
- **Time Handling**: `chrono` with `chrono-tz` for timezone support
- **File System**: `dirs` for cross-platform directory management

**Development Tools:**

- **Error Handling**: `anyhow` for comprehensive error management
- **UUID Generation**: `uuid` for unique identifiers
- **Build Tools**: Custom build scripts for platform-specific features

### First Run

1. **Launch Application**: Double-click the executable or run from terminal
2. **Create Account**: Click "Register" and create your first user account
3. **Set Strong Password**: Use a memorable but secure password (minimum 6 characters)
4. **Start Writing**: Create your first note and start writing!

## 📖 Documentation

- **[Extended Documentation](https://dthfan2007.github.io/RustNoteApp/)** - `mdbook` documentation with extra snippet explanations

## 🤝 Contributing

This is an individual academic project, but feedback and suggestions are welcome:

1. **Issues**: Report bugs or request features via GitHub issues
2. **Documentation**: Improvements to documentation are appreciated
3. **Testing**: Additional test cases and scenarios
4. **Security**: Security reviews and vulnerability reports

### Development Guidelines

- Follow Rust best practices and idioms
- Maintain comprehensive test coverage
- Document all public APIs and complex logic
- Prioritize security in all implementations
- Ensure cross-platform compatibility

## 📄 License

This project is developed as an academic assignment for ZLI BLJ. Please respect intellectual property rights and academic integrity policies.

**Academic Use**: This code is provided for educational purposes and academic review.

---

## 🎯 Project Goals Achieved

- ✅ **Advanced Rust Programming** - Demonstrates complex Rust concepts and patterns
- ✅ **GUI Development** - Native desktop application with modern interface
- ✅ **Cryptography Implementation** - Military-grade encryption and security
- ✅ **Multi-User Architecture** - Complete user isolation and management
- ✅ **Cross-Platform Compatibility** - Works on Windows, macOS, and Linux
- ✅ **Professional Documentation** - Comprehensive user and technical documentation
- ✅ **Security Best Practices** - Hardware binding and advanced security features
- ✅ **Performance Optimization** - Efficient handling of large datasets

---

*Built with ❤️ and 🦀 Rust*

**Author**: Matteo Cipriani

**Company**: Soreco AG

**Year**: 2025
