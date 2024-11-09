use ratatui::{layout::Rect, Frame};

pub trait Drawer {
    fn draw(self, frame: &mut Frame, area: Rect);
}

pub trait StatefulDrawer {
    type State;

    fn draw_stateful(self, frame: &mut Frame, area: Rect, state: &mut Self::State);
}
