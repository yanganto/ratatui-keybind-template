# Contributing to Ratatui Keybind Template

Thank you for your interest in contributing! This document provides guidelines for extending and customizing the template.

## Extending the Template

### Adding New Actions

1. **Define the Action**: Add a new variant to the `Action` enum in `src/keybinds.rs`:
   ```rust
   pub enum Action {
       // ... existing actions
       YourNewAction,
   }
   ```

2. **Register Keybindings**: Add keybindings for your action in `KeyBindings::default()`:
   ```rust
   bindings.bind(
       KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE),
       Mode::Normal,
       Action::YourNewAction,
   );
   ```

3. **Implement Action Logic**: Handle the action in `App::perform_action()` in `src/app.rs`:
   ```rust
   match action {
       // ... existing actions
       Action::YourNewAction => {
           // Your implementation here
       }
   }
   ```

### Adding New Modes

1. **Define the Mode**: Add a variant to the `Mode` enum in `src/app.rs`:
   ```rust
   #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
   pub enum Mode {
       Normal,
       Insert,
       Command,
       YourNewMode,
   }
   ```

2. **Add Mode-Specific Bindings**: Register keybindings for the new mode in `src/keybinds.rs`

3. **Add Transition Actions**: Create actions to enter/exit your new mode

4. **Update UI**: Modify `src/ui.rs` to display the new mode appropriately

### Adding Application State

Add new fields to the `App` struct in `src/app.rs`:

```rust
pub struct App {
    pub mode: Mode,
    // ... existing fields
    pub your_new_field: YourType,
}
```

Don't forget to initialize it in `App::new()`.

### Customizing UI

The UI is rendered in `src/ui.rs`. The main `render()` function divides the screen into sections:
- Header: Application title
- Main content: Your main UI elements
- Input area: Mode-specific input display
- Status bar: Status messages

Modify these functions to customize the appearance of your application.

## Best Practices

### Keybinding Organization

- Group related keybindings together
- Use consistent key patterns across modes
- Document non-obvious keybindings
- Consider adding a help screen with all keybindings

### Action Design

- Keep actions atomic and focused on a single task
- Use meaningful action names
- Group related actions in the same section of the enum

### State Management

- Keep application state in the `App` struct
- Use the `perform_action` method for all state changes
- Avoid side effects in `handle_key` - it should only map keys to actions

### Mode Management

- Clearly define the purpose of each mode
- Provide easy ways to switch between modes (usually ESC to return to normal)
- Display the current mode prominently in the UI

## Testing

Add tests for your new functionality:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_your_new_feature() {
        // Your test here
    }
}
```

Run tests with:
```bash
cargo test
```

## Code Style

This project follows standard Rust formatting:
```bash
cargo fmt
```

And linting:
```bash
cargo clippy
```

## Loading Keybindings from Configuration

For a real application, you might want to load keybindings from a configuration file. Here's a pattern you can follow:

1. Use `serde` to deserialize keybinding configuration
2. Create a method to load bindings from a config file
3. Call this method when initializing `KeyBindings`

Example:
```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct KeyBindingConfig {
    key: String,
    modifiers: Vec<String>,
    mode: String,
    action: String,
}

impl KeyBindings {
    pub fn from_config(config: Vec<KeyBindingConfig>) -> Self {
        let mut bindings = Self::new();
        for cfg in config {
            // Parse and register bindings
        }
        bindings
    }
}
```

## Questions or Issues?

If you have questions or run into issues while extending the template, please open an issue on GitHub.
