// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::{HandlerResult, MyDialog};
use teloxide::prelude::Message;
use teloxide::requests::Requester;
use teloxide::Bot;

pub(crate) async fn cancel(bot: Bot, dialog: MyDialog, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Cancelling the dialog.")
        .await?;
    dialog.exit().await?;
    Ok(())
}
