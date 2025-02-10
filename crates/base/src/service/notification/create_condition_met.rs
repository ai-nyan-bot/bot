// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::TokenPairId;
use crate::model::{NotificationChannel, NotificationKind, NotificationPayload, UserId};
use crate::repo::NotificationCreateCmd;
use crate::service::notification::NotificationService;
use common::repo::Tx;
use common::service::ServiceResult;
use serde_json::Map;
use sqlx::types::JsonValue;

pub struct NotificationConditionMet {
    pub user: UserId,
    pub token_pair: TokenPairId,
}

impl NotificationService {

    pub async fn create_condition_met(&self, notification: NotificationConditionMet) -> ServiceResult<()> {
        let mut tx = self.pool.begin().await?;
        self.create_condition_met_tx(&mut tx, notification).await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn create_condition_met_tx<'a>(&self, tx: &mut Tx<'a>, notification: NotificationConditionMet) -> ServiceResult<()> {
        self.repo
            .create(
                tx,
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
        Ok(())
    }
    
}
