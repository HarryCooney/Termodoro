use crate::config::Config;

pub struct Pomodoro<'a> {
    config: Option<&'a Config>,
    timer: f64,
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
            timer: 0.0,
            cyclesRemaining: 0,
            state: PomodoroState::working,
            paused: true
        }
    }
}
