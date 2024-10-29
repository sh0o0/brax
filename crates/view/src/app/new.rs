use crate::screen::new::{Field, Screen, State};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{prelude::Backend, Terminal};
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
            self.handle_event(timeout)?;
        }

        Ok(())
    }

    fn handle_event(&mut self, timeout: Duration) -> util::error::Result<()> {
        if !event::poll(timeout)? {
            return Ok(());
        }

        if let Event::Key(key) = event::read()? {
            if key.modifiers == event::KeyModifiers::CONTROL {
                match key.code {
                    KeyCode::Char('p') => self.on_up(),
                    KeyCode::Char('n') => self.on_down(),
                    KeyCode::Char('h') => self.on_delete(),
                    _ => {}
                }
                return Ok(());
            }

            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Up | KeyCode::Char('k') => self.on_up(),
                    KeyCode::Down | KeyCode::Char('j') => self.on_down(),
                    KeyCode::Left => self.on_left(),
                    KeyCode::Right => self.on_right(),
                    KeyCode::Char('q') => self.quit(),
                    KeyCode::Esc => self.on_escape(),
                    KeyCode::Delete | KeyCode::Backspace => self.on_delete(),
                    KeyCode::Char(c) => self.on_input(c),
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

    fn on_left(&mut self) {
        match self.state.selecting_field {
            Some(Field::Title) => self.state.title.move_cursor_left(),
            _ => {}
        }
    }

    fn on_right(&mut self) {
        match self.state.selecting_field {
            Some(Field::Title) => self.state.title.move_cursor_right(),
            _ => {}
        }
    }

    fn on_up(&mut self) {
        self.state.select_up();
    }

    fn on_down(&mut self) {
        self.state.select_down();
    }

    fn on_input(&mut self, c: char) {
        match self.state.selecting_field {
            Some(Field::Title) => self.state.title.enter_char(c),
            _ => {}
        }
    }

    fn on_delete(&mut self) {
        match self.state.selecting_field {
            Some(Field::Title) => {
                self.state.title.delete_char();
            }
            _ => {}
        }
    }

    fn on_escape(&mut self) {
        self.state.unselect();
    }
}
