use crate::config::Config;
use crate::clock::Clock;
use std::time::*;

pub struct Pomodoro {
    config: Option<Config>,
    timer: Duration,
    cyclesRemaining: usize,
    state: PomodoroState,
    paused: bool,
    pub cycle_string_cache: String
}

enum PomodoroState {
    Working,
    Resting,
    Done
}

impl Pomodoro {
    pub fn new() -> Pomodoro {
        Self {
            config: None,
            timer: Duration::new(0, 0),
            cyclesRemaining: 0,
            state: PomodoroState::Working,
            paused: true,
            cycle_string_cache: String::new()
        }
    }

    pub fn load_config(&mut self, config: Config) {
        self.config = Some(config);
        self.timer = config.work;
        self.cyclesRemaining = config.cycles;
        self.state = PomodoroState::Working;
        self.paused = false;
        self.update_cycles_string_cache();
    }

    pub fn update_pomodoro_timer(&mut self, delta_time: &Duration) {
        match self.timer.checked_sub(*delta_time) {
            Some(time) => {
                self.timer = time;
            },
            None => {
                self.update_cycles();
                self.handle_pomodoro_state();
                self.update_cycles_string_cache();
            }
        }
    }
    
    fn handle_pomodoro_state(&mut self) {
        if self.cyclesRemaining > 0 {
            self.switch_between_work_and_break();
        } else {
            self.state = PomodoroState::Done;
        }
    }
    fn switch_between_work_and_break(&mut self) {
        match self.state {
            PomodoroState::Working => {
                self.state = PomodoroState::Resting;
                if let Some(config) = self.config {
                    self.timer = config.rest;
                }
            }
            PomodoroState::Resting => {
                self.state = PomodoroState::Working;
                if let Some(config) = self.config {
                    self.timer = config.work;
                }
            },
            _ => {}
        }
    }

    fn update_cycles(&mut self) {
        if let  PomodoroState::Resting = self.state {
            self.cyclesRemaining = self.cyclesRemaining.saturating_sub(1);
        }
    }

    pub fn time_remaining_to_string(&self) -> String {
        match self.state {
            PomodoroState::Done => String::from("Study Finished!"),
            _ => {
                let minutes = self.timer.as_secs() / 60;
                let seconds = self.timer.as_secs() % 60;
                format!("{} : {}", minutes, seconds)
            }
        }
    }

    //TODO: Test this for wayy more inputs
    pub fn update_cycles_string_cache(&mut self) {
        if let Some(config) = self.config {
            let cycles_compeleted = config.cycles.saturating_sub(self.cyclesRemaining);
            let mut sb = String::new();
            for _ in 0..cycles_compeleted {
                sb.push_str("■ ");
            }
            if self.cyclesRemaining > 0 {
                sb.push_str("⬚ ");
            }
            for _ in 0..self.cyclesRemaining.saturating_sub(1) {
                sb.push_str("□ ");
            }
            return self.cycle_string_cache = sb.trim().to_string();
        }

        self.cycle_string_cache = "".to_string()
    }

}
