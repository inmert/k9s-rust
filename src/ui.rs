use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::app::App;

pub fn render(f: &mut Frame, app: &App) {
    // Partition the screen into Header and Body
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Body
        ])
        .split(f.size());

    let header = Paragraph::new(" RustK9s - Press 'q' to quit ")
        .block(Block::default().borders(Borders::ALL));

    let body = Paragraph::new(format!("Pods Found: {:?}", app.pods))
        .block(Block::default().borders(Borders::ALL).title(" Pods "));

    f.render_widget(header, chunks[0]);
    f.render_widget(body, chunks[1]);
}