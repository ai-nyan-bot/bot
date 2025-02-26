// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::UserId;
use common::model::CreatedAt;
use serde::de::DeserializeOwned;

pub use channel::*;
pub use id::*;
pub use payload::*;
pub use r#type::*;
pub use telegram::*;

mod channel;
mod id;
mod payload;
mod telegram;
mod r#type;

#[derive(Debug)]
pub struct Notification {
    pub id: NotificationId,
    pub user: UserId,
    pub channel: NotificationChannel,
    pub ty: NotificationType,
    pub payload: NotificationPayload,
    pub created_at: CreatedAt,
}

impl Notification {
    pub fn payload<T: DeserializeOwned>(&self, index: &str) -> Option<T> {
        let value = self.payload.0.get(index)?;
        serde_json::from_value::<T>(value.clone()).ok()
    }
}
