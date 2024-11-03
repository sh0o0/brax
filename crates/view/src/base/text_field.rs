use crate::config::colors::COLORS;
use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    style::{Style, Stylize},
    text::Text,
    widgets::{Block, Paragraph, StatefulWidget, StatefulWidgetRef, Widget},
    Frame,
};

#[derive(Debug, Clone)]
pub struct TextFieldState {
    text: String,
    cursor_index: usize,
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

    pub fn cursor_index(&self) -> usize {
        self.cursor_index
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_index.saturating_sub(1);
        self.cursor_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_index.saturating_add(1);
        self.cursor_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        if !self.has_modified {
            self.has_modified = true;
        }

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

    pub fn delete_char(&mut self) {
        if !self.has_modified {
            self.has_modified = true;
        }

        let is_not_cursor_leftmost = self.cursor_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.cursor_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.text.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.text.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.text = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.text.chars().count())
    }

    pub fn reset_cursor(&mut self) {
        self.cursor_index = 0;
    }
}

pub type Validator = fn(String) -> Option<String>;

#[derive(Debug, strum::EnumIs)]
pub enum Mode {
    Display,
    Edit,
}

pub struct TextField<'a> {
    block: Option<Block<'a>>,
    validator: Option<Validator>,
    helper: Option<Text<'a>>,
    mode: Mode,
}

impl<'a> TextField<'a> {
    pub fn new() -> Self {
        Self {
            block: None,
            validator: None,
            helper: None,
            mode: Mode::Display,
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

    pub fn mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
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

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut TextFieldState) {
        let mut paragraph = Paragraph::new(match state.text() {
            "" => self.helper.clone().unwrap_or_default().dark_gray(),
            text => text.gray().into(),
        })
        .style(match self.mode {
            Mode::Display => Style::default(),
            Mode::Edit => Style::default().fg(COLORS.primary),
        });

        let mut block = match self.block.clone() {
            Some(block) => block,
            None => Block::default(),
        };

        if state.has_modified {
            if let Some(validator) = self.validator {
                let text = state.text().to_string();
                let validated_text = validator(text);
                match validated_text {
                    Some(validated_text) => {
                        block = block.title_bottom(validated_text.not_bold());
                        paragraph = paragraph.red();
                    }
                    None => {}
                }
            }
        }

        paragraph.block(block).render(area, buf);
    }
}

pub trait TextFieldFrame {
    fn render_text_field(&mut self, text_field: TextField, area: Rect, state: &mut TextFieldState);
}

impl<'a> TextFieldFrame for Frame<'a> {
    fn render_text_field(&mut self, text_field: TextField, area: Rect, state: &mut TextFieldState) {
        if text_field.mode.is_edit() {
            let cursor_pos = state.cursor_index() as u16;
            let position = Position {
                x: area.x + cursor_pos + 1,
                y: area.y + 1,
            };
            self.set_cursor_position(position);
        }

        self.render_stateful_widget(text_field, area, state);
    }
}
