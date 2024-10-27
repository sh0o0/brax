use domain::brag::{Impact, Type};

use strum::VariantArray;

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
    pub fn idx(self) -> usize {
        Field::VARIANTS.iter().position(|f| *f == self).unwrap()
    }

    pub fn next(self) -> Option<Self> {
        let index = self.idx();
        if index == Field::VARIANTS.len() - 1 {
            None
        } else {
            Some(Field::VARIANTS[index + 1].clone())
        }
    }

    pub fn prev(self) -> Option<Self> {
        let index = self.idx();
        if index == 0 {
            None
        } else {
            Some(Field::VARIANTS[index - 1].clone())
        }
    }
}

#[derive(Debug)]
pub struct Inputs {
    pub typ: Type,
    pub impact: Impact,
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

pub struct State {
    pub selecting_field: Option<Field>,
    pub inputs: Inputs,
    pub should_quit: bool,
}

impl State {
    pub fn empty() -> Self {
        Self {
            selecting_field: None,
            inputs: Inputs {
                typ: Type::Project,
                impact: Impact::Trivial,
            },
            should_quit: false,
        }
    }

    pub fn unselect(&mut self) {
        self.selecting_field = None;
    }

    pub fn can_quit(&self) -> bool {
        self.selecting_field == None
    }

    pub fn quit(&mut self) {
        if self.can_quit() {
            self.should_quit = true;
        }
    }
}
