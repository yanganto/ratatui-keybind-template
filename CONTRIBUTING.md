# Contributing to Ratatui Keybind Template

Thank you for your interest in contributing! This document provides guidelines for extending and customizing the template.

## Extending the Template

### Adding New Keybinding Events

This template uses the [crossterm-keybind](https://github.com/yanganto/crossterm-keybind) crate for keybinding management.

1. **Define the Event**: Add a new variant to the `KeyEvent` enum in `src/keybinds.rs`:
   ```rust
   use crossterm_keybind::KeyBind;
   
   #[derive(KeyBind)]
   pub enum KeyEvent {
       /// Quit the application
       #[keybindings["Control+c", "Q", "q"]]
       Quit,
       
       /// Your new action - supports multiple key combinations
       /// You can use format: "Key", "Control+Key", "Alt+Key", "Shift+Key", etc.
       #[keybindings["h", "F1", "?"]]
       ShowHelp,
   }
   ```

2. **Handle the Event**: Update `App::handle_key()` in `src/app.rs`:
   ```rust
   use crossterm_keybind::KeyBindTrait;
   
   impl App {
       pub fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> bool {
           if KeyBindEvent::Quit.match_any(&key) {
               return false;
           }
           if KeyBindEvent::ShowHelp.match_any(&key) {
               // Your implementation here
               return true;
           }
           true
       }
   }
   ```

3. **Update UI** (Optional): Display the keybinding in `src/ui.rs`:
   ```rust
   let help_keys = crate::keybinds::KeyEvent::ShowHelp.key_bindings_display();
   Line::from(format!("  {} - Show help", help_keys))
   ```

### Adding Application State

Add new fields to the `App` struct in `src/app.rs`:

```rust
pub struct App {
    // ... existing fields
    pub your_new_field: YourType,
}
```

Don't forget to initialize it in `App::new()`.

### Customizing UI

The UI is rendered in `src/ui.rs`. The main `render()` function divides the screen into sections:
- Header: Application title
- Main content: Your main UI elements
- Status bar: Status messages

Modify the `render()` function to customize the appearance of your application.

## Best Practices

### Keybinding Organization with crossterm-keybind

- **Use descriptive comments**: Document what each keybinding does above the variant
- **Multiple bindings**: Assign multiple key combinations for common actions (e.g., 'q' and 'Q' for quit)
- **Standard conventions**: Follow common patterns (Ctrl+C for quit, F1 for help, etc.)
- **Generate config examples**: Use `KeyEvent::to_toml_example()` to create user-customizable configs

### Keybinding Syntax

The `crossterm-keybind` crate supports various key formats:
- Single keys: `"a"`, `"A"`, `"1"`, `"F1"`, `"Enter"`, `"Esc"`
- With modifiers: `"Control+c"`, `"Alt+Enter"`, `"Shift+Tab"`
- Special keys: `"Up"`, `"Down"`, `"Left"`, `"Right"`, `"PageUp"`, `"Home"`, etc.

### State Management

- Keep application state in the `App` struct
- Return `false` from `handle_key()` to quit the application
- Return `true` from `handle_key()` to continue running

## Testing

For testing keybindings, it's recommended to:
1. Test the application manually by running `cargo run`
2. Test keybinding parsing by verifying the TOML example generation
3. Write integration tests for your application logic

Note: Unit testing crossterm-keybind's match_any requires careful initialization handling.

Run your application with:
```bash
cargo run
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

## Configuration File Support

The `crossterm-keybind` crate provides built-in config file support:

### Generating a Config Template

```rust
use crossterm_keybind::KeyBindTrait;
use crate::keybinds::KeyEvent;

// Generate a TOML config file with default keybindings
KeyEvent::to_toml_example("keybinds.toml")?;
```

This creates a file like:
```toml
# Quit the application
quit = ["Control+c", "Q", "q"]

# Show help
show_help = ["h", "F1"]
```

### Loading Custom Configuration

In `main.rs`, load the config file:
```rust
// Load default keybindings (no config file)
KeyEvent::init_and_load(None)?;

// Or load from a custom config file
KeyEvent::init_and_load(Some("custom_keybinds.toml".into()))?;
```

### Allowing User Customization

1. Generate a default config on first run
2. Save it to a user config directory (e.g., `~/.config/yourapp/keybinds.toml`)
3. Load the config on subsequent runs
4. Users can edit the TOML file to customize keybindings

## Questions or Issues?

If you have questions or run into issues while extending the template, please open an issue on GitHub.
