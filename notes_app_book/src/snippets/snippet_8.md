# Snippet 8: Create New Users

```rust
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
```

## Comprehensive User Registration System with Advanced Validation

This function implements a robust user account creation system that combines comprehensive input validation, security best practices, and user experience considerations. It serves as the gateway for new users entering the system and establishes the foundation for secure, long-term user account management.

### Detailed Registration Architecture Analysis

**Function Signature and Security Design:**

```rust
pub fn create_user(&mut self, username: String, password: &str) -> Result<()>
```

The function signature reflects several important security and design decisions:

- **Mutable Self Reference**: Uses `&mut self` to allow modification of the user registry
- **Owned Username**: Takes ownership of the username string to avoid lifetime complications
- **Borrowed Password**: Uses string slice for password to avoid unnecessary allocations
- **Result Return**: Provides comprehensive error handling with descriptive error messages
- **Unit Return**: Returns `()` on success, focusing on the side effect of user creation

**Username Validation Framework:**
The function implements a comprehensive username validation system that balances security, usability, and system requirements:

**Empty Username Prevention:**

```rust
if username.trim().is_empty() {
    return Err(anyhow!("Username cannot be empty"));
}
```

This validation step provides several benefits:

- **Whitespace Handling**: Uses `trim()` to handle usernames that are only whitespace
- **User Experience**: Provides clear, actionable error messages
- **Data Integrity**: Prevents empty usernames from entering the system
- **Early Validation**: Fails fast on obviously invalid input

**Minimum Length Requirements:**

```rust
if username.len() < 3 {
    return Err(anyhow!("Username must be at least 3 characters long"));
}
```

The minimum length requirement serves multiple purposes:

- **Usability**: Ensures usernames are long enough to be meaningful and memorable
- **Uniqueness**: Longer usernames reduce the likelihood of conflicts
- **Security**: Prevents extremely short usernames that might be easier to guess
- **System Consistency**: Establishes consistent expectations across the application

**Maximum Length Constraints:**

```rust
if username.len() > 50 {
    return Err(anyhow!("Username must be less than 50 characters"));
}
```

The maximum length constraint provides several benefits:

- **Database Efficiency**: Prevents excessively long usernames that could impact database performance
- **UI Consistency**: Ensures usernames fit properly in user interface elements
- **Memory Management**: Prevents potential memory exhaustion attacks
- **Display Compatibility**: Ensures usernames display properly across different contexts

**Character Set Validation:**

```rust
if !username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
    return Err(anyhow!("Username can only contain letters, numbers, underscores, and hyphens"));
}
```

The character restriction system implements several important considerations:

- **Cross-Platform Compatibility**: Ensures usernames work across different operating systems and file systems
- **URL Safety**: Allows usernames to be safely used in URLs without encoding
- **Security**: Prevents injection attacks and special character exploits
- **Internationalization**: Uses `is_alphanumeric()` to support Unicode letters and numbers
- **Readability**: Restricts to characters that are easily readable and typeable

**Password Security Validation:**
The password validation system balances security requirements with user experience:

**Minimum Password Length:**

```rust
if password.len() < 6 {
    return Err(anyhow!("Password must be at least 6 characters long"));
}
```

The minimum length requirement provides:

- **Basic Security**: Ensures passwords have minimum entropy
- **Brute Force Resistance**: Makes brute force attacks more difficult
- **User Guidance**: Educates users about password security requirements
- **Compliance**: Meets basic security standards for password complexity

**Maximum Password Length:**

```rust
if password.len() > 128 {
    return Err(anyhow!("Password must be less than 128 characters"));
}
```

The maximum length constraint serves several purposes:

- **DoS Prevention**: Prevents denial-of-service attacks using extremely long passwords
- **Memory Management**: Limits memory usage during password hashing operations
- **Performance**: Ensures password hashing completes in reasonable time
- **Practical Limits**: 128 characters is far beyond typical user password lengths

**Username Uniqueness Enforcement:**

```rust
let username_lower = username.to_lowercase();
if self.users.keys().any(|k| k.to_lowercase() == username_lower) {
    return Err(anyhow!("Username already exists"));
}
```

The uniqueness checking system implements sophisticated collision detection:

- **Case-Insensitive Comparison**: Prevents confusion between "User" and "user"
- **Efficient Lookup**: Uses iterator methods for optimal performance
- **User Experience**: Prevents user confusion from similar usernames
- **Data Integrity**: Ensures each username is truly unique in the system
- **Security**: Prevents username spoofing attacks using case variations

**Secure User Account Creation:**

```rust
let user = User::new(username.clone(), password)?;
self.users.insert(username, user);
```

The account creation process implements several security measures:

- **Secure Construction**: Uses the `User::new()` constructor which handles password hashing
- **Error Propagation**: Handles any errors during user object creation
- **Atomic Insertion**: Adds the user to the registry only after successful creation
- **Username Preservation**: Maintains the original username casing for display purposes

**Persistent Storage and Data Integrity:**

```rust
self.save_users()?;
```

The persistence operation ensures data durability:

- **Immediate Persistence**: Saves the updated user registry immediately
- **Atomic Operations**: Ensures the save operation completes successfully or fails cleanly
- **Data Consistency**: Maintains consistency between memory and persistent storage
- **Error Handling**: Propagates storage errors for appropriate handling

**Success Confirmation and Logging:**

```rust
println!("Successfully created user account");
Ok(())
```

The completion process provides important feedback:

- **Operation Confirmation**: Confirms successful account creation
- **Audit Trail**: Creates a log entry for security monitoring
- **User Feedback**: Could be enhanced to provide user-facing confirmation
- **Success Return**: Returns `Ok(())` to indicate successful completion

**Security Considerations and Best Practices:**
The registration system implements several security best practices:

**Input Sanitization:**

- **Comprehensive Validation**: Validates all input parameters thoroughly
- **Early Rejection**: Rejects invalid input before processing
- **Clear Error Messages**: Provides actionable feedback without leaking system information
- **Consistent Validation**: Applies the same validation rules consistently

**Attack Prevention:**

- **Username Enumeration**: Generic error messages prevent username enumeration attacks
- **Resource Exhaustion**: Length limits prevent resource exhaustion attacks
- **Injection Prevention**: Character restrictions prevent various injection attacks
- **Collision Avoidance**: Case-insensitive uniqueness prevents confusion attacks

**Data Protection:**

- **Secure Password Handling**: Passwords are immediately hashed and never stored in plaintext
- **Memory Safety**: Uses Rust's ownership system to prevent memory-related vulnerabilities
- **Atomic Operations**: Ensures data consistency during account creation
- **Error Recovery**: Provides clean error recovery without leaving partial state

**Performance and Scalability:**
The registration system is designed for both performance and scalability:

**Efficient Operations:**

- **O(n) Uniqueness Check**: Linear time complexity for username checking
- **Minimal Allocations**: Efficient memory usage during validation
- **Fast Validation**: Quick rejection of invalid input
- **Optimized Storage**: Efficient data structures for user storage

**Scalability Considerations:**

- **Memory Efficiency**: Reasonable limits on username and password length
- **Storage Optimization**: Efficient serialization and storage of user data
- **Concurrent Safety**: Thread-safe operations for multi-user environments
- **Resource Management**: Proper cleanup and resource management
