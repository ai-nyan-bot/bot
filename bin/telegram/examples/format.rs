// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::TokenPairId;
use common::repo::pool::PostgresConfig;
use common::ConfigValue;
use dotenv::dotenv;

use render::{render, Font, FontType, RenderBackground, RenderWatermark};
use std::env;
use telegram::{markdown, AppState, Config, TelegramConfig, WalletConfig};
use teloxide::payloads::SendPhotoSetters;
use teloxide::prelude::ChatId;
use teloxide::requests::Requester;
use teloxide::types::{InputFile, ParseMode, Recipient};

#[tokio::main]
pub async fn main() {
    dotenv().ok();

    let token_pair_id = TokenPairId::from(1109);

    let state = AppState::setup(Config {
        telegram: TelegramConfig {
            token: ConfigValue::Value(
                env::var("TEST_TELEGRAM_TOKEN").expect("TEST_TELEGRAM_TOKEN must be set"),
            ),
            webapp_url: ConfigValue::Value("http://telegram.nyan.bot".to_string()),
        },
        postgres: PostgresConfig {
            connection_string: ConfigValue::Value(
                "postgres://root:tor@localhost:5432/dev?sslmode=disable".to_string(),
            ),
            pool_min: Default::default(),
            pool_max: Default::default(),
            timeout_acquire_ms: Default::default(),
        },
        wallet: WalletConfig { secret: Default::default() },
    })
    .await;

    // Markdown::with_block(
    //
    // )

    let symbol = "NYAN/WSOL";
    let progress = "12.34";

    // let swaps = 100;
    let buy_swaps = 75;
    let sell_swaps = 25;

    // let font = Font::default();
    // let bold_font = Font::new(Style::Bold);

    // let text = markdown!(
    //     r#"
    //     ;* {symbol} ;*
    //     is ;* {progress} % ;* along the bonding curve and on its way to graduate to Raydium 🔥🚀
    //
    //     ;*Swaps;*
    //     ;`All:    ;`;*{swaps};*;`  ;`(+23 | +15.42%)🚀
    //     ;`Buy:    ;`;*{buy_swaps};*;`  ;`(+24 | +12.42%)🚀
    //     ;`Sell:   ;`;*{sell_swaps};*;`    ;`(+12 | +23.42%)🚀
    //
    //     ;*Swaps;*
    //     All:;`     ;`;*{swaps};*;`  ;`(+23 | +15.42%)🚀
    //     Buy:;`     ;`;*{buy_swaps};*;`  ;`(+24 | +12.42%)🚀
    //     Sell:;`   ;`;*{sell_swaps};*;`    ;`(+12 | +23.42%)🚀
    //
    //     {}
    //     {}
    // "#,
    //     font.format("All: 23%"),
    //     bold_font.format("All: 23%"),
    // );

    // let text = "All:";
    // let text = "All:";
    // let width = UnicodeWidthStr::width_cjk(font.format(text).as_str());
    // println!("{}:{}", text, width);

    // let mut builder = String::new();
    // builder += markdown!("{}", bold_font.format(symbol)).as_str();
    // builder += "\n";
    // builder += markdown!("{}", font.format("is")).as_str();
    // builder += " ";
    // builder += markdown!("{}", bold_font.format(progress)).as_str();
    // builder += markdown!(
    //     "{}",
    //     font.format("% along the bonding curve and on its way to graduate to Raydium 🔥🚀")
    // )
    // .as_str();
    // builder += "\n";
    // builder += "\n";
    //
    // let block_formatter = BlockFormatter::default();
    //
    // let builder = format!(
    //     "{}{}",
    //     builder,
    //     block_formatter.swaps(SummarySwaps {
    //         buy: SwapsWithChange {
    //             swaps: 0.into(),
    //             change: None,
    //             change_percent: None,
    //         },
    //         sell: SwapsWithChange {
    //             swaps: 0.into(),
    //             change: None,
    //             change_percent: None,
    //         },
    //         all: SwapsWithChange {
    //             swaps: 0.into(),
    //             change: None,
    //             change_percent: None,
    //         },
    //     })
    // );

    // builder += markdown!("{}",bold_font.format("Swaps")).as_str();
    // builder += "\n";
    // builder += markdown!("{}",font.format("All:")).as_str();
    // builder += "`  `";
    // builder += markdown!(";`{}{};`",bold_font.format("12.4"), font.format("K")).as_str();
    // builder += "` `";
    // builder += markdown!("{}",font.format(";` + ;`( 15 | 23.4% )")).as_str();
    // builder += "\n";
    // builder += markdown!("{}",font.format("Buy:")).as_str();
    // builder += "`  `";
    // builder += markdown!(";`   {} ;`",bold_font.format("4")).as_str();
    // builder += "` `";
    // builder += markdown!("{}",font.format(";` - ;`( 15 | 23.4% )")).as_str();
    // builder += "\n";
    // builder += markdown!("{}",font.format("Sell:")).as_str();
    // builder += "` `";
    // builder += markdown!(";` {}{};`",bold_font.format("3.4"), font.format("M")).as_str();
    // builder += "` `";
    // builder += markdown!("{}",font.format(";` + ;`( 15 | 23.4% )")).as_str();
    // builder += "\n";

    //     let text = markdown!(
    //         r#"
    // {};` ;`{} %
    //     "#,
    //         font.format("All:"),
    //         bold_font.format("23"),
    //         // bold_font.format("abcdefghijklmnopqrstuvxyz"),
    //         // italic_font.format("abcdefghijklmnopqrstuvxyz"),
    //     );

    // state
    //     .bot
    //     .send_message(Recipient::Id(ChatId(6884537674)), builder)
    //     .parse_mode(ParseMode::Html)
    //     .await
    //     .unwrap();

    // let start = Instant::now();
    // create_image().expect("Failed to generate image");
    let image_path = render(|img| {
        let font = Font::new(FontType::DejaVuSans);
        RenderBackground::render(img);
        RenderWatermark::render(img);
    })
    .await
    .unwrap();
    let file = InputFile::file(image_path);
    //
    // println!("took: {}", Instant::now().duration_since(start).as_millis());

    state
        .bot
        .send_photo(Recipient::Id(ChatId(6886037674)), file)
        .caption(markdown!(
            r#"
        	;*{symbol};*
	        is ;* {progress} % ;* along the bonding curve and on its way to graduate to Raydium 🔥🚀
		"#
        ))
        .parse_mode(ParseMode::MarkdownV2)
        .await
        .unwrap();
}
