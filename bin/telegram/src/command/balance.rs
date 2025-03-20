// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::command::CommandResult;
use crate::AppState;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{Message, Requester};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, ParseMode, WebAppInfo};
use url::Url;

pub(crate) async fn balance(state: AppState, msg: Message) -> CommandResult {
    let keyboard = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::web_app(
        "📊 Show details",
        WebAppInfo {
            url: Url::parse("https://telegram.nyan.bot/rules").unwrap(),
        },
    )]]);

    state
        .bot
        .send_message(
            msg.chat.id,
            r#"
Balance of $133
"#,
        )
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}
