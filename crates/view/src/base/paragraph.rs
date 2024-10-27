use ratatui::{text::Text, widgets::Paragraph};

pub trait AppParagraph<'a> {
    fn app_default<T>(text: T) -> Paragraph<'a>
    where
        T: Into<Text<'a>>;
}

impl<'a> AppParagraph<'a> for Paragraph<'a> {
    fn app_default<T>(text: T) -> Self
    where
        T: Into<Text<'a>>,
    {
        let block = Paragraph::new(text);
        block
    }
}
