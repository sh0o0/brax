use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{Block, List, ListItem, ListState, StatefulWidget, StatefulWidgetRef},
};

#[derive(Debug, Clone)]
pub struct LoopListState {
    pub offset: usize,
    pub selected: Option<usize>,
    pub len: usize,
}

impl Into<ListState> for &mut LoopListState {
    fn into(self) -> ListState {
        ListState::default()
            .with_offset(self.offset)
            .with_selected(self.selected)
    }
}

impl LoopListState {
    pub fn default(len: usize) -> Self {
        Self {
            offset: 0,
            selected: None,
            len,
        }
    }

    pub fn with_selected(mut self, selected: Option<usize>) -> Self {
        self.selected = selected;
        self
    }

    pub fn select_next(&mut self) {
        if self.len == 0 {
            return;
        }

        match self.selected {
            None => self.selected = Some(0),
            Some(selected) => {
                if selected < self.len - 1 {
                    self.selected = Some(selected + 1);
                } else {
                    self.selected = Some(0);
                }
            }
        }
    }

    pub fn select_previous(&mut self) {
        if self.len == 0 {
            return;
        }

        match self.selected {
            None => self.selected = Some(self.len - 1),
            Some(selected) => {
                if selected > 0 {
                    self.selected = Some(selected - 1);
                } else {
                    self.selected = Some(self.len - 1);
                }
            }
        }
    }
}

pub struct LoopList<'a> {
    list: List<'a>,
}

impl<'a> LoopList<'a> {
    pub fn new<T>(items: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<ListItem<'a>>,
    {
        Self {
            list: List::new(items),
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.list = self.list.block(block);
        self
    }

    pub fn highlight_style(mut self, style: Style) -> Self {
        self.list = self.list.highlight_style(style);
        self
    }

    pub fn highlight_symbol(mut self, symbol: &'a str) -> Self {
        self.list = self.list.highlight_symbol(symbol);
        self
    }
}

impl<'a> StatefulWidget for LoopList<'a> {
    type State = LoopListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.render_ref(area, buf, state);
    }
}

impl<'a> StatefulWidgetRef for LoopList<'a> {
    type State = LoopListState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut LoopListState) {
        StatefulWidgetRef::render_ref(&self.list, area, buf, &mut state.into());
    }
}
