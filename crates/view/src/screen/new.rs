#[derive(Debug, PartialEq, Eq)]
enum SelectingField {
    None,
    Title,
    // Organization,
    // Skills,
    // Languages,
    // StartDate,
    // EndDate,
    // Type,
    // Impact,
    // Url,
    // Position,
    // Content,
}

#[derive(Debug, PartialEq, Eq)]

struct Inputs {
    title: String,
    // organization: String,
    // skills: Vec<String>,
    // languages: Vec<String>,
    // start_date: Date,
    // end_date: Option<Date>,
    // type_: Type,
    // impact: Impact,
    // url: Option<URL>,
    // position: Option<String>,
    // content: String,
}

use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout},
    prelude::Backend,
    style::{Color, Style},
    widgets::{block::title, Block, Paragraph},
    Frame, Terminal,
};

pub struct App {
    selecting_field: SelectingField,
    inputs: Inputs,
    should_quit: bool,
}

impl App {
    pub fn default() -> Self {
        Self {
            selecting_field: SelectingField::None,
            inputs: Inputs {
                title: String::new(),
            },
            should_quit: false,
        }
    }

    fn unselect(&mut self) {
        self.selecting_field = SelectingField::None;
    }

    fn up(&mut self) {
        match self.selecting_field {
            SelectingField::None => self.selecting_field = SelectingField::Title,
            SelectingField::Title => self.selecting_field = SelectingField::Title,
        }
    }

    fn down(&mut self) {
        match self.selecting_field {
            SelectingField::None => self.selecting_field = SelectingField::Title,
            SelectingField::Title => self.selecting_field = SelectingField::Title,
        }
    }

    fn input(&mut self, c: char) {
        match self.selecting_field {
            SelectingField::Title => self.inputs.title.push(c),
            _ => {}
        }
    }

    fn delete(&mut self) {
        match self.selecting_field {
            SelectingField::Title => {
                self.inputs.title.pop();
            }
            _ => {}
        }
    }

    fn can_quit(&self) -> bool {
        self.selecting_field == SelectingField::None
    }

    fn quit(&mut self) {
        if self.can_quit() {
            self.should_quit = true;
        }
    }
}

pub fn run<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    tick_rate: Duration,
) -> Result<(), Box<dyn core::error::Error>> {
    let last_tick = Instant::now();

    loop {
        terminal.draw(|frame| ui(frame, app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.modifiers == event::KeyModifiers::CONTROL {
                    match key.code {
                        KeyCode::Char('p') => app.up(),
                        KeyCode::Char('n') => app.down(),
                        _ => {}
                    }
                    continue;
                }

                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Up | KeyCode::Char('k') => app.up(),
                        KeyCode::Down | KeyCode::Char('j') => app.down(),
                        KeyCode::Char('q') => {
                            if app.can_quit() {
                                app.quit()
                            } else {
                                app.input('q');
                            }
                        }
                        KeyCode::Esc => app.unselect(),
                        KeyCode::Delete | KeyCode::Backspace => app.delete(),
                        KeyCode::Char(c) => app.input(c),
                        _ => {}
                    }
                    continue;
                }
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn ui(frame: &mut Frame, app: &mut App) {
    // let chunks = Layout::horizontal([Constraint::Fill(1)]).split(frame.area());

    // let block = Block::bordered().title("Details");
    // frame.render_widget(block, chunks[0]);

    // UI
    // - type: select (default: Project)
    // - impact: select (default: Notable)
    // - start date: input 2024, 2024-01, 2024-01-01 (default: this month)
    // - end date: input (option)
    // - Advance
    //     1. organization (option)
    //     2. skill (option)
    //     3. languages (option)
    //     4. url (option)
    //     5. position (option)
    // - content
    let [title_area] = Layout::vertical([Constraint::Length(3)]).areas(frame.area());

    let title = Paragraph::new(app.inputs.title.as_str())
        .style(match app.selecting_field {
            SelectingField::Title => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        })
        .block(Block::bordered().title("Title"));
    frame.render_widget(title, title_area);
}
