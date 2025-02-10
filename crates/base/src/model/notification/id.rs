// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};

#[derive(Eq, Hash, Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct NotificationId(pub i32);

impl AsRef<NotificationId> for NotificationId {
    fn as_ref(&self) -> &NotificationId {
        &self
    }
}

impl PartialEq<i32> for NotificationId {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl From<i32> for NotificationId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
