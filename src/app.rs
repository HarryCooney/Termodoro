use ratatui::DefaultTerminal;
use color_eyre::Result;
use crate::pomodoro::Pomodoro;
use crate::menu::Menu;
use crate::config::Config;
use crate::config_menu::Config_Menu;
use crate::clock::Clock;

pub struct App<'a> {
    running: bool,
    pub state: AppState,
    config: Option<Config>,
    pomodoro: Pomodoro<'a>,
    pub menu: Menu,
    config_menu: Config_Menu,
    clock: Clock
}

pub enum AppState {
    Menu,
    ConfirmReturnToMenu,
    PomodoroState,
    ConfigSelection,
    NewConfig
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        Self {
            running: false,
            state: AppState::Menu,
            config: None,
            pomodoro: Pomodoro::new(),
            menu: Menu::new(),
            config_menu: Config_Menu::new(),
            clock: Clock::new()
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events();
        }
        Ok(())
    }

    pub fn set_state(&mut self, state: AppState) {
        self.state = state;
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
