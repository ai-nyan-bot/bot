// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::callback::{Callback, CallbackActionButton, CallbackStore, IGNORE_CALLBACK};
use crate::AppState;
use base::model::{
    Notification, NotificationType, RuleId, TelegramActionButtonConfig, TokenPairId,
};
use base::service::NotificationResult;
use teloxide::payloads::SendMessageSetters;
use teloxide::requests::Requester;
use teloxide::types::{ChatId, InlineKeyboardButton, InlineKeyboardMarkup, ParseMode, Recipient};

pub(crate) async fn rule_matched(
    state: AppState,
    notification: Notification,
) -> NotificationResult<()> {
    assert_eq!(notification.ty, NotificationType::RuleMatched);

    let user = state.user_service().get_by_id(notification.user).await?;

    if let Some(telegram_id) = user.telegram_id {
        let token_pair_id: TokenPairId = notification.payload("token_pair").unwrap();
        let token_pair = state.token_service().get_pair(token_pair_id).await?;
        dbg!(&token_pair);

        let rule_id: RuleId = notification.payload("rule").unwrap();
        let rule = state.rule_service().get_by_id(rule_id).await?;
        let rule_name = rule.name;

        let x = state
            .bot
            .send_message(
                Recipient::Id(ChatId(telegram_id.0.parse::<i64>().unwrap())),
                format!(
                    r#"
*{rule_name}* matched: [{token_pair}](https://nyan.bot)
"#
                ),
            )
            .parse_mode(ParseMode::MarkdownV2)
            .reply_markup(create_keyboard(state.callback_store.clone(), &notification).await)
            .await
            .unwrap();
    }

    dbg!(&notification);
    Ok(())
}

async fn create_keyboard(
    store: CallbackStore,
    notification: &Notification,
) -> InlineKeyboardMarkup {
    let buttons: Vec<Option<TelegramActionButtonConfig>> = (0..6)
        .map(|i| notification.payload::<TelegramActionButtonConfig>(format!("button_{i}").as_str()))
        .collect::<Vec<_>>();

    let mut rows: Vec<Vec<InlineKeyboardButton>> = vec![vec![], vec![]];

    for (i, button_config) in buttons.iter().enumerate() {
        let row_index = if i < 3 { 0 } else { 1 };
        let button = match button_config {
            Some(TelegramActionButtonConfig::None) | None => {
                InlineKeyboardButton::callback(" ", IGNORE_CALLBACK)
            }
            Some(cfg) => InlineKeyboardButton::callback(
                text(cfg),
                store
                    .store(Callback::ActionButton(CallbackActionButton {
                        config: cfg.clone(),
                    }))
                    .await,
            ),
        };
        rows[row_index].push(button);
    }

    let all_none = buttons
        .iter()
        .all(|cfg| matches!(cfg, Some(TelegramActionButtonConfig::None) | None));
    if all_none {
        InlineKeyboardMarkup::default()
    } else {
        InlineKeyboardMarkup::new(rows)
    }
}

fn text(cfg: &TelegramActionButtonConfig) -> String {
    match cfg {
        TelegramActionButtonConfig::None => unreachable!(),
        TelegramActionButtonConfig::Buy { value } => format!("Buy: {value}"),
        TelegramActionButtonConfig::Sell { value } => format!("Sell: {value}"),
    }
}
