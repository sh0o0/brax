use crate::{
    base::text_field::{Mode, TextFieldState},
    screen::new::{Field, Screen, State},
};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{prelude::Backend, Terminal};
use std::time::{Duration, Instant};
use strum::VariantArray;

pub struct App {
    should_quit: bool,
    state: State,
}

impl App {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            state: State::default(),
        }
    }

    pub fn run<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        tick_rate: Duration,
    ) -> util::error::Result<()> {
        let last_tick = Instant::now();

        while !self.should_quit {
            terminal.draw(|frame| {
                Screen::new(frame, &mut self.state).render();
            })?;

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
            // if key.modifiers == event::KeyModifiers::CONTROL {
            //     match key.code {
            //         KeyCode::Char('p') => self.on_up(),
            //         KeyCode::Char('n') => self.on_down(),
            //         KeyCode::Char('h') => self.on_delete(),
            //         _ => {}
            //     }
            //     return Ok(());
            // }

            if key.kind == KeyEventKind::Press {
                match key.code {
                    // KeyCode::Up | KeyCode::Char('k') => self.on_up(),
                    // KeyCode::Down | KeyCode::Char('j') => self.on_down(),
                    // KeyCode::Left => self.on_left(),
                    // KeyCode::Right => self.on_right(),
                    KeyCode::Char('q') => self.quit(),
                    // KeyCode::Esc => self.state.unselect(),
                    // KeyCode::Delete | KeyCode::Backspace => self.on_delete(),
                    // KeyCode::Char(c) => self.on_input(c),
                    _ => {}
                }
                return Ok(());
            }
        }

        Ok(())
    }

    fn quit(&mut self) {
        self.should_quit = true;
    }

    // fn on_left(&mut self) {
    //     match self.state.selecting_field {
    //         Some(Field::Title) => self.state.title.move_cursor_left(),
    //         _ => {}
    //     }
    // }

    // fn on_right(&mut self) {
    //     match self.state.selecting_field {
    //         Some(Field::Title) => self.state.title.move_cursor_right(),
    //         _ => {}
    //     }
    // }

    // fn on_up(&mut self) {
    //     match self.state.selecting_field.clone() {
    //         None => self.state.selecting_field = Some(Field::VARIANTS.last().unwrap().clone()),
    //         Some(field) => self.state.selecting_field = field.prev(),
    //     }
    //     self.on_selecting_field_changed();
    // }

    // fn on_down(&mut self) {
    //     match self.state.selecting_field.clone() {
    //         None => self.state.selecting_field = Some(Field::VARIANTS.first().unwrap().clone()),
    //         Some(field) => self.state.selecting_field = field.next(),
    //     }
    //     self.on_selecting_field_changed();
    // }

    // fn on_input(&mut self, c: char) {
    //     match self.state.selecting_field {
    //         Some(Field::Title) => self.state.title.enter_char(c),
    //         _ => {}
    //     }
    // }

    // fn on_delete(&mut self) {
    //     match self.state.selecting_field {
    //         Some(Field::Title) => {
    //             self.state.title.delete_char();
    //         }
    //         _ => {}
    //     }
    // }

    // fn on_selecting_field_changed(&mut self) {
    //     match self.state.selecting_field {
    //         Some(Field::Title) => self.state.title.change_mode(Mode::Edit),
    //         _ => self.state.title.change_mode(Mode::Display),
    //     }
    // }
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
