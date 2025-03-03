// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

mod rule_matched;

use crate::notify::rule_matched::rule_matched;
use crate::AppState;
use base::model::{Notification, NotificationType};
use base::service::NotificationResult;
use common::Signal;
use log::info;
use std::time::Duration;
use tokio::select;
use tokio::task::JoinHandle;

pub fn notify(state: AppState, mut signal: Signal) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        loop {
            select! {
                _ = signal.recv() => {
                    info!("Signal received");
                    break;
                }
                _ = interval.tick() => {
                    next_notifications(state.clone()).await
                }
            }
        }
    })
}

async fn next_notifications(state: AppState) {
    let _ = state
        .notification_service()
        .pop(1, {
            let state = state.clone();
            move |notification| send_notification(state.clone(), notification)
        })
        .await;
}

pub async fn send_notification(
    state: AppState,
    notification: Notification,
) -> NotificationResult<()> {
    let state = state.clone();
    match notification.ty {
        NotificationType::RuleMatched => rule_matched(state, notification).await?,
    };
    Ok(())
}
