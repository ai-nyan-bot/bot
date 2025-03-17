// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(i16)]
pub enum RuleStatus {
    // Rule is active and can be triggered any time
    Active = 1,
    // Rule was deactivated and will not be triggered
    Inactive = 2,
    // Rule is active but all invocations are exhausted, once it is not exhausted anymore it will become active again
    ActiveExhausted = 3,
    // Rule was deactivated and is exhausted, once it is not exhausted anymore it becomes inactive
    InactiveExhausted = 4,
    // Rule will not be triggered and shown in the ui
    Archived = 5,
    // Rule will not be triggered and shown in the ui - it becomes hidden once it is not exhausted anymore
    ArchivedExhausted = 6,
}

impl Display for RuleStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RuleStatus::Active => f.write_str("Active"),
            RuleStatus::Inactive => f.write_str("Inactive"),
            RuleStatus::ActiveExhausted => f.write_str("Active(exhausted)"),
            RuleStatus::InactiveExhausted => f.write_str("Inactive(exhausted)"),
            RuleStatus::Archived => f.write_str("Archived"),
            RuleStatus::ArchivedExhausted => f.write_str("Archived(exhausted)"),
        }
    }
}

impl RuleStatus {
    pub fn able_to_receive_notifications(&self) -> bool {
        match self {
            RuleStatus::Active => true,
            RuleStatus::Inactive => false,
            RuleStatus::ActiveExhausted => true,
            RuleStatus::InactiveExhausted => false,
            RuleStatus::Archived => false,
            RuleStatus::ArchivedExhausted => false,
        }
    }
}
