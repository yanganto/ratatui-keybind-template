use crate::keybinds::KeyEvent as KeyBindEvent;
use crossterm::event::KeyEvent;
use crossterm_keybind::KeyBindTrait;

/// Application state and logic
pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }

    /// Handle a key event and return false if the app should quit
    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        if KeyBindEvent::Quit.match_any(&key) {
            return false;
        }
        // Add more keybinding matches here as you extend the application
        true
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
