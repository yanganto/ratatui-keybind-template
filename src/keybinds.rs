use crossterm_keybind::KeyBind;

/// Key binding events for the application
///
/// This enum uses the crossterm-keybind crate to provide a clean and
/// configurable way to handle keybindings. Each variant can have multiple
/// keybindings defined via the #[keybindings] attribute.
///
/// Users can generate a config file with `KeyEvent::to_toml_example("keybinds.toml")`
/// and customize keybindings by editing the TOML file, then loading it with
/// `KeyEvent::init_and_load(Some("keybinds.toml".into()))`.
#[derive(KeyBind)]
pub enum KeyEvent {
    /// Quit the application
    /// Default keybindings: Control+c, Q, q
    #[keybindings["Control+c", "Q", "q"]]
    Quit,
}

// Note: Testing keybinds with crossterm-keybind requires careful handling
// of initialization. For integration tests, run the full application instead.
