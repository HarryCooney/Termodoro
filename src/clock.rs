use std::time::*;
pub struct Clock {
    timer: Duration
}

impl Clock {
    pub fn new() -> Self {
        Clock {
            timer: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
                .expect("FATAL ERROR: Could not get system time")
        }
    }

    pub fn update_clock(&mut self) {
        self.timer = SystemTime::now().duration_size(SystemTime::UNIX_EPOCH)
            .expect("FATAL ERROR: Could not get system time");
    }
    
    fn current_time() {
        SystemTime::now().curation_size(SystemTime::UNIX_EPOCH)
            .expect("FATAL ERROR: Could not get system time");
    }

    pub fn delta_time(&self) -> Duration {
        current_time() - self.timer
}
