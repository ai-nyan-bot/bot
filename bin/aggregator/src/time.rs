// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use rand::{rng, Rng};
use std::time::Duration;

#[macro_export]
macro_rules! log_ms {
    ($name:expr, $func:expr) => {{
        use std::time::Instant;

        let start = Instant::now();
        let result = $func.await;
        let duration = start.elapsed().as_millis();

        log::debug!("{} took: {} ms", $name, duration);
        result
    }};
    ($name:expr, $partition:expr, $func:expr) => {{
        use std::time::Instant;

        let start = Instant::now();
        let result = $func.await;
        let duration = start.elapsed().as_millis();

        log::debug!(
            "{} took: {} ms",
            format!("{}_{}", $name, $partition),
            duration
        );
        result
    }};
}

pub async fn sleep_ms(lower: u64, upper: u64) {
    let sleep_time = rng().random_range(lower..=upper);
    tokio::time::sleep(Duration::from_millis(sleep_time)).await;
}
