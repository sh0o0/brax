use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    text::Text,
    widgets::{Block, Paragraph, Widget, WidgetRef},
    Frame,
};

enum Status {
    Editing,
}

#[derive(Debug)]
pub struct TextFieldController {
    text: String,
    cursor_pos: usize,
}

impl<'a> TextFieldController {
    pub fn default() -> Self {
        Self::new(String::new(), 0)
    }

    fn new(text: String, cursor_pos: usize) -> Self {
        Self { text, cursor_pos }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn cursor_pos(&self) -> usize {
        self.cursor_pos
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_pos.saturating_sub(1);
        self.cursor_pos = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_pos.saturating_add(1);
        self.cursor_pos = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
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
            .nth(self.cursor_pos)
            .unwrap_or(self.text.len())
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_pos != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.cursor_pos;
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
        self.cursor_pos = 0;
    }
}

pub struct TextField<'a, F: FnMut(Position)> {
    controller: &'a TextFieldController,
    block: Option<Block<'a>>,
    set_cursor_position: Option<F>,
}

impl<'a, F: FnMut(Position)> TextField<'a, F> {
    pub fn new(controller: &'a TextFieldController) -> Self {
        Self {
            controller,
            block: None,
            set_cursor_position: None,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn cursor(mut self, set_cursor_position: F) -> Self {
        self.set_cursor_position = Some(set_cursor_position);
        self
    }
}

impl<'a, F: FnMut(Position)> TextField<'a, F> {
    fn render_cursor(self, area: Rect) {
        if let Some(mut set_cursor_position) = self.set_cursor_position {
            let cursor_pos = self.controller.cursor_pos as u16;
            let cursor_x = area.x + cursor_pos + 1;
            let cursor_y = area.y + 1;
            set_cursor_position(Position {
                x: cursor_x,
                y: cursor_y,
            });
        }
    }
}

impl<'a, F: FnMut(Position)> Widget for TextField<'a, F> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_ref(area, buf);
        self.render_cursor(area);
    }
}

impl<'a, F: FnMut(Position)> WidgetRef for TextField<'a, F> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let text = Text::raw(self.controller.text());
        let paragraph = Paragraph::new(text).block(self.block.as_ref().unwrap().clone());
        paragraph.render(area, buf);
    }
}
