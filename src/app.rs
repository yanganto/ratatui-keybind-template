use crate::keybinds::{Action, KeyBindings};
use crossterm::event::KeyEvent;

/// Application state and logic
pub struct App {
    /// Current mode of the application
    pub mode: Mode,
    /// Input buffer for text input mode
    pub input: String,
    /// Counter for demonstration purposes
    pub counter: u32,
    /// Key bindings configuration
    keybindings: KeyBindings,
    /// Status message to display
    pub status_message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
    Normal,
    Insert,
    Command,
}

impl App {
    pub fn new() -> Self {
        Self {
            mode: Mode::Normal,
            input: String::new(),
            counter: 0,
            keybindings: KeyBindings::default(),
            status_message: "Press '?' for help".to_string(),
        }
    }

    /// Handle a key event and return the corresponding action
    pub fn handle_key(&self, key: KeyEvent) -> Option<Action> {
        self.keybindings.get_action(key, self.mode)
    }

    /// Perform an action and return false if the app should quit
    pub fn perform_action(&mut self, action: Action) -> bool {
        match action {
            Action::Quit => return false,
            Action::Increment => {
                self.counter += 1;
                self.status_message = format!("Counter incremented to {}", self.counter);
            }
            Action::Decrement => {
                self.counter = self.counter.saturating_sub(1);
                self.status_message = format!("Counter decremented to {}", self.counter);
            }
            Action::Reset => {
                self.counter = 0;
                self.status_message = "Counter reset".to_string();
            }
            Action::EnterInsertMode => {
                self.mode = Mode::Insert;
                self.status_message = "-- INSERT --".to_string();
            }
            Action::EnterCommandMode => {
                self.mode = Mode::Command;
                self.input.clear();
                self.status_message = "Enter command:".to_string();
            }
            Action::EnterNormalMode => {
                self.mode = Mode::Normal;
                self.input.clear();
                self.status_message = "-- NORMAL --".to_string();
            }
            Action::InsertChar(c) => {
                self.input.push(c);
            }
            Action::DeleteChar => {
                self.input.pop();
            }
            Action::ExecuteCommand => {
                self.execute_command();
                self.mode = Mode::Normal;
            }
            Action::ShowHelp => {
                self.status_message =
                    "Help: q=quit, i=insert, :=command, +/k=inc, -/j=dec, r=reset".to_string();
            }
        }
        true
    }

    fn execute_command(&mut self) {
        match self.input.as_str() {
            "q" | "quit" => {
                self.status_message = "Use 'q' in normal mode to quit".to_string();
            }
            "reset" => {
                self.counter = 0;
                self.status_message = "Counter reset via command".to_string();
            }
            cmd => {
                self.status_message = format!("Unknown command: {}", cmd);
            }
        }
        self.input.clear();
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
