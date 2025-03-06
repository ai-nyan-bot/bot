// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Change(pub BigDecimal);

impl From<i32> for Change {
    fn from(value: i32) -> Self {
        Self(BigDecimal::from(value))
    }
}

impl PartialEq<i32> for Change {
    fn eq(&self, other: &i32) -> bool {
        self.0 == BigDecimal::from(*other)
    }
}
