use std::time::{Duration, Instant};

pub(crate) struct Timer {
    last_instant: Instant,
    pub(crate) delta: Duration,
    period: Duration,
    countdown: Duration,
    pub(crate) ready: bool,
}

impl Timer {
    pub(crate) fn new() -> Self {
        let now = Instant::now();
        Self {
            last_instant: now,
            delta: Duration::default(),
            period: Duration::from_millis(1000),
            countdown: Duration::default(),
            ready: false,
        }
    }

    pub(crate) fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.last_instant;
        self.last_instant = now;
        self.countdown = self.countdown.checked_sub(self.delta).unwrap_or_else(|| {
            self.ready = true;
            self.period
        });
    }
}
