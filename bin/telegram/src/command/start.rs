// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::i18n::{Language, I18N};
use crate::state::AppState;
use crate::HandlerResult;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::Message;
use teloxide::requests::Requester;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, WebAppInfo};
use teloxide::Bot;
use url::Url;

pub(crate) async fn start(bot: Bot, msg: Message, state: AppState) -> HandlerResult {
    let i18n = I18N::load(Language::En).await;

    let options = InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::web_app(
            i18n.button_text_rules,
            WebAppInfo {
                url: Url::parse("https://telegram.nyan.bot/rules").unwrap(),
            },
        )],
        // vec![
        //     InlineKeyboardButton::callback(format!("↑ {}", i18n.button_text_send), "send"),
        //     InlineKeyboardButton::callback(format!("↓ {}", i18n.button_text_receive), "receive"),
        //     InlineKeyboardButton::web_app(
        //         "↑↓ Swap",
        //         WebAppInfo {
        //             url: Url::parse("https://nyanbot.com/wallet/swap").unwrap(),
        //         },
        //     ),
        // ],
        // vec![
        //     InlineKeyboardButton::callback(i18n.start, "send"),
        //     InlineKeyboardButton::callback(i18n.help, "receive"),
        // ],
        // vec![
        // InlineKeyboardButton::callback(format!("📊 {}", i18n.button_text_balance), "balance"),
        // InlineKeyboardButton::callback(format!("🏦 {}", i18n.button_text_wallet), "wallet"),
        // ],
    ]);

    if let Some(user) = msg.from {
        if !user.is_bot {
            // println!("User {user:#?} started");
            // println!("{:#?}", state.config);
            let (_user, _wallet, _created) = state
                .user_service()
                .get_or_create_telegram_user(user.id.0)
                .await
                .unwrap();
        }
    }

    bot.send_message(msg.chat.id, "Nyanbot")
        .reply_markup(options)
        .await?;
    Ok(())
}
