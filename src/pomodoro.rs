use crate::config::Config;
use crate::clock::Clock;
use std::time::*;

pub struct Pomodoro {
    config: Option<Config>,
    timer: Duration,
    cyclesRemaining: usize,
    state: PomodoroState,
    paused: bool
}

enum PomodoroState {
    Working,
    Resting
}

impl Pomodoro {
    pub fn new() -> Pomodoro {
        Self {
            config: None,
            timer: Duration::new(0, 0),
            cyclesRemaining: 0,
            state: PomodoroState::Working,
            paused: true
        }
    }

    pub fn load_config(&mut self, config: Config) {
        self.config = Some(config);
        self.timer = config.work;
        self.cyclesRemaining = config.cycles;
        self.state = PomodoroState::Working;
        self.paused = false;
    }

    pub fn update_pomodoro_timer(&mut self, delta_time: &Duration) {
        match self.timer.checked_sub(*delta_time) {
            Some(time) => {
                self.timer = time;
            },
            None => self.change_pomodoro_state()
        }
    }

    fn change_pomodoro_state(&mut self) {}
}

impl ToString for Pomodoro {
    fn to_string(&self) -> String {
        let minutes = self.timer.as_secs() / 60;
        let seconds = self.timer.as_secs() % 60;
        String::from(format!("{} : {}", minutes, seconds))
    }
}
