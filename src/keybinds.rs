use crate::app::Mode;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::collections::HashMap;

/// Actions that can be performed in the application
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Quit,
    Increment,
    Decrement,
    Reset,
    EnterInsertMode,
    EnterCommandMode,
    EnterNormalMode,
    InsertChar(char),
    DeleteChar,
    ExecuteCommand,
    ShowHelp,
}

/// Key binding configuration
///
/// This struct manages the mapping between key events and actions for different modes.
/// It provides a centralized way to handle keybindings, making it easy to:
/// - Add new keybindings
/// - Change existing keybindings
/// - Support mode-specific keybindings
/// - Load keybindings from configuration files
pub struct KeyBindings {
    /// Map of (KeyEvent, Mode) to Action
    bindings: HashMap<(KeyEvent, Mode), Action>,
}

impl KeyBindings {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    /// Register a keybinding for a specific mode
    pub fn bind(&mut self, key: KeyEvent, mode: Mode, action: Action) {
        self.bindings.insert((key, mode), action);
    }

    /// Get the action for a key event in the current mode
    pub fn get_action(&self, key: KeyEvent, mode: Mode) -> Option<Action> {
        // First try to find a mode-specific binding
        if let Some(action) = self.bindings.get(&(key, mode)) {
            return Some(*action);
        }

        // In insert mode, any printable character becomes an InsertChar action
        if mode == Mode::Insert {
            if let KeyCode::Char(c) = key.code {
                if key.modifiers == KeyModifiers::NONE || key.modifiers == KeyModifiers::SHIFT {
                    return Some(Action::InsertChar(c));
                }
            }
        }

        None
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        let mut bindings = Self::new();

        // Normal mode bindings
        bindings.bind(
            KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE),
            Mode::Normal,
            Action::Quit,
        );
        bindings.bind(
            KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE),
            Mode::Normal,
            Action::Increment,
        );
        bindings.bind(
            KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE),
            Mode::Normal,
            Action::Decrement,
        );
        bindings.bind(
            KeyEvent::new(KeyCode::Char('+'), KeyModifiers::NONE),
            Mode::Normal,
            Action::Increment,
        );
        bindings.bind(
            KeyEvent::new(KeyCode::Char('-'), KeyModifiers::NONE),
            Mode::Normal,
            Action::Decrement,
        );
        bindings.bind(
            KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE),
            Mode::Normal,
            Action::Reset,
        );
        bindings.bind(
            KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE),
            Mode::Normal,
            Action::EnterInsertMode,
        );
        bindings.bind(
            KeyEvent::new(KeyCode::Char(':'), KeyModifiers::SHIFT),
            Mode::Normal,
            Action::EnterCommandMode,
        );
        bindings.bind(
            KeyEvent::new(KeyCode::Char('?'), KeyModifiers::SHIFT),
            Mode::Normal,
            Action::ShowHelp,
        );

        // Insert mode bindings
        bindings.bind(
            KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
            Mode::Insert,
            Action::EnterNormalMode,
        );
        bindings.bind(
            KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE),
            Mode::Insert,
            Action::DeleteChar,
        );

        // Command mode bindings
        bindings.bind(
            KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
            Mode::Command,
            Action::EnterNormalMode,
        );
        bindings.bind(
            KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
            Mode::Command,
            Action::ExecuteCommand,
        );
        bindings.bind(
            KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE),
            Mode::Command,
            Action::DeleteChar,
        );

        // Command mode also handles character input
        for c in 'a'..='z' {
            bindings.bind(
                KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE),
                Mode::Command,
                Action::InsertChar(c),
            );
        }

        bindings
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_mode_quit() {
        let bindings = KeyBindings::default();
        let key = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
        assert_eq!(bindings.get_action(key, Mode::Normal), Some(Action::Quit));
    }

    #[test]
    fn test_insert_mode_char() {
        let bindings = KeyBindings::default();
        let key = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE);
        assert_eq!(
            bindings.get_action(key, Mode::Insert),
            Some(Action::InsertChar('a'))
        );
    }

    #[test]
    fn test_mode_specific_bindings() {
        let bindings = KeyBindings::default();
        let esc_key = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);

        // ESC in insert mode should return to normal
        assert_eq!(
            bindings.get_action(esc_key, Mode::Insert),
            Some(Action::EnterNormalMode)
        );

        // ESC in normal mode should do nothing (None)
        assert_eq!(bindings.get_action(esc_key, Mode::Normal), None);
    }
}
