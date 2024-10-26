use domain::error::Result;
use ratatui::{
    layout::{Position, Rect},
    widgets::{Clear, Widget},
    Frame,
};

pub trait AppFrame {
    fn set_app_cursor(&mut self, area: Rect, pos: u16);
    fn render_popup<W: Widget>(&mut self, widget: W, area: Rect);
}

impl<'a> AppFrame for Frame<'a> {
    fn set_app_cursor(&mut self, area: Rect, pos: u16) {
        #[allow(clippy::cast_possible_truncation)]
        self.set_cursor_position(Position::new(area.x + pos + 1, area.y + 1));
    }

    fn render_popup<W: Widget>(&mut self, widget: W, area: Rect) {
        self.render_widget(Clear, area);
        self.render_widget(widget, area);
    }
}
