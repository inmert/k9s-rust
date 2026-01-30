mod app;
mod ui;
mod events;
mod k8s;
mod handlers;

use std::{io, time::Duration};
use ratatui::{backend::CrosstermBackend, Terminal};
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use anyhow::Result;
use crate::app::App;

#[tokio::main]
async fn main() -> Result<()> {
    // --- 1. TERMINAL SETUP ---
    // We "take over" the terminal.
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // --- 2. APP STATE ---
    let mut app = App::default();

    // --- 3. THE MAIN LOOP ---
    loop {
        // Draw the UI based on current state
        terminal.draw(|f| ui::render(f, &app))?;

        // Check for user input
        // poll(Duration) prevents the loop from eating 100% CPU
        if event::poll(Duration::from_millis(100))? {
            if let CEvent::Key(key) = event::read()? {
                match key.code {
                    // Simple logic to quit
                    KeyCode::Char('q') => {
                        app.should_quit = true;
                    }
                    // We will move this logic to handlers/ later
                    _ => {}
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    // --- 4. TERMINAL RESTORATION ---
    // Critical: Put the terminal back to normal before exiting!
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )?;
    terminal.show_cursor()?;

    Ok(())
}