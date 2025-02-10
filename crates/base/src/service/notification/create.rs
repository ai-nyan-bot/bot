// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{NotificationChannel, NotificationKind, NotificationPayload, UserId};
use crate::repo::NotificationCreateCmd;
use crate::service::notification::NotificationService;
use common::model::TokenPairId;
use common::service::ServiceResult;
use serde_json::Map;
use sqlx::types::JsonValue;

pub struct NotificationConditionMet {
    pub user: UserId,
    pub token_pair: TokenPairId,
}

impl NotificationService {
    pub async fn condition_met(&self, notification: NotificationConditionMet) -> ServiceResult<()> {
        let mut tx = self.pool.begin().await?;
        self.repo
            .create(
                &mut tx,
                NotificationCreateCmd {
                    user: notification.user,
                    kind: NotificationKind::ConditionMet,
                    channel: NotificationChannel::Telegram,
                    payload: NotificationPayload(JsonValue::Object({
                        let mut map = Map::new();
                        map.insert("token_pair_id".to_string(), JsonValue::String(notification.token_pair.to_string()));
                        map
                    })),
                },
            )
            .await?;
        tx.commit().await?;
        Ok(())
    }
}
