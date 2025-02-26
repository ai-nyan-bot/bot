// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct NotificationPayload(pub JsonValue);

impl Deref for NotificationPayload {
	type Target = JsonValue;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl From<String> for NotificationPayload {
	fn from(value: String) -> Self {
		Self(value.as_str().into())
	}
}

impl From<&str> for NotificationPayload {
	fn from(value: &str) -> Self {
		Self(JsonValue::from(value))
	}
}

impl PartialEq<&str> for NotificationPayload {
	fn eq(&self, other: &&str) -> bool {
		match self.0.as_str() {
			None => false,
			Some(s) => s == *other,
		}
	}
}