// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};

#[derive(
    Eq, Hash, Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Deserialize, Serialize, sqlx::Type,
)]
#[sqlx(transparent)]
pub struct AddressId(pub i64);

impl AsRef<AddressId> for AddressId {
    fn as_ref(&self) -> &AddressId {
        self
    }
}

impl PartialEq<i64> for AddressId {
    fn eq(&self, other: &i64) -> bool {
        self.0 == *other
    }
}

impl From<i64> for AddressId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
