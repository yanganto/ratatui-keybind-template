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
        // You can handle your event in one of following ways

        // Option 1: match_any
        if KeyBindEvent::Quit.match_any(&key) {
            return false;
        }

        // Option 2 match events from dispatch method
        for event in KeyBindEvent::dispatch(&key) {
            match event {
                KeyBindEvent::Quit => return false,
            }
        }
        false
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
