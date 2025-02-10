// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Timeframe {
    S1,
    M1,
    M5,
    M15,
    H1,
    H4,
    D1,
    W1,
}
