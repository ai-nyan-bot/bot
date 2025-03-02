// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TimeUnit {
    MilliSecond,
    Second,
    Minute,
    Hour,
    Day,
}

impl Display for TimeUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeUnit::MilliSecond => f.write_str("ms"),
            TimeUnit::Second => f.write_str("s"),
            TimeUnit::Minute => f.write_str("m"),
            TimeUnit::Hour => f.write_str("h"),
            TimeUnit::Day => f.write_str("d"),
        }
    }
}
