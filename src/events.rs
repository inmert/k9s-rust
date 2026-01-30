
use crossterm::event::KeyEvent;

pub enum Event {
    Input(KeyEvent),
    Tick,
}