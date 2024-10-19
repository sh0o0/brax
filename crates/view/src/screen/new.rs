// struct BragInput {
//     title: String,
//     organization: String,
//     skills: Vec<String>,
//     languages: Vec<String>,
//     start_date: Date,
//     end_date: Option<Date>,
//     type_: Type,
//     impact: Impact,
//     url: Option<URL>,
//     position: Option<String>,
//     content: String,
// }

use std::time::Duration;

use ratatui::{prelude::Backend, Terminal};

pub struct App {}

pub fn run<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    tick_rate: Duration,
) -> Result<(), Box<dyn core::error::Error>> {
    Ok(())
}
