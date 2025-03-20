// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::callback::{Callback, CallbackRefresh};
use crate::command::CommandResult;
use crate::message::send_pumpfun_summary;
use crate::AppState;
use base::model::TokenPairId;
use regex::Regex;
use teloxide::prelude::Message;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use url::Url;

pub(crate) async fn token(state: AppState, msg: Message) -> CommandResult {
    if let Some(text) = msg.text() {
        let re = Regex::new(r"^/token (\d+)$").unwrap();

        if let Some(caps) = re.captures(text) {
            let number = caps.get(1).unwrap().as_str();

            let pair: TokenPairId = TokenPairId::from(number.parse::<i64>().unwrap());
            let pair = state.token_service().get_pair(pair).await?;
            let base_mint = pair.base.mint;

            send_pumpfun_summary(
                state.clone(),
                msg.chat.id,
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
        } else {
            println!("No match.");
        }
    }

    Ok(())
}
