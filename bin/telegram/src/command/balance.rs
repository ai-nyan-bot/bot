// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::command::CommandResult;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{Message, Requester};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, WebAppInfo};
use teloxide::Bot;
use url::Url;

pub(crate) async fn balance(bot: Bot, msg: Message) -> CommandResult {
    let keyboard = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::web_app(
        "📊 Show details",
        WebAppInfo {
            url: Url::parse("https://nyanbot.com/balance").unwrap(),
        },
    )]]);

    bot.send_message(
        msg.chat.id,
        r#"
Your balance is: $133.70
====================================
Sol:    10 ($     230)
BTC:    23 ($ 214.413)
"#,
    )
    .reply_markup(keyboard)
    .await?;
    Ok(())
}
