use ratatui::widgets::Block;

pub trait AppBlock<'a> {
    fn app_default() -> Block<'a>;
}

impl<'a> AppBlock<'a> for Block<'a> {
    fn app_default() -> Self {
        Block::bordered()
    }
}
