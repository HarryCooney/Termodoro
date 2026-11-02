use std::time::*;
pub struct Clock {
    pub timer: Duration,
    pub tick: usize, //Clock tick is 20 TPS
    pub tick_elapse: Duration,
    frame_duration: Duration
}

impl Clock {
    pub fn new() -> Self {
        let clock_start_time  = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
            .expect("FATAL ERROR: Could not get system time");
        Clock {
            timer: clock_start_time,
            tick: 0,
            tick_elapse: clock_start_time,
            frame_duration: Duration::new(0, 33333333)
        }
    }

    ///Updates app clock and tick
    pub fn update_clock(&mut self) {
        self.timer = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
            .expect("FATAL ERROR: Could not get system time");
        if self.timer.as_nanos() - self.tick_elapse.as_nanos() > 100_000_000 {
            self.tick_elapse = self.timer;
            self.tick += 1;
        }
        
    }

    pub fn time_to_next_frame(&self) -> Duration {
        if let Some(time) = self.frame_duration.checked_sub(self.tick_elapse) {
            return time
        }
        Duration::new(0, 0)
    }
    
    fn current_time() -> Duration {
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
            .expect("FATAL ERROR: Could not get system time")
    }

    ///Returns the time elapsed since last clock update.
    pub fn delta_time(&self) -> Duration {
        if let Some(delta_time) = Clock::current_time().checked_sub(self.timer) {
            return delta_time;
        }
        Duration::new(0, 0)
    }
}
