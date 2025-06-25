# Testing

## Application Components

The application consists of:

- **Authentication System** - User login/registration
- **Encryption Engine** - Data protection
- **Storage Layer** - File management
- **User Interface** - Note editing and management
- **Note Management** - Create, edit, delete notes

## Testing Strategy

### Unit Testing

#### Authentication Testing (`auth.rs`, `user.rs`)

**Test Cases:**

```plaintext
- User registration with valid/invalid data
- Password verification (correct/incorrect)
- User login authentication
- Password change functionality
- Account deletion
```

#### Encryption Testing (`crypto.rs`)

**Test Cases:**

```plaintext
- Data encryption produces different outputs
- Decryption recovers original data exactly
- Hardware fingerprint generation
- Key derivation with different passwords
- Security audit detection
```

#### Storage Testing (`storage.rs`)

**Test Cases:**

```plaintext
- Save/load encrypted notes
- User data isolation
- File permission security
- Data deletion completeness
```

#### Note Management Testing (`note.rs`)

**Test Cases:**

```plaintext
- Note creation and modification
- Timestamp updates
- Time formatting (relative/absolute)
- Timezone conversion accuracy
```

### Integration Testing

#### User Registration Flow

**Test Steps:**

```plaintext
1. Launch application
2. Select "Register" mode
3. Enter username and password
4. Verify user creation
5. Confirm automatic login
6. Check empty notes state
```

#### Login Authentication Flow

**Test Steps:**

```plaintext
1. Launch with existing user
2. Enter credentials
3. Verify authentication success
4. Check notes loading
5. Validate hardware fingerprint
```

#### Note Lifecycle Testing

**Test Steps:**

```plaintext
1. Create new note
2. Add/edit content
3. Verify auto-save
4. Logout and login
5. Confirm note persistence
6. Export note to file
```

### Security Testing

#### Data Protection

**Test Cases:**

```plaintext
- Users cannot access other users' notes
- Encrypted files are unreadable without key
- Hardware changes trigger security warnings
- Password changes invalidate old passwords
```

#### Authentication Security

**Test Cases:**

```plaintext
- Wrong passwords are rejected
- Account lockout after failed attempts
- Secure password requirements enforced
- Session management works correctly
```

### User Interface Testing

#### Main UI Functions

**Manual Test Cases:**

```plaintext
- Note creation and editing
- Context menu operations (right-click)
- Settings dialog functionality
- Time format switching
- Note export feature
```

#### Keyboard Shortcuts

**Test Cases:**

```plaintext
- Ctrl+N: New note
- Ctrl+S: Save note
- Ctrl+E: Export note
- Ctrl+T: Toggle time format
- Escape: Close dialogs
```

### Error Handling Testing

#### Common Error Scenarios

**Test Cases:**

```plaintext
- Disk full during save
- Corrupted data files
- Invalid user credentials
- Missing configuration files
- Hardware fingerprint changes
```

### Performance Testing

#### Key Metrics

**Test Areas:**

```plaintext
- Authentication time (target: 5-10 seconds)
- Note loading speed
- Large note handling (>100KB)
- UI responsiveness during operations
```

### Test Execution

#### Automated Testing

**Schedule:**

```plaintext
- Unit tests: Every commit
- Integration tests: Daily
- Security tests: Weekly
```

#### Manual Testing

**Before Each Release:**

```plaintext
- Complete user workflow testing
- Security validation
- UI functionality check
```

### Test Checklist

#### Core Functionality

**Must Pass:**

```plaintext
- [x] User registration works
- [x] User login/logout works
- [x] Notes can be created and edited
- [x] Notes persist across sessions
- [x] Users cannot see other users' notes
- [x] Password change works
- [x] Note export works
- [x] All keyboard shortcuts work
```

#### Security Requirements

**Must Pass:**

```plaintext
- [x] Data files are encrypted
- [x] Wrong passwords are rejected
- [x] Hardware fingerprint validation works
- [x] User data is isolated
- [x] Account deletion removes all data
```

### Test Environment

#### Setup Requirements

**Tools Needed:**

```plaintext
- Rust toolchain for unit tests
- Multiple test user accounts
- Various note sizes for testing
- Clean state reset procedures
```

### Risk Priorities

#### High Priority (Must Test)

```plaintext
- User authentication
- Data encryption/decryption
- User data isolation
- Password security
```

#### Medium Priority

```plaintext
- Note management operations
- UI functionality
- Error handling
- Performance
```

#### Low Priority

```plaintext
- Time formatting
- UI cosmetics
- Optional shortcuts
```

## Conclusion

This testing strategy focuses on the most critical aspects: security, data integrity, and core functionality. Regular execution of these tests ensures the application maintains user trust and operates reliably across all supported platforms.plaintext
