// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, sqlx::Type)]
#[repr(i16)]
pub enum NotificationType {
    RuleMatched = 1,
}

impl From<i16> for NotificationType {
    fn from(value: i16) -> Self {
        match value {
            1 => NotificationType::RuleMatched,
            _ => panic!("Invalid NotificationType: {}", value),
        }
    }
}

impl From<NotificationType> for i16 {
    fn from(ty: NotificationType) -> Self {
        ty as i16
    }
}
