use std::sync::atomic::{AtomicU64, Ordering};

pub struct StatsCollector {
    total_requests: AtomicU64,
    rate_limited: AtomicU64,
}

impl StatsCollector {
    pub fn new() -> Self {
        Self {
            total_requests: AtomicU64::new(0),
            rate_limited: AtomicU64::new(0),
        }
    }

    pub fn record_request(&self) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_limited(&self) {
        self.rate_limited.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get_stats(&self) -> (u64, u64) {
        (
            self.total_requests.load(Ordering::Relaxed),
            self.rate_limited.load(Ordering::Relaxed),
        )
    }
}
