use crate::config::Config;
use crate::clock::Clock;
use std::time::*;

pub struct Pomodoro {
    config: Option<Config>,
    timer: Duration,
    cyclesRemaining: usize,
    state: PomodoroState,
    cyclesDisplayMode: CyclesDisplayMode,
    paused: bool,
    pub cycle_string_cache: String
}

enum PomodoroState {
    Working,
    Resting,
    Done
}

enum CyclesDisplayMode {
    Blocks,
    Fraction
}

impl Pomodoro {
    pub fn new() -> Pomodoro {
        Self {
            config: None,
            timer: Duration::new(0, 0),
            cyclesRemaining: 0,
            state: PomodoroState::Working,
            cyclesDisplayMode: CyclesDisplayMode::Blocks,
            paused: true,
            cycle_string_cache: String::new()
        }
    }

    pub fn load_config(&mut self, config: Config) {
        self.config = Some(config);
        self.timer = config.work;
        self.cyclesRemaining = config.cycles;
        self.state = PomodoroState::Working;
        if let Some(config) = self.config && config.cycles > 7 {
                self.cyclesDisplayMode = CyclesDisplayMode::Fraction
        }
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
        match self.cyclesDisplayMode {
            CyclesDisplayMode::Blocks => self.cycle_string_cache = self.cycles_block_format(),
            CyclesDisplayMode::Fraction => self.cycle_string_cache = self.cycles_fraction_format()
        }
    }

    fn cycles_block_format(&mut self) -> String {
        if let Some(config) = self.config {
            let cycles_completed = config.cycles.saturating_sub(self.cyclesRemaining);
            let mut sb = String::new();
            for _ in 0..cycles_completed {
                sb.push_str("■ ");
            }
            if self.cyclesRemaining > 0 {
                sb.push_str("⬚ ");
            }
            for _ in 0..self.cyclesRemaining.saturating_sub(1) {
                sb.push_str("□ ");
            }
            return sb.trim().to_string();
        }

        "".to_string()
    }

    fn cycles_fraction_format(&mut self) -> String {
        if let Some(config) = self.config {
            let cycles_completed = config.cycles.saturating_sub(self.cyclesRemaining);
            return format!("{} / {}", cycles_completed, config.cycles);
        }

        "".to_string()
    }

}
