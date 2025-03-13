// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::{sleep_until, Instant};

pub struct Limiter {
    max_calls: usize,
    interval: Duration,
    data: Mutex<Data>,
}

struct Data {
    last_reset: Instant,
    calls: usize,
}

impl Limiter {
    pub fn new(max_calls: usize, interval: Duration) -> Self {
        Self {
            max_calls,
            interval,
            data: Mutex::new(Data {
                last_reset: Instant::now(),
                calls: 0,
            }),
        }
    }

    pub fn new_per_second(max_calls: usize) -> Self {
        Self::new(max_calls, Duration::from_secs(1))
    }

    pub async fn limit(&self) {
        let mut data = self.data.lock().await;
        let now = Instant::now();

        if now.duration_since(data.last_reset) >= self.interval {
            data.calls = 0;
            data.last_reset = Instant::now();
        }

        if data.calls >= self.max_calls {
            let reset_time = data.last_reset + self.interval;
            sleep_until(reset_time).await;
            data.calls = 0;
            data.last_reset = Instant::now();
        }

        data.calls += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_max_out_within() {
        let limiter = Limiter::new(5, Duration::from_millis(10)); // 5 calls per 10 millis

        let start = Instant::now();
        for _ in 0..5 {
            limiter.limit().await;
        }

        assert!(start.elapsed().as_millis() < 1);
    }

    #[tokio::test]
    async fn test_exceeds_limit() {
        let limiter = Limiter::new(2, Duration::from_millis(10));

        let start = Instant::now();
        limiter.limit().await;
        limiter.limit().await;
        assert!(start.elapsed().as_millis() < 1);
        limiter.limit().await; // This one should be delayed
        assert!(start.elapsed().as_millis() >= 10);
    }

    #[tokio::test]
    async fn test_resets_after_interval_completed() {
        let limiter = Limiter::new(2, Duration::from_millis(10));

        limiter.limit().await;
        sleep(Duration::from_millis(10)).await;

        let start = Instant::now();
        limiter.limit().await;
        limiter.limit().await;
        assert!(start.elapsed().as_millis() < 1);
    }
}
