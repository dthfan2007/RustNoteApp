# Rust Notes App

A secure, encrypted notes application built entirely in Rust with a focus on data safety and user privacy.

## 📋 Project Overview

This project is an individual final project for the ZLI BLJ, demonstrating advanced Rust programming concepts including GUI development, cryptography, and secure data storage. The application provides a simple yet secure way to create, store, and manage encrypted notes.

- [Rust Notes App](#rust-notes-app)
  - [📋 Project Overview](#-project-overview)
  - [Folder Structure](#folder-structure)
  - [✨ Features](#-features)
    - [Core Functionality](#core-functionality)
    - [Advanced Features](#advanced-features)
  - [🛡️ Security Features](#️-security-features)
  - [🏗️ Architecture](#️-architecture)
    - [Encryption Process](#encryption-process)
    - [Tech Stack](#tech-stack)
  - [🚀 Getting Started](#-getting-started)
    - [Prerequisites](#prerequisites)
    - [Installation](#installation)
    - [Building for Distribution](#building-for-distribution)

## Folder Structure

``` plaintext
RustNoteApp/
├── docs/                    # Documentation, including assets
│   ├── assets/              # Assets for documenting
│   │   ├── images/
│   │   └── mmd/             # Mermaid diagrams
│   ├ documentation.pdf      # Finished Documentation
│   └ documentation.md       # Raw Documentation, rendered using LaTeX
├ egui_test/                 # First Tests with egui & eframe
│   ├ assets/                # Assets used for this playground
│   │   └ images/
│   └ src/                   # Sourcecode
├ notes_app/                 # The actual project
│   ├ src/                   # Sourcecode
│   │   ├── crypto.rs        # Encryption
│   │   ├── main.rs          # Application entry point
│   │   ├── note.rs          # Note Struct & Timestamp
│   │   └── storage.rs       # Storage manager
│   └ Cargo.toml             # Cargo dependencies
└ README.md                  # README
```

## ✨ Features

### Core Functionality

- 🔐 **Password Protection** - Secure authentication using Argon2id hashing
- 🖥️ **Native GUI** - Clean, responsive interface built with egui
- 📄 **Note Management** - Create, edit, view, and delete notes
- 🔒 **End-to-End Encryption** - Notes encrypted with ChaCha20-Poly1305
- 💾 **Secure Storage** - Encrypted file storage with integrity verification

### Advanced Features

- 🎨 **Modern UI** - Clean minimalistic interface
- ⏰ **Timestamps** - Track creation and modification times
- 📤 **Export Functionality** - Export notes to plain text
- ⌨️ **Keyboard Shortcuts** - Efficient navigation and actions
- 📦 **Windows Packaging** - Standalone executable with installer

## 🛡️ Security Features

- **Password Hashing**: Argon2id for secure password storage
- **Note Encryption**: ChaCha20-Poly1305 with random nonces
- **Key Derivation**: PBKDF2/Argon2 for deriving encryption keys from passwords
- **Data Integrity**: SHA-256 checksums for tamper detection
- **Obfuscation**: Fake SQLite headers to disguise encrypted files

## 🏗️ Architecture

### Encryption Process

1. **Key Derivation**: Random 16-byte salt + Argon2id → 32-byte key
2. **Encryption**: AES-256-GCM with random 12-byte nonce
3. **Metadata**: Bundle salt, hash, nonce, and ciphertext
4. **Integrity**: SHA-256 checksum for verification

### Tech Stack

- **GUI Framework**: `egui` + `eframe`
- **Cryptography**: `argon2`, `chacha20poly1305`, `aes-gcm`
- **Serialization**: `serde`, `serde_json`, `bincode`
- **Utilities**: `chrono`, `uuid`, `directories-next`
- **Error Handling**: `anyhow`, `thiserror`

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+ (latest stable recommended)

### Installation

```bash
# Clone the repository
git clone https://github.com/dthfan2007/RustNoteApp
cd RustNoteApp/notes_app

# Build and run
cargo run
```

### Building for Distribution

```bash
# Build optimized release
cargo build --release

# Create Windows installer (requires cargo-wix)
cargo install cargo-wix
cargo wix
```

---

_Built with ❤️ and 🦀_
