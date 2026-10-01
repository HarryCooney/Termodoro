use std::time::*;
#[derive(Copy, Clone)]
pub struct Config {
    pub work: Duration,
    pub rest: Duration,
    pub cycles: usize
}

impl Config {
    pub fn new(work_duration: Duration, rest_duration: Duration, cycles: usize) -> Config{
        Self {
            work: work_duration,
            rest: rest_duration,
            cycles
        }
    }
}

impl Default for Config {
    fn default() -> Config {
        Self {
            work: Duration::new(50, 0),
            rest: Duration::new(50, 0),
            cycles: 3
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
