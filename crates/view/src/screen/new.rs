use std::fmt::Display;

use crate::{
    base::{
        block::AppBlock,
        frame::AppFrame,
        paragraph::AppParagraph,
        text_field::{Mode, TextField, TextFieldFrame, TextFieldState},
    },
    case::type_list::{self, TypeList, TypeListState},
    utils::{self, text::Txt},
};
use domain::brag::Impact;

use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, BorderType, Clear, Paragraph, StatefulWidget},
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
    pub selecting_field: Field,
    pub title: TextFieldState,
    pub typ: TypeListState,
}

impl<'a> State {
    pub fn default() -> Self {
        Self {
            selecting_field: Field::VARIANTS.first().unwrap().clone(),
            title: TextFieldState::default(),
            typ: TypeListState::default().with_selected(Some(0)),
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
                Field::Title => Mode::Edit,
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

        let typ = Paragraph::app_default(typ_text)
            .block(Block::bordered().title(Field::Type.text()))
            .style(match self.state.selecting_field {
                Field::Type => Style::default().bold(),
                _ => Style::default().dark_gray(),
            });

        self.frame.render_widget(typ, area);
    }

    fn render_impact(&mut self, area: Rect) {
        let impact = Paragraph::app_default("yyy")
            .block(Block::bordered().title(Field::Impact.text()))
            .style(match self.state.selecting_field {
                Field::Impact => Style::default().bold(),
                _ => Style::default().dark_gray(),
            });

        self.frame.render_widget(impact, area);
    }

    fn render_typ_popup_if_selecting(&mut self, typ_area: Rect) {
        if self.state.selecting_field != Field::Type {
            return;
        }

        let typ_list = TypeList::new().block(Block::app_default().border_type(BorderType::Double));

        self.render_stateful_popup_below_anchor(typ_list, &mut self.state.typ.clone(), typ_area);
    }

    // fn render_impact_popup_if_selecting(&mut self, impact_area: Rect) {
    //     if self.state.selecting_field != Field::Type {
    //         return;
    //     }

    //     let typ_list = TypeList::new().block(Block::app_default().border_type(BorderType::Double));

    //     self.render_stateful_popup_below_anchor(typ_list, &mut self.state.typ.clone(), typ_area);
    // }

    fn render_stateful_popup_below_anchor<W: StatefulWidget>(
        &mut self,
        widget: W,
        state: &mut W::State,
        anchor: Rect,
    ) {
        let area = Rect {
            x: anchor.left(),
            y: anchor.bottom(),
            width: anchor.width,
            height: 8.min(self.frame.area().height),
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
