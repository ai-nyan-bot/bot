// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Notification, NotificationId, NotificationKind, NotificationPayload};
use crate::model::{NotificationChannel, UserId};
use crate::repo::notification::NotificationRepo;
use common::model::CreatedAt;
use common::repo::{RepoResult, Tx};
use sqlx::{query, Row};

impl NotificationRepo {
    pub async fn get_by_id<'a>(&self, tx: &mut Tx<'a>, id: impl Into<NotificationId> + Send) -> RepoResult<Notification> {
        Ok(query("select * from nyanbot.notification where id = $1;")
            .bind(id.into())
            .fetch_one(&mut **tx)
            .await
            .map(|r| Notification {
                id: r.get::<NotificationId, _>("id"),
                user: r.get::<UserId, _>("user_id"),
                channel: r.get::<NotificationChannel, _>("channel"),
                kind: r.get::<NotificationKind, _>("kind"),
                payload: r.get::<NotificationPayload, _>("payload"),
                created_at: r.get::<CreatedAt, _>("created_at"),
            })?)
    }
}
