# Secure Notes - User Guide

## Table of Contents

- [Secure Notes - User Guide](#secure-notes---user-guide)
  - [Table of Contents](#table-of-contents)
  - [Getting Started](#getting-started)
    - [First Launch](#first-launch)
    - [System Requirements](#system-requirements)
  - [User Account Management](#user-account-management)
    - [Creating Your First Account](#creating-your-first-account)
    - [Logging In](#logging-in)
    - [Multiple Users](#multiple-users)
  - [Creating and Managing Notes](#creating-and-managing-notes)
    - [Creating Your First Note](#creating-your-first-note)
    - [Note Management Features](#note-management-features)
      - [Note List (Sidebar)](#note-list-sidebar)
      - [Editing Notes](#editing-notes)
      - [Note Operations](#note-operations)
    - [Time Display Options](#time-display-options)
  - [Security Features](#security-features)
    - [Encryption](#encryption)
    - [Hardware Binding](#hardware-binding)
    - [Data Protection](#data-protection)
  - [Settings and Preferences](#settings-and-preferences)
    - [Accessing Settings](#accessing-settings)
    - [Account Settings](#account-settings)
      - [Change Password](#change-password)
      - [Account Information](#account-information)
      - [Delete Account](#delete-account)
    - [Security Settings](#security-settings)
      - [Security Information Panel](#security-information-panel)
      - [Security Audit](#security-audit)
  - [Keyboard Shortcuts](#keyboard-shortcuts)
    - [Global Shortcuts](#global-shortcuts)
    - [Time Format Shortcuts](#time-format-shortcuts)
    - [Navigation Shortcuts](#navigation-shortcuts)
    - [Text Editing](#text-editing)
  - [Exporting Notes](#exporting-notes)
    - [Export Methods](#export-methods)
    - [Export Process](#export-process)
    - [Export Format](#export-format)
    - [Use Cases](#use-cases)
  - [Troubleshooting](#troubleshooting)
    - [Common Issues](#common-issues)
      - [Authentication Problems](#authentication-problems)
      - [Note Management Issues](#note-management-issues)
      - [Performance Issues](#performance-issues)
    - [Error Messages](#error-messages)
      - ["Failed to initialize crypto manager"](#failed-to-initialize-crypto-manager)
      - ["Permission denied"](#permission-denied)
      - ["Disk full"](#disk-full)
    - [Recovery Procedures](#recovery-procedures)
      - [Forgot Password](#forgot-password)
      - [Corrupted Data Files](#corrupted-data-files)
      - [Hardware Changes](#hardware-changes)
  - [Advanced Features](#advanced-features)
    - [Multi-User Workflows](#multi-user-workflows)
      - [Family/Shared Computer Use](#familyshared-computer-use)
      - [Work/Personal Separation](#workpersonal-separation)
    - [Data Management](#data-management)
      - [Storage Locations](#storage-locations)
      - [Backup Strategies](#backup-strategies)
    - [Security Best Practices](#security-best-practices)
      - [Password Management](#password-management)
      - [Hardware Security](#hardware-security)
  - [Data and Privacy](#data-and-privacy)
    - [Data Storage](#data-storage)
    - [Privacy Features](#privacy-features)
    - [Data Portability](#data-portability)
    - [Compliance](#compliance)
  - [Support and Resources](#support-and-resources)
    - [Getting Help](#getting-help)
    - [Version Information](#version-information)
    - [Community](#community)

## Getting Started

### First Launch

When you first start Secure Notes, you'll see the authentication dialog. This is your gateway to secure note storage.

### System Requirements

- **Windows**: Windows 10 or later
- **macOS**: macOS 10.14 or later
- **Linux**: Most modern distributions
- **Storage**: At least 50MB free space
- **Memory**: 512MB RAM minimum

## User Account Management

### Creating Your First Account

1. **Launch the Application**
   - Double-click the Secure Notes executable
   - The authentication dialog will appear

2. **Register a New Account**
   - Click the "Register" tab
   - Enter a unique username (3-50 characters)
   - Create a strong password (minimum 6 characters)
   - Confirm your password
   - Click "Register"

**Username Requirements:**

- 3-50 characters long
- Letters, numbers, underscores, and hyphens only
- Must be unique (case-insensitive)

**Password Requirements:**

- Minimum 6 characters
- Maximum 128 characters
- Use a strong, memorable password
- Consider using a passphrase for better security

### Logging In

1. **Enter Credentials**
   - Select "Login" tab
   - Enter your username and password
   - Click "Login" or press Enter

2. **Authentication Process**
   - The app will show "Processing... Please wait"
   - Authentication typically takes 5-10 seconds
   - Hardware fingerprint verification occurs automatically

**Note**: The authentication process includes hardware binding for enhanced security. This may take longer on first login or after hardware changes.

### Multiple Users

Secure Notes supports multiple user accounts on the same computer:

- Each user has completely isolated data
- Users cannot access other users' notes
- Switch users by logging out and logging in with different credentials

## Creating and Managing Notes

### Creating Your First Note

1. **New Note Creation**
   - Click "New Note" button in the sidebar
   - Or use keyboard shortcut: `Ctrl+N`
   - Enter a title for your note
   - Click "Create" or press Enter

2. **Note Editor**
   - The note editor opens automatically
   - Start typing your content
   - Notes auto-save every 2 seconds
   - Manual save: `Ctrl+S`

### Note Management Features

#### Note List (Sidebar)

- **Sorted by Date**: Most recently modified notes appear first
- **Note Preview**: Shows title and last modified time
- **Selection**: Click any note to open it for editing
- **Context Menu**: Right-click for additional options

#### Editing Notes

- **Rich Text Support**: Plain text with full Unicode support
- **Auto-Save**: Changes are saved automatically
- **Timestamps**: Creation and modification times are tracked
- **Unlimited Length**: No practical limit on note size

#### Note Operations

- **Edit**: Click on any note to start editing
- **Delete**: Right-click → "Delete Note"
- **Export**: Right-click → "Export to file" or `Ctrl+E`
- **Search**: Use your browser's find function (`Ctrl+F`) within notes

### Time Display Options

Switch between two time formats:

- **Relative Time**: "2 hours ago", "Yesterday", "1 week ago"
- **Absolute Time**: "15.12.2024 14:30" (Swiss timezone)

**Switching Methods:**

- Use the toggle buttons in the sidebar
- Keyboard shortcuts: `Ctrl+R` (Relative) or `Ctrl+Alt+A` (Absolute)
- Quick toggle: `Ctrl+T`

## Security Features

### Encryption

- **Algorithm**: ChaCha20Poly1305 (industry-standard encryption)
- **Key Derivation**: Argon2id with hardware binding
- **Security Level**: Military-grade encryption
- **Performance**: Optimized for desktop use (5-10 second authentication)

### Hardware Binding

Your account is bound to your computer's hardware fingerprint:

- **Components Tracked**: Username, home directory, OS, architecture, computer name
- **Purpose**: Prevents unauthorized access from other devices
- **Flexibility**: Allows minor hardware changes (non-critical components)
- **Security Warnings**: Alerts you to significant hardware changes

### Data Protection

- **File Encryption**: All note files are encrypted on disk
- **User Isolation**: Each user's data is completely separate
- **Secure Storage**: Files stored in system-appropriate locations
- **File Permissions**: Restricted to owner only (Unix systems)

## Settings and Preferences

### Accessing Settings

- Click "Settings" button in the sidebar
- Or use the user menu in the top-right corner

### Account Settings

#### Change Password

1. Click "Change Password" in settings
2. Enter your current password
3. Enter new password (minimum 6 characters)
4. Confirm new password
5. Click "Change Password"

**Important**: Changing your password will re-encrypt all your data with the new password.

#### Account Information

- View username and account creation date
- Check data storage size
- Review security status

#### Delete Account

1. Click "Delete Account" in settings (Danger Zone)
2. Type "DELETE" to confirm
3. Click "Delete Account"

**Warning**: This action is irreversible and will permanently delete all your notes and account data.

### Security Settings

#### Security Information Panel

- Click "Security Info" to view:
  - Encryption details
  - Hardware fingerprint status
  - Security audit results
  - Account creation information

#### Security Audit

- Automatic security checks on login
- Manual audit available in Security Info panel
- Warnings for hardware changes
- Recommendations for security improvements

## Keyboard Shortcuts

### Global Shortcuts

| Shortcut | Action               |
| -------- | -------------------- |
| `Ctrl+N` | Create new note      |
| `Ctrl+S` | Save current note    |
| `Ctrl+E` | Export current note  |
| `Escape` | Close dialogs/panels |

### Time Format Shortcuts

| Shortcut     | Action                  |
| ------------ | ----------------------- |
| `Ctrl+T`     | Toggle time format      |
| `Ctrl+R`     | Switch to relative time |
| `Ctrl+Alt+A` | Switch to absolute time |

### Navigation Shortcuts

| Shortcut | Action                       |
| -------- | ---------------------------- |
| `Tab`    | Navigate between UI elements |
| `Enter`  | Confirm dialogs              |
| `Escape` | Cancel operations            |

### Text Editing

Standard text editing shortcuts work in the note editor:

- `Ctrl+A` - Select all
- `Ctrl+C` - Copy
- `Ctrl+V` - Paste
- `Ctrl+X` - Cut
- `Ctrl+Z` - Undo
- `Ctrl+Y` - Redo

## Exporting Notes

### Export Methods

1. **Context Menu**: Right-click note → "Export to file"
2. **Keyboard**: Select note and press `Ctrl+E`
3. **Current Note**: Export the currently open note

### Export Process

1. **File Dialog**: Choose save location and filename
2. **Default Name**: Based on note title (sanitized for file system)
3. **Format**: Plain text (.txt) with metadata header
4. **Content**: Includes title, timestamps, and full note content

### Export Format

```txt
Title: My Important Note
Created: 15.12.2024 10:30
Modified: 15.12.2024 14:45
ID: unique-note-identifier
==================================================

Your note content appears here...
```

### Use Cases

- **Backup**: Create external backups of important notes
- **Sharing**: Share notes with others (unencrypted)
- **Migration**: Move notes to other applications
- **Printing**: Export for printing or PDF creation

## Troubleshooting

### Common Issues

#### Authentication Problems

**"Invalid username or password"**

- Verify username spelling (case-sensitive)
- Check password carefully
- Ensure Caps Lock is not on
- Try typing password in a text editor first

**"Hardware fingerprint has changed"**

- Normal after hardware upgrades
- Check Security Info panel for details
- Contact support if persistent

**Authentication takes too long**

- Normal range: 5-10 seconds
- Longer times may indicate hardware issues
- Try restarting the application

#### Note Management Issues

**Notes not saving**

- Check available disk space
- Verify file permissions
- Try manual save (`Ctrl+S`)
- Restart application if persistent

**Notes disappeared**

- Ensure you're logged in as the correct user
- Check if you accidentally deleted them
- Look for backup files in application directory

**Cannot create new notes**

- Check disk space
- Verify application permissions
- Try restarting the application

#### Performance Issues

**Slow application startup**

- Normal on first run or after updates
- Check available system memory
- Close other resource-intensive applications

**Slow note loading**

- May occur with very large notes (>100KB)
- Consider breaking large notes into smaller ones
- Check available system memory

### Error Messages

#### "Failed to initialize crypto manager"

- Usually indicates corrupted security files
- Try logging out and back in
- May require account recreation in severe cases

#### "Permission denied"

- Check file system permissions
- Ensure application has write access to config directory
- Run as administrator (Windows) if necessary

#### "Disk full"

- Free up disk space
- Notes are automatically saved, so no data loss
- Application will resume normal operation when space is available

### Recovery Procedures

#### Forgot Password

- Unfortunately, passwords cannot be recovered due to encryption
- You will need to create a new account
- Previous notes cannot be recovered without the password

#### Corrupted Data Files

1. Close the application
2. Navigate to the application data directory
3. Look for `.backup` files
4. Contact support for recovery assistance

#### Hardware Changes

1. Check Security Info panel for details
2. Minor changes are usually handled automatically
3. Major changes may require re-authentication
4. Contact support if unable to access account

## Advanced Features

### Multi-User Workflows

#### Family/Shared Computer Use

- Each family member can have their own account
- Complete data isolation between users
- No shared access to notes
- Individual security settings

#### Work/Personal Separation

- Create separate accounts for work and personal notes
- Switch between accounts as needed
- Different security policies per account

### Data Management

#### Storage Locations

- **Windows**: `%APPDATA%\secure_notes`
- **macOS**: `~/Library/Application Support/secure_notes/`
- **Linux**: `~/.config/secure_notes/`

#### Backup Strategies

1. **Export Important Notes**: Regular exports to external storage
2. **User Data Folder**: Back up entire user data directory
3. **Account Recreation**: Keep account credentials secure for recovery

### Security Best Practices

#### Password Management

- Use unique, strong passwords
- Consider using a password manager
- Don't share passwords with others
- Change passwords periodically

#### Hardware Security

- Keep your computer physically secure
- Use full-disk encryption if available
- Regular security updates for your operating system
- Be aware of hardware fingerprint changes

## Data and Privacy

### Data Storage

- **Local Only**: All data stored locally on your computer
- **No Cloud Sync**: No data transmitted to external servers
- **Encrypted Storage**: All files encrypted with your password
- **User Control**: You have complete control over your data

### Privacy Features

- **No Telemetry**: Application doesn't collect usage data
- **No Network Access**: No internet connection required
- **Offline Operation**: Fully functional without network
- **Anonymous Usage**: No personal information collected

### Data Portability

- **Export Function**: Export notes to standard text format
- **Open Format**: Exported files are plain text
- **No Lock-in**: Easy to migrate to other applications
- **User Ownership**: You own all your data

### Compliance

- **GDPR Compliant**: No personal data processing
- **Local Processing**: All operations performed locally
- **User Consent**: No data collection requiring consent
- **Right to Deletion**: Complete data removal available

## Support and Resources

### Getting Help

- Check this user guide first
- Review troubleshooting section
- Check application logs for error details
- Contact support with specific error messages

### Version Information

- Check "About" in application menu for version details
- Keep application updated for latest security features
- Review release notes for new features

### Community

- Share feedback and suggestions
- Report bugs with detailed information
- Request new features
- Help other users in community forums

---

**Remember**: Secure Notes prioritizes your privacy and security. All data remains on your computer, encrypted with your password. Keep your password safe and secure!
