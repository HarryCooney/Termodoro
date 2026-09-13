use std::time::*;
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
