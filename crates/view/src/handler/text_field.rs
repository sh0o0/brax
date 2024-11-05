use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::base::text_field::TextFieldState;

use super::handler::KeyEventHandler;

impl KeyEventHandler for TextFieldState {
    fn handle_key_event(&mut self, key: &KeyEvent) -> util::error::Result<()> {
        if key.modifiers == KeyModifiers::CONTROL {
            match key.code {
                KeyCode::Char('b') => self.move_cursor_left(),
                KeyCode::Char('f') => self.move_cursor_right(),
                KeyCode::Char('e') => self.move_cursor_to_end(),
                KeyCode::Char('a') => self.move_cursor_to_start(),
                KeyCode::Char('d') => self.delete_right_char(),
                KeyCode::Char('k') => self.delete_right_all(),
                KeyCode::Char('h') => self.delete_left_char(),
                _ => {}
            };
            return Ok(());
        }

        match key.code {
            KeyCode::Left => self.move_cursor_left(),
            KeyCode::Right => self.move_cursor_right(),
            KeyCode::Backspace => self.delete_left_char(),
            KeyCode::Delete => self.delete_right_char(),
            KeyCode::Char(c) => self.enter_char(c),
            _ => {}
        }

        return Ok(());
    }
}
