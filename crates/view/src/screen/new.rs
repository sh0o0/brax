use crate::base::{block::AppBlock, frame::AppFrame, list::AppList, paragraph::AppParagraph};

use domain::brag::{Impact, Type};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Flex, Layout, Offset, Position, Rect},
    prelude::Backend,
    style::{Color, Style},
    text::{self, Span, Text},
    widgets::{block::title, Block, BorderType, Clear, List, ListItem, Paragraph, Widget},
    Frame, Terminal,
};
use std::{
    borrow::Borrow,
    collections::LinkedList,
    ops::Index,
    time::{Duration, Instant},
};
use strum::{IntoEnumIterator, VariantArray};

#[derive(Debug, PartialEq, Eq, Clone, strum::VariantArray)]
enum Field {
    Title,
    Type,
    Impact,
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

impl Field {
    fn idx(self) -> usize {
        Field::VARIANTS.iter().position(|f| *f == self).unwrap()
    }

    fn next(self) -> Option<Self> {
        let index = self.idx();
        if index == Field::VARIANTS.len() - 1 {
            None
        } else {
            Some(Field::VARIANTS[index + 1].clone())
        }
    }

    fn prev(self) -> Option<Self> {
        let index = self.idx();
        if index == 0 {
            None
        } else {
            Some(Field::VARIANTS[index - 1].clone())
        }
    }
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

pub struct App {
    selecting_field: Option<Field>,
    inputs: Inputs,
    should_quit: bool,
    cursor_pos: usize,
}

impl App {
    pub fn default() -> Self {
        Self {
            selecting_field: None,
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
        self.selecting_field = None;
    }

    fn up(&mut self) {
        match self.selecting_field.clone() {
            None => self.selecting_field = Some(Field::VARIANTS.last().unwrap().clone()),
            Some(field) => self.selecting_field = field.prev(),
        }
    }

    fn down(&mut self) {
        match self.selecting_field.clone() {
            None => self.selecting_field = Some(Field::VARIANTS.first().unwrap().clone()),
            Some(field) => self.selecting_field = field.next(),
        }
    }

    fn input(&mut self, c: char) {
        match self.selecting_field {
            Some(Field::Title) => self.enter_char(c),
            _ => {}
        }
    }

    fn delete(&mut self) {
        match self.selecting_field {
            Some(Field::Title) => {
                self.delete_char();
            }
            _ => {}
        }
    }

    fn can_quit(&self) -> bool {
        self.selecting_field == None
    }

    fn quit(&mut self) {
        if self.can_quit() {
            self.should_quit = true;
        }
    }
}

fn ui(frame: &mut Frame, app: &App) {
    let [title_area, typ_area, impact_area] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
    ])
    .areas(frame.area());

    let title = Paragraph::app_default(app.inputs.title.as_str())
        .selecting(app.selecting_field == Some(Field::Title))
        .block(Block::bordered().title("Title"));

    if app.selecting_field == Some(Field::Title) {
        frame.set_app_cursor(title_area, app.cursor_pos as u16);
    }

    let typ = Paragraph::app_default(match app.inputs.typ {
        Type::Project => "Project",
        Type::CollaborationAndMembership => "Collaboration and Membership",
        Type::DesignAndDocumentation => "Design and Documentation",
        Type::CompanyBuilding => "Company Building",
        Type::Learning => "Learning",
        Type::OutsideOfWork => "Outside of Work",
    })
    .selecting(app.selecting_field == Some(Field::Type))
    .block(Block::app_default().title("Type"));

    let impact = Paragraph::app_default(match app.inputs.impact {
        Impact::Trivial => "Trivial",
        Impact::Ordinary => "Ordinary",
        Impact::Notable => "Notable",
        Impact::Remarkable => "Remarkable",
        Impact::Extraordinary => "Extraordinary",
    })
    .selecting(app.selecting_field == Some(Field::Impact))
    .block(Block::app_default().title("Impact"));

    frame.render_widget(title, title_area);
    frame.render_widget(typ, typ_area);
    frame.render_widget(impact, impact_area);

    if app.selecting_field == Some(Field::Type) {
        let block = Block::bordered().border_type(BorderType::Double);
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

        let types = List::new(type_items).block(block).app_highlight();

        render_popup_below_area(frame, types, typ_area, None, Some(8));
    }
}

fn render_popup_below_area<W: Widget>(
    frame: &mut Frame,
    popup: W,
    anchor: Rect,
    max_width: Option<u16>,
    max_height: Option<u16>,
) {
    let popup_area = Rect {
        x: anchor.left(),
        y: anchor.bottom(),
        width: max_width.unwrap_or(anchor.width),
        height: max_height.unwrap_or(anchor.height),
    };
    let intersection = popup_area.intersection(frame.area());
    frame.render_popup(popup, intersection);
}
