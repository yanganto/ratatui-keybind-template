# ratatui-keybind-template
This is a [Ratatui](https://github.com/ratatui/ratatui) template that provides a modular keybinding system using the [crossterm-keybind](https://github.com/yanganto/crossterm-keybind) crate for building terminal user interfaces. It demonstrates best practices for handling keybindings in a scalable way, making it easy to manage keybindings as your application grows.  
Current template is for ratatui v0.30.0, you may check on tags for the template with different version of ratatui.

## Features

- **crossterm-keybind Integration**: Uses the crossterm-keybind crate for powerful keybinding management
- **Configurable Keybindings**: Users can customize keybindings via TOML configuration files
- **Backward Compatible**: Legacy key configs still work with new release if there are only additions in keybind enum.
- **Multiple Keybindings per Action**: Support for assigning multiple key combinations to a single action
- **Auto-generated Config**: Generate example configuration files with `KeyEvent::to_toml_example()`
- **Clean Syntax**: Define keybindings with derive macros using simple string notation
- **Extensible Design**: Easy to add new keybindings and actions

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

**Default Keybindings:**
- `q` - Quit the application
- `Q` - Quit the application
- `Ctrl+c` - Quit the application

You can customize these by generating a config file (see Configuration section below).

## Project Structure

```
src/
├── main.rs       # Application entry point and terminal setup
├── app.rs        # Application state and logic
├── keybinds.rs   # Keybinding definitions using crossterm-keybind
└── ui.rs         # UI rendering logic
```

## Architecture

### Keybinding System with crossterm-keybind

The keybinding system uses the `crossterm-keybind` crate, which provides:

1. **KeyEvent Enum**: Define keybindings using derive macros
2. **Multiple Bindings**: Assign multiple key combinations to one action
3. **Config Files**: Load custom keybindings from TOML files
4. **Display Helpers**: Auto-generate help text for keybindings

### Adding New Keybindings

To add a new keybinding event:

1. Add a new variant to the `KeyEvent` enum in `src/keybinds.rs`
2. Use the `#[keybindings[...]]` attribute to define key combinations
3. Handle the event in `App::handle_key()` in `src/app.rs`

Example:

```rust
// 1. Add to KeyEvent enum in src/keybinds.rs
use crossterm_keybind::KeyBind;

#[derive(KeyBind)]
pub enum KeyEvent {
    /// Quit the application
    #[keybindings["Control+c", "Q", "q"]]
    Quit,
    
    /// Your new action - press 'h' or 'F1' to trigger
    #[keybindings["h", "F1"]]
    ShowHelp,
}

// 2. Handle in src/app.rs
impl App {
    pub fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> bool {
        // Option 1: match_any
        if KeyBindEvent::Quit.match_any(&key) {
            return false;
        }
        if KeyBindEvent::ShowHelp.match_any(&key) {
            // Show help logic
        }

        // Option 2 match events from dispatch method
        for event in KeyBindEvent::dispatch(&key) {
            match event {
                KeyBindEvent::Quit => return false,
                KeyBindEvent::ShowHelp => {
                  // Show help logic
                }
            }
        }
        true
    }
}
```

### Configuration

Generate a default configuration file:

```rust
use crossterm_keybind::KeyBindTrait;
use crate::keybinds::KeyEvent;

// Generate example config
KeyEvent::to_toml_example("keybinds.toml").unwrap();
```

This creates a `keybinds.toml` file:

```toml
# Quit the application
quit = ["Control+c", "Q", "q"]

# Show help
show_help = ["h", "F1"]
```

Users can then customize the keybindings and load them:

```rust
// In main.rs
KeyEvent::init_and_load(Some("keybinds.toml".into()))?;
```

## Dependencies

- [ratatui](https://github.com/ratatui/ratatui) - Terminal UI library
- [crossterm](https://github.com/crossterm-rs/crossterm) - Terminal manipulation library
- [crossterm-keybind](https://github.com/yanganto/crossterm-keybind) - Keybinding management with config support
- [color-eyre](https://github.com/eyre-rs/color-eyre) - Error handling
- [serde](https://github.com/serde-rs/serde) - Serialization framework (required by crossterm-keybind)

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
