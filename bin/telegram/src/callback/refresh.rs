// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::callback::{Callback, CallbackRefresh, CallbackResult};
use crate::message::send_pumpfun_summary;
use crate::AppState;
use teloxide::prelude::CallbackQuery;
use teloxide::requests::Requester;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use url::Url;

pub(crate) async fn refresh_button(
    state: AppState,
    callback: CallbackRefresh,
    query: CallbackQuery,
) -> CallbackResult {
    match callback {
        CallbackRefresh::PumpfunSummary { pair } => {
            let message = query.message.unwrap();

            let pair = state.token_service().get_pair(pair).await?;
            let base_mint = pair.base.mint;

            send_pumpfun_summary(
                state.clone(),
                message.chat().id,
                pair.id,
                Some(InlineKeyboardMarkup::new(vec![
                    vec![InlineKeyboardButton::callback(
                        "⟳ Refresh",
                        state
                            .callback_store
                            .store(Callback::Refresh(CallbackRefresh::PumpfunSummary {
                                pair: pair.id,
                            }))
                            .await,
                    )],
                    vec![InlineKeyboardButton::url(
                        "💰 Buy on pump.fun",
                        Url::parse(format!("https://pump.fun/{base_mint}").as_str()).unwrap(),
                    )],
                ])),
            )
            .await?;

            state
                .bot
                .delete_message(message.chat().id, message.id())
                .await?;
        }
    }

    state.bot.answer_callback_query(&query.id).await?;
    Ok(())
}
