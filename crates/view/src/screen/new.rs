use crate::{
    base::{block::AppBlock, frame::AppFrame, list::AppList, paragraph::AppParagraph},
    utils::{self, text::Txt},
};

use domain::brag::{Impact, Type};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout},
    prelude::Backend,
    text::{self, Span},
    widgets::{Block, BorderType, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::time::{Duration, Instant};
use strum::VariantArray;

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

impl utils::text::Txt for Field {
    fn text(&self) -> String {
        match self {
            Field::Title => "Title".to_string(),
            Field::Type => "Type".to_string(),
            Field::Impact => "Impact".to_string(),
            // Field::Organization => "Organization".to_string(),
            // Field::Skills => "Skills".to_string(),
            // Field::Languages => "Languages".to_string(),
            // Field::StartDate => "Start Date".to_string(),
            // Field::EndDate => "End Date".to_string(),
            // Field::Type => "Type".to_string(),
            // Field::Url => "URL".to_string(),
            // Field::Position => "Position".to_string(),
            // Field::Content => "Content".to_string(),
        }
    }
}

impl utils::text::Txt for Impact {
    fn text(&self) -> String {
        match self {
            Impact::Trivial => "Trivial".to_string(),
            Impact::Ordinary => "Ordinary".to_string(),
            Impact::Notable => "Notable".to_string(),
            Impact::Remarkable => "Remarkable".to_string(),
            Impact::Extraordinary => "Extraordinary".to_string(),
        }
    }
}

impl utils::text::Txt for Type {
    fn text(&self) -> String {
        match self {
            Type::Project => "Project".to_string(),
            Type::CollaborationAndMembership => "Collaboration and Membership".to_string(),
            Type::DesignAndDocumentation => "Design and Documentation".to_string(),
            Type::CompanyBuilding => "Company Building".to_string(),
            Type::Learning => "Learning".to_string(),
            Type::OutsideOfWork => "Outside of Work".to_string(),
        }
    }
}

#[derive(Debug)]

struct Inputs {
    title: TextAreaState,
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

#[derive(Debug)]
struct TextAreaState {
    text: String,
    cursor_pos: usize,
}

impl TextAreaState {
    fn empty() -> Self {
        Self {
            text: String::new(),
            cursor_pos: 0,
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
        self.text.insert(index, new_char);
        self.move_cursor_right();
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can be contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    fn byte_index(&self) -> usize {
        self.text
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.cursor_pos)
            .unwrap_or(self.text.len())
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
            let before_char_to_delete = self.text.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.text.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.text = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.text.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.cursor_pos = 0;
    }
}

struct State {
    selecting_field: Option<Field>,
    inputs: Inputs,
    should_quit: bool,
}

impl State {
    fn empty() -> Self {
        Self {
            selecting_field: None,
            inputs: Inputs {
                title: TextAreaState::empty(),
                typ: Type::Project,
                impact: Impact::Trivial,
            },
            should_quit: false,
        }
    }

    fn unselect(&mut self) {
        self.selecting_field = None;
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

pub struct App {
    state: State,
}

impl App {
    pub fn default() -> Self {
        Self {
            state: State::empty(),
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
                            KeyCode::Char('p') => self.on_up(),
                            KeyCode::Char('n') => self.on_down(),
                            KeyCode::Char('h') => self.on_delete(),
                            _ => {}
                        }
                        continue;
                    }

                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Up | KeyCode::Char('k') => self.on_up(),
                            KeyCode::Down | KeyCode::Char('j') => self.on_down(),
                            KeyCode::Left => self.on_left(),
                            KeyCode::Right => self.on_right(),
                            KeyCode::Char('q') => {
                                if self.state.can_quit() {
                                    self.state.quit()
                                } else {
                                    self.on_input('q');
                                }
                            }
                            KeyCode::Esc => self.state.unselect(),
                            KeyCode::Delete | KeyCode::Backspace => self.on_delete(),
                            KeyCode::Char(c) => self.on_input(c),
                            _ => {}
                        }
                        continue;
                    }
                }
            }

            if self.state.should_quit {
                return Ok(());
            }
        }
    }

    fn on_left(&mut self) {
        match self.state.selecting_field {
            Some(Field::Title) => self.state.inputs.title.move_cursor_left(),
            _ => {}
        }
    }

    fn on_right(&mut self) {
        match self.state.selecting_field {
            Some(Field::Title) => self.state.inputs.title.move_cursor_right(),
            _ => {}
        }
    }

    fn on_up(&mut self) {
        match self.state.selecting_field.clone() {
            None => self.state.selecting_field = Some(Field::VARIANTS.last().unwrap().clone()),
            Some(field) => self.state.selecting_field = field.prev(),
        }
    }

    fn on_down(&mut self) {
        match self.state.selecting_field.clone() {
            None => self.state.selecting_field = Some(Field::VARIANTS.first().unwrap().clone()),
            Some(field) => self.state.selecting_field = field.next(),
        }
    }

    fn on_input(&mut self, c: char) {
        match self.state.selecting_field {
            Some(Field::Title) => self.state.inputs.title.enter_char(c),
            _ => {}
        }
    }

    fn on_delete(&mut self) {
        match self.state.selecting_field {
            Some(Field::Title) => {
                self.state.inputs.title.delete_char();
            }
            _ => {}
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

    let title = Paragraph::app_default(app.state.inputs.title.text.as_str())
        .selecting(app.state.selecting_field == Some(Field::Title))
        .block(Block::bordered().title(Field::Title.text()));

    if app.state.selecting_field == Some(Field::Title) {
        frame.set_app_cursor(title_area, app.state.inputs.title.cursor_pos as u16);
    }

    let typ = Paragraph::app_default(app.state.inputs.typ.text())
        .selecting(app.state.selecting_field == Some(Field::Type))
        .block(Block::app_default().title(Field::Type.text()));

    let impact = Paragraph::app_default(app.state.inputs.impact.text())
        .selecting(app.state.selecting_field == Some(Field::Impact))
        .block(Block::app_default().title(Field::Impact.text()));

    frame.render_widget(title, title_area);
    frame.render_widget(typ, typ_area);
    frame.render_widget(impact, impact_area);

    if app.state.selecting_field == Some(Field::Type) {
        let typ_items = Type::VARIANTS
            .iter()
            .map(|t| ListItem::new(vec![text::Line::from(Span::raw(t.text()))]))
            .collect::<Vec<_>>();
        let types = List::new(typ_items).block(Block::popup()).app_highlight();

        frame.render_popup_below_anchor(types, typ_area, None, Some(8));
    }
}
