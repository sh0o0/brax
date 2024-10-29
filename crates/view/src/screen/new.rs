use std::fmt::Display;

use crate::{
    base::{
        block::AppBlock,
        paragraph::AppParagraph,
        text_field::{Mode, TextField, TextFieldFrame, TextFieldState},
    },
    case::type_list::{TypeList, TypeListState},
    utils::{self, text::Txt},
};
use domain::brag::Impact;

use ratatui::{
    layout::{Constraint, Layout, Rect},
    widgets::{Block, BorderType, Clear, Paragraph},
    Frame,
};
use strum::{Display, VariantArray};

#[derive(Debug, PartialEq, Eq, Clone, strum::VariantArray)]
pub enum Field {
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

#[derive(Debug)]
pub struct State {
    pub selecting_field: Option<Field>,
    pub title: TextFieldState,
    pub typ: TypeListState,
}

impl<'a> State {
    pub fn default() -> Self {
        Self {
            selecting_field: None,
            title: TextFieldState::default(),
            typ: TypeListState::default().with_selected(Some(0)),
        }
    }

    pub fn select_next_field(&mut self) {
        match &self.selecting_field {
            None => self.selecting_field = Some(Field::VARIANTS.first().unwrap().clone()),
            Some(field) => self.selecting_field = field.clone().next(),
        }
    }

    pub fn select_previous_field(&mut self) {
        match &self.selecting_field {
            None => self.selecting_field = Some(Field::VARIANTS.last().unwrap().clone()),
            Some(field) => self.selecting_field = field.clone().prev(),
        }
    }

    pub fn unselect(&mut self) {
        self.selecting_field = None;
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
            self.state
                .selecting_field
                .clone()
                .map(|f| f.text())
                .unwrap_or("None".to_string())
        );

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
        let title = TextField::new()
            .block(Block::bordered().title(Field::Title.text()))
            .mode(match self.state.selecting_field {
                Some(Field::Title) => Mode::Edit,
                _ => Mode::Display,
            });

        self.frame
            .render_text_field(title, area, &mut self.state.title);
    }

    fn render_typ(&mut self, area: Rect) {
        let typ_text = match self.state.typ.selected_type() {
            Some(typ) => typ.text(),
            None => "Select a type".to_string(),
        };

        let typ =
            Paragraph::app_default(typ_text).block(Block::bordered().title(Field::Type.text()));

        self.frame.render_widget(typ, area);
    }

    fn render_impact(&mut self, area: Rect) {
        let impact =
            Paragraph::app_default("yyy").block(Block::bordered().title(Field::Impact.text()));

        self.frame.render_widget(impact, area);
    }

    fn render_typ_popup_if_selecting(&mut self, typ_area: Rect) {
        if self.state.selecting_field != Some(Field::Type) {
            return;
        }

        let typ_list = TypeList::new().block(Block::app_default().border_type(BorderType::Double));

        let popup_area = Rect {
            x: typ_area.left(),
            y: typ_area.bottom(),
            width: typ_area.width,
            height: 8.min(self.frame.area().height),
        };
        let intersection = popup_area.intersection(self.frame.area());
        self.frame.render_widget(Clear, intersection);
        self.frame
            .render_stateful_widget(typ_list, intersection, &mut self.state.typ);
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
