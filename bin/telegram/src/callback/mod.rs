// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::callback::action_button::action_button;
use crate::callback::close::close_button;
use crate::callback::refresh::refresh_button;
use crate::{AppState};
use base::model::{TelegramActionButtonConfig, TokenPairId};
pub use store::CallbackStore;
use teloxide::payloads::EditMessageReplyMarkupSetters;
use teloxide::prelude::CallbackQuery;
use teloxide::prelude::Requester;
use teloxide::types::InlineKeyboardMarkup;

mod action_button;
mod close;
mod refresh;
mod store;

#[derive(Clone, Debug, PartialEq)]
pub enum Callback {
    ActionButton(CallbackActionButton),
    Close,
    Refresh(CallbackRefresh),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CallbackRefresh {
    PumpfunSummary { pair: TokenPairId },
}

#[derive(Clone, Debug, PartialEq)]
pub struct CallbackActionButton {
    pub config: TelegramActionButtonConfig,
}

pub type CallbackResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;


pub const IGNORE_CALLBACK: &str = "@!IGNORE!@";

pub(crate) async fn callback(state: AppState, query: CallbackQuery) -> CallbackResult {
    if let Some(data) = query.data.as_ref() {
        if data == IGNORE_CALLBACK {
            state.bot.answer_callback_query(query.id.clone()).await?;
            return Ok(());
        }

        if let Some(callback) = state.callback_store.peak(data).await {
            match callback {
                Callback::ActionButton(cb) => action_button(state, cb, query).await?,
                Callback::Close => close_button(state, query).await?,
                Callback::Refresh(callback) => refresh_button(state, callback, query).await?,
            }
        } else if let Some(msg) = &query.message {
            let bot = state.bot.clone();

            bot.answer_callback_query(query.id.clone()).await?;

            bot.edit_message_text(
                msg.chat().id,
                msg.id(),
                r#"
        ⚠️ Button Expired!
        This button was valid for 15 minutes to help prevent accidental actions.
                    "#
                .to_string(),
            )
            .await?;

            bot.edit_message_reply_markup(msg.chat().id, msg.id())
                .reply_markup(InlineKeyboardMarkup::default())
                .await?;
        }
    }

    Ok(())
}
