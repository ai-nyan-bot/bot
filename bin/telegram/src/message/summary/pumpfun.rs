// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::message::MessageResult;
use crate::{markdown, AppState};
use base::model::TokenPairId;
use render::page::pumpfun::{pumpfun_summary, PumpfunSummary};
use render::render;
use teloxide::payloads::SendPhotoSetters;
use teloxide::prelude::Requester;
use teloxide::types::{InlineKeyboardMarkup, InputFile, ParseMode, Recipient};

pub async fn send_pumpfun_summary(
    state: AppState,
    recipient: impl Into<Recipient>,
    pair: impl Into<TokenPairId>,
    keyboard: Option<InlineKeyboardMarkup>,
) -> MessageResult<()> {

    let recipient = recipient.into();
    let pair = pair.into();

    let token_summary = state.pumpfun_token_service().summarise(pair).await.unwrap();
    let symbol = token_summary.pair.symbol().to_string();

    let progress = token_summary.current.progress;
    let progress = format!("{:.2}", progress);

    let caption = markdown!(
        r#"
        ;*{symbol};*
        is ;*{progress}%;* along the bonding curve and on its way to graduate to Raydium 🔥🚀
    "#
    );

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

    let mut message = state
        .bot
        .send_photo(recipient, file)
        .caption(caption)
        .parse_mode(ParseMode::MarkdownV2);

    if let Some(keyboard) = keyboard {
        message = message.reply_markup(keyboard);
    }

    message.await?;
    Ok(())
}
