use std::ops::Index;

use crate::{
    base::{
        frame::AppFrame,
        loop_list::{LoopList, LoopListState},
        text_field::{Mode, TextField, TextFieldFrame, TextFieldState},
    },
    utils::{self, text::Txt},
};
use domain::brag::{Impact, Type};

use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::{self, Span, Text},
    widgets::{Block, BorderType, ListItem, Paragraph, StatefulWidget},
    Frame,
};
use regex::Regex;
use strum::{EnumCount, VariantArray};

#[derive(Debug, PartialEq, Eq, Clone, strum::VariantArray)]
pub enum Field {
    Title,
    Type,
    Impact,
    StartDate,
    EndDate,
    // Organization,
    // Skills,
    // Languages,
    // Url,
    // Position,
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

#[derive(Debug)]
pub struct State {
    pub selecting_field: Field,
    pub title: TextFieldState,
    pub typ: LoopListState,
    pub impact: LoopListState,
    pub start_date: TextFieldState,
    pub end_date: TextFieldState,
    pub content: String,
}

impl<'a> State {
    pub fn default() -> Self {
        let start_date = &mut TextFieldState::default();
        start_date.set_text(chrono::Local::now().naive_local().date().to_string());

        Self {
            selecting_field: Field::VARIANTS.first().unwrap().clone(),
            title: TextFieldState::default(),
            typ: LoopListState::default(Type::COUNT).with_selected(Some(0)),
            impact: LoopListState::default(Impact::COUNT).with_selected(Some(0)),
            start_date: start_date.clone(),
            end_date: TextFieldState::default(),
            content: "".to_string(),
        }
    }

    pub fn select_next_field(&mut self) {
        self.selecting_field = self
            .selecting_field
            .clone()
            .next()
            .unwrap_or(Field::VARIANTS.first().unwrap().clone());
    }

    pub fn select_previous_field(&mut self) {
        self.selecting_field = self
            .selecting_field
            .clone()
            .prev()
            .unwrap_or(Field::VARIANTS.last().unwrap().clone());
    }
}

pub struct Screen<'a, 'b> {
    frame: &'a mut Frame<'b>,
    state: &'a mut State,
}

impl<'a, 'b> Screen<'a, 'b> {
    pub fn new(frame: &'a mut Frame<'b>, state: &'a mut State) -> Self {
        Self { frame, state }
    }

    pub fn render(&mut self) {
        log::info!(
            "Rendering screen, {}",
            self.state.selecting_field.clone().text()
        );

        let [title_area, typ_area, impact_area, start_date_area, end_date_area, content_area, commands_area] =
            Layout::vertical([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Fill(1),
                Constraint::Length(2),
            ])
            .areas(self.frame.area());

        self.render_title(title_area);
        self.render_typ(typ_area);
        self.render_impact(impact_area);
        self.render_start_date(start_date_area);
        self.render_end_date(end_date_area);
        self.render_content(content_area);
        self.render_commands(commands_area);

        self.render_typ_popup_if_selecting(typ_area);
        self.render_impact_popup_if_selecting(impact_area);
    }

    fn render_title(&mut self, area: Rect) {
        let title = TextField::new()
            .block(Block::bordered().title(Field::Title.text()))
            .mode(match self.state.selecting_field {
                Field::Title => Mode::Edit,
                _ => Mode::Display,
            })
            .validator(|text| {
                if text.is_empty() {
                    return Some("Required".to_string());
                }
                if text.len() > 100 {
                    return Some("Too long".to_string());
                }

                None
            });

        self.frame
            .render_text_field(title, area, &mut self.state.title);
    }

    fn render_typ(&mut self, area: Rect) {
        let typ_text = match self.state.typ.selected.map(|i| Type::VARIANTS.index(i)) {
            Some(typ) => typ.text(),
            None => "Select a type".to_string(),
        };

        let typ = Paragraph::app_default(typ_text)
            .block(Block::bordered().title(Field::Type.text()))
            .style(match self.state.selecting_field {
                Field::Type => Style::default().bold(),
                _ => Style::default().gray(),
            });

        self.frame.render_widget(typ, area);
    }

    fn render_impact(&mut self, area: Rect) {
        let impact_text = match self
            .state
            .impact
            .selected
            .map(|i| Impact::VARIANTS.index(i))
        {
            Some(impact) => impact.text(),
            None => "Select an impact".to_string(),
        };
        let impact = Paragraph::app_default(impact_text)
            .block(Block::bordered().title(Field::Impact.text()))
            .style(match self.state.selecting_field {
                Field::Impact => Style::default().bold(),
                _ => Style::default().gray(),
            });

        self.frame.render_widget(impact, area);
    }

    fn render_start_date(&mut self, area: Rect) {
        let start_date = TextField::new()
            .block(Block::bordered().title(Field::StartDate.text()))
            .mode(match self.state.selecting_field {
                Field::StartDate => Mode::Edit,
                _ => Mode::Display,
            })
            .helper("2024, 2024-01, 2024-01-01".into())
            .validator(|text| {
                if text.is_empty() {
                    return Some("Required".to_string());
                }

                if !DATE_REGEX.is_match(&text) {
                    return Some("Invalid date".to_string());
                }

                None
            });

        self.frame
            .render_text_field(start_date, area, &mut self.state.start_date);
    }

    fn render_end_date(&mut self, area: Rect) {
        let end_date = TextField::new()
            .block(Block::bordered().title(Field::EndDate.text()))
            .mode(match self.state.selecting_field {
                Field::EndDate => Mode::Edit,
                _ => Mode::Display,
            })
            .helper("2024, 2024-01, 2024-01-01".into())
            .validator(|text| {
                if text.is_empty() {
                    return None;
                }
                if !DATE_REGEX.is_match(&text) {
                    return Some("Invalid date".to_string());
                }

                None
            });

        self.frame
            .render_text_field(end_date, area, &mut self.state.end_date);
    }

    fn render_content(&mut self, area: Rect) {
        let content = Paragraph::new(self.state.content.to_string()).block(Block::bordered());

        self.frame.render_widget(content, area);
    }

    fn render_commands(&mut self, area: Rect) {
        let commands =
            Paragraph::app_default("Edit Content: CTRL + E | Save: CTRL + S | Cancel: CTRL + C");
        self.frame.render_widget(commands, area);
    }

    fn render_typ_popup_if_selecting(&mut self, typ_area: Rect) {
        if self.state.selecting_field != Field::Type {
            return;
        }

        let items = Type::VARIANTS
            .iter()
            .map(|t| ListItem::new(vec![text::Line::from(Span::raw(t.text()))]))
            .collect::<Vec<_>>();
        let list = LoopList::new(items)
            .highlight_style(Style::default().bold())
            .highlight_symbol("> ")
            .block(Block::bordered().border_type(BorderType::Double));

        self.render_stateful_popup_below_anchor(list, &mut self.state.typ.clone(), typ_area, 8);
    }

    fn render_impact_popup_if_selecting(&mut self, impact_area: Rect) {
        if self.state.selecting_field != Field::Impact {
            return;
        }

        let items = Impact::VARIANTS
            .iter()
            .map(|i| ListItem::new(vec![text::Line::from(Span::raw(i.text()))]))
            .collect::<Vec<_>>();

        let list = LoopList::new(items)
            .highlight_style(Style::default().bold())
            .highlight_symbol("> ")
            .block(Block::bordered().border_type(BorderType::Double));

        self.render_stateful_popup_below_anchor(
            list,
            &mut self.state.impact.clone(),
            impact_area,
            7,
        );
    }

    fn render_stateful_popup_below_anchor<W: StatefulWidget>(
        &mut self,
        widget: W,
        state: &mut W::State,
        anchor: Rect,
        max_height: u16,
    ) {
        let area = Rect {
            x: anchor.left(),
            y: anchor.bottom(),
            width: anchor.width,
            height: max_height.min(self.frame.area().height),
        };
        let area = area.intersection(self.frame.area());

        self.frame.render_stateful_popup(widget, area, state);
    }
}

impl utils::text::Txt for Field {
    fn text(&self) -> String {
        match self {
            Field::Title => "Title".to_string(),
            Field::Type => "Type".to_string(),
            Field::Impact => "Impact".to_string(),
            Field::StartDate => "Start Date".to_string(),
            Field::EndDate => "End Date".to_string(),
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

trait BlockExt<'a> {
    fn app_default() -> Block<'a> {
        Block::bordered()
    }
    fn popup() -> Block<'a> {
        Block::bordered().border_type(BorderType::Double)
    }
}

impl<'a> BlockExt<'a> for Block<'a> {}

trait ParagraphExt<'a> {
    fn app_default<T>(text: T) -> Paragraph<'a>
    where
        T: Into<Text<'a>>,
    {
        Paragraph::new(text)
    }
}

impl<'a> ParagraphExt<'a> for Paragraph<'a> {}

lazy_static::lazy_static! {
    static ref DATE_REGEX: Regex = Regex::new(r"^\d{4}(-\d{2}(-\d{2})?)?$").unwrap();
}
