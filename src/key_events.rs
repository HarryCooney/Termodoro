use color_eyre::Result;
use crate::config::Config;
use crossterm::event::{self, Event, KeyEvent, KeyEventKind, KeyCode};
use crate::app::{App, AppState};

impl App {
    pub fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_events(&key_event);
            },
            _ => {}
        }
        Ok(())
    }
    //TODO
    //Add handling for CTRL events

    fn handle_key_events(&mut self, key_event: &KeyEvent) {
        match self.state {
            AppState::Menu => self.handle_menu_events(key_event),
            AppState::ConfigSelection => self.handle_config_selection_events(key_event),
            AppState::ConfirmReturnToMenu => self.handle_return_to_menu_popup_events(key_event),
            AppState::NewConfig => self.handle_new_config_events(key_event),
            AppState::PomodoroState => self.handle_pomodoro_events(key_event)
        }
    }

    fn handle_menu_events(&mut self, key_event: &KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.quit(),
            KeyCode::Char('h') => self.menu.choose_previous_option(),
            KeyCode::Char('l') => self.menu.choose_next_option(),
            KeyCode::Enter => self.enter_on_menu(),
            _ => {}
        }
    }
    fn handle_config_selection_events(&mut self, key_event: &KeyEvent) {}
    fn handle_return_to_menu_popup_events(&mut self, key_event: &KeyEvent) {}
    fn handle_new_config_events(&mut self, key_event: &KeyEvent) {}
    fn handle_pomodoro_events(&mut self, key_event: &KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.quit(),
            _ => {}
        }
    }

    fn enter_on_menu(&mut self) {
        match self.menu.selected() {
            0 => {},
            1 => {
                self.set_state(AppState::PomodoroState);
                if let Some(config) = self.config {
                    self.pomodoro.load_config(config);
                }
            },
            2 => {},
            _ => {}
        }
    }
}
