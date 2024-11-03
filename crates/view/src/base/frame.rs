use ratatui::{
    layout::Rect,
    widgets::{Clear, StatefulWidget, Widget},
    Frame,
};

pub trait AppFrame {
    fn render_popup<W: Widget>(&mut self, widget: W, area: Rect);
    fn render_stateful_popup<W: StatefulWidget>(
        &mut self,
        widget: W,
        area: Rect,
        state: &mut W::State,
    );
}

impl<'a> AppFrame for Frame<'a> {
    fn render_popup<W: Widget>(&mut self, widget: W, area: Rect) {
        self.render_widget(Clear, area);
        self.render_widget(widget, area);
    }

    fn render_stateful_popup<W: StatefulWidget>(
        &mut self,
        widget: W,
        area: Rect,
        state: &mut W::State,
    ) {
        self.render_widget(Clear, area);
        self.render_stateful_widget(widget, area, state);
    }
}
