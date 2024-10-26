use ratatui::{
    layout::{Position, Rect},
    widgets::{Clear, Widget},
    Frame,
};

pub trait AppFrame {
    fn render_popup<W: Widget>(&mut self, widget: W, area: Rect);
    fn render_popup_below_anchor<W: Widget>(
        &mut self,
        popup: W,
        anchor: Rect,
        max_width: Option<u16>,
        max_height: Option<u16>,
    );
    fn set_app_cursor_position(&mut self, p: Position);
}

impl<'a> AppFrame for Frame<'a> {
    fn render_popup<W: Widget>(&mut self, widget: W, area: Rect) {
        self.render_widget(Clear, area);
        self.render_widget(widget, area);
    }

    // Not recommended to use this function as it is WIP.
    fn render_popup_below_anchor<W: Widget>(
        &mut self,
        popup: W,
        anchor: Rect,
        max_width: Option<u16>,
        max_height: Option<u16>,
    ) {
        let popup_area = Rect {
            x: anchor.left(),
            y: anchor.bottom(),
            width: max_width.unwrap_or(anchor.width),
            height: max_height.unwrap_or(anchor.height),
        };
        let intersection = popup_area.intersection(self.area());
        self.render_popup(popup, intersection);
    }

    fn set_app_cursor_position(&mut self, position: Position) {
        self.set_cursor_position(position);
    }
}
