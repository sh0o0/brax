use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{Block, ListItem, StatefulWidget, StatefulWidgetRef},
};

use crate::utils::tui::StatefulDrawer;

use super::{
    loop_list::{LoopList, LoopListState},
    text_field::{TextField, TextFieldState},
};

#[derive(Debug, Clone)]
pub struct AutocompleteTextFieldState<'a, T> {
    items: &'a Vec<T>,
    text_field: TextFieldState,
    offset: usize,
    selected: Option<usize>,
    filter: fn(&T, &str) -> bool,
    filtered_items: Vec<&'a T>,
}

impl<'a, T> AutocompleteTextFieldState<'a, T> {
    pub fn new(items: &'a Vec<T>, filter: fn(&T, &str) -> bool) -> Self {
        Self {
            items: items,
            filter: filter,
            text_field: TextFieldState::default(),
            offset: 0,
            selected: None,
            filtered_items: Vec::new(),
        }
    }

    pub fn update_items(&mut self, items: &'a Vec<T>) {
        self.items = items;
        self.update_filtered_items();
    }

    pub fn set_is_editing(&mut self, is_editing: bool) {
        self.text_field.set_is_editing(is_editing);
    }

    pub fn move_cursor_left(&mut self) {
        self.text_field.move_cursor_left();
    }

    pub fn move_cursor_right(&mut self) {
        self.text_field.move_cursor_right();
    }

    pub fn enter_char(&mut self, c: char) {
        self.text_field.enter_char(c);
        self.update_filtered_items();
    }

    pub fn delete_char(&mut self) {
        self.text_field.delete_char();
        self.update_filtered_items();
    }

    fn update_filtered_items(&mut self) {
        self.filtered_items = self
            .items
            .iter()
            .filter(|item| (self.filter)(item, self.text_field.text()))
            .collect();
    }
}

impl<'a, T> Into<LoopListState> for &mut AutocompleteTextFieldState<'a, T> {
    fn into(self) -> LoopListState {
        LoopListState::new(self.filtered_items.len())
            .with_selected(self.selected)
            .with_offset(self.offset)
    }
}

#[derive(Debug, Default)]
pub struct AutocompleteTextField<'a, T> {
    text_field: TextField<'a>,

    _marker: std::marker::PhantomData<&'a T>,
}

impl<'a, T> AutocompleteTextField<'a, T> {
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.text_field = self.text_field.block(block);
        self
    }

    pub fn validator(mut self, validator: fn(String) -> Option<String>) -> Self {
        self.text_field = self.text_field.validator(validator);
        self
    }

    pub fn helper(mut self, helper: &'a str) -> Self {
        self.text_field = self.text_field.helper(helper.into());
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.text_field = self.text_field.style(style);
        self
    }
}

impl<'a, T> StatefulWidget for AutocompleteTextField<'a, T> {
    type State = AutocompleteTextFieldState<'a, T>;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.render_ref(area, buf, state);
    }
}

impl<'a, T> StatefulWidgetRef for AutocompleteTextField<'a, T> {
    type State = AutocompleteTextFieldState<'a, T>;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.text_field.render_ref(area, buf, &mut state.text_field);
    }
}

impl<'a, T> StatefulDrawer for AutocompleteTextField<'a, T> {
    type State = AutocompleteTextFieldState<'a, T>;

    fn draw(self, frame: &mut ratatui::Frame, area: Rect, state: &mut Self::State) {
        self.text_field.draw(frame, area, &mut state.text_field);
    }
}

pub struct AutocompleteTextFieldList<'a, T> {
    item_renderer: fn(&T) -> ListItem,
    loop_list: LoopList<'a>,

    _marker: std::marker::PhantomData<&'a T>,
}

impl<'a, T> AutocompleteTextFieldList<'a, T> {
    pub fn new(item_renderer: fn(&T) -> ListItem) -> Self {
        Self {
            item_renderer: item_renderer,
            loop_list: LoopList::default(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'a, T> StatefulWidget for AutocompleteTextFieldList<'a, T> {
    type State = AutocompleteTextFieldState<'a, T>;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.render_ref(area, buf, state);
    }
}

impl<'a, T> StatefulWidgetRef for AutocompleteTextFieldList<'a, T> {
    type State = AutocompleteTextFieldState<'a, T>;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let items = state
            .filtered_items
            .iter()
            .map(|item| (self.item_renderer)(item))
            .collect::<Vec<_>>();

        self.loop_list
            .clone()
            .items(items)
            .render_ref(area, buf, &mut state.into());
    }
}
