// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::{markdown, AppState};
use base::model::{Notification, TokenPairId, User};
use base::service::{NotificationError, NotificationResult};
use render::page::pumpfun::{pumpfun_summary, PumpfunSummary};
use render::render;
use teloxide::payloads::SendPhotoSetters;
use teloxide::requests::Requester;
use teloxide::types::{
    ChatId, InlineKeyboardButton, InlineKeyboardMarkup, InputFile, ParseMode, Recipient,
};
use teloxide::{ApiError, RequestError};
use url::Url;

pub(crate) async fn send(
    state: AppState,
    user: User,
    notification: Notification,
) -> NotificationResult<()> {
    let telegram_id = user.telegram_id.expect("missing telegram id");
    let token_pair_id: TokenPairId = notification.payload("token_pair").unwrap();

    let token_summary = state
        .pumpfun_token_service()
        .summarise(token_pair_id)
        .await
        .unwrap();

    let symbol = token_summary.pair.symbol().to_string();

    // todo!();

    // let token_pair = payload.token_pair;
    let base_mint = token_summary.pair.base.mint.clone();

    let buttons = InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::url(
            "💰 Buy on pump.fun",
            Url::parse(format!("https://pump.fun/{base_mint}").as_str()).unwrap(),
        )],
        // vec![InlineKeyboardButton::callback(
        //     "⛌ Close",
        //     state.callback_store.store(Callback::Close).await,
        // )],
    ]);

    //         let _x = state
    //             .bot
    //             .send_message(
    //                 Recipient::Id(ChatId(telegram_id.0.parse::<i64>().unwrap())),
    //                 format!(
    //                     r#"
    // ⚠️*{token_pair}*⚠️
    // 🚀🔥 is *xx %& along the bonding curve and on its way to graduate to Raydium 🔥🚀
    //
    // Market Cap:
    // Liquidity:
    // Price:
    // Vol:
    // Swaps:
    // Age: xx days or hours
    // "#
    //                 ),

    let progress = token_summary.current.progress;
    let progress = format!("{:.2}", progress);

    // let swaps = token_summary.summary.swaps.all.swaps.0;
    // let swaps_change = token_summary.summary.swaps.all.change.unwrap().0;
    // let swaps_percent = token_summary.summary.swaps.all.change_percent.unwrap().0;
    //
    // let buy_swaps = token_summary.summary.swaps.buy.swaps.0;
    // let sell_swaps = token_summary.summary.swaps.sell.swaps.0;

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

    Ok(())
}

// async fn create_keyboard(
//     store: CallbackStore,
//     notification: &Notification,
// ) -> InlineKeyboardMarkup {
//     let buttons: Vec<Option<TelegramActionButtonConfig>> = (0..6)
//         .map(|i| notification.payload::<TelegramActionButtonConfig>(format!("button_{i}").as_str()))
//         .collect::<Vec<_>>();
//
//     let mut rows: Vec<Vec<InlineKeyboardButton>> = vec![vec![], vec![]];
//
//     for (i, button_config) in buttons.iter().enumerate() {
//         let row_index = if i < 3 { 0 } else { 1 };
//         let button = match button_config {
//             Some(TelegramActionButtonConfig::None) | None => {
//                 InlineKeyboardButton::callback(" ", IGNORE_CALLBACK)
//             }
//             Some(cfg) => InlineKeyboardButton::callback(
//                 text(cfg),
//                 store
//                     .store(Callback::ActionButton(CallbackActionButton {
//                         config: cfg.clone(),
//                     }))
//                     .await,
//             ),
//         };
//         rows[row_index].push(button);
//     }
//
//     let all_none = buttons
//         .iter()
//         .all(|cfg| matches!(cfg, Some(TelegramActionButtonConfig::None) | None));
//     if all_none {
//         InlineKeyboardMarkup::default()
//     } else {
//         InlineKeyboardMarkup::new(rows)
//     }
// }
//
// fn text(cfg: &TelegramActionButtonConfig) -> String {
//     match cfg {
//         TelegramActionButtonConfig::None => unreachable!(),
//         TelegramActionButtonConfig::Buy { value } => format!("Buy: {value}"),
//         TelegramActionButtonConfig::Sell { value } => format!("Sell: {value}"),
//     }
// }
