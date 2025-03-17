// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

mod pumpfun;

use crate::AppState;
use base::model::{Notification, NotificationType, RuleId, Venue};
use base::service::NotificationResult;
use log::debug;

pub(crate) async fn rule_matched(
    state: AppState,
    notification: Notification,
) -> NotificationResult<()> {
    assert_eq!(notification.ty, NotificationType::RuleMatched);

    match notification.payload::<RuleId>("rule") {
        Some(rule_id) => {
            let rule = state.rule_service().get_by_id(rule_id).await?;
            if !rule.status.able_to_receive_notifications() {
                debug!("rule {} can not receive notifications", rule_id);
                return Ok(());
            }
        }
        None => {}
    };

    let user = state.user_service().get_by_id(notification.user).await?;

    if let Some(_) = &user.telegram_id {
        // let token_pair_id: TokenPairId = notification.payload("token_pair").unwrap();
        let venue: Venue = notification.payload("venue").unwrap();
        // let token_pair = state.token_service().get_pair(token_pair_id).await?;

        println!("{venue}");

        match venue {
            Venue::PumpFun => {
                return pumpfun::send(state, user, notification).await;
            }
            Venue::Jupiter => unimplemented!(),
            Venue::Raydium => unimplemented!(),
        }

        // let mint = token_pair.base.mint.clone();

        // let rule_id: RuleId = notification.payload("rule").unwrap();
        // let rule = state.rule_service().get_by_id(rule_id).await?;
        // let rule_name = rule.name;
    }
    Ok(())
}
