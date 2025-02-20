// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

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
