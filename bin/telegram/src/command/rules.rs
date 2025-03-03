// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::{AppState, HandlerResult};
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{Message, Requester};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, WebAppInfo};
use teloxide::Bot;
use url::Url;

pub(crate) async fn rules(bot: Bot, msg: Message, _state: AppState) -> HandlerResult {
    let options = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::web_app(
        "Open rules".to_string(),
        WebAppInfo {
            url: Url::parse("https://telegram.nyan.bot/rules").unwrap(),
        },
    )]]);

    bot.send_message(msg.chat.id, r#"Rules"#)
        .reply_markup(options)
        .await?;
    Ok(())
}
