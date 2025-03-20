// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::callback::{Callback, CallbackRefresh};
use crate::{markdown, AppState, HandlerResult};
use base::service::NotificationError;
use render::page::pumpfun::{pumpfun_summary, PumpfunSummary};
use render::render;
use teloxide::payloads::SendPhotoSetters;
use teloxide::prelude::CallbackQuery;
use teloxide::requests::Requester;
use teloxide::types::{
    InlineKeyboardButton, InlineKeyboardMarkup, InputFile, ParseMode, Recipient,
};
use teloxide::{ApiError, RequestError};
use tokio::time::Instant;
use url::Url;

pub(crate) async fn refresh_button(
    state: AppState,
    callback: CallbackRefresh,
    query: CallbackQuery,
) -> HandlerResult {
    println!("Refresh {callback:?}");

    match callback {
        CallbackRefresh::PumpfunSummary { pair } => {
            let start = Instant::now();

            let token_summary = state.pumpfun_token_service().summarise(pair).await.unwrap();

            let symbol = token_summary.pair.symbol().to_string();

            let base_mint = token_summary.pair.base.mint.clone();
            let buttons = InlineKeyboardMarkup::new(vec![vec![
                InlineKeyboardButton::callback(
                    "⟳ Refresh Summary",
                    state
                        .callback_store
                        .store(Callback::Refresh(CallbackRefresh::PumpfunSummary { pair }))
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

            let message = query.message.unwrap();

            let _x = state
                .bot
                .send_photo(Recipient::Id(message.chat().id), file)
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

            state
                .bot
                .delete_message(message.chat().id, message.id())
                .await?;

            println!("Refreshed successfully {} ms", start.elapsed().as_millis());
        }
    }

    // if let Some(message) = query.message {
    //     // state
    //     // 	.bot
    //     // 	.delete_message(message.chat().id, message.id())
    //     // 	.await?;
    // }
    state.bot.answer_callback_query(&query.id).await?;
    Ok(())
}
