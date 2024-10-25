use ratatui::{
    style::{Color, Style},
    widgets::List,
};

pub trait AppList<'a> {
    fn app_highlight(self) -> List<'a>;
}

impl<'a> AppList<'a> for List<'a> {
    fn app_highlight(self) -> Self {
        self.highlight_style(Style::default().fg(Color::Yellow))
            .highlight_symbol("> ")
    }
}
