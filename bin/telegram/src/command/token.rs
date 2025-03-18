// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::callback::{Callback, CallbackRefresh};
use crate::{markdown, AppState, HandlerResult};
use base::model::TokenPairId;
use base::service::NotificationError;
use regex::Regex;
use render::page::pumpfun::{pumpfun_summary, PumpfunSummary};
use render::render;
use teloxide::payloads::SendPhotoSetters;
use teloxide::prelude::{ChatId, Message};
use teloxide::requests::Requester;
use teloxide::types::{
    InlineKeyboardButton, InlineKeyboardMarkup, InputFile, ParseMode, Recipient,
};
use teloxide::{ApiError, Bot, RequestError};
use url::Url;

pub(crate) async fn token(_bot: Bot, msg: Message, state: AppState) -> HandlerResult {
    if let Some(text) = msg.text() {
        let re = Regex::new(r"^/token (\d+)$").unwrap();

        if let Some(caps) = re.captures(text) {
            let number = caps.get(1).unwrap().as_str();
            println!("Extracted number: {}", number);

            let token_pair_id: TokenPairId = TokenPairId::from(number.parse::<i64>().unwrap());

            let telegram_id = msg.chat.id;

            let token_summary = state
                .pumpfun_token_service()
                .summarise(token_pair_id)
                .await
                .unwrap();

            let symbol = token_summary.pair.symbol().to_string();

            let base_mint = token_summary.pair.base.mint.clone();
            let buttons = InlineKeyboardMarkup::new(vec![vec![
                InlineKeyboardButton::callback(
                    "⟳ Refresh Summary",
                    state
                        .callback_store
                        .store(Callback::Refresh(CallbackRefresh::PumpfunSummary {
                            pair: token_pair_id,
                        }))
                        .await,
                ),
                InlineKeyboardButton::url(
                    "💰 Buy on pump.fun",
                    Url::parse(format!("https://pump.fun/{base_mint}").as_str()).unwrap(),
                ),
            ]]);

            let progress = token_summary.current.progress;
            let progress = format!("{:.2}", progress);

            let caption = markdown!(
                r#"
        ;*{symbol};*
        is ;*{progress}%;* along the bonding curve and on its way to graduate to Raydium 🔥🚀
    "#
            );

            // println!("{}", text);

            let image_path = render(|img| {
                pumpfun_summary(
                    img,
                    PumpfunSummary {
                        m1: token_summary.m1,
                        m5: token_summary.m5,
                        m15: token_summary.m15,
                        h1: token_summary.h1,
                        h6: token_summary.h6,
                        d1: token_summary.d1,
                    },
                )
            })
            .await
            .unwrap();

            let file = InputFile::file(image_path);

            let _x = state
                .bot
                .send_photo(Recipient::Id(ChatId(telegram_id.0)), file)
                .caption(caption)
                .parse_mode(ParseMode::MarkdownV2)
                // .reply_markup(create_keyboard(state.callback_store.clone(), &notification).await)
                .reply_markup(buttons)
                .await
                .map_err(|err| match err {
                    RequestError::Api(err) => match err {
                        ApiError::BotBlocked
                        | ApiError::InvalidToken
                        | ApiError::MessageNotModified
                        | ApiError::BotKicked
                        | ApiError::BotKickedFromSupergroup
                        | ApiError::UserDeactivated
                        | ApiError::CantTalkWithBots => NotificationError::Ignore(err.to_string()),

                        _ => NotificationError::Rollback(err.to_string()),
                    },
                    _ => NotificationError::Rollback(err.to_string()),
                })?;
        } else {
            println!("No match.");
        }
    }

    Ok(())
}
