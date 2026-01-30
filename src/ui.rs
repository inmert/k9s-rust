use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};
use crate::app::App;

pub fn render(f: &mut Frame, app: &mut App) { // Note the &mut App here
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(f.size());

    // 1. Header
    let header = Block::default()
        .borders(Borders::ALL)
        .title(" RustK9s ");
    f.render_widget(header, chunks[0]);

    // 2. Pod List
    let items: Vec<ListItem> = app.pods
        .iter()
        .map(|name| ListItem::new(name.as_str()))
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Pods "))
        .highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> "); // The "cursor" icon

    // We use render_stateful_widget to link the list with our App's state
    f.render_stateful_widget(list, chunks[1], &mut app.state);
}