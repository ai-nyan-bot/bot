// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use teloxide::prelude::Message;
use teloxide::requests::Requester;
use teloxide::Bot;
use crate::command::CommandResult;

pub(crate) async fn help(bot: Bot, msg: Message) -> CommandResult {
    bot.send_message(msg.chat.id, "Getting help here").await?;
    Ok(())
}
