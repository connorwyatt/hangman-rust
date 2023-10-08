use crate::ratatui_game_runner::{app::App, event_handler::Event};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub(crate) fn update(app: &mut App, event: Event) {
    match event {
        Event::Tick => {}
        Event::Key(key) => update_key(app, key),
        Event::Mouse(_) => {}
        Event::Resize(_, _) => {}
    }
}

fn update_key(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        }
        _ => {}
    }
}
