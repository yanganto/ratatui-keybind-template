# ratatui-keybind-template

A template to avoid keybinding issues when your TUI goes big.

This is a [Ratatui](https://github.com/ratatui/ratatui) template that provides a modular keybinding system for building terminal user interfaces. It demonstrates best practices for handling keybindings in a scalable way, making it easy to manage keybindings as your application grows.

## Features

- **Modular Keybinding System**: Centralized keybinding management with mode-specific bindings
- **Multiple Modes**: Support for Normal, Insert, and Command modes (inspired by Vim)
- **Action-based Architecture**: Separates key handling from action execution
- **Extensible Design**: Easy to add new keybindings and actions
- **Example Application**: Includes a working example demonstrating the keybinding system
- **Tests**: Unit tests for keybinding behavior

## Using This Template

Click the "Use this template" button on GitHub to create a new repository based on this template. Then clone your new repository and start building your TUI application.

```bash
git clone https://github.com/YOUR_USERNAME/YOUR_NEW_REPO.git
cd YOUR_NEW_REPO
cargo run
```

## Quick Start

Run the example application:

```bash
cargo run
```

### Keybindings

**Normal Mode:**
- `q` - Quit the application
- `k` or `+` - Increment counter
- `j` or `-` - Decrement counter
- `r` - Reset counter
- `i` - Enter insert mode
- `:` - Enter command mode
- `?` - Show help

**Insert Mode:**
- `ESC` - Return to normal mode
- Type any character to add it to the input buffer

**Command Mode:**
- `ESC` - Return to normal mode
- `Enter` - Execute command
- `quit` or `q` - Quit application
- `reset` - Reset counter

## Project Structure

```
src/
├── main.rs       # Application entry point and terminal setup
├── app.rs        # Application state and logic
├── keybinds.rs   # Keybinding system (Action enum and KeyBindings struct)
└── ui.rs         # UI rendering logic
```

## Architecture

### Keybinding System

The keybinding system is built around three main concepts:

1. **Actions**: Enum representing all possible actions in the application
2. **KeyBindings**: Maps key events to actions based on the current mode
3. **Mode**: Current application mode (Normal, Insert, Command)

### Adding New Keybindings

To add a new keybinding:

1. Add a new action to the `Action` enum in `src/keybinds.rs`
2. Register the keybinding in `KeyBindings::default()` in `src/keybinds.rs`
3. Handle the action in `App::perform_action()` in `src/app.rs`

Example:

```rust
// 1. Add action
pub enum Action {
    // ... existing actions
    MyNewAction,
}

// 2. Register keybinding
impl Default for KeyBindings {
    fn default() -> Self {
        let mut bindings = Self::new();
        // ... existing bindings
        bindings.bind(
            KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE),
            Mode::Normal,
            Action::MyNewAction,
        );
        bindings
    }
}

// 3. Handle action
impl App {
    pub fn perform_action(&mut self, action: Action) -> bool {
        match action {
            // ... existing actions
            Action::MyNewAction => {
                // Your logic here
            }
        }
        true
    }
}
```

## Dependencies

- [ratatui](https://github.com/ratatui/ratatui) - Terminal UI library
- [crossterm](https://github.com/crossterm-rs/crossterm) - Terminal manipulation library
- [color-eyre](https://github.com/eyre-rs/color-eyre) - Error handling
- [serde](https://github.com/serde-rs/serde) - Serialization framework (for future config file support)

## Testing

Run tests with:

```bash
cargo test
```

## License

MIT License - See [LICENSE](LICENSE) file for details

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## Acknowledgments

This template is inspired by the [ratatui templates](https://github.com/ratatui/templates) project and follows best practices from the Ratatui community.
