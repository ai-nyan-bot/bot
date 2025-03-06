// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Volume(pub f64);

impl From<f64> for Volume {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl PartialEq<f64> for Volume {
    fn eq(&self, other: &f64) -> bool {
        self.0 == *other
    }
}
