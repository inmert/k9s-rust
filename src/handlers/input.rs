use crossterm::event::{KeyCode, KeyEvent};
use crate::app::App;

pub fn handle_key_events(key: KeyEvent, app: &mut App) {
    match key.code {
        // Exit
        KeyCode::Char('q') => {
            app.should_quit = true;
        }
        // Navigation
        KeyCode::Char('j') | KeyCode::Down => {
            app.next_pod();
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app.previous_pod();
        }
        _ => {}
    }
}