// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::{markdown, AppState};
use base::model::{Notification, TokenPairId, User};
use base::service::NotificationResult;
use common::model::Percent;
use render::page::{pumpfun, PumpfunContext};
use render::render;
use solana::model::{
    ProgressWithChange, Summary, SummaryCurveProgress, SummaryTrade, TradesWithChange,
};
use teloxide::payloads::SendPhotoSetters;
use teloxide::requests::Requester;
use teloxide::types::{
    ChatId, InlineKeyboardButton, InlineKeyboardMarkup, InputFile, ParseMode, Recipient,
};
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
        .await.unwrap();

    let symbol = token_summary.pair.symbol().to_string();
    // dbg!(&token_summary);

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
    // Trades:
    // Age: xx days or hours
    // "#
    //                 ),

    let progress = token_summary.curve.progress;
    let progress = format!("{:.2}", progress);

    // let trades = token_summary.summary.trades.all.trades.0;
    // let trades_change = token_summary.summary.trades.all.change.unwrap().0;
    // let trades_percent = token_summary.summary.trades.all.change_percent.unwrap().0;
    //
    // let buy_trades = token_summary.summary.trades.buy.trades.0;
    // let sell_trades = token_summary.summary.trades.sell.trades.0;

    let caption = markdown!(
        r#"
        ;* {symbol} ;*
        is ;* {progress} % ;* along the bonding curve and on its way to graduate to Raydium 🔥🚀
    "#
    );

    // println!("{}", text);

    let image_path = render(|img| {
        pumpfun(
            img,
            PumpfunContext {
                m1: None,
                h1: Some(Summary {
                    token_pair: 1.into(),
                    curve_progress: SummaryCurveProgress {
                        open: ProgressWithChange {
                            progress: None,
                            change: None,
                        },
                        high: ProgressWithChange {
                            progress: None,
                            change: None,
                        },
                        low: ProgressWithChange {
                            progress: None,
                            change: None,
                        },
                        close: ProgressWithChange {
                            progress: None,
                            change: None,
                        },
                        avg: ProgressWithChange {
                            progress: None,
                            change: None,
                        },
                    },
                    trade: SummaryTrade {
                        all: TradesWithChange {
                            count: 3.into(),
                            change: Some(1.into()),
                            percent: Some(33.3.into()),
                        },
                        buy: TradesWithChange {
                            count: 2.into(),
                            change: Some(2.into()),
                            percent: Some(100.0.into()),
                        },
                        sell: TradesWithChange {
                            count: 1.into(),
                            change: Some(1.into()),
                            percent: Some(Percent::from(-50.0)),
                        },
                    },
                }),
                d1: None,
            },
        )
    })
    .await
    .unwrap();

    let file = InputFile::file(image_path);

    let _x = state
        .bot
        .send_photo(
            Recipient::Id(ChatId(telegram_id.0.parse::<i64>().unwrap())),
            file,
        )
        .caption(caption)
        .parse_mode(ParseMode::MarkdownV2)
        // .reply_markup(create_keyboard(state.callback_store.clone(), &notification).await)
        .reply_markup(buttons)
        .await
        .unwrap();

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
