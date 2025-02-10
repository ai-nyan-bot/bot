// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Notification, NotificationChannel, NotificationId, NotificationKind, NotificationPayload, UserId};
use crate::repo::notification::NotificationRepo;
use common::repo::{RepoResult, Tx};
use sqlx::{query, Row};

pub struct NotificationCreateCmd {
    pub user: UserId,
    pub kind: NotificationKind,
    pub channel: NotificationChannel,
    pub payload: NotificationPayload,
}

impl NotificationRepo {
    pub async fn create<'a>(&self, tx: &mut Tx<'a>, cmd: NotificationCreateCmd) -> RepoResult<Notification> {
        let notification_id = query("insert into nyanbot.notification (user_id, kind, channel, payload) values ($1, $2, $3, $4) returning id")
            .bind(cmd.user)
            .bind(cmd.kind)
            .bind(cmd.channel)
            .bind(cmd.payload)
            .fetch_one(&mut **tx)
            .await
            .map(|r| r.get::<NotificationId, _>("id"))?;

        self.get_by_id(tx, notification_id).await
    }
}
