use ratatui::widgets::{Block, BorderType};

pub trait AppBlock<'a> {
    fn app_default() -> Block<'a> {
        Block::bordered()
    }
    fn popup() -> Block<'a> {
        Block::bordered().border_type(BorderType::Double)
    }
}

impl<'a> AppBlock<'a> for Block<'a> {}
