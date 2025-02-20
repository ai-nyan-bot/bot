// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Notification, NotificationChannel, NotificationId, NotificationKind, NotificationPayload, UserId};
use crate::repo::NotificationRepo;
use common::model::{CreatedAt, Limit};
use common::repo::{RepoResult, Tx};
use sqlx::{query, Row};

impl NotificationRepo {
    pub async fn pop<'a>(&self, tx: &mut Tx<'a>, limit: impl Into<Limit>) -> RepoResult<Box<[Notification]>> {
        let limit = limit.into();
        Ok(query(
            r#"
            delete from nyanbot.notification
            where id in (
                select id from nyanbot.notification
                order by id ASC
                limit $1
            )
            returning id, user_id, kind, channel, payload, created_at
            "#,
        )
        .bind(limit)
        .fetch_all(&mut **tx)
        .await?
        .into_iter()
        .map(|r| Notification {
            id: r.get::<NotificationId, _>("id"),
            user: r.get::<UserId, _>("user_id"),
            channel: r.get::<NotificationChannel, _>("channel"),
            kind: r.get::<NotificationKind, _>("kind"),
            payload: r.get::<NotificationPayload, _>("payload"),
            created_at: r.get::<CreatedAt, _>("created_at"),
        })
        .collect::<Vec<_>>()
        .into_boxed_slice())
    }
}
