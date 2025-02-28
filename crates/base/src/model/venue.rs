// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, sqlx::Type)]
#[repr(i16)]
pub enum Venue {
    PumpFun = 1,
    Jupiter = 2,
    Raydium = 3,
}

impl Display for Venue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Venue::PumpFun => f.write_str("PumpFun"),
            Venue::Jupiter => f.write_str("Jupiter"),
            Venue::Raydium => f.write_str("Raydium"),
        }
    }
}
