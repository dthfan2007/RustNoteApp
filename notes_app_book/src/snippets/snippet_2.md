# Snippet 2: First tests with variables

```rust
egui::CentralPanel::default().show(ctx, |ui| {
    ui.heading("Encrypted Text Application");

    ui.horizontal(|ui| {
        let name_label = ui.label("Your name: ");
        let response = ui
            .text_edit_multiline(&mut self.name)
            .labelled_by(name_label.id);

        if response.changed() {
            if let Err(e) = self.save_encrypted_data() {
                eprintln!("Failed to save data: {}", e);
            }
        }
    });

    ui.add(egui::Slider::new(&mut self.age, 0..=120).text("years old"));
    ui.label(format!(
        "Hello {}, you are {} years old!", self.name, self.age
    ));
```

This snippet demonstrates basic UI interaction with persistent data storage. The code:

- **Creates a text input field**: Allows users to enter their name with automatic labeling
- **Implements real-time saving**: Automatically encrypts and saves data when the text changes
- **Adds an age slider**: Interactive slider component for selecting age (0-120 years)
- **Displays formatted output**: Shows a personalized greeting using the entered data

This represents early testing of the GUI components with encrypted data persistence.

## Interactive User Interface Components and Real-Time Data Binding

This code snippet demonstrates the core user interface implementation using egui's immediate mode GUI paradigm. It showcases how to create interactive components that respond to user input in real-time while maintaining data persistence through encryption.

### Comprehensive Component Analysis

**Central Panel Layout System:**
The `egui::CentralPanel::default().show(ctx, |ui|)` creates the main content area of the application window. This central panel automatically fills the available space and serves as the primary container for all user interface elements. The immediate mode nature means the UI is redrawn every frame, allowing for dynamic updates and real-time responsiveness.

**Application Title Header:**
`ui.heading("Encrypted Text Application")` creates a prominent heading that serves multiple purposes:

- **Visual Hierarchy**: Establishes clear information hierarchy with larger, bold text
- **Application Identity**: Immediately communicates to users what application they're using
- **Professional Appearance**: Provides a polished, finished look to the interface

**Horizontal Layout Container:**
The `ui.horizontal()` closure creates a horizontal arrangement of UI elements, which is essential for creating intuitive form-like interfaces where labels and input fields appear side by side.

**Advanced Text Input Implementation:**

```rust
let name_label = ui.label("Your name: ");
let response = ui.text_edit_multiline(&mut self.name).labelled_by(name_label.id);
```

This sophisticated text input system includes several advanced features:

- **Accessibility Integration**: The `labelled_by()` method creates proper accessibility relationships between the label and input field, ensuring screen readers and other assistive technologies can properly understand the interface
- **Multi-line Capability**: Uses `text_edit_multiline()` instead of single-line input, allowing users to enter longer text that can span multiple lines
- **Response Handling**: Captures the response object which contains information about user interactions with the text field
- **Mutable Reference**: Uses `&mut self.name` to directly bind the input field to the application's state, enabling immediate updates

**Real-Time Data Persistence:**

```rust
if response.changed() {
    if let Err(e) = self.save_encrypted_data() {
        eprintln!("Failed to save data: {}", e);
    }
}
```

This automatic save functionality provides several critical benefits:

- **Immediate Persistence**: Data is saved as soon as the user makes changes, preventing data loss
- **Encryption Integration**: All saves go through the encryption system, ensuring data security
- **Error Handling**: Comprehensive error handling with logging to help diagnose issues
- **Non-Blocking Operation**: Save operations don't interrupt the user's typing experience

**Interactive Age Slider Component:**
`ui.add(egui::Slider::new(&mut self.age, 0..=120).text("years old"))` creates a sophisticated slider control with:

- **Range Validation**: Automatically constrains values between 0 and 120, preventing invalid age entries
- **Visual Feedback**: Provides immediate visual representation of the current value
- **Mouse and Keyboard Support**: Users can interact via mouse dragging or keyboard input
- **Descriptive Text**: The `.text("years old")` provides context about what the slider represents

**Dynamic Content Display:**

```rust
ui.label(format!("Hello {}, you are {} years old!", self.name, self.age));
```

This dynamic label demonstrates real-time data binding:

- **String Interpolation**: Uses Rust's `format!` macro for efficient string construction
- **Live Updates**: The display updates immediately as users modify either the name or age
- **Personalized Experience**: Creates a more engaging, interactive experience for users

**Immediate Mode GUI Benefits:**
This implementation showcases the power of immediate mode GUIs:

- **Simplicity**: No complex event handling or callback systems required
- **Real-time Updates**: All changes are immediately reflected in the interface
- **State Synchronization**: UI and application state are always synchronized
- **Debugging Friendly**: Easy to understand and debug due to linear execution flow
