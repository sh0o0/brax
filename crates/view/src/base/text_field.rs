use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    style::{Style, Stylize},
    text::Text,
    widgets::{Block, Paragraph, StatefulWidget, StatefulWidgetRef, Widget},
    Frame,
};
use unicode_width::UnicodeWidthStr;

use crate::utils::drawer::StatefulDrawer;

#[derive(Debug, Clone)]
pub struct TextFieldState {
    text: String,
    cursor_index: usize,
    is_editing: bool,
    has_modified: bool,
}

impl<'a> TextFieldState {
    pub fn default() -> Self {
        Self::new(String::new(), 0)
    }

    fn new(text: String, cursor_index: usize) -> Self {
        Self {
            text,
            cursor_index,
            is_editing: false,
            has_modified: false,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.cursor_index = self.text.chars().count();
    }

    pub fn set_is_editing(&mut self, is_editing: bool) {
        self.is_editing = is_editing;
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_index.saturating_sub(1);
        self.cursor_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_index.saturating_add(1);
        self.cursor_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn move_cursor_to_start(&mut self) {
        self.cursor_index = 0;
    }

    pub fn move_cursor_to_end(&mut self) {
        self.cursor_index = self.text.chars().count();
    }

    pub fn enter_char(&mut self, new_char: char) {
        self.modify();

        let index = self.byte_index();
        self.text.insert(index, new_char);
        self.move_cursor_right();
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can be contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    pub fn byte_index(&self) -> usize {
        self.text
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.cursor_index)
            .unwrap_or(self.text.len())
    }

    pub fn unicode_index(&self) -> usize {
        self.text[..self.byte_index()].width()
    }

    pub fn delete_left_char(&mut self) {
        self.modify();

        let is_not_cursor_leftmost = self.cursor_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.cursor_index;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.text.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.text.chars().skip(current_index);

            self.text = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    pub fn delete_right_char(&mut self) {
        self.modify();

        let is_not_cursor_rightmost = self.cursor_index != self.text.chars().count();
        if is_not_cursor_rightmost {
            let current_index = self.cursor_index;
            let from_current_to_right_index = current_index + 1;

            let before_char_to_delete = self.text.chars().take(current_index);
            let after_char_to_delete = self.text.chars().skip(from_current_to_right_index);

            self.text = before_char_to_delete.chain(after_char_to_delete).collect();
        }
    }

    pub fn delete_right_all(&mut self) {
        self.modify();

        self.text = self.text.chars().take(self.cursor_index).collect();
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.text.chars().count())
    }

    fn modify(&mut self) {
        if !self.has_modified {
            self.has_modified = true;
        }
    }
}

pub type Validator = fn(String) -> Option<String>;

#[derive(Debug, Default)]
pub struct TextField<'a> {
    block: Option<Block<'a>>,
    validator: Option<Validator>,
    helper: Option<Text<'a>>,
    style: Option<Style>,
}

impl<'a> TextField<'a> {
    pub fn new() -> Self {
        Self {
            block: None,
            validator: None,
            helper: None,
            style: None,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn validator(mut self, validator: Validator) -> Self {
        self.validator = Some(validator);
        self
    }

    pub fn helper(mut self, helper: Text<'a>) -> Self {
        self.helper = Some(helper);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }
}

impl<'a> StatefulWidget for TextField<'a> {
    type State = TextFieldState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.render_ref(area, buf, state);
    }
}

impl<'a> StatefulWidgetRef for TextField<'a> {
    type State = TextFieldState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut paragraph = Paragraph::new(match state.text() {
            "" => self.helper.clone().unwrap_or_default().dark_gray(),
            text => text.gray().into(),
        });

        if let Some(style) = &self.style {
            paragraph = paragraph.style(style.clone());
        }

        let mut block = match &self.block {
            Some(block) => block.clone(),
            None => Block::default(),
        };

        if state.has_modified {
            if let Some(validator) = self.validator {
                let text = state.text().to_string();
                let validated_text = validator(text);
                match validated_text {
                    Some(validated_text) => {
                        block = block.title_bottom(validated_text.not_bold()).red();
                        paragraph = paragraph.red();
                    }
                    None => {}
                }
            }
        }

        paragraph.block(block).render(area, buf);
    }
}

impl StatefulDrawer for TextField<'_> {
    type State = TextFieldState;

    fn draw(self, frame: &mut Frame, area: Rect, state: &mut Self::State) {
        if state.is_editing {
            let cursor_pos = state.unicode_index() as u16;
            let position = Position {
                x: area.x + cursor_pos + 1,
                y: area.y + 1,
            };
            frame.set_cursor_position(position);
        }

        frame.render_stateful_widget(self, area, state);
    }
}
