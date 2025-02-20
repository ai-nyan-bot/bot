// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::de::DeserializeOwned;
use crate::model::UserId;
use common::model::CreatedAt;

pub use channel::*;
pub use id::*;
pub use kind::*;
pub use payload::*;
pub use telegram::*;

mod channel;
mod id;
mod kind;
mod payload;
mod telegram;

#[derive(Debug)]
pub struct Notification {
	pub id: NotificationId,
	pub user: UserId,
	pub channel: NotificationChannel,
	pub kind: NotificationKind,
	pub payload: NotificationPayload,
	pub created_at: CreatedAt,
}

impl Notification {
	pub fn payload<T: DeserializeOwned>(&self, index: &'static str) -> Option<T> {
		let value= self.payload.0.get(index)?;
		serde_json::from_value::<T>(value.clone()).ok()
	}
}