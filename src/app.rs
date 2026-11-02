use ratatui::DefaultTerminal;
use std::path::Path;
use std::fs::write;
use color_eyre::Result;
use crossterm::event::{self, poll, read};
use std::time::Duration;
use crate::pomodoro::{Pomodoro, PomodoroTimerState};
use crate::menu::Menu;
use crate::config::Config;
use crate::config_menu::ConfigMenu;
use crate::clock::Clock;
use crate::animation::Animation;

pub struct App {
    running: bool,
    pub state: AppState,
    pub config: Option<Config>,
    pub pomodoro: Pomodoro,
    pub menu: Menu,
    pub config_menu: ConfigMenu,
    pub clock: Clock,
    pub i: usize,
    pub animations: Vec<Animation>,
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
            config_menu: ConfigMenu::default(),
            clock: Clock::new(),
            i: 0,
            animations: Animation::default_animations()
        }
    }
    
    fn read_events(&mut self) -> Result<()> {
        while event::poll(Duration::from_millis(34))? {
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.running = true;
        self.config_menu.read_configs_from_file()?;
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
        self.handle_animations();
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

    fn handle_animations(&mut self) {
        self.schedule_animations();
        self.run_animations();
    }

    fn schedule_animations(&mut self) {
        match self.state {
            AppState::PomodoroState => self.schedule_pomodoro_animations(),
            _ => {}
        }
    }

    fn schedule_pomodoro_animations(&mut self) {
        match self.pomodoro.timer_state {
            PomodoroTimerState::Working => {
                if self.clock.tick % 120 == 0 {
                    self.animations[0].running = true;
                }
            },
            PomodoroTimerState::Resting => {
                self.animations[1].running = true;
            },
            _ => {}
        }
    }

    fn run_animations(&mut self) {
        self.animations[0].next_frame(self.clock.timer, 70_000_000);
        self.animations[1].next_frame(self.clock.timer, 2_000_000_000);
    }
}
