// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::callback::CallbackRefresh;
use crate::{AppState, HandlerResult};
use teloxide::prelude::CallbackQuery;
use teloxide::requests::Requester;

pub(crate) async fn refresh_button(
    state: AppState,
    callback: CallbackRefresh,
    query: CallbackQuery,
) -> HandlerResult {
    println!("Refresh {callback:?}");
    // if let Some(message) = query.message {
    //     // state
    //     // 	.bot
    //     // 	.delete_message(message.chat().id, message.id())
    //     // 	.await?;
    // }
    state.bot.answer_callback_query(&query.id).await?;
    Ok(())
}
