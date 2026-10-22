use std::time::*; #[derive(Copy, Clone)]
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
}

impl Default for Config {
    fn default() -> Config {
        Self {
            work: Duration::new(1, 0),
            rest: Duration::new(1, 0),
            cycles: 8
        }
    }
}
/*
impl Clone for Config {
    fn clone(&self) -> Self {
        Config {
            work: self.work,
            rest: self.rest,
            cycles: self.cycles
        }
    }
}
*/
