use domain::brag::Type;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{self, Span},
    widgets::{Block, List, ListItem, ListState, StatefulWidget, StatefulWidgetRef},
};
use strum::VariantArray;

use crate::{
    base::list::AppList,
    utils::{self, text::Txt},
};

#[derive(Debug)]
pub struct TypeListState {
    pub offset: usize,
    pub selected: Option<usize>,
}

impl TypeListState {
    pub fn default() -> Self {
        Self {
            offset: 0,
            selected: None,
        }
    }

    pub fn with_selected(mut self, selected: Option<usize>) -> Self {
        self.selected = selected;
        self
    }

    pub fn select_next(&mut self) {
        // Type::VARIANTS

        match self.selected {
            None => self.selected = Some(0),
            Some(selected) => {
                if selected < Type::VARIANTS.len() - 1 {
                    self.selected = Some(selected + 1);
                } else {
                    self.selected = Some(0);
                }
            }
        }
    }

    pub fn select_previous(&mut self) {
        match self.selected {
            None => self.selected = Some(Type::VARIANTS.len() - 1),
            Some(selected) => {
                if selected > 0 {
                    self.selected = Some(selected - 1);
                } else {
                    self.selected = Some(Type::VARIANTS.len() - 1);
                }
            }
        }
    }

    pub fn selected_type(&self) -> Option<Type> {
        self.selected.map(|i| Type::VARIANTS[i].clone())
    }
}

impl Into<ListState> for &mut TypeListState {
    fn into(self) -> ListState {
        ListState::default()
            .with_offset(self.offset)
            .with_selected(self.selected)
    }
}

pub struct TypeList<'a> {
    block: Option<Block<'a>>,
}

impl<'a> TypeList<'a> {
    pub fn new() -> Self {
        Self { block: None }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}

impl<'a> StatefulWidget for TypeList<'a> {
    type State = TypeListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.render_ref(area, buf, state);
    }
}

impl<'a> StatefulWidgetRef for TypeList<'a> {
    type State = TypeListState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let typ_items = Type::VARIANTS
            .iter()
            .map(|t| ListItem::new(vec![text::Line::from(Span::raw(t.text()))]))
            .collect::<Vec<_>>();
        let mut types = List::new(typ_items).app_highlight();

        if let Some(block) = &self.block {
            types = types.block(block.clone());
        }

        types.render(area, buf, &mut state.into());
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
