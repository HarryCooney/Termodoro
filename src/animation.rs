use std::time::*;

pub struct Animation {
    pub frames: usize,
    pub running: bool,
    pub current_frame: usize,
    pub frame_snapshot: Duration
}

enum AnimationType {
    BookTurningPage,
    CoffeeSteam
}

impl Animation {
    pub fn begin_animation(&mut self, time_snapshot: Duration) {
        self.running = true;
        self.current_frame = 0;
        self.frame_snapshot = time_snapshot;
    }

    pub fn next_frame(&mut self, new_frame_snapshot: Duration, frame_duration: u128) {
        if self.is_running() {
            if new_frame_snapshot.as_nanos() - self.frame_snapshot.as_nanos() > frame_duration {
                self.current_frame += 1;
                self.frame_snapshot = new_frame_snapshot;
            }
            if self.current_frame >= self.frames {
                self.current_frame = 0;
                self.running = false
            }
        }
    }
    
    pub fn new(animation_type: AnimationType) -> Animation {
        match animation_type {
            AnimationType::BookTurningPage => Self {
                frames: 6,
                running: false,
                current_frame: 0,
                frame_snapshot: Duration::new(0, 0)
            },
            AnimationType::CoffeeSteam => Self {
                frames: 2,
                running: false,
                current_frame: 0,
                frame_snapshot: Duration::new(0, 0)
            }
        }
    }

    pub fn default_animations() -> Vec<Animation> {
        vec![
            Animation::new(AnimationType::BookTurningPage),
            Animation::new(AnimationType::CoffeeSteam)
        ]
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}

