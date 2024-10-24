#[derive(Debug, PartialEq, Eq)]
enum SelectingField {
    None,
    Title,
    Type,
    // Impact,
    // Organization,
    // Skills,
    // Languages,
    // StartDate,
    // EndDate,
    // Type,
    // Url,
    // Position,
    // Content,
}

#[derive(Debug, PartialEq, Eq)]
enum Type {
    Project,
    CollaborationAndMembership,
    DesignAndDocumentation,
    CompanyBuilding,
    Learning,
    OutsideOfWork,
}

#[derive(Debug, PartialEq, Eq)]
enum Impact {
    Trivial,
    Ordinary,
    Notable,
    Remarkable,
    Extraordinary,
}

#[derive(Debug, PartialEq, Eq)]

struct Inputs {
    title: String,
    typ: Type,
    impact: Impact,
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
    layout::{Constraint, Flex, Layout, Offset, Position, Rect},
    prelude::Backend,
    style::{Color, Style},
    text::{self, Span},
    widgets::{block::title, Block, BorderType, Clear, List, ListItem, Paragraph},
    Frame, Terminal,
};

pub struct App {
    selecting_field: SelectingField,
    inputs: Inputs,
    should_quit: bool,
    cursor_pos: usize,
}

impl App {
    pub fn default() -> Self {
        Self {
            selecting_field: SelectingField::None,
            inputs: Inputs {
                title: String::new(),
                typ: Type::Project,
                impact: Impact::Notable,
            },
            should_quit: false,
            cursor_pos: 0,
        }
    }

    pub fn run<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        tick_rate: Duration,
    ) -> Result<(), Box<dyn core::error::Error>> {
        let last_tick = Instant::now();

        loop {
            terminal.draw(|frame| ui(frame, self))?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());

            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.modifiers == event::KeyModifiers::CONTROL {
                        match key.code {
                            KeyCode::Char('p') => self.up(),
                            KeyCode::Char('n') => self.down(),
                            KeyCode::Char('h') => self.delete(),
                            _ => {}
                        }
                        continue;
                    }

                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Up | KeyCode::Char('k') => self.up(),
                            KeyCode::Down | KeyCode::Char('j') => self.down(),
                            KeyCode::Left => self.move_cursor_left(),
                            KeyCode::Right => self.move_cursor_right(),
                            KeyCode::Char('q') => {
                                if self.can_quit() {
                                    self.quit()
                                } else {
                                    self.input('q');
                                }
                            }
                            KeyCode::Esc => self.unselect(),
                            KeyCode::Delete | KeyCode::Backspace => self.delete(),
                            KeyCode::Char(c) => self.input(c),
                            _ => {}
                        }
                        continue;
                    }
                }
            }

            if self.should_quit {
                return Ok(());
            }
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_pos.saturating_sub(1);
        self.cursor_pos = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_pos.saturating_add(1);
        self.cursor_pos = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.inputs.title.insert(index, new_char);
        self.move_cursor_right();
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can be contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    fn byte_index(&self) -> usize {
        self.inputs
            .title
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.cursor_pos)
            .unwrap_or(self.inputs.title.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_pos != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.cursor_pos;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.inputs.title.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.inputs.title.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.inputs.title = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.inputs.title.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.cursor_pos = 0;
    }

    fn unselect(&mut self) {
        self.selecting_field = SelectingField::None;
    }

    fn up(&mut self) {
        match self.selecting_field {
            SelectingField::None => self.selecting_field = SelectingField::Title,
            SelectingField::Title => self.selecting_field = SelectingField::Title,
            SelectingField::Type => self.selecting_field = SelectingField::Title,
        }
    }

    fn down(&mut self) {
        match self.selecting_field {
            SelectingField::None => self.selecting_field = SelectingField::Title,
            SelectingField::Title => self.selecting_field = SelectingField::Type,
            SelectingField::Type => self.selecting_field = SelectingField::Type,
        }
    }

    fn input(&mut self, c: char) {
        match self.selecting_field {
            SelectingField::Title => self.enter_char(c),
            _ => {}
        }
    }

    fn delete(&mut self) {
        match self.selecting_field {
            SelectingField::Title => {
                self.delete_char();
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

fn ui(frame: &mut Frame, app: &App) {
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
    let [title_area, typ_area, impact_area] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
    ])
    .areas(frame.area());

    let title = Paragraph::new(app.inputs.title.as_str())
        .style(match app.selecting_field {
            SelectingField::Title => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        })
        .block(Block::bordered().title("Title"));
    if app.selecting_field == SelectingField::Title {
        #[allow(clippy::cast_possible_truncation)]
        frame.set_cursor_position(Position::new(
            // Draw the cursor at the current position in the input field.
            // This position is can be controlled via the left and right arrow key
            title_area.x + app.cursor_pos as u16 + 1,
            // Move one line down, from the border to the input line
            title_area.y + 1,
        ));
    }

    let typ = Paragraph::new(match app.inputs.typ {
        Type::Project => "Project",
        Type::CollaborationAndMembership => "Collaboration and Membership",
        Type::DesignAndDocumentation => "Design and Documentation",
        Type::CompanyBuilding => "Company Building",
        Type::Learning => "Learning",
        Type::OutsideOfWork => "Outside of Work",
    })
    .style(match app.selecting_field {
        SelectingField::Type => Style::default().fg(Color::Yellow),
        _ => Style::default(),
    })
    .block(Block::bordered().title("Type"));

    let impact = Paragraph::new(match app.inputs.impact {
        Impact::Trivial => "Trivial",
        Impact::Ordinary => "Ordinary",
        Impact::Notable => "Notable",
        Impact::Remarkable => "Remarkable",
        Impact::Extraordinary => "Extraordinary",
    });

    frame.render_widget(title, title_area);
    frame.render_widget(typ, typ_area);
    frame.render_widget(impact, impact_area);

    if app.selecting_field == SelectingField::Type {
        let y = typ_area.y + typ_area.height;
        let offset_area = Rect {
            x: typ_area.x,
            y: y,
            width: typ_area.width,
            height: frame.area().height - y,
        };
        let [popup_area] = Layout::vertical([Constraint::Max(8)]).areas(offset_area);

        let block = Block::bordered().border_type(BorderType::Double);

        //     let brags: Vec<ListItem> = app
        //     .brags
        //     .items
        //     .iter()
        //     .map(|&i| {
        //         ListItem::new(vec![text::Line::from(Span::raw(format!(
        //             "{} {}",
        //             i.start_date.format("%Y-%m-%d").to_string(),
        //             i.title.clone()
        //         )))])
        //     })
        //     .collect();
        // let tasks = List::new(brags)
        //     .block(Block::bordered().title("Brags"))
        //     .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        //     .highlight_symbol("> ");

        let type_items: Vec<ListItem> = [
            "Project",
            "Collaboration and Membership",
            "Design and Documentation",
            "Company Building",
            "Learning",
            "Outside of Work",
        ]
        .iter()
        .map(|&i| ListItem::new(vec![text::Line::from(Span::raw(i))]))
        .collect();

        let types = List::new(type_items)
            .block(block)
            .highlight_style(Style::default().fg(Color::Yellow))
            .highlight_symbol("> ");

        frame.render_widget(Clear, popup_area);
        frame.render_widget(types, popup_area);
    }
}

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
