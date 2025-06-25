# Snippet 3: First playtesting with functions

```rust
struct MyApp {
    name: String,
    age: u32,
}

impl MyApp {
    fn new() -> Self {
        let mut app = Self {
            name: String::new(),
            age: 18,
        };
    }

    fn increment(&mut self) {
        if self.age < 120 {
            self.age += 1;
        }
    }

    fn decrement(&mut self) {
        if self.age > 0 {
            self.age -= 1;
        }
    }
}

egui::CentralPanel::default().show(ctx, |ui| {
    ui.add(egui::Slider::new(&mut self.age, 0..=120).text("years old"));

    ui.horizontal(|ui| {
        if ui.button("Increment").clicked() {
            self.increment(); // Increment function call
        }

        if ui.button("Decrement").clicked() {
            self.decrement(); // Decrement function call
        }
    });
})
```

This snippet introduces function-based interaction and demonstrates proper struct organization. The code includes:

**MyApp struct**: Contains the application state with name and age fields

**Core functions**:

- **`new()`**: Constructor that initializes the app with default values
- **`increment()`**: Safely increases age with upper bound checking (max 120)
- **`decrement()`**: Safely decreases age with lower bound checking (min 0)

**Interactive UI**: Combines slider control with button-based increment/decrement functionality

This shows the evolution from simple variables to structured, function-based application logic.

## Core Application Architecture and State Management

This code snippet establishes the fundamental application structure and demonstrates object-oriented programming principles in Rust. It showcases how to create a well-structured application with encapsulated state management and safe mutation methods.

### Detailed Structural Analysis

**Application State Structure:**

```rust
struct MyApp {
    name: String,
    age: u32,
}
```

This struct represents the core application state with carefully chosen data types:

- **`name: String`**: Uses Rust's owned `String` type rather than string slices, providing full ownership and mutability of the text data. This allows for dynamic text that can grow, shrink, and be modified throughout the application's lifetime
- **`age: u32`**: Uses an unsigned 32-bit integer, which is perfect for representing age values. The choice of `u32` over smaller types like `u8` provides plenty of range while being efficient, and the unsigned nature prevents negative age values

**Constructor Implementation:**

```rust
fn new() -> Self {
    let mut app = Self {
        name: String::new(),
        age: 18,
    };
}
```

The constructor follows Rust conventions and best practices:

- **Default Initialization**: Creates a new instance with sensible default values
- **Empty String**: Initializes name as an empty string, ready for user input
- **Reasonable Default Age**: Sets age to 18, which is a common default for adult-oriented applications
- **Self Return**: Returns the constructed instance using Rust's `Self` keyword for better maintainability

**Safe Increment Method:**

```rust
fn increment(&mut self) {
    if self.age < 120 {
        self.age += 1;
    }
}
```

This method demonstrates defensive programming principles:

- **Mutable Self Reference**: Takes `&mut self` to allow modification of the instance state
- **Bounds Checking**: Implements an upper limit of 120 years, which is a reasonable maximum human age
- **Overflow Prevention**: Prevents integer overflow that could cause undefined behavior or crashes
- **Silent Boundary Handling**: When the maximum is reached, the method gracefully does nothing rather than throwing errors
- **Single Responsibility**: The method has one clear purpose - safely incrementing the age value

**Safe Decrement Method:**

```rust
fn decrement(&mut self) {
    if self.age > 0 {
        self.age -= 1;
    }
}
```

This method mirrors the increment functionality with appropriate safeguards:

- **Lower Bound Protection**: Prevents age from going below zero, which would be nonsensical
- **Underflow Prevention**: Since `u32` is unsigned, attempting to subtract from zero would cause underflow
- **Consistent Behavior**: Matches the increment method's approach to boundary conditions
- **Graceful Degradation**: Silently handles boundary conditions without disrupting user experience

**User Interface Integration:**
The UI implementation demonstrates several important patterns:

**Age Slider Integration:**

```rust
ui.add(egui::Slider::new(&mut self.age, 0..=120).text("years old"));
```

- **Direct State Binding**: The slider directly modifies the application state
- **Range Consistency**: Uses the same 0-120 range as the increment/decrement methods
- **Immediate Feedback**: Changes are instantly reflected in the UI

**Interactive Button Implementation:**

```rust
ui.horizontal(|ui| {
    if ui.button("Increment").clicked() {
        self.increment();
    }
    if ui.button("Decrement").clicked() {
        self.decrement();
    }
});
```

This button layout showcases several UI design principles:

- **Horizontal Grouping**: Places related buttons side by side for intuitive interaction
- **Event-Driven Actions**: Buttons trigger specific methods when clicked
- **Method Delegation**: UI events are handled by calling appropriate business logic methods
- **Consistent Interaction**: Both buttons follow the same interaction pattern

**Architectural Benefits:**
This design pattern provides several advantages:

- **Encapsulation**: State and behavior are contained within the struct
- **Safety**: All mutations go through controlled methods with validation
- **Testability**: Methods can be easily unit tested in isolation
- **Maintainability**: Clear separation between UI and business logic
- **Extensibility**: New methods and state can be easily added following the same patterns

**Memory Safety and Performance:**

- **Stack Allocation**: The struct is lightweight and can be stack-allocated
- **No Dynamic Allocation**: The age field requires no heap allocation
- **Efficient Updates**: Direct field access provides optimal performance
- **Rust Ownership**: Leverages Rust's ownership system for memory safety without garbage collection
