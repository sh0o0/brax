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

    fn on_escape(&mut self) {
        self.state.is_edit_mode = false;
    }

    fn on_enter(&mut self) {
        if self.state.selecting_field.is_advanced() {
            self.state.toggle_expand_advanced();
            return;
        }

        if !self.state.is_edit_mode {
            self.state.is_edit_mode = true;
            return;
        }

        match self.state.selecting_field {
            SelectableField::Organization => self.state.organization.confirm(|org| org.to_string()),
            _ => {}
        }
        self.state.select_next_field();
    }

    fn on_tab(&mut self) {
        self.state.select_next_field();
    }

    fn on_back_tab(&mut self) {
        self.state.select_previous_field();
    }

    fn on_delete_right_all(&mut self) {
        if !self.state.is_edit_mode {
            return;
        }

        match self.state.selecting_field {
            SelectableField::Title => self.state.title.delete_right_all(),
            SelectableField::StartDate => self.state.start_date.delete_right_all(),
            SelectableField::EndDate => self.state.end_date.delete_right_all(),
            SelectableField::Organization => self.state.organization.delete_right_all(),
            _ => {}
        }
    }

    fn on_delete_right(&mut self) {
        if !self.state.is_edit_mode {
            return;
        }

        match self.state.selecting_field {
            SelectableField::Title => self.state.title.delete_right_char(),
            SelectableField::StartDate => self.state.start_date.delete_right_char(),
            SelectableField::EndDate => self.state.end_date.delete_right_char(),
            SelectableField::Organization => self.state.organization.delete_right_char(),
            _ => {}
        }
    }

    fn on_end(&mut self) {
        if !self.state.is_edit_mode {
            return;
        }

        match self.state.selecting_field {
            SelectableField::Title => self.state.title.move_cursor_to_end(),
            SelectableField::StartDate => self.state.start_date.move_cursor_to_end(),
            SelectableField::EndDate => self.state.end_date.move_cursor_to_end(),
            SelectableField::Organization => self.state.organization.move_cursor_to_end(),
            _ => {}
        }
    }

    fn on_start(&mut self) {
        if !self.state.is_edit_mode {
            return;
        }

        match self.state.selecting_field {
            SelectableField::Title => self.state.title.move_cursor_to_start(),
            SelectableField::StartDate => self.state.start_date.move_cursor_to_start(),
            SelectableField::EndDate => self.state.end_date.move_cursor_to_start(),
            SelectableField::Organization => self.state.organization.move_cursor_to_start(),
            _ => {}
        }
    }

    fn on_left(&mut self) {
        if !self.state.is_edit_mode {
            return;
        }

        match self.state.selecting_field {
            SelectableField::Title => self.state.title.move_cursor_left(),
            SelectableField::StartDate => self.state.start_date.move_cursor_left(),
            SelectableField::EndDate => self.state.end_date.move_cursor_left(),
            SelectableField::Organization => self.state.organization.move_cursor_left(),
            _ => {}
        }
    }

    fn on_right(&mut self) {
        if !self.state.is_edit_mode {
            return;
        }

        match self.state.selecting_field {
            SelectableField::Title => self.state.title.move_cursor_right(),
            SelectableField::StartDate => self.state.start_date.move_cursor_right(),
            SelectableField::EndDate => self.state.end_date.move_cursor_right(),
            SelectableField::Organization => self.state.organization.move_cursor_right(),
            _ => {}
        }
    }

    fn on_up(&mut self) {
        if !self.state.is_edit_mode {
            self.state.select_previous_field();
            return;
        }

        match self.state.selecting_field {
            SelectableField::Type => self.state.typ.select_previous(),
            SelectableField::Impact => self.state.impact.select_previous(),
            SelectableField::Organization => self.state.organization.select_previous(),
            _ => self.state.select_previous_field(),
        }
    }

    fn on_down(&mut self) {
        if !self.state.is_edit_mode {
            self.state.select_next_field();
            return;
        }

        match self.state.selecting_field {
            SelectableField::Type => self.state.typ.select_next(),
            SelectableField::Impact => self.state.impact.select_next(),
            SelectableField::Organization => self.state.organization.select_next(),
            _ => self.state.select_next_field(),
        }
    }

    fn on_char(&mut self, c: char) {
        if !self.state.is_edit_mode {
            return;
        }

        match self.state.selecting_field {
            SelectableField::Title => self.state.title.enter_char(c),
            SelectableField::StartDate => self.state.start_date.enter_char(c),
            SelectableField::EndDate => self.state.end_date.enter_char(c),
            SelectableField::Organization => self.state.organization.enter_char(c),
            _ => {}
        }
    }

    fn on_delete(&mut self) {
        if !self.state.is_edit_mode {
            return;
        }

        match self.state.selecting_field {
            SelectableField::Title => self.state.title.delete_left_char(),
            SelectableField::StartDate => self.state.start_date.delete_left_char(),
            SelectableField::EndDate => self.state.end_date.delete_left_char(),
            SelectableField::Organization => self.state.organization.delete_left_char(),
            _ => {}
        }
    }

    fn on_k(&mut self) {
        if !self.state.is_edit_mode {
            self.on_up();
            return;
        }

        const K: char = 'k';

        match self.state.selecting_field {
            SelectableField::Type | SelectableField::Impact => self.on_up(),
            _ => self.on_char(K),
        }
    }

    fn on_j(&mut self) {
        if !self.state.is_edit_mode {
            self.on_down();
            return;
        }

        const J: char = 'j';

        match self.state.selecting_field {
            SelectableField::Type | SelectableField::Impact => self.on_down(),
            _ => self.on_char(J),
        }
    }

    fn get_key_event_handler(&mut self) -> Option<&mut dyn KeyEventHandler> {
        match self.state.selecting_field {
            SelectableField::Title => Some(&mut self.state.title),
            SelectableField::StartDate => Some(&mut self.state.start_date),
            SelectableField::EndDate => Some(&mut self.state.end_date),
            _ => None,
        }
    }
}

impl KeyEventHandler for App {
    fn handle_key_event(&mut self, key: &KeyEvent) -> util::error::Result<()> {
        if let Some(handler) = self.get_key_event_handler() {
            return handler.handle_key_event(key);
        }

        if key.modifiers == event::KeyModifiers::CONTROL {
            match key.code {
                KeyCode::Char('p') => self.on_up(),
                KeyCode::Char('n') => self.on_down(),
                KeyCode::Char('h') => self.on_delete(),
                KeyCode::Char('b') => self.on_left(),
                KeyCode::Char('f') => self.on_right(),
                KeyCode::Char('c') => self.quit(),
                KeyCode::Char('e') => self.on_end(),
                KeyCode::Char('a') => self.on_start(),
                KeyCode::Char('d') => self.on_delete_right(),
                KeyCode::Char('k') => self.on_delete_right_all(),
                KeyCode::Char('o') => self.edit_content()?,
                _ => {}
            }
            return Ok(());
        }

        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Esc => self.on_escape(),
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

        Ok(())
    }
}
