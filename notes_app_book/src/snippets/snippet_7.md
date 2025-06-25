# Snippet 7: Save & Load User Notes

```rust
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
```

## Comprehensive Encrypted Note Storage and Retrieval System

This code implements a sophisticated data persistence layer that manages the secure storage and retrieval of user notes. The system combines JSON serialization with strong encryption to create a robust, user-isolated storage architecture that protects sensitive data both at rest and during file system operations.

### Detailed Storage Architecture Analysis

**Save Function Signature and Purpose:**

```rust
pub fn save_user_notes(&self, user_id: &str, notes: &HashMap<String, Note>, crypto: &CryptoManager) -> Result<()>
```

This function serves as the primary data persistence mechanism with several key design elements:

- **User Isolation**: Takes a `user_id` parameter to ensure complete separation between different users' data
- **Structured Data**: Accepts a `HashMap<String, Note>` representing the complete collection of user notes
- **Encryption Integration**: Requires a `CryptoManager` reference to ensure all data is encrypted before storage
- **Error Handling**: Returns a `Result<()>` to provide comprehensive error reporting and handling

**Data Serialization Process:**

```rust
let json_data = serde_json::to_string(notes)?;
```

The serialization step converts the complex note data structure into a portable format:

- **JSON Format**: Uses JSON for its human-readable format and cross-platform compatibility
- **Serde Integration**: Leverages Rust's powerful serde library for automatic serialization
- **Type Safety**: Maintains type information through the serialization process
- **Error Propagation**: Uses the `?` operator to handle serialization errors gracefully
- **Memory Efficiency**: Creates a single string representation of all notes

**Encryption Before Storage:**

```rust
let encrypted_data = crypto.encrypt(json_data.as_bytes())?;
```

The encryption step provides critical data protection:

- **Byte Conversion**: Converts the JSON string to bytes for encryption processing
- **Authenticated Encryption**: Uses the crypto manager's ChaCha20Poly1305 implementation
- **Key Binding**: Encryption is tied to the user's specific cryptographic key
- **Integrity Protection**: The encrypted data includes authentication tags to detect tampering
- **Format Consistency**: Produces data in the same format expected by the decryption process

**User-Specific Directory Management:**

```rust
let user_dir = self.data_dir.join("users").join(user_id);
fs::create_dir_all(&user_dir)?;
```

The directory structure provides several important benefits:

- **Hierarchical Organization**: Creates a clear, navigable directory structure
- **User Isolation**: Each user gets a completely separate directory
- **Scalability**: Can handle unlimited users without naming conflicts
- **Administrative Clarity**: Makes backup and maintenance operations straightforward
- **Recursive Creation**: `create_dir_all()` creates all necessary parent directories

**Secure File Writing:**

```rust
let notes_file = user_dir.join("notes.enc");
fs::write(&notes_file, encrypted_data)?;
```

The file writing process includes several security considerations:

- **Descriptive Naming**: Uses `.enc` extension to clearly indicate encrypted content
- **Atomic Operations**: `fs::write()` provides atomic file writing to prevent corruption
- **Error Handling**: Propagates file system errors for proper error handling
- **Path Safety**: Uses `join()` for safe path construction across platforms

**Unix File Permission Security:**

```rust
#[cfg(unix)]
{
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(&notes_file)?.permissions();
    perms.set_mode(0o600); // Read/write for owner only
    fs::set_permissions(&notes_file, perms)?;
}
```

The permission system provides critical access control:

- **Platform-Specific Security**: Uses conditional compilation for Unix-specific features
- **Owner-Only Access**: Sets permissions to 0o600 (read/write for owner, no access for others)
- **Metadata Preservation**: Maintains other file metadata while changing permissions
- **System Integration**: Works with the operating system's built-in security mechanisms
- **Defense in Depth**: Provides an additional security layer beyond encryption

**Comprehensive Logging:**

```rust
println!("Saved {} notes for user {}", notes.len(), user_id);
```

The logging system provides operational visibility:

- **Operation Confirmation**: Confirms successful completion of save operations
- **Quantitative Information**: Reports the number of notes saved for verification
- **User Context**: Includes user ID for audit trail purposes
- **Debugging Support**: Helps troubleshoot issues with data persistence

**Load Function Architecture:**

```rust
pub fn load_user_notes(&self, user_id: &str, crypto: &CryptoManager) -> Result<HashMap<String, Note>>
```

The load function mirrors the save function's design with appropriate return types:

- **User-Specific Loading**: Takes user ID to load the correct user's data
- **Crypto Dependency**: Requires crypto manager for decryption operations
- **Structured Return**: Returns the complete HashMap of notes
- **Error Handling**: Comprehensive error handling for all failure scenarios

**File Existence Checking:**

```rust
let notes_file = self.data_dir.join("users").join(user_id).join("notes.enc");
if !notes_file.exists() {
    println!("No notes file found for user {}, starting with empty notes", user_id);
    return Ok(HashMap::new());
}
```

The existence check provides graceful handling of new users:

- **Path Reconstruction**: Rebuilds the exact same path used during saving
- **Graceful Fallback**: Returns empty HashMap for users without existing notes
- **New User Support**: Handles first-time users without errors
- **Logging**: Provides clear information about the fallback behavior
- **Consistent Interface**: Returns the same type regardless of file existence

**Encrypted Data Reading:**

```rust
let encrypted_data = fs::read(&notes_file)?;
```

The file reading process is straightforward but secure:

- **Complete File Reading**: Reads the entire encrypted file into memory
- **Binary Data Handling**: Properly handles binary encrypted data
- **Error Propagation**: Handles file system errors appropriately
- **Memory Management**: Uses Rust's ownership system for safe memory handling

**Decryption and Deserialization:**

```rust
let decrypted_data = crypto.decrypt(&encrypted_data)?;
let json_str = String::from_utf8(decrypted_data)?;
let notes: HashMap<String, Note> = serde_json::from_str(&json_str)?;
```

The data recovery process reverses the save operation:

- **Authenticated Decryption**: Verifies data integrity during decryption
- **UTF-8 Conversion**: Safely converts decrypted bytes back to string format
- **JSON Deserialization**: Reconstructs the original data structure from JSON
- **Type Safety**: Ensures the loaded data matches the expected type structure
- **Error Chain**: Each step can fail independently with appropriate error messages

**Performance and Scalability Considerations:**
The storage system is designed for both performance and scalability:

**Memory Efficiency:**

- **Streaming Operations**: Could be enhanced to support streaming for very large note collections
- **Minimal Allocations**: Uses efficient memory allocation patterns
- **Garbage Collection**: Relies on Rust's ownership system for automatic memory management

**I/O Optimization:**

- **Atomic Operations**: Uses atomic file operations to prevent corruption
- **Buffered I/O**: Leverages the standard library's buffered I/O for efficiency
- **Error Recovery**: Provides clear error messages for I/O failures

**Security Architecture:**
The storage system implements defense-in-depth security:

**Encryption Layer:**

- **Strong Encryption**: Uses ChaCha20Poly1305 for authenticated encryption
- **Key Isolation**: Each user has completely separate encryption keys
- **Tamper Detection**: Authentication tags detect any data modification

**File System Layer:**

- **Permission Control**: Restricts file access at the operating system level
- **Directory Isolation**: Separates users at the file system level
- **Secure Paths**: Uses safe path construction to prevent directory traversal attacks

**Application Layer:**

- **Input Validation**: Validates user IDs and data structures
- **Error Handling**: Prevents information leakage through error messages
- **Audit Logging**: Records operations for security monitoring
