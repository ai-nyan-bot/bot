// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::user::{get_or_create_another_user, get_or_create_test_user};
use base::model::{Notification, NotificationChannel, NotificationPayload, NotificationType};
use base::repo::{NotificationCreateCmd, NotificationRepo};
use common::model::Count;
use common::repo::{RepoResult, Tx};

pub async fn create_notification_for_test_user<'a>(
    tx: &mut Tx<'a>,
    ty: NotificationType,
    payload: NotificationPayload,
) -> RepoResult<Notification> {
    let test_user = get_or_create_test_user(tx).await;
    NotificationRepo::new()
        .create(
            tx,
            NotificationCreateCmd {
                user: test_user.id,
                ty,
                channel: NotificationChannel::Telegram,
                payload,
            },
        )
        .await
}

pub async fn create_notification_for_another_user<'a>(
    tx: &mut Tx<'a>,
    ty: NotificationType,
    payload: NotificationPayload,
) -> RepoResult<Notification> {
    let another_user = get_or_create_another_user(tx).await;
    NotificationRepo::new()
        .create(
            tx,
            NotificationCreateCmd {
                user: another_user.id,
                ty,
                channel: NotificationChannel::Telegram,
                payload,
            },
        )
        .await
}

pub async fn count_all<'a>(tx: &mut Tx<'a>) -> Count {
    NotificationRepo::new().count_all(tx).await.unwrap()
}
