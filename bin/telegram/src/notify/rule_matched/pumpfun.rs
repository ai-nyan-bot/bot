// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::callback::{Callback, CallbackRefresh};
use crate::message::{send_pumpfun_summary, MessageError};
use crate::AppState;
use base::model::{Notification, TokenPairId, User};
use base::service::{NotificationError, NotificationResult};
use teloxide::types::{ChatId, InlineKeyboardButton, InlineKeyboardMarkup, Recipient};
use url::Url;

pub(crate) async fn send(
    state: AppState,
    user: User,
    notification: Notification,
) -> NotificationResult<()> {
    let telegram_id = user.telegram_id.expect("missing telegram id");
    let pair: TokenPairId = notification.payload("token_pair").unwrap();
    let pair = state.token_service().get_pair(pair).await?;
    let base_mint = pair.base.mint;

    match send_pumpfun_summary(
        state.clone(),
        Recipient::Id(ChatId(telegram_id.0)),
        pair.id,
        Some(InlineKeyboardMarkup::new(vec![
            vec![InlineKeyboardButton::callback(
                "⟳ Refresh",
                state
                    .callback_store
                    .store(Callback::Refresh(CallbackRefresh::PumpfunSummary {
                        pair: pair.id,
                    }))
                    .await,
            )],
            vec![InlineKeyboardButton::url(
                "💰 Buy on pump.fun",
                Url::parse(format!("https://pump.fun/{base_mint}").as_str()).unwrap(),
            )],
        ])),
    )
    .await
    {
        Ok(_) => {}
        Err(err) => {
            return match err {
                MessageError::UnableToSend(msg) => Err(NotificationError::Ignore(msg)),
                MessageError::Unknown(msg) => Err(NotificationError::Rollback(msg)),
            }
        }
    }

    Ok(())
}

// async fn create_keyboard(
//     store: CallbackStore,
//     notification: &Notification,
// ) -> InlineKeyboardMarkup {
//     let buttons: Vec<Option<TelegramActionButtonConfig>> = (0..6)
//         .map(|i| notification.payload::<TelegramActionButtonConfig>(format!("button_{i}").as_str()))
//         .collect::<Vec<_>>();
//
//     let mut rows: Vec<Vec<InlineKeyboardButton>> = vec![vec![], vec![]];
//
//     for (i, button_config) in buttons.iter().enumerate() {
//         let row_index = if i < 3 { 0 } else { 1 };
//         let button = match button_config {
//             Some(TelegramActionButtonConfig::None) | None => {
//                 InlineKeyboardButton::callback(" ", IGNORE_CALLBACK)
//             }
//             Some(cfg) => InlineKeyboardButton::callback(
//                 text(cfg),
//                 store
//                     .store(Callback::ActionButton(CallbackActionButton {
//                         config: cfg.clone(),
//                     }))
//                     .await,
//             ),
//         };
//         rows[row_index].push(button);
//     }
//
//     let all_none = buttons
//         .iter()
//         .all(|cfg| matches!(cfg, Some(TelegramActionButtonConfig::None) | None));
//     if all_none {
//         InlineKeyboardMarkup::default()
//     } else {
//         InlineKeyboardMarkup::new(rows)
//     }
// }
//
// fn text(cfg: &TelegramActionButtonConfig) -> String {
//     match cfg {
//         TelegramActionButtonConfig::None => unreachable!(),
//         TelegramActionButtonConfig::Buy { value } => format!("Buy: {value}"),
//         TelegramActionButtonConfig::Sell { value } => format!("Sell: {value}"),
//     }
// }
