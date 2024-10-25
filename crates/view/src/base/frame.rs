use ratatui::{
    layout::{Position, Rect},
    Frame,
};

pub trait AppFrame {
    fn set_app_cursor(&mut self, area: &Rect, pos: u16);
}

impl<'a> AppFrame for Frame<'a> {
    fn set_app_cursor(&mut self, area: &Rect, pos: u16) {
        #[allow(clippy::cast_possible_truncation)]
        self.set_cursor_position(Position::new(area.x + pos + 1, area.y + 1));
    }
}
