// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::HandlerResult;
use teloxide::prelude::Message;
use teloxide::requests::Requester;
use teloxide::Bot;

pub(crate) async fn invalid(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        "Unable to handle the message. Type /help to see the usage.",
    )
    .await?;
    Ok(())
}
