// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::{AppState, HandlerResult};
use teloxide::requests::Requester;
use teloxide::types::CallbackQuery;

pub(crate) async fn close_button(state: AppState, query: CallbackQuery) -> HandlerResult {
    if let Some(message) = query.message {
        state
            .bot
            .delete_message(message.chat().id, message.id())
            .await?;
    }
    state.bot.answer_callback_query(&query.id).await?;
    Ok(())
}
