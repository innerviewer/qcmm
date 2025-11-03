use crate::app::App;
use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

pub fn handle_events(app: &mut App) -> io::Result<()> {
    match event::read()? {
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            handle_key_event(app, key_event)
        }
        _ => {}
    };
    Ok(())
}

pub fn handle_key_event(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') => app.exit(),
        KeyCode::Char('t') => app.test_qcm(),
        _ => {}
    }
}
