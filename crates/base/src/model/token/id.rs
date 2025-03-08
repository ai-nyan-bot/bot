// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

#[derive(
    Eq, Hash, Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Deserialize, Serialize, sqlx::Type,
)]
#[sqlx(transparent)]
pub struct TokenId(pub i64);

impl AsRef<TokenId> for TokenId {
    fn as_ref(&self) -> &TokenId {
        self
    }
}

impl PartialEq<i64> for TokenId {
    fn eq(&self, other: &i64) -> bool {
        self.0 == *other
    }
}

impl From<i64> for TokenId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

#[derive(
    Eq, Hash, Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Deserialize, Serialize, sqlx::Type,
)]
#[sqlx(transparent)]
pub struct TokenPairId(pub i64);

impl Display for TokenPairId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl PartialEq<i64> for TokenPairId {
    fn eq(&self, other: &i64) -> bool {
        self.0 == *other
    }
}

impl From<i64> for TokenPairId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
