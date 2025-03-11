// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Timeframe {
    S1,
    M1,
    M5,
    M15,
    H1,
    H6,
    D1,
}

impl Timeframe {
    pub fn all() -> [Timeframe; 7] {
        [
            Timeframe::S1,
            Timeframe::M1,
            Timeframe::M5,
            Timeframe::M15,
            Timeframe::H1,
            Timeframe::H6,
            Timeframe::D1,
        ]
    }
}

impl Timeframe {
    pub fn table(&self) -> &'static str {
        match self {
            Timeframe::S1 => "1s",
            Timeframe::M1 => "1m",
            Timeframe::M5 => "5m",
            Timeframe::M15 => "15m",
            Timeframe::H1 => "1h",
            Timeframe::H6 => "6h",
            Timeframe::D1 => "1d",
        }
    }
}

impl Distribution<Timeframe> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Timeframe {
        let variants = Timeframe::all();
        variants[rng.gen_range(0..variants.len())]
    }
}
