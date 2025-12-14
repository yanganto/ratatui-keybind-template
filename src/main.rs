use color_eyre::Result;
use crossterm::event::{self, Event};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;
use ratatui::prelude::*;
use std::io::{stdout, Stdout};

mod app;
mod keybinds;
mod ui;

use app::App;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut terminal = setup_terminal()?;
    let result = run(&mut terminal);

    restore_terminal()?;

    result
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|frame| ui::render(frame, &app))?;

        if let Event::Key(key) = event::read()? {
            if let Some(action) = app.handle_key(key) {
                if !app.perform_action(action) {
                    break;
                }
            }
        }
    }

    Ok(())
}
