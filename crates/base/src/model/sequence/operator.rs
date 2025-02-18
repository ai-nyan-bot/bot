// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Operator {
    Equal,
    NotEqual,

    IncreasedByMoreThan,
    IncreasedByMoreThanEqual,
    IncreasedByLessThan,
    IncreasedByLessThanEqual,

    DecreasedByMoreThan,
    DecreasedByMoreThanEqual,
    DecreasedByLessThan,
    DecreasedByLessThanEqual,

    MoreThan,
    MoreThanEqual,

    LessThan,
    LessThanEqual,
}
