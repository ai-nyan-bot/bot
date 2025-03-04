// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use ab_glyph::{FontRef, PxScale};
use base::model::TokenPairId;
use common::repo::pool::PostgresConfig;
use common::ConfigValue;
use dotenv::dotenv;
use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_filled_rect, draw_text_mut};
use imageproc::rect::Rect;
use std::env;
use rand::Rng;
use rusttype::Font;
use telegram::{AppState, Config, TelegramConfig};
use teloxide::payloads::SendPhotoSetters;
use teloxide::prelude::ChatId;
use teloxide::requests::Requester;
use teloxide::types::{InputFile, Recipient};
use tokio::time::Instant;

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
	})
		.await;

	// Markdown::with_block(
	//
	// )

	let symbol = "NYAN/WSOL";
	let progress = "12.34";

	// let trades = 100;
	let buy_trades = 75;
	let sell_trades = 25;

	// let font = Font::default();
	// let bold_font = Font::new(Style::Bold);

	// let text = markdown!(
	//     r#"
	//     ;* {symbol} ;*
	//     is ;* {progress} % ;* along the bonding curve and on its way to graduate to Raydium 🔥🚀
	//
	//     ;*Trades;*
	//     ;`All:    ;`;*{trades};*;`  ;`(+23 | +15.42%)🚀
	//     ;`Buy:    ;`;*{buy_trades};*;`  ;`(+24 | +12.42%)🚀
	//     ;`Sell:   ;`;*{sell_trades};*;`    ;`(+12 | +23.42%)🚀
	//
	//     ;*Trades;*
	//     All:;`     ;`;*{trades};*;`  ;`(+23 | +15.42%)🚀
	//     Buy:;`     ;`;*{buy_trades};*;`  ;`(+24 | +12.42%)🚀
	//     Sell:;`   ;`;*{sell_trades};*;`    ;`(+12 | +23.42%)🚀
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
	//     block_formatter.trades(SummaryTrades {
	//         buy: TradesWithChange {
	//             trades: 0.into(),
	//             change: None,
	//             change_percent: None,
	//         },
	//         sell: TradesWithChange {
	//             trades: 0.into(),
	//             change: None,
	//             change_percent: None,
	//         },
	//         all: TradesWithChange {
	//             trades: 0.into(),
	//             change: None,
	//             change_percent: None,
	//         },
	//     })
	// );

	// builder += markdown!("{}",bold_font.format("Trades")).as_str();
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

	let start = Instant::now();
	create_image().expect("Failed to generate image");
	let file = InputFile::file(IMAGE_PATH);
	
	println!("took: {}", Instant::now().duration_since(start).as_millis());

	state
		.bot
		.send_photo(Recipient::Id(ChatId(6886037674)), file)
		.caption("NYAN/WSOL")
		.await
		.unwrap();
}



const IMAGE_PATH: &str = "nyanbot_image.png";

fn create_image() -> Result<(), Box<dyn std::error::Error>> {
	let img_width = 1280;
	let img_height = 1280;
	let mut img = RgbImage::new(img_width, img_height);

	let white = Rgb([255, 255, 255]);
	let black = Rgb([0, 0, 0]);
	let red = Rgb([255, 0, 0]);
	let green = Rgb([0, 255, 0]);

	// let dark_blue = Rgb([10, 20, 50]); // Background color
	let grey = Rgb([128, 128, 128]);  // Star color
	// // Fill the background
	// for pixel in img.pixels_mut() {
	// 	*pixel = dark_blue;
	// }
	// let mut rng = rand::thread_rng();
	// 
	// // Generate random stars


	let font = FontRef::try_from_slice(include_bytes!("DejaVuSans.ttf")).unwrap();


	let space_color = Rgb([10, 10, 30]); // Dark blue/black space

	for pixel in img.pixels_mut() {
		*pixel = space_color;
	}

	let mut rng = rand::thread_rng();
	let neon_colors = [
		Rgb([255, 0, 255,]),    // Magenta
		Rgb([0, 255, 255,]),    // Cyan
		Rgb([255, 255, 0,]),    // Yellow
		Rgb([0, 255, 0, ]),      // Lime Green
		Rgb([255, 165, 0,]),    // Orange
	];

	// Draw celestial objects
	for _ in 0..50 {
		let x = rng.gen_range(10..img_width - 10) as i32;
		let y = rng.gen_range(10..img_height - 10) as i32;
		let size = rng.gen_range(1..5);
		let color = neon_colors[rng.gen_range(0..neon_colors.len())];

		if rng.gen_bool(0.6) {
			draw_text_mut(&mut img, color, x, y, PxScale { x: size as f32, y: size as f32 }, &font, "*");
		} else {
			imageproc::drawing::draw_filled_circle_mut(&mut img, (x as i32, y as i32), size, color);
		}
	}

	for _ in 0..100 { // Adjust number of stars
		let x = rng.gen_range(0..img_width);
		let y = rng.gen_range(0..img_height);

		let size = rng.gen_range(1..=3); // Small and large stars

		for dx in 0..size {
			for dy in 0..size {
				if x + dx < img_width && y + dy < img_height {
					img.put_pixel(x + dx, y + dy, grey);
				}
			}
		}
	}



	// Fill background with white
	// for pixel in img.pixels_mut() {
	// 	*pixel = white;
	// }

	// Load a font

	let s = 45;

	let scale = PxScale { x: s as f32, y: s as f32 };



	// Draw text
	let mut x = 0;
	let mut y = 0;
	// for line in 0..45 {
	// 	draw_text_mut(&mut img, black, x, y, scale, &font, "Trades");
	// 	draw_text_mut(&mut img, black, x, y + s, scale, &font, "All: 1200");
	// 	// draw_text_mut(&mut img, green, 400, y + 90, scale, &font, "+200");
	// 	draw_text_mut(&mut img, green, 300, y + s, scale, &font, "3.24% (200)");
	//
	// 	y += s;
	// 	draw_text_mut(&mut img, black, x, y + s, scale, &font, "Buy: 100");
	// 	draw_text_mut(&mut img, red, 300, y + s, scale, &font, "3.24% (200)");
	//
	// 	y += s;
	// 	draw_text_mut(&mut img, black, x, y + s, scale, &font, "Sell: 100");
	// 	draw_text_mut(&mut img, red, 300, y + s, scale, &font, "3.24% (200)");


		draw_text_mut(&mut img, white, x +s , y, scale, &font, "Trades: All 1200");
		draw_text_mut(&mut img, green, 300 +s, y , scale, &font, "(3.24%)");
		draw_text_mut(&mut img, white, 450 +s, y, scale, &font, "Buy 1200");
		draw_text_mut(&mut img, green, 650 +s, y , scale, &font, "(3.24%)");
		draw_text_mut(&mut img, white, 800 +s, y, scale, &font, "Sell 1200");
		draw_text_mut(&mut img, green, 1000 +s, y , scale, &font, "(3.24%)");
		y += s; // Move down for the next line
		draw_text_mut(&mut img, white, x +s, y, scale, &font, "Volume: All 1200");
		draw_text_mut(&mut img, green, 300 +s, y , scale, &font, "(3.24%)");
		draw_text_mut(&mut img, white, 450 +s, y, scale, &font, "Buy 1200");
		draw_text_mut(&mut img, green, 650 +s, y , scale, &font, "(3.24%)");
		draw_text_mut(&mut img, white, 800 +s, y, scale, &font, "Sell 1200");
		draw_text_mut(&mut img, green, 1000 +s, y , scale, &font, "(3.24%)");
	// }

	// Save image
	img.save(IMAGE_PATH)?;

	Ok(())
}
