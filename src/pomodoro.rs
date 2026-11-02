use crate::config::Config;
use std::time::*;

pub struct Pomodoro {
    config: Option<Config>,
    timer: Duration,
    cycles_remaining: usize,
    pub timer_state: PomodoroTimerState,
    cycles_display_mode: CyclesDisplayMode,
    pub timer_active: bool,
    paused: bool,
    pub cycle_string_cache: String
}

pub enum PomodoroTimerState {
    Working,
    Resting,
    Done,
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
            cycles_remaining: 0,
            timer_state: PomodoroTimerState::Working,
            cycles_display_mode: CyclesDisplayMode::Blocks,
            timer_active: false,
            paused: true,
            cycle_string_cache: String::new()
        }
    }

    pub fn load_config(&mut self, config: Config) {
        self.config = Some(config);
        self.timer = config.work;
        self.cycles_remaining = config.cycles;
        self.timer_state = PomodoroTimerState::Working;
        if let Some(config) = self.config && config.cycles > 7 {
                self.cycles_display_mode = CyclesDisplayMode::Fraction
        }
        self.paused = false;
        self.update_cycles_string_cache();
    }

    pub fn update_pomodoro_timer(&mut self, delta_time: &Duration) {
        if !self.paused {
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
    }
    
    fn handle_pomodoro_state(&mut self) {
        if self.cycles_remaining > 0 {
            self.switch_between_work_and_break();
        } else {
            self.timer_state = PomodoroTimerState::Done;
            self.timer_active = false;
        }
    }
    fn switch_between_work_and_break(&mut self) {
        match self.timer_state {
            PomodoroTimerState::Working => {
                self.timer_state = PomodoroTimerState::Resting;
                if let Some(config) = self.config {
                    self.timer = config.rest;
                }
            }
            PomodoroTimerState::Resting => {
                self.timer_state = PomodoroTimerState::Working;
                if let Some(config) = self.config {
                    self.timer = config.work;
                }
            },
            _ => {}
        }
    }
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn pause(&mut self) { self.paused = true }
    pub fn unpause(&mut self) { self.paused = false }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused
    }
    fn update_cycles(&mut self) {
        if let  PomodoroTimerState::Resting = self.timer_state {
            self.cycles_remaining = self.cycles_remaining.saturating_sub(1);
        }
    }

    pub fn clock_text_to_string(&self) -> String {
        match self.timer_state {
            PomodoroTimerState::Done => String::from("Study Finished!"),
            _ => {
                let minutes = self.timer.as_secs() / 60;
                let seconds = self.timer.as_secs() % 60;
                if self.paused {
                    format!("Paused - {} : {}", minutes, seconds)
                } else {
                    format!("{} : {}", minutes, seconds)
                }
            }
        }
    }

    //TODO: Test this for wayy more inputs
    pub fn update_cycles_string_cache(&mut self) {
        match self.cycles_display_mode {
            CyclesDisplayMode::Blocks => self.cycle_string_cache = self.cycles_block_format(),
            CyclesDisplayMode::Fraction => self.cycle_string_cache = self.cycles_fraction_format()
        }
    }

    fn cycles_block_format(&mut self) -> String {
        if let Some(config) = self.config {
            let cycles_completed = config.cycles.saturating_sub(self.cycles_remaining);
            let mut sb = String::new();
            for _ in 0..cycles_completed {
                sb.push_str("■ ");
            }
            if self.cycles_remaining > 0 {
                sb.push_str("⬚ ");
            }
            for _ in 0..self.cycles_remaining.saturating_sub(1) {
                sb.push_str("□ ");
            }
            return sb.trim().to_string();
        }

        "".to_string()
    }

    fn cycles_fraction_format(&mut self) -> String {
        if let Some(config) = self.config {
            let cycles_completed = config.cycles.saturating_sub(self.cycles_remaining);
            return format!("{} / {}", cycles_completed, config.cycles);
        }

        "".to_string()
    }

}
