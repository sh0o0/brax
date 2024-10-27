use crate::{
    base::{block::AppBlock, frame::AppFrame, list::AppList, paragraph::AppParagraph},
    case::text_field::{Mode, TextField, TextFieldController, TextFieldFrame},
    utils::{self, text::Txt},
};

use domain::brag::{Impact, Type};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    prelude::Backend,
    text::{self, Span},
    widgets::{Block, List, ListItem, Paragraph},
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
    title_controller: TextFieldController,
}

impl App {
    pub fn default() -> Self {
        Self {
            state: State::empty(),
            title_controller: TextFieldController::default(),
        }
    }

    pub fn run<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        tick_rate: Duration,
    ) -> util::error::Result<()> {
        let last_tick = Instant::now();

        while !self.state.should_quit {
            terminal.draw(|frame| Screen::new(frame, self).render())?;
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            self.handle_event(timeout)?;
        }

        Ok(())
    }

    fn handle_event(&mut self, timeout: Duration) -> util::error::Result<()> {
        if !event::poll(timeout)? {
            return Ok(());
        }

        if let Event::Key(key) = event::read()? {
            if key.modifiers == event::KeyModifiers::CONTROL {
                match key.code {
                    KeyCode::Char('p') => self.on_up(),
                    KeyCode::Char('n') => self.on_down(),
                    KeyCode::Char('h') => self.on_delete(),
                    _ => {}
                }
                return Ok(());
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
                return Ok(());
            }
        }

        Ok(())
    }

    fn on_left(&mut self) {
        match self.state.selecting_field {
            Some(Field::Title) => self.title_controller.move_cursor_left(),
            _ => {}
        }
    }

    fn on_right(&mut self) {
        match self.state.selecting_field {
            Some(Field::Title) => self.title_controller.move_cursor_right(),
            _ => {}
        }
    }

    fn on_up(&mut self) {
        match self.state.selecting_field.clone() {
            None => self.state.selecting_field = Some(Field::VARIANTS.last().unwrap().clone()),
            Some(field) => self.state.selecting_field = field.prev(),
        }
        self.on_selecting_field_changed();
    }

    fn on_down(&mut self) {
        match self.state.selecting_field.clone() {
            None => self.state.selecting_field = Some(Field::VARIANTS.first().unwrap().clone()),
            Some(field) => self.state.selecting_field = field.next(),
        }
        self.on_selecting_field_changed();
    }

    fn on_input(&mut self, c: char) {
        match self.state.selecting_field {
            Some(Field::Title) => self.title_controller.enter_char(c),
            _ => {}
        }
    }

    fn on_delete(&mut self) {
        match self.state.selecting_field {
            Some(Field::Title) => {
                self.title_controller.delete_char();
            }
            _ => {}
        }
    }

    fn on_selecting_field_changed(&mut self) {
        match self.state.selecting_field {
            Some(Field::Title) => self.title_controller.mode(Mode::Edit),
            _ => self.title_controller.mode(Mode::Display),
        }
    }
}

struct Screen<'a, 'b> {
    frame: &'a mut Frame<'b>,
    app: &'a App,
}

impl<'a, 'b> Screen<'a, 'b> {
    pub fn new(frame: &'a mut Frame<'b>, app: &'a App) -> Self {
        Self { frame, app }
    }

    fn render(&mut self) {
        let [title_area, typ_area, impact_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .areas(self.frame.area());

        self.render_title(title_area);
        self.render_typ(typ_area);
        self.render_impact(impact_area);
        self.render_typ_popup_if_selecting(typ_area);
    }

    fn render_title(&mut self, area: Rect) {
        let title = TextField::new(&self.app.title_controller)
            .block(Block::bordered().title(Field::Title.text()));

        self.frame.render_text_field(title, area);
    }

    fn render_typ(&mut self, area: Rect) {
        let typ = Paragraph::app_default(self.app.state.inputs.typ.text())
            .block(Block::bordered().title(Field::Type.text()));

        self.frame.render_widget(typ, area);
    }

    fn render_impact(&mut self, area: Rect) {
        let impact = Paragraph::app_default(self.app.state.inputs.impact.text())
            .block(Block::bordered().title(Field::Impact.text()));

        self.frame.render_widget(impact, area);
    }

    fn render_typ_popup_if_selecting(&mut self, typ_area: Rect) {
        if self.app.state.selecting_field == Some(Field::Type) {
            let typ_items = Type::VARIANTS
                .iter()
                .map(|t| ListItem::new(vec![text::Line::from(Span::raw(t.text()))]))
                .collect::<Vec<_>>();
            let types = List::new(typ_items).block(Block::popup()).app_highlight();

            self.frame
                .render_popup_below_anchor(types, typ_area, None, Some(8));
        }
    }
}
