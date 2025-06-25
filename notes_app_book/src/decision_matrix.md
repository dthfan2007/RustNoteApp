# Decision Matrix

## Basic Functionality Components (MS 2)

| **Feature**               | **Recommended Crates**                 | **Implementation Suggestions**                                                                                                                                                                                            |
| ------------------------- | -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Password Verification** | `argon2`, `bcrypt`, or `pbkdf2`        | Use Argon2id for password hashing - it's modern and secure. Store the hash in a config file using `confy` or `config`. For verification, use the crate's verify function to compare entered password against stored hash. |
| **GUI Framework**         | `egui`, `iced`, or `druid`             | `egui` is lightweight and easy to use for beginners. `iced` offers a more Elm-like architecture. Both have good documentation and active communities.                                                                     |
| **Note Struct**           | `serde`, `chrono`, `uuid`              | Use `chrono` for timestamps, `uuid` for unique IDs, and `serde` with the `derive` feature for serialization. Consider implementing `Display` and `Debug` traits.                                                          |
| **Note Encryption**       | `aes-gcm`, `chacha20poly1305`, `orion` | ChaCha20-Poly1305 is modern and fast on all platforms. Use `ring` or `orion` for key derivation from password (PBKDF2 or Argon2). Store a random salt with each note.                                                     |
| **File Storage**          | `serde_json`, `bincode`, `postcard`    | `bincode` for efficient binary serialization or `serde_json` for human-readable storage. Use `directories-next` to find appropriate app data directory.                                                                   |
| **Loading Notes**         | Same as above + `anyhow`               | Use `anyhow` or `thiserror` for error handling during file operations. Consider lazy loading for large note collections.                                                                                                  |
| **Note Viewing**          | GUI framework                          | Implement a split view with note list on left and content on right. Use virtual list if you expect many notes.                                                                                                            |
| **Deleting Notes**        | GUI framework + `confirm_dialog`       | Use context menus from your GUI framework. Consider soft deletion (marking as deleted) before permanent removal.                                                                                                          |
| **Session Persistence**   | `keyring`, `directories-next`          | Store an encrypted token in the system keyring or in a hidden file in the app directory. Use `directories-next` to find appropriate locations.                                                                            |

## Finalization & Polish Components (MS 3)

| **Feature**            | **Recommended Crates**       | **Implementation Suggestions**                                                                                                                          |
| ---------------------- | ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Error Handling**     | `thiserror`, `anyhow`, `log` | Define custom error types with `thiserror`. Use `log` with `env_logger` or `fern` for logging. Show user-friendly messages in UI while logging details. |
| **GUI Styling**        | GUI framework theming        | Most Rust GUI frameworks support theming. Use system colors or implement dark/light mode toggle. Consider accessibility (contrast, font sizes).         |
| **Timestamps**         | `chrono`, `time`             | Store UTC timestamps internally, convert to local time for display. Format with `chrono`'s formatting options.                                          |
| **Note Sorting**       | Standard library             | Use Rust's built-in sorting with custom comparators. Consider allowing multiple sort options (recent, alphabetical).                                    |
| **Plain Text Export**  | `std::fs`, GUI file dialog   | Use native file dialogs from your GUI framework. Consider supporting multiple formats (txt, md, html).                                                  |
| **Settings Page**      | GUI framework, `confy`       | Store settings with `confy` which handles serialization. Create a modal dialog or separate tab for settings.                                            |
| **Windows Executable** | `cargo-wix`, `cargo-bundle`  | Use GitHub Actions for CI/CD to automate builds. Test on Windows VM before release.                                                                     |
| **App Packaging**      | `cargo-wix`, `cargo-bundle`  | Create an installer with `cargo-wix` for Windows. Include a nice icon and proper metadata.                                                              |
| **Documentation**      | `mdbook`                     | Write user docs in Markdown, possibly generate with `mdbook`. Include screenshots and keyboard shortcuts.                                               |
| **Keyboard Shortcuts** | GUI framework                | Most GUI frameworks have built-in shortcut handling. Map to actions using a configuration struct.                                                       |
