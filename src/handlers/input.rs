use crossterm::event::{KeyCode, KeyEvent};
use crate::app::App;

pub fn handle_key_events(key: KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Char('q') => {
            app.should_quit = true;
        }
        KeyCode::Down => {
            app.next_pod();
        }
        KeyCode::Up => {
            app.previous_pod();
        }
        _ => {}
    }
}