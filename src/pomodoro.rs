use crate::config::Config;
use std::time::*;

pub struct Pomodoro<'a> {
    config: Option<&'a Config>,
    timer: Duration,
    cyclesRemaining: usize,
    state: PomodoroState,
    paused: bool
}

pub enum PomodoroState {
    working,
    resting
}

impl<'a> Pomodoro<'a> {
    pub fn new() -> Pomodoro<'a> {
        Self {
            config: None,
            timer: Duration::new(0, 0),
            cyclesRemaining: 0,
            state: PomodoroState::working,
            paused: true
        }
    }

    pub fn load_config(&mut self, config: &Config) {
        self.config = Some(config);
    }
}
