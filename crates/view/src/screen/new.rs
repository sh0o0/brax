use crate::{
    app::new::App,
    base::{block::AppBlock, frame::AppFrame, list::AppList, paragraph::AppParagraph},
    case::text_field::{TextField, TextFieldFrame},
    model::new::Field,
    utils::{self, text::Txt},
};

use domain::brag::{Impact, Type};

use ratatui::{
    layout::{Constraint, Layout, Rect},
    text::{self, Span},
    widgets::{Block, List, ListItem, Paragraph},
    Frame,
};
use strum::VariantArray;

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

pub struct Screen<'a, 'b> {
    frame: &'a mut Frame<'b>,
    app: &'a App,
}

impl<'a, 'b> Screen<'a, 'b> {
    pub fn new(frame: &'a mut Frame<'b>, app: &'a App) -> Self {
        Self { frame, app }
    }

    pub fn render(&mut self) {
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
