use std::ops::Index;

use crate::{
    base::{
        autocomplete_text_field::{
            AutocompleteTextField, AutocompleteTextFieldList, AutocompleteTextFieldState,
        },
        loop_list::LoopListState,
        text_field::{TextField, TextFieldState},
    },
    config::colors::COLORS,
    utils::{
        self,
        constant::{TEXT_DOWN_ARROW, TEXT_RIGHT_ARROW},
        text::Txt,
        tui::StatefulDrawer,
    },
};
use domain::brag::{Impact, Type};

use lazy_static::lazy_static;
use ratatui::{
    layout::{Constraint, Layout, Offset, Rect},
    style::{Style, Stylize},
    text::{self, Span},
    widgets::{Block, BorderType, Clear, List, ListItem, Paragraph, StatefulWidget},
    Frame,
};
use regex::Regex;
use strum::{EnumCount, VariantArray};

lazy_static! {
    static ref ORGANIZATIONS: Vec<&'static str> = vec![
        "Google",
        "Facebook",
        "Amazon",
        "Netflix",
        "Apple",
        "Microsoft",
        "Twitter",
        "LinkedIn",
        "Uber",
        "Airbnb",
    ];
}

#[derive(Debug, PartialEq, Eq, Clone, strum::VariantArray, strum::EnumIs)]
pub enum SelectableField {
    Title,
    Type,
    Impact,
    StartDate,
    EndDate,
    Advanced,
    Organization,
    // Skills,
    // Languages,
    // Url,
    // Position,
}

const ADVANCED_FIELDS: &[SelectableField] = &[SelectableField::Organization];

impl SelectableField {
    fn idx(self) -> usize {
        SelectableField::VARIANTS
            .iter()
            .position(|f| *f == self)
            .unwrap()
    }

    fn next(self) -> Option<Self> {
        let index = self.idx();
        if index == SelectableField::VARIANTS.len() - 1 {
            None
        } else {
            Some(SelectableField::VARIANTS[index + 1].clone())
        }
    }

    fn prev(self) -> Option<Self> {
        let index = self.idx();
        if index == 0 {
            None
        } else {
            Some(SelectableField::VARIANTS[index - 1].clone())
        }
    }
}

#[derive(Debug)]
pub struct State {
    pub selecting_field: SelectableField,
    pub is_edit_mode: bool,
    pub is_expand_advanced: bool,

    pub title: TextFieldState,
    pub typ: LoopListState,
    pub impact: LoopListState,
    pub start_date: TextFieldState,
    pub end_date: TextFieldState,

    // TODO(sh0o0): Change 'static to 'a
    pub organization: AutocompleteTextFieldState<'static, &'static str>,

    pub content: String,
}

impl State {
    pub fn default() -> Self {
        let start_date = &mut TextFieldState::default();
        start_date.set_text(chrono::Local::now().naive_local().date().to_string());

        Self {
            selecting_field: SelectableField::VARIANTS.first().unwrap().clone(),
            is_edit_mode: true,
            is_expand_advanced: false,
            title: TextFieldState::default(),
            typ: LoopListState::new(Type::COUNT).with_selected(Some(0)),
            impact: LoopListState::new(Impact::COUNT).with_selected(Some(0)),
            start_date: start_date.clone(),
            end_date: TextFieldState::default(),
            organization: AutocompleteTextFieldState::new(&ORGANIZATIONS, |o, t| {
                o.to_lowercase().contains(&t.to_lowercase())
            }),
            content: "".to_string(),
        }
    }

    pub fn select_next_field(&mut self) {
        self.selecting_field = self
            .selecting_field
            .clone()
            .next()
            .unwrap_or(SelectableField::VARIANTS.first().unwrap().clone());

        if self.should_skip_advanced_fields() {
            self.selecting_field = SelectableField::VARIANTS.first().unwrap().clone();
        }
    }

    pub fn select_previous_field(&mut self) {
        self.selecting_field = self
            .selecting_field
            .clone()
            .prev()
            .unwrap_or(SelectableField::VARIANTS.last().unwrap().clone());

        if self.should_skip_advanced_fields() {
            self.select_advanced_field();
        }
    }

    pub fn toggle_expand_advanced(&mut self) {
        self.is_expand_advanced = !self.is_expand_advanced;

        if self.should_skip_advanced_fields() {
            self.select_advanced_field();
        }
    }

    fn select_advanced_field(&mut self) {
        self.selecting_field = SelectableField::Advanced;
    }

    fn should_skip_advanced_fields(&self) -> bool {
        if self.is_expand_advanced {
            return false;
        }

        ADVANCED_FIELDS.contains(&self.selecting_field)
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
            self.state.selecting_field.clone().title()
        );

        let [title_area, typ_area, impact_area, start_date_area, end_date_area, advanced_area, content_area, commands_area] =
            Layout::vertical([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                match self.state.is_expand_advanced {
                    true => Constraint::Max(10),
                    false => Constraint::Length(1),
                },
                Constraint::Fill(1),
                Constraint::Length(2),
            ])
            .areas(self.frame.area());

        self.render_title(title_area);
        self.render_typ(typ_area);
        self.render_impact(impact_area);
        self.render_start_date(start_date_area);
        self.render_end_date(end_date_area);
        self.render_advanced(advanced_area);
        self.render_content(content_area);
        self.render_commands(commands_area);

        self.render_typ_popup_if_selecting(typ_area);
        self.render_impact_popup_if_selecting(impact_area);
    }

    fn render_title(&mut self, area: Rect) {
        let status = self.status_of(SelectableField::Title);
        let title = TextField::new()
            .block(status.block().title(SelectableField::Title.title()))
            .style(status.style())
            .validator(|text| {
                if text.is_empty() {
                    return Some("Required".to_string());
                }
                if text.len() > 100 {
                    return Some("Too long".to_string());
                }

                None
            });

        title.draw(self.frame, area, &mut self.state.title);
    }

    fn render_typ(&mut self, area: Rect) {
        let typ_text = match self
            .state
            .typ
            .list_state()
            .selected()
            .map(|i| Type::VARIANTS.index(i))
        {
            Some(typ) => typ.text(),
            None => "Select a type".to_string(),
        };

        let status = self.status_of(SelectableField::Type);
        let typ = Paragraph::new(typ_text.app_default())
            .block(status.block().title(SelectableField::Type.title()))
            .style(status.style());

        self.frame.render_widget(typ, area);
    }

    fn render_impact(&mut self, area: Rect) {
        let impact_text = match self
            .state
            .impact
            .list_state()
            .selected()
            .map(|i| Impact::VARIANTS.index(i))
        {
            Some(impact) => impact.text(),
            None => "Select an impact".to_string(),
        };

        let status = self.status_of(SelectableField::Impact);
        let impact = Paragraph::new(impact_text.app_default())
            .block(status.block().title(SelectableField::Impact.title()))
            .style(status.style());

        self.frame.render_widget(impact, area);
    }

    fn render_start_date(&mut self, area: Rect) {
        let status = self.status_of(SelectableField::StartDate);
        let start_date = TextField::new()
            .block(status.block().title(SelectableField::StartDate.title()))
            .style(status.style())
            .helper("2024, 2024-01 or 2024-01-01".into())
            .validator(|text| {
                if text.is_empty() {
                    return Some("Required".to_string());
                }

                if !DATE_REGEX.is_match(&text) {
                    return Some("Invalid date".to_string());
                }

                None
            });

        start_date.draw(self.frame, area, &mut self.state.start_date);
    }

    fn render_end_date(&mut self, area: Rect) {
        let status = self.status_of(SelectableField::EndDate);
        let end_date = TextField::new()
            .block(status.block().title(SelectableField::EndDate.title()))
            .style(status.style())
            .helper("2024, 2024-01 or 2024-01-01".into())
            .validator(|text| {
                if text.is_empty() {
                    return None;
                }
                if !DATE_REGEX.is_match(&text) {
                    return Some("Invalid date".to_string());
                }

                None
            });

        end_date.draw(self.frame, area, &mut self.state.end_date);
    }

    fn render_advanced(&mut self, area: Rect) {
        let style = match self.state.selecting_field {
            SelectableField::Advanced => Style::default().fg(COLORS.primary),
            _ => Style::default(),
        };

        if !self.state.is_expand_advanced {
            let advanced = Paragraph::new(format!(
                "{} {}",
                TEXT_RIGHT_ARROW,
                SelectableField::Advanced.title()
            ))
            .style(style);
            self.frame.render_widget(advanced, area);
            return;
        }

        let [advanced_area, fields_area] =
            Layout::vertical([Constraint::Length(1), Constraint::default()]).areas(area);
        let [_, fields_area] =
            Layout::horizontal([Constraint::Length(1), Constraint::default()]).areas(fields_area);

        let advanced = Paragraph::new(format!(
            "{} {}",
            TEXT_DOWN_ARROW,
            SelectableField::Advanced.title()
        ))
        .style(style);

        self.frame.render_widget(advanced, advanced_area);
        self.render_advanced_fields(fields_area);
    }

    fn render_advanced_fields(&mut self, area: Rect) {
        let [organization_area] = Layout::vertical([Constraint::Max(3)]).areas(area);

        self.render_organization(organization_area);
        self.render_organizations_popup_if_selecting(organization_area);
    }

    fn render_organization(&mut self, area: Rect) {
        let status = self.status_of(SelectableField::Organization);
        let organization = AutocompleteTextField::default()
            .block(status.block().title(SelectableField::Organization.title()))
            .style(status.style());

        organization.draw(self.frame, area, &mut self.state.organization);
    }

    fn render_content(&mut self, area: Rect) {
        let content = Paragraph::new(self.state.content.to_string())
            .block(Status::Displaying.block().title("Content"));

        self.frame.render_widget(content, area);
    }

    fn render_commands(&mut self, area: Rect) {
        let commands = Paragraph::new("Edit Content: CTRL + E | Save: CTRL + S | Cancel: CTRL + C");
        self.frame.render_widget(commands, area);
    }

    fn render_typ_popup_if_selecting(&mut self, typ_area: Rect) {
        if !self.state.selecting_field.is_type() || !self.state.is_edit_mode {
            return;
        }

        let items = Type::VARIANTS
            .iter()
            .map(|t| ListItem::new(vec![text::Line::from(Span::raw(t.text()))]))
            .collect::<Vec<_>>();
        let list = List::new(items).app_highlight().block(Block::popup());

        self.render_stateful_popup_below_anchor(
            list,
            &mut self.state.typ.list_state().clone(),
            typ_area,
            8,
        );
    }

    fn render_impact_popup_if_selecting(&mut self, impact_area: Rect) {
        if !self.state.selecting_field.is_impact() || !self.state.is_edit_mode {
            return;
        }

        let items = Impact::VARIANTS
            .iter()
            .map(|i| ListItem::new(vec![text::Line::from(Span::raw(i.text()))]))
            .collect::<Vec<_>>();

        let list = List::new(items).app_highlight().block(Block::popup());

        self.render_stateful_popup_below_anchor(
            list,
            &mut self.state.impact.list_state().clone(),
            impact_area,
            7,
        );
    }

    fn render_organizations_popup_if_selecting(&mut self, organization_area: Rect) {
        if !self.state.selecting_field.is_organization() || !self.state.is_edit_mode {
            return;
        }

        let list = AutocompleteTextFieldList::new(|o: &&str| {
            ListItem::new(vec![text::Line::from(Span::raw(*o))])
        })
        .block(Block::popup())
        .app_highlight();

        self.render_stateful_popup_below_anchor(
            list,
            &mut self.state.organization.clone(),
            organization_area,
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
        let area = Rect::new(
            anchor.left(),
            anchor.bottom(),
            anchor.width,
            max_height.min(self.frame.area().height),
        );
        let area = area.intersection(self.frame.area());
        let indented_area = area.offset(Offset { x: 1, y: 0 }).intersection(area);

        self.frame.render_widget(Clear, area);
        self.frame
            .render_stateful_widget(widget, indented_area, state);
    }

    fn status_of(&self, field: SelectableField) -> Status {
        match (&self.state.selecting_field, self.state.is_edit_mode) {
            (f, false) if f == &field => Status::Selecting,
            (f, true) if f == &field => Status::Editing,
            _ => Status::Displaying,
        }
    }
}

impl SelectableField {
    fn title(&self) -> String {
        match self {
            SelectableField::Title => "✏️ Title".to_string(),
            SelectableField::Type => "📋 Type".to_string(),
            SelectableField::Impact => "🌟 Impact".to_string(),
            SelectableField::StartDate => "💨 Start Date".to_string(),
            SelectableField::EndDate => "🏁 End Date".to_string(),
            SelectableField::Advanced => "Advanced".to_string(),
            SelectableField::Organization => "🏢 Organization".to_string(),
        }
    }
}

enum Status {
    Displaying,
    Selecting,
    Editing,
}

impl Status {
    fn block(&self) -> Block<'static> {
        match self {
            Status::Displaying => Block::bordered(),
            Status::Selecting => Block::bordered().border_type(BorderType::Double),
            Status::Editing => Block::bordered().fg(COLORS.primary),
        }
    }

    fn style(&self) -> Style {
        match self {
            Status::Displaying => Style::default(),
            Status::Selecting => Style::default(),
            Status::Editing => Style::default().fg(COLORS.primary),
        }
    }
}

trait StringExt {
    fn app_default(self) -> Span<'static>;
}

impl StringExt for String {
    fn app_default(self) -> Span<'static> {
        self.gray()
    }
}

trait BlockExt<'a> {
    fn popup() -> Block<'a> {
        Block::bordered().border_type(BorderType::Double)
    }
}

impl<'a> BlockExt<'a> for Block<'a> {}

trait ListExt {
    fn app_highlight(self) -> Self;
}

impl<'a> ListExt for List<'a> {
    fn app_highlight(self) -> Self {
        self.highlight_style(Style::default().bold())
            .highlight_symbol("> ")
    }
}

impl<T> AutocompleteTextFieldList<'_, T> {
    fn app_highlight(self) -> Self {
        self.highlight_style(Style::default().bold())
            .highlight_symbol("> ")
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

lazy_static! {
    static ref DATE_REGEX: Regex = Regex::new(r"^\d{4}(-\d{2}(-\d{2})?)?$").unwrap();
}
