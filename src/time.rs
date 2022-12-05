use std::time::{Duration, Instant};

pub struct TimeContext {
    now: Duration,
    last: Duration,
    delta: Duration,
    frame_delay: Duration,
    start: Instant,
}

impl TimeContext {
    pub fn new(target_fps: u32) -> Self {
        let frame_delay = Duration::from_secs_f32(1. / target_fps as f32);
        Self {
            last: Duration::ZERO,
            delta: frame_delay,
            frame_delay,
            start: Instant::now(),
            now: Duration::ZERO,
        }
    }

    pub fn update(&mut self) {
        self.now = self.start.elapsed();
        self.delta = self.now - self.last;
        self.last = self.now;
    }

    pub fn wait_sync(&mut self) {
        if let Some(frame_delta) = self
            .frame_delay
            .checked_sub(self.start.elapsed() - self.last)
        {
            std::thread::sleep(frame_delta);
        }
    }

    pub fn now(&self) -> Duration {
        self.now
    }

    pub fn delta(&self) -> Duration {
        self.delta
    }

    pub fn frame_rate(&self) -> f32 {
        1. / self.delta.as_secs_f32()
    }
}
