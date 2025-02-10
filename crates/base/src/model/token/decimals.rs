// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Decimals(pub i16);

impl From<i16> for Decimals {
    fn from(value: i16) -> Self {
        Self(value)
    }
}

impl PartialEq<i16> for Decimals {
    fn eq(&self, other: &i16) -> bool {
        self.0 == *other
    }
}
