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
        self.timer = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
            .expect("FATAL ERROR: Could not get system time");
    }
    
    fn current_time() -> Duration {
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
            .expect("FATAL ERROR: Could not get system time")
    }

    pub fn delta_time(&self) -> Duration {
        if let Some(delta_time) = Clock::current_time().checked_sub(self.timer) {
            return delta_time;
        }
        Duration::new(0, 0)
    }
}
