use ratatui::{
    style::{Color, Style, Stylize},
    text::Text,
    widgets::Paragraph,
};

pub trait AppParagraph<'a> {
    fn app_default<T>(text: T) -> Paragraph<'a>
    where
        T: Into<Text<'a>>;
    fn selecting(self, is_selecting: bool) -> Paragraph<'a>;
}

impl<'a> AppParagraph<'a> for Paragraph<'a> {
    fn app_default<T>(text: T) -> Self
    where
        T: Into<Text<'a>>,
    {
        let block = Paragraph::new(text);
        block
    }

    fn selecting(self, is_selecting: bool) -> Self {
        if is_selecting {
            self.style(Style::default().bold())
        } else {
            self.style(Style::default().fg(Color::DarkGray))
        }
    }
}
