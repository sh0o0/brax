use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{Block, List, ListItem, StatefulWidget, StatefulWidgetRef},
};

use crate::utils::tui::StatefulDrawer;

use super::{
    loop_list::LoopListState,
    text_field::{TextField, TextFieldState},
};

#[derive(Debug, Clone)]
pub struct AutocompleteTextFieldState<'a, T> {
    items: &'a Vec<T>,
    text_field: TextFieldState,
    filter: fn(&T, &str) -> bool,
    filtered_items: Vec<&'a T>,
    loop_list_state: LoopListState,
    extract_text_for_confirm: fn(&T) -> String,
}

impl<'a, T> AutocompleteTextFieldState<'a, T> {
    pub fn new(
        items: &'a Vec<T>,
        filter: fn(&T, &str) -> bool,
        extract_text_for_confirm: fn(&T) -> String,
    ) -> Self {
        let mut i = Self {
            items: items,
            filter: filter,
            text_field: TextFieldState::default(),
            filtered_items: Vec::new(),
            loop_list_state: LoopListState::new(0),
            extract_text_for_confirm: extract_text_for_confirm,
        };
        i.filter();
        i
    }

    pub fn update_items(&mut self, items: &'a Vec<T>) {
        self.items = items;
        self.filter();
    }

    pub fn select_next(&mut self) {
        self.loop_list_state.select_next();
    }

    pub fn select_previous(&mut self) {
        self.loop_list_state.select_previous();
    }

    pub fn confirm(&mut self) {
        if let Some(selected) = self.loop_list_state.list_state().selected() {
            let item = self.filtered_items[selected];
            let text = (self.extract_text_for_confirm)(item);
            self.text_field.set_text(text);
        }
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

    pub fn move_cursor_to_start(&mut self) {
        self.text_field.move_cursor_to_start();
    }

    pub fn move_cursor_to_end(&mut self) {
        self.text_field.move_cursor_to_end();
    }

    pub fn enter_char(&mut self, c: char) {
        self.text_field.enter_char(c);
        self.filter();
    }

    pub fn delete_left_char(&mut self) {
        self.text_field.delete_left_char();
        self.filter();
    }

    pub fn delete_right_char(&mut self) {
        self.text_field.delete_right_char();
        self.filter();
    }

    pub fn delete_right_all(&mut self) {
        self.text_field.delete_right_all();
        self.filter();
    }

    fn filter(&mut self) {
        let text = self.text_field.text();

        if text.is_empty() {
            self.filtered_items = self.items.iter().collect();
            self.update_loop_list_state();
            return;
        }

        self.filtered_items = self
            .items
            .iter()
            .filter(|item| (self.filter)(item, text))
            .collect();

        self.update_loop_list_state()
    }

    fn update_loop_list_state(&mut self) {
        self.loop_list_state = LoopListState::new(self.filtered_items.len());
        self.loop_list_state.select_first();
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
    loop_list: List<'a>,

    _marker: std::marker::PhantomData<&'a T>,
}

impl<'a, T> AutocompleteTextFieldList<'a, T> {
    pub fn new(item_renderer: fn(&T) -> ListItem) -> Self {
        Self {
            item_renderer: item_renderer,
            loop_list: List::default(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.loop_list = self.loop_list.block(block);
        self
    }

    pub fn highlight_style(mut self, style: Style) -> Self {
        self.loop_list = self.loop_list.highlight_style(style);
        self
    }

    pub fn highlight_symbol(mut self, symbol: &'a str) -> Self {
        self.loop_list = self.loop_list.highlight_symbol(symbol);
        self
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

        self.loop_list.clone().items(items).render_ref(
            area,
            buf,
            &mut state.loop_list_state.list_state().clone(),
        );
    }
}
