use ratatui::widgets::ListState;

#[derive(Debug, Clone, Default)]
pub struct LoopListState {
    list_state: ListState,
    len: usize,
}

impl LoopListState {
    pub fn new(len: usize) -> Self {
        Self {
            list_state: ListState::default(),
            len,
        }
    }

    pub fn list_state(&self) -> &ListState {
        &self.list_state
    }

    pub fn with_offset(mut self, offset: usize) -> Self {
        self.list_state = self.list_state.with_offset(offset);
        self
    }

    pub fn with_selected(mut self, selected: Option<usize>) -> Self {
        self.list_state = self.list_state.with_selected(selected);
        self
    }

    pub fn select_first(&mut self) {
        if self.len == 0 {
            self.list_state.select(None);
            return;
        }

        self.list_state.select_first();
    }

    pub fn select_last(&mut self) {
        if self.len == 0 {
            self.list_state.select(None);
            return;
        }

        self.list_state.select(Some(self.len - 1));
    }

    pub fn select_next(&mut self) {
        if self.len == 0 {
            self.list_state.select(None);
            return;
        }

        match self.list_state.selected() {
            None => self.select_first(),
            Some(_) => {
                if self.is_selecting_last() {
                    self.select_first();
                } else {
                    self.list_state.select_next();
                }
            }
        }
    }

    pub fn select_previous(&mut self) {
        if self.len == 0 {
            self.list_state.select(None);
            return;
        }

        match self.list_state.selected() {
            None => self.select_last(),
            Some(_) => {
                if self.is_selecting_first() {
                    self.select_last();
                } else {
                    self.list_state.select_previous();
                }
            }
        }
    }

    fn is_selecting_last(&self) -> bool {
        match self.list_state.selected() {
            None => false,
            Some(selected) => selected == self.len - 1,
        }
    }

    fn is_selecting_first(&self) -> bool {
        match self.list_state.selected() {
            None => false,
            Some(selected) => selected == 0,
        }
    }
}
