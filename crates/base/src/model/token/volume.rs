// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use bigdecimal::{BigDecimal, FromPrimitive};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Type)]
#[sqlx(transparent)]
pub struct Volume(pub BigDecimal);

impl From<f64> for Volume {
    fn from(value: f64) -> Self {
        BigDecimal::from_f64(value)
            .map(Self)
            .expect("Failed to create BigDecimal from f64")
    }
}

impl PartialEq<f64> for Volume {
    fn eq(&self, other: &f64) -> bool {
        BigDecimal::from_f64(*other)
            .map(|bd| self.0 == bd)
            .unwrap_or(false)
    }
}

impl PartialOrd<f64> for Volume {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        BigDecimal::from_f64(*other).and_then(|bd| self.0.partial_cmp(&bd))
    }
}
