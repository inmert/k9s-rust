mod app;
mod ui;
mod events;
mod k8s;
mod handlers;

use std::{io, time::Duration};
use ratatui::{backend::CrosstermBackend, Terminal};
use crossterm::{
    event::{self, Event as CEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use anyhow::Result;
use crate::app::App;

#[tokio::main]
async fn main() -> Result<()> {
    // Setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::default();

    // Main Loop
    loop {
        terminal.draw(|f| ui::render(f, &mut app))?;

        if event::poll(Duration::from_millis(100))? {
            if let CEvent::Key(key) = event::read()? {
                // Delegation occurs here
                handlers::input::handle_key_events(key, &mut app);
            }
        }

        if app.should_quit {
            break;
        }
    }

    // Restore
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}