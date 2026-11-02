use std::time::*; 
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Config {
    pub work: Duration,
    pub rest: Duration,
    pub cycles: usize
}

impl Config {
    pub fn new(work_duration: Duration, rest_duration: Duration, cycles: usize) -> Option<Config> {
        if work_duration <= Duration::new(0, 0) || rest_duration <= Duration::new(0, 0) || cycles <= 0 {
            return None;
        }
        Some(Config {
            work: work_duration,
            rest: rest_duration,
            cycles
        })
    }

    pub fn to_string_icon_format(&self) -> String {
         let work_string = time_duration_to_string(&self.work);
        let rest_string = time_duration_to_string(&self.rest);
        format!("W {}  R {}  C {}",
            work_string, rest_string, self.cycles)
    }
}

impl Default for Config {
    fn default() -> Config {
        Self {
            work: Duration::new(0, 0),
            rest: Duration::new(0, 0),
            cycles: 0
        }
    }
}

impl ToString for Config {
    fn to_string(&self) -> String {
        let work_string = time_duration_to_string(&self.work);
        let rest_string = time_duration_to_string(&self.rest);
        format!("Work: {:02}   Rest - {:02}   Cycles - {}",
            work_string, rest_string, self.cycles)
    }
}
pub fn time_duration_to_string(time: &Duration) -> String {
    let time_as_secs = time.as_secs();
    let hours = time_as_secs / 3600;
    let minutes = (time_as_secs % 3600) / 60;
    let secs = time_as_secs % 60;
    if hours == 0 {
        format!("{:02}:{:02}", minutes, secs)
    } else {
        format!("{:02}:{:02}:{:02}",hours, minutes, secs)
    }
}
