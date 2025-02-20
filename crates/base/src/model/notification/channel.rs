// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, sqlx::Type)]
#[repr(i16)]
pub enum NotificationChannel {
	Telegram = 1,         // same chat as bot
	TelegramChannelOne = 2, // separate notification bot
}

impl From<i16> for NotificationChannel {
	fn from(value: i16) -> Self {
		match value {
			1 => NotificationChannel::Telegram,
			2 => NotificationChannel::TelegramChannelOne,
			_ => panic!("Invalid NotificationChannel value: {}", value),
		}
	}
}

impl From<NotificationChannel> for i16 {
	fn from(channel: NotificationChannel) -> Self {
		channel as i16
	}
}
