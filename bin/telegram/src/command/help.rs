// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::HandlerResult;
use teloxide::prelude::Message;
use teloxide::requests::Requester;
use teloxide::Bot;

pub(crate) async fn help(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Getting help here").await?;
    Ok(())
}
