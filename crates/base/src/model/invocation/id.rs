// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};

#[derive(Eq, Hash, Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct InvocationId(pub i64);

impl AsRef<InvocationId> for InvocationId {
    fn as_ref(&self) -> &InvocationId {
        self
    }
}

impl PartialEq<i64> for InvocationId {
    fn eq(&self, other: &i64) -> bool {
        self.0 == *other
    }
}

impl From<i64> for InvocationId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
