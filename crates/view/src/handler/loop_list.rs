use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::base::loop_list::LoopListState;

use super::handler::KeyEventHandler;

impl KeyEventHandler for LoopListState {
    fn handle_key_event(&mut self, key: &KeyEvent) -> util::error::Result<()> {
        if key.modifiers == KeyModifiers::CONTROL {
            match key.code {
                KeyCode::Char('n') => self.select_next(),
                KeyCode::Char('p') => self.select_previous(),
                _ => {}
            };
            return Ok(());
        }

        match key.code {
            KeyCode::Up => self.select_previous(),
            KeyCode::Down => self.select_next(),
            KeyCode::Char('k') => self.select_previous(),
            KeyCode::Char('j') => self.select_next(),
            _ => {}
        }

        return Ok(());
    }
}
