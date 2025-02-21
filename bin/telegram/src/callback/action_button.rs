// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::callback::CallbackActionButton;
use crate::{AppState, HandlerResult};
use base::model::TelegramActionButtonConfig;
use std::time::Duration;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::payloads::{AnswerCallbackQuerySetters, EditMessageReplyMarkupSetters};
use teloxide::prelude::CallbackQuery;
use teloxide::requests::Requester;
use teloxide::types::InlineKeyboardMarkup;
use tokio::time::sleep;

pub(crate) async fn action_button(
    state: AppState,
    callback: CallbackActionButton,
    query: CallbackQuery,
) -> HandlerResult {

    if let Some(msg) = &query.message {
        state
            .bot
            .edit_message_reply_markup(msg.chat().id, msg.id())
            .reply_markup(InlineKeyboardMarkup::default())
            .await?;
    }

    match callback.config {
        TelegramActionButtonConfig::None => unreachable!(),
        TelegramActionButtonConfig::Buy { value } => {
            let message = state
                .bot
                .send_message(query.chat_id().unwrap(), format!("Buying: {value}"))
                .await
                .unwrap();

            sleep(Duration::from_secs(3)).await;

            // if let Some(message) = query.message {
            state
                .bot
                .edit_message_text(
                    message.chat_id().unwrap(),
                    message.id,
                    format!("✅ Bought: {value}"),
                )
                .await?;

            if let Some(msg) = &query.message {
                state
                    .bot
                    .edit_message_reply_markup(msg.chat().id, msg.id())
                    .reply_markup(InlineKeyboardMarkup::default())
                    .await?;

                state
                    .bot
                    .answer_callback_query(query.id.clone())
                    .show_alert(false)
                    .await?;
            }
        }
        TelegramActionButtonConfig::Sell { value } => {
            state
                .bot
                .send_message(query.chat_id().unwrap(), format!("Sell: {value}"))
                .await
                .unwrap();
        }
    }

    state.bot.answer_callback_query(&query.id).await?;
    Ok(())
}
