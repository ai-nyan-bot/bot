// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, sqlx::Type)]
#[repr(i16)]
pub enum NotificationKind {
    ConditionMet = 1,
}

impl From<i16> for NotificationKind {
    fn from(value: i16) -> Self {
        match value {
            1 => NotificationKind::ConditionMet,
            _ => panic!("Invalid NotificationKind value: {}", value),
        }
    }
}

impl From<NotificationKind> for i16 {
    fn from(kind: NotificationKind) -> Self {
        kind as i16
    }
}
