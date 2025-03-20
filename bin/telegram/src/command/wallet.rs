// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::command::CommandResult;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::Message;
use teloxide::requests::Requester;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, WebAppInfo};
use teloxide::Bot;
use url::Url;

pub(crate) async fn wallet(bot: Bot, msg: Message) -> CommandResult {
    let keyboard = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::web_app(
        "🏦 Open",
        WebAppInfo {
            url: Url::parse("https://nyanbot.com/wallet").unwrap(),
        },
    )]]);

    bot.send_message(
        msg.chat_id().unwrap(),
        r#"
Your solana address is: 0123...6789
"#,
    )
    .reply_markup(keyboard)
    .await?;
    Ok(())
}
