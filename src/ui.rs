use crate::app::App;
use crossterm_keybind::KeyBindTrait;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame, _app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(1)])
        .split(frame.area());

    // Header
    let title = Paragraph::new("Ratatui Keybinding Template")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, chunks[0]);

    // Main content
    let quit_keys = crate::keybinds::KeyEvent::Quit.key_bindings_display();
    let content = vec![
        Line::from(""),
        Line::from("Welcome to the Ratatui Keybinding Template!"),
        Line::from(""),
        Line::from("This template uses crossterm-keybind for powerful keybinding management."),
        Line::from(""),
        Line::from("Keybindings:"),
        Line::from(format!("  {} - Quit", quit_keys)),
        Line::from(""),
        Line::from("Features:"),
        Line::from("  • Add keybindings in src/keybinds.rs using derive macros"),
        Line::from("  • Generate config files with KeyEvent::to_toml_example()"),
        Line::from("  • Users can customize keybindings via TOML files"),
        Line::from("  • Multiple key combinations per action"),
        Line::from(""),
        Line::from("Extend this template by adding your own KeyEvent variants!"),
    ];

    let paragraph = Paragraph::new(content).block(Block::default().borders(Borders::ALL));
    frame.render_widget(paragraph, chunks[1]);

    // Status bar
    let status = Paragraph::new("Press 'q' to quit")
        .style(Style::default().fg(Color::White).bg(Color::DarkGray));
    frame.render_widget(status, chunks[2]);
}
