use ratatui::DefaultTerminal;
use color_eyre::Result;
use crossterm::event::{self, poll, read};
use std::time::Duration;
use crate::pomodoro::Pomodoro;
use crate::menu::Menu;
use crate::config::Config;
use crate::config_menu::Config_Menu;
use crate::clock::Clock;

pub struct App {
    running: bool,
    pub state: AppState,
    pub config: Option<Config>,
    pub pomodoro: Pomodoro,
    pub menu: Menu,
    pub config_menu: Config_Menu,
    pub clock: Clock,
    pub i: usize
}

pub enum AppState {
    Menu,
    ConfirmReturnToMenu,
    PomodoroState,
    ConfigSelection,
    NewConfig
}

impl App {
    pub fn new() -> Self {
        Self {
            running: false,
            state: AppState::Menu,
            config: Some(Config::default()),
            pomodoro: Pomodoro::new(),
            menu: Menu::new(),
            config_menu: Config_Menu::new(),
            clock: Clock::new(),
            i: 0
        }
    }
    
    fn read_events(&mut self) -> Result<()> {
        while event::poll(Duration::from_millis(0))? {
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.read_events()?;
            self.clock_events();
        }
        Ok(())
    }

    pub fn set_state(&mut self, state: AppState) {
        self.state = state;
    }

    fn clock_events(&mut self) {
        self.pomodoro.update_pomodoro_timer(&self.clock.delta_time());
        self.clock.update_clock();
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    fn clear_config(&mut self) {
        self.config = None;
    }

    fn confirm_menu_choice(&mut self) {
        match self.menu.selected() {
            0 => self.set_state(AppState::PomodoroState),
            1 => self.set_state(AppState::ConfigSelection),
            2 => self.set_state(AppState::NewConfig),
            _ => {}
        }
    }
}
