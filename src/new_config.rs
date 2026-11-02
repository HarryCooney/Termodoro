use crate::app::{App, AppState};
use std::time::Duration;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter)]
pub enum CurrentlyEditing {
    Work,
    Rest,
    Cycles,
    NotEditing,
}

impl App {
    pub fn increment_work_duration(&mut self, minutes: usize)  {
        if let Some(mut config) = self.new_config_buffer {
            let new_duration = Duration::new(minutes as u64 * 60, 0).saturating_add(config.work)
                .clamp(Duration::new(0, 0), Duration::new(86399, 0)); //This is a limit for 23
                                                                      //hours and 59 minutes
            config.work = new_duration;
            self.new_config_buffer = Some(config);
        }
    }
    pub fn decrement_work_duration(&mut self, minutes: usize)  {
        if let Some(mut config) = self.new_config_buffer {
            let new_duration = config.work.saturating_sub(Duration::new(minutes as u64 * 60, 0))
                .clamp(Duration::new(0, 0), Duration::new(86399, 0)); //This is a limit for 23
                                                                      //hours and 59 minutes
            config.work = new_duration;
            self.new_config_buffer = Some(config);
        }
    }
    pub fn increment_rest_duration(&mut self, minutes: usize)  {
        if let Some(mut config) = self.new_config_buffer {
            let new_duration = Duration::new(minutes as u64 * 60, 0).saturating_add(config.rest)
                .clamp(Duration::new(0, 0), Duration::new(86399, 0)); //This is a limit for 23
                                                                      //hours and 59 minutes
            config.rest = new_duration;
            self.new_config_buffer = Some(config);
        }
    }
    pub fn decrement_rest_duration(&mut self, minutes: usize)  {
        if let Some(mut config) = self.new_config_buffer {
            let new_duration = config.rest.saturating_sub(Duration::new(minutes as u64 * 60, 0))
                .clamp(Duration::new(0, 0), Duration::new(86399, 0)); //This is a limit for 23
                                                                      //hours and 59 minutes
            config.rest = new_duration;
            self.new_config_buffer = Some(config);
        }
    }

    pub fn increment_cycles(&mut self, cycles: usize)  {
        if let Some(mut config) = self.new_config_buffer {
            let new_cycles = config.cycles.saturating_add(cycles);
            config.cycles = new_cycles;
            self.new_config_buffer = Some(config);
        }
    }
    pub fn decrement_cycles(&mut self, cycles: usize)  {
        if let Some(mut config) = self.new_config_buffer {
            let new_cycles = config.cycles.saturating_sub(cycles);
            config.cycles = new_cycles;
            self.new_config_buffer = Some(config);
        }
    }

    pub fn save_new_config(&mut self) {
        if let Some(config) = self.new_config_buffer {
                self.config_menu.add_config(config)
        }
    }

    //TODO: Add a pop up after adding a new config which asks the user whether
    //they want to just save the config or also set it as the current timer
}
