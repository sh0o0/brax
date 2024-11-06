use crate::{
    handler::handler::KeyEventHandler,
    screen::new::{Screen, SelectableField, State},
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{prelude::Backend, Terminal};
use std::time::{Duration, Instant};

pub struct App {
    should_quit: bool,
    state: State,
    should_clear_terminal: bool,
}

impl App {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            state: State::default(),
            should_clear_terminal: false,
        }
    }

    pub fn run<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        tick_rate: Duration,
    ) -> util::error::Result<()> {
        let last_tick = Instant::now();

        while !self.should_quit {
            if self.should_clear_terminal {
                terminal.clear()?;
                self.should_clear_terminal = false;
            }

            self.before_render();

            terminal.draw(|frame| {
                log::info!("{:?}", &self.state);
                Screen::new(frame, &mut self.state).render();
            })?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    self.handle_key_event(&key)?;
                }
            }
        }

        Ok(())
    }

    fn quit(&mut self) {
        self.should_quit = true;
    }

    fn edit_content(&mut self) -> util::error::Result<()> {
        self.should_clear_terminal = true;

        let result = dialoguer::Editor::new().edit(&self.state.content)?;
        result.map(|content| self.state.content = content);
        Ok(())
    }

    fn before_render(&mut self) {
        self.state
            .title
            .set_is_editing(self.state.selecting_field == SelectableField::Title);
        self.state
            .start_date
            .set_is_editing(self.state.selecting_field == SelectableField::StartDate);
        self.state
            .end_date
            .set_is_editing(self.state.selecting_field == SelectableField::EndDate);
        self.state
            .organization
            .set_is_editing(self.state.selecting_field == SelectableField::Organization);
    }
}

impl KeyEventHandler for App {
    fn handle_key_event(&mut self, key: &KeyEvent) -> util::error::Result<()> {
        if self.may_intercept_key_event(key)? {
            return Ok(());
        }

        if !self.state.is_edit_mode {
            if key.modifiers == event::KeyModifiers::CONTROL {
                match key.code {
                    KeyCode::Char('c') => self.quit(),
                    KeyCode::Char('o') => self.edit_content()?,
                    _ => {}
                }
                return Ok(());
            }

            match key.code {
                KeyCode::Up => self.state.select_previous_field(),
                KeyCode::Down => self.state.select_next_field(),
                KeyCode::Enter => self.state.mode_edit(),
                KeyCode::Char('k') => self.state.select_previous_field(),
                KeyCode::Char('j') => self.state.select_next_field(),
                _ => {}
            }

            return Ok(());
        }

        match self.state.selecting_field {
            SelectableField::Title => self.state.title.handle_key_event(key)?,
            SelectableField::Type => self.state.typ.handle_key_event(key)?,
            SelectableField::Impact => self.state.impact.handle_key_event(key)?,
            SelectableField::StartDate => self.state.start_date.handle_key_event(key)?,
            SelectableField::EndDate => self.state.end_date.handle_key_event(key)?,
            SelectableField::Organization => self.state.organization.handle_key_event(key)?,
            _ => {}
        }

        match key.code {
            KeyCode::Enter => self.state.select_next_field(),
            _ => {}
        }

        Ok(())
    }
}

impl App {
    fn may_intercept_key_event(&mut self, key: &KeyEvent) -> util::error::Result<bool> {
        if key.modifiers == event::KeyModifiers::CONTROL {
            match key.code {
                KeyCode::Char('c') => self.quit(),
                KeyCode::Char('o') => self.edit_content()?,
                _ => return Ok(false),
            }

            return Ok(true);
        }

        if self.state.selecting_field.is_advanced() && key.code == KeyCode::Enter {
            self.state.toggle_expand_advanced();
            return Ok(true);
        }

        match key.code {
            KeyCode::Esc => self.state.mode_select(),
            KeyCode::Tab => self.state.select_next_field(),
            KeyCode::BackTab => self.state.select_previous_field(),
            _ => return Ok(false),
        };

        Ok(true)
    }
}
