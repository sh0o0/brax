use crossterm::event::KeyEvent;

pub trait KeyEventHandler {
    fn handle_key_event(&mut self, key: &KeyEvent) -> util::error::Result<()>;
}
