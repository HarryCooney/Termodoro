pub struct Config {
    pub work: usize,
    pub rest: usize,
    pub cycles: usize
}

impl Config {
    pub fn new(work_duration: usize, rest_duration: usize, cycles: usize) -> Config{
        Self {
            work: work_duration,
            rest: rest_duration,
            cycles
        }
    }
}
