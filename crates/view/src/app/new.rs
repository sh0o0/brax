use crate::{
    base::frame,
    screen::new::{Screen, SelectableField, State},
    utils,
};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use domain::error::{Error, Result};
use ratatui::{backend, prelude::Backend, Terminal};
use std::time::{Duration, Instant};

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
            self.handle_event(timeout, terminal)?;
        }

        Ok(())
    }

    fn handle_event<B: Backend>(
        &mut self,
        timeout: Duration,
        terminal: &mut Terminal<B>,
    ) -> util::error::Result<()> {
        if !event::poll(timeout)? {
            return Ok(());
        }

        if let Event::Key(key) = event::read()? {
            if key.modifiers == event::KeyModifiers::CONTROL {
                match key.code {
                    KeyCode::Char('p') => self.on_up(),
                    KeyCode::Char('n') => self.on_down(),
                    KeyCode::Char('h') => self.on_delete(),
                    KeyCode::Char('b') => self.on_left(),
                    KeyCode::Char('f') => self.on_right(),
                    KeyCode::Char('c') => self.quit(),
                    KeyCode::Char('e') => self.edit_content(terminal)?,
                    _ => {}
                }
                return Ok(());
            }

            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Tab => self.on_tab(),
                    KeyCode::BackTab => self.on_back_tab(),
                    KeyCode::Up => self.on_up(),
                    KeyCode::Down => self.on_down(),
                    KeyCode::Left => self.on_left(),
                    KeyCode::Right => self.on_right(),
                    KeyCode::Delete | KeyCode::Backspace => self.on_delete(),
                    KeyCode::Enter => self.on_enter(),
                    KeyCode::Char('k') => self.on_k(),
                    KeyCode::Char('j') => self.on_j(),
                    KeyCode::Char(c) => self.on_char(c),
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

    fn edit_content<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> util::error::Result<()> {
        let result = dialoguer::Editor::new().edit(&self.state.content)?;
        result.map(|content| self.state.content = content);

        terminal.clear()?;
        Ok(())
    }

    fn on_enter(&mut self) {
        match self.state.selecting_field {
            SelectableField::Advanced => self.state.toggle_expand_advanced(),
            _ => self.state.select_next_field(),
        }
    }

    fn on_tab(&mut self) {
        self.state.select_next_field();
    }

    fn on_back_tab(&mut self) {
        self.state.select_previous_field();
    }

    fn on_left(&mut self) {
        match self.state.selecting_field {
            SelectableField::Title => self.state.title.move_cursor_left(),
            SelectableField::StartDate => self.state.start_date.move_cursor_left(),
            SelectableField::EndDate => self.state.end_date.move_cursor_left(),
            _ => {}
        }
    }

    fn on_right(&mut self) {
        match self.state.selecting_field {
            SelectableField::Title => self.state.title.move_cursor_right(),
            SelectableField::StartDate => self.state.start_date.move_cursor_right(),
            SelectableField::EndDate => self.state.end_date.move_cursor_right(),
            _ => {}
        }
    }

    fn on_up(&mut self) {
        match self.state.selecting_field {
            SelectableField::Type => self.state.typ.select_previous(),
            SelectableField::Impact => self.state.impact.select_previous(),
            _ => self.state.select_previous_field(),
        }
    }

    fn on_down(&mut self) {
        match self.state.selecting_field {
            SelectableField::Type => self.state.typ.select_next(),
            SelectableField::Impact => self.state.impact.select_next(),
            _ => self.state.select_next_field(),
        }
    }

    fn on_char(&mut self, c: char) {
        match self.state.selecting_field {
            SelectableField::Title => self.state.title.enter_char(c),
            SelectableField::StartDate => self.state.start_date.enter_char(c),
            SelectableField::EndDate => self.state.end_date.enter_char(c),
            _ => {}
        }
    }

    fn on_delete(&mut self) {
        match self.state.selecting_field {
            SelectableField::Title => self.state.title.delete_char(),
            SelectableField::StartDate => self.state.start_date.delete_char(),
            SelectableField::EndDate => self.state.end_date.delete_char(),
            _ => {}
        }
    }

    fn on_k(&mut self) {
        const K: char = 'k';

        match self.state.selecting_field {
            SelectableField::Type | SelectableField::Impact => self.on_up(),
            _ => self.on_char(K),
        }
    }

    fn on_j(&mut self) {
        const J: char = 'j';

        match self.state.selecting_field {
            SelectableField::Type | SelectableField::Impact => self.on_down(),
            _ => self.on_char(J),
        }
    }
}
