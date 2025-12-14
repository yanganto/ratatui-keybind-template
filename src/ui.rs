use crate::app::{App, Mode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .split(frame.area());

    render_header(frame, chunks[0]);
    render_main_content(frame, app, chunks[1]);
    render_input_area(frame, app, chunks[2]);
    render_status_bar(frame, app, chunks[3]);
}

fn render_header(frame: &mut Frame, area: Rect) {
    let title = Paragraph::new("Ratatui Keybinding Template")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, area);
}

fn render_main_content(frame: &mut Frame, app: &App, area: Rect) {
    let content = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("Counter: "),
            Span::styled(
                app.counter.to_string(),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from("Keybindings:"),
        Line::from("  Normal Mode:"),
        Line::from("    q       - Quit"),
        Line::from("    k or +  - Increment counter"),
        Line::from("    j or -  - Decrement counter"),
        Line::from("    r       - Reset counter"),
        Line::from("    i       - Enter insert mode"),
        Line::from("    :       - Enter command mode"),
        Line::from("    ?       - Show help"),
        Line::from(""),
        Line::from("  Insert Mode:"),
        Line::from("    ESC     - Return to normal mode"),
        Line::from("    [chars] - Type characters"),
        Line::from(""),
        Line::from("  Command Mode:"),
        Line::from("    ESC     - Return to normal mode"),
        Line::from("    Enter   - Execute command"),
        Line::from("    quit    - Quit application (via command)"),
        Line::from("    reset   - Reset counter (via command)"),
    ];

    let paragraph = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title("Main"))
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

fn render_input_area(frame: &mut Frame, app: &App, area: Rect) {
    let (title, content) = match app.mode {
        Mode::Normal => (
            "Normal Mode",
            "Press 'i' for insert, ':' for command".to_string(),
        ),
        Mode::Insert => ("Insert Mode", format!("Input: {}", app.input)),
        Mode::Command => ("Command Mode", format!(":{}", app.input)),
    };

    let style = match app.mode {
        Mode::Normal => Style::default().fg(Color::Green),
        Mode::Insert => Style::default().fg(Color::Yellow),
        Mode::Command => Style::default().fg(Color::Cyan),
    };

    let input = Paragraph::new(content)
        .style(style)
        .block(Block::default().borders(Borders::ALL).title(title));

    frame.render_widget(input, area);
}

fn render_status_bar(frame: &mut Frame, app: &App, area: Rect) {
    let status = Paragraph::new(app.status_message.as_str())
        .style(Style::default().fg(Color::White).bg(Color::DarkGray));
    frame.render_widget(status, area);
}
