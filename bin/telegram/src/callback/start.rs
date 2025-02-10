// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::{HandlerResult, MessageState, MyDialog};
use teloxide::prelude::CallbackQuery;
use teloxide::requests::Requester;
use teloxide::Bot;

pub(crate) async fn start(
    bot: Bot,
    dialog: MyDialog,
    state: MessageState,
    q: CallbackQuery,
) -> HandlerResult {
    bot.answer_callback_query(&q.id).await?;

    println!("Called back");

    // if let Some(data) = &q.data {
    //     match data.as_str() {
    //         "balance" => show_balance(bot, dialog.chat_id()).await?,
    //         "wallet" => show_wallet(bot, dialog.chat_id()).await?,
    //         _ => unimplemented!(),
    //     }
    // }
    //

    // bot.send_message(dialog.chat_id(), "Let's start! What's your full name?").await?;
    // dialog.update(State::ReceiveFullName).await?;
    Ok(())
}
