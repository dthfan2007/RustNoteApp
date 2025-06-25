# Function Descriptions

## `CryptoManager::initialize_for_user()`

- **Reference:** Code Snippet No. 4
- **Location:** `crypto.rs:44-154`
- **Description:**
    This is the **security cornerstone** of the entire application. It implements a sophisticated multi-layered security system:

  - **Hardware Binding:** Creates a unique fingerprint based on username, home directory, OS, architecture, and computer name. This prevents easy data portability between machines.
  - **Dual Authentication:** Stores both a password hash for verification AND derives an encryption key from the password.
  - **Backward Compatibility:** Handles metadata format upgrades gracefully.
  - **Performance Monitoring:** Times the entire process and provides detailed logging.
  - **Critical vs Non-Critical Changes:** Distinguishes between hardware changes that should block access (username/OS changes) vs those that shouldn't (computer name changes).

The function essentially creates a "vault" that's locked with both the user's password AND the specific hardware it was created on.

## `NotesApp::start_authentication()`

- **Reference:** Code Snippet No. 5
- **Location:** `app.rs:108-174`
- **Description:**
    This function implements **asynchronous authentication** to prevent UI freezing during CPU-intensive operations. Key features:

  - **Thread Safety:** Uses message passing instead of shared memory to communicate between threads.
  - **Dual Flow Handling:** Manages both registration (create user → authenticate → initialize crypto) and login (authenticate → initialize crypto) in a single function.
  - **Error Propagation:** Comprehensive error handling with detailed error messages for debugging.
  - **Performance Tracking:** Records start time for performance monitoring.
  - **UI Responsiveness:** Keeps the UI thread free while expensive crypto operations run in background.

The registration flow is particularly complex as it requires three sequential operations, each of which can fail independently.

## `CryptoManager::encrypt()` & `decrypt()`

- **Reference:** Code Snippet No. 6
- **Location:** `crypto.rs:280-306`
- **Description:**
    These functions provide **authenticated encryption** using ChaCha20Poly1305, which is considered state-of-the-art:

  - **ChaCha20Poly1305:** Combines ChaCha20 stream cipher with Poly1305 MAC for both confidentiality and authenticity.
  - **Nonce Management:** Each encryption uses a fresh random nonce, preventing replay attacks and ensuring semantic security.
  - **Data Format:** Encrypted data format is `[12-byte nonce][variable-length ciphertext+tag]`.
  - **Error Handling:** Validates input lengths and provides meaningful error messages.
  - **Performance:** ChaCha20 is faster than AES on systems without hardware AES acceleration.

The authenticated encryption prevents both eavesdropping AND tampering - if someone modifies the encrypted data, decryption will fail.

## `StorageManager::save_user_notes()` & `load_user_notes()`

- **Reference:** Code Snippet No. 7
- **Location:** `storage.rs:18-67`
- **Description:**
    These functions implement **encrypted data persistence** with user isolation:

  - **Data Flow:** Notes HashMap → JSON → UTF-8 bytes → Encrypted bytes → File (and reverse for loading).
  - **User Isolation:** Each user gets their own directory under `users/{user_id}/`.
  - **File Permissions:** On Unix systems, sets 0o600 (owner read/write only) for additional security.
  - **Graceful Handling:** Returns empty HashMap if no notes file exists (new user scenario).
  - **Error Propagation:** Each step can fail independently with specific error messages.

The serialization chain ensures that all note data (including metadata like timestamps) is preserved across application restarts.

## `UserManager::create_user()`

- **Reference:** Code Snippet No.8
- **Location:** `user.rs:65-107`
- **Description:**
    This function implements **comprehensive user registration** with extensive validation:

  - **Input Sanitization:** Checks for empty, too short, too long, and invalid characters in usernames.
  - **Security Constraints:** Enforces password length limits (6-128 characters) to prevent both weak passwords and potential DoS attacks.
  - **Uniqueness Enforcement:** Case-insensitive username checking prevents confusion.
  - **Character Restrictions:** Only allows alphanumeric characters, underscores, and hyphens in usernames.
  - **Atomic Operations:** Either the entire user creation succeeds, or it fails completely (no partial state).

The validation is particularly thorough - it prevents common security issues like SQL injection (though not applicable here), ensures usernames are filesystem-safe, and enforces reasonable security policies.

## `UserManager::authenticate()`

- **Reference:** Code Snippet No. 9
- **Location:** `user.rs:108-118`
- **Description:**
    This function implements **secure authentication** with several security best practices:

  - **Generic Error Messages:** Returns "Invalid username or password" for both non-existent users and wrong passwords, preventing username enumeration attacks.
  - **Secure Password Verification:** Delegates to `User::verify_password()` which uses Argon2 for secure hash comparison.
  - **Timing Attack Resistance:** Argon2 verification takes consistent time regardless of password correctness.
  - **User Object Return:** Returns a complete User object on success, providing all necessary user data for the session.

The function is deliberately simple but secure - it doesn't leak information about whether a username exists or not.

## `User::verify_password()`

- **Reference:** Code Snippet No. 10
- **Location:** `user.rs:35-43`
- **Description:**
    This function provides **cryptographically secure password verification**:

  - **Argon2 Verification:** Uses the Argon2 password hashing function, winner of the Password Hashing Competition.
  - **Salt Handling:** The stored hash includes the salt, so no separate salt management is needed.
  - **Timing Attack Resistance:** Argon2 takes consistent time regardless of password correctness.
  - **Error Handling:** Distinguishes between parsing errors (corrupted data) and verification failures (wrong password).

Argon2 is specifically designed to be memory-hard and resistant to both GPU and ASIC attacks, making it the gold standard for password hashing.

## `NotesApp::check_authentication_result()`

- **Reference:** Code Snippet No. 11
- **Location:** `app.rs:175-226`
- **Description:**
    This function **bridges asynchronous authentication with the UI thread**:

  - **Non-blocking Polling:** Uses `try_recv()` to check for results without blocking the UI.
  - **Complete State Transition:** On success, performs all necessary setup: loads notes, migrates legacy data, runs security audit, updates UI state.
  - **Performance Monitoring:** Tracks and reports authentication duration.
  - **Cleanup:** Properly cleans up authentication state and clears sensitive input fields.
  - **Error Handling:** Handles both explicit errors and channel disconnection scenarios.

This function is called every UI frame, making it the bridge between the background authentication thread and the main application state.

## `Note::relative_time()`

- **Reference:** Code Snippet No. 12
- **Location:** `note.rs:50-85`
- **Description:**
    This function provides **human-friendly time display** with intelligent granularity:

  - **Timezone Handling:** Converts UTC storage time to Swiss local time for accurate relative calculations.
  - **Granular Time Ranges:** Different descriptions for seconds, minutes, hours, days, and weeks.
  - **Singular/Plural Handling:** Proper grammar for "1 minute ago" vs "5 minutes ago".
  - **Fallback to Absolute:** For very old notes (>4 weeks), shows actual date instead of "X weeks ago".
  - **User Experience:** Makes it easy to quickly understand when notes were last modified.

This kind of relative time display is crucial for user experience - it's much easier to understand "2 hours ago" than "2025-06-23 14:30:15".

## `Note::new()`

- **Reference:** Code Snippet No. 13
- **Location:** `note.rs:17-26`
- **Description:**
    This function creates **new note instances** with proper initialization:

  - **Unique Identification:** Uses UUID v4 for globally unique note IDs, preventing conflicts even across different users.
  - **Timestamp Management:** Sets both created_at and modified_at to the same UTC timestamp initially.
  - **Empty Content:** Starts with empty content, allowing users to immediately begin typing.
  - **UTC Storage:** Stores timestamps in UTC to avoid timezone confusion, converting to local time only for display.

The UUID ensures that even if two users create notes simultaneously, there will be no ID conflicts.

## `NotesApp::create_new_note()`

- **Reference:** Code Snippet No. 14
- **Location:** `app.rs:295-306`
- **Description:**
    This function handles **note creation with user experience considerations**:

  - **Input Sanitization:** Handles empty titles gracefully by providing a default "Untitled Note".
  - **Immediate Selection:** Automatically selects the newly created note, allowing immediate editing.
  - **Persistent Storage:** Immediately saves the new note to prevent data loss.
  - **State Management:** Updates both the notes collection and the UI selection state.

The function ensures that users can never create a note without a title, preventing UI confusion.

## `NotesApp::auto_save_if_needed()`

- **Reference:** Code Snippet No. 15
- **Location:** `app.rs:318-323`
- **Description:**
    This function implements **intelligent auto-save functionality**:

  - **Debouncing:** Waits for a 2-second delay after the last change before saving, preventing excessive disk I/O.
  - **Performance Optimization:** Avoids saving on every keystroke, which would be inefficient with encryption.
  - **Data Safety:** Ensures that changes are persisted regularly without user intervention.
  - **Timer Reset:** Updates the last save time after each save to restart the delay period.

This is called every UI frame, but only actually saves when the delay threshold is met.

## `CryptoManager::generate_stable_hardware_fingerprint()`

- **Reference:** Code Snippet No. 16
- **Location:** `crypto.rs:156-195`
- **Description:**
    This function creates a **hardware-bound security fingerprint**:

  - **Cross-Platform Compatibility:** Uses different environment variables for Windows vs Unix systems.
  - **Stability Prioritization:** Chooses components that rarely change (username, OS) over volatile ones (IP address, running processes).
- **Deterministic Hashing:** Sorts components to ensure consistent hash generation across runs.
  - **Graceful Degradation:** Provides fallback values if environment variables aren't available.
  - **Security Through Binding:** Makes encrypted data difficult to transfer between machines.

This fingerprint prevents someone from copying encrypted files to another machine and accessing them, even with the correct password.

## `StorageManager::migrate_legacy_data_if_needed()`

- **Reference:** Code Snippet No. 17
- **Location:** `storage.rs:85-108`
- **Description:**
    This function handles **backward compatibility and data migration**:

  - **Legacy Detection:** Checks for old single-user data files from previous versions.
  - **Safe Migration:** Moves data to new user-specific structure without data loss.
  - **Backup Creation:** Renames old files instead of deleting them, providing a safety net.
  - **Conditional Operation:** Only performs migration if legacy data actually exists.
  - **User Feedback:** Provides clear logging about what migration actions were taken.

This ensures that users upgrading from single-user to multi-user versions don't lose their existing notes.

## `NotesApp::render_auth_dialog()`

- **Reference:** Code Snippet No. 18
- **Location:** `auth.rs:15-130`
- **Description:**
    This function renders the **complete authentication interface** with sophisticated UX features:

  - **Dual Mode Interface:** Seamlessly switches between login and registration modes with conditional UI elements.
  - **Real-time Validation:** Shows validation errors as users type, preventing submission of invalid data.
  - **Progress Feedback:** During authentication, shows spinner, elapsed time, and escalating warnings for long operations.
  - **Keyboard Navigation:** Supports Enter key submission and proper tab order.
  - **Responsive Layout:** Calculates text widths and centers elements properly across different screen sizes.
  - **User Feedback:** Shows current user count and time for context.
  - **Error Handling:** Displays authentication errors with appropriate color coding.

The function handles the complex state transitions between idle, validating, authenticating, and error states while maintaining a clean user experience.
