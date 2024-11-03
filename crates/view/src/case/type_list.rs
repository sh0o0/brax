use domain::brag::Type;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{self, Span},
    widgets::{Block, List, ListItem, ListState, StatefulWidget, StatefulWidgetRef, Widget},
};
use strum::VariantArray;

use crate::{
    base::{
        list::AppList,
        loop_list::{LoopList, LoopListState},
    },
    utils::{self, text::Txt},
};

pub struct TypeList<'a> {
    loop_list: LoopList<'a>,
}

impl<'a> TypeList<'a> {
    pub fn default<T>(items: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<ListItem<'a>>,
    {
        Self {
            loop_list: LoopList::new(items),
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.loop_list = self.loop_list.block(block);
        self
    }
}

impl<'a> StatefulWidget for TypeList<'a> {
    type State = LoopListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.render_ref(area, buf, state);
    }
}

impl<'a> StatefulWidgetRef for TypeList<'a> {
    type State = LoopListState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {}
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
