use std::time::{Duration, Instant};

pub struct Clock {
    instant: Instant,
}

impl Clock {
    pub fn new() -> Self {
        Self { instant: Instant::now() }
    }

    pub fn elapsed(&self) -> Duration {
        Instant::now().duration_since(self.instant)
    }
}