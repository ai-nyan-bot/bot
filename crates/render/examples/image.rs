// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use ab_glyph::{FontRef, PxScale};
use image::imageops::{overlay, resize, FilterType};
use image::{DynamicImage, Rgba, RgbaImage};
use imageproc::drawing::{draw_text_mut, Canvas};
use rand::Rng;
use std::time::Instant;

pub fn main() {
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

    println!("took: {}", Instant::now().duration_since(start).as_millis());
}

const IMAGE_PATH: &str = "/tmp/nyanbot_image.png";

fn create_image() -> Result<(), Box<dyn std::error::Error>> {
    let img_width = 1200;
    let img_height = 1200;
    let mut img = RgbaImage::new(img_width, img_height);

    let white = Rgba([255, 255, 255, 255]);
    let black = Rgba([0, 0, 0, 255]);
    let red = Rgba([255, 0, 0, 255]);
    let green = Rgba([0, 255, 0, 255]);

    // let dark_blue = Rgb([10, 20, 50]); // Background color
    let grey = Rgba([128, 128, 128, 255]); // Star color
                                           // // Fill the background
                                           // for pixel in img.pixels_mut() {
                                           // 	*pixel = dark_blue;
                                           // }
                                           // let mut rng = rand::thread_rng();
                                           //
                                           // // Generate random stars

    let font = FontRef::try_from_slice(include_bytes!("../asset/DejaVuSans.ttf")).unwrap();

    let space_color = Rgba([10, 10, 30, 255]); // Dark blue/black space

    for pixel in img.pixels_mut() {
        *pixel = space_color;
    }

    let mut rng = rand::thread_rng();
    let neon_colors = [
        Rgba([255, 0, 255, 255]), // Magenta
        Rgba([0, 255, 255, 255]), // Cyan
        Rgba([255, 255, 0, 255]), // Yellow
        Rgba([0, 255, 0, 255]),   // Lime Green
        Rgba([255, 165, 0, 255]), // Orange
    ];

    // Draw celestial objects
    for _ in 0..50 {
        let x = rng.gen_range(10..img_width - 10) as i32;
        let y = rng.gen_range(10..img_height - 10) as i32;
        let size = rng.gen_range(1..5);
        let color = neon_colors[rng.gen_range(0..neon_colors.len())];

        if rng.gen_bool(0.6) {
            draw_text_mut(
                &mut img,
                color,
                x,
                y,
                PxScale {
                    x: size as f32,
                    y: size as f32,
                },
                &font,
                "*",
            );
        } else {
            imageproc::drawing::draw_filled_circle_mut(&mut img, (x as i32, y as i32), size, color);
        }
    }

    for _ in 0..100 {
        // Adjust number of stars
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

    let scale = PxScale {
        x: s as f32,
        y: s as f32,
    };

    let mut dyn_img = DynamicImage::ImageRgba8(img); // Convert to DynamicImage

    // let logo = image::open("/home/ddymke/repo/nyanbot/bin/telegram/examples/logo_256.png")?; // Load the image
    // let resized_logo = resize(&logo, 100, 100, FilterType::Lanczos3); // Resize to fit
    // overlay(&mut dyn_img, &resized_logo, 0, 0); // Adjust position as needed
    // overlay(&mut dyn_img, &resized_logo, 100, 0); // Adjust position as needed
    // overlay(&mut dyn_img, &resized_logo, 200, 0); // Adjust position as needed
    // overlay(&mut dyn_img, &resized_logo, 300, 0); // Adjust position as needed
    // overlay(&mut dyn_img, &resized_logo, 400, 0); // Adjust position as needed
    // overlay(&mut dyn_img, &resized_logo, 500, 0); // Adjust position as needed
    // overlay(&mut dyn_img, &resized_logo, 600, 0); // Adjust position as needed
    // overlay(&mut dyn_img, &resized_logo, 700, 0); // Adjust position as needed
    // overlay(&mut dyn_img, &resized_logo, 800, 0); // Adjust position as needed
    // overlay(&mut dyn_img, &resized_logo, 900, 0); // Adjust position as needed
    // overlay(&mut dyn_img, &resized_logo, 1000, 0); // Adjust position as needed
    // overlay(&mut dyn_img, &resized_logo, 1100, 0); // Adjust position as needed
    // overlay(&mut dyn_img, &resized_logo, 1200, 0); // Adjust position as needed
    //
    // // overlay(&mut dyn_img, &resized_logo, 0, 1200); // Adjust position as needed
    // // overlay(&mut dyn_img, &resized_logo, 1200, 0); // Adjust position as needed
    // // overlay(&mut dyn_img, &resized_logo, 1200, 1200); // Adjust position as needed
    //
    // overlay(&mut dyn_img, &resized_logo, 0, 1100); // Adjust position as needed
    // overlay(&mut dyn_img, &resized_logo, 100, 1100); // Adjust position as needed
    // overlay(&mut dyn_img, &resized_logo, 200, 1100); // Adjust position as needed
    // overlay(&mut dyn_img, &resized_logo, 300, 1100); // Adjust position as needed
    // overlay(&mut dyn_img, &resized_logo, 400, 1100); // Adjust position as needed
    // overlay(&mut dyn_img, &resized_logo, 500, 1100); // Adjust position as needed
    // overlay(&mut dyn_img, &resized_logo, 600, 1100); // Adjust position as needed
    // overlay(&mut dyn_img, &resized_logo, 700, 1100); // Adjust position as needed
    // overlay(&mut dyn_img, &resized_logo, 800, 1100); // Adjust position as needed
    // overlay(&mut dyn_img, &resized_logo, 900, 1100); // Adjust position as needed
    // overlay(&mut dyn_img, &resized_logo, 1000, 1100); // Adjust position as needed
    // overlay(&mut dyn_img, &resized_logo, 1100, 1100); // Adjust position as needed
    // overlay(&mut dyn_img, &resized_logo, 1200, 1100); // Adjust position as needed

    let mut logo = image::load_from_memory(include_bytes!("../asset/logo_256.png"))?.to_rgba8();
    let mut watermark = resize(&logo, 1200, 1200, FilterType::Lanczos3); // Resize to fit

    // Reduce opacity to make it a watermark
    for pixel in watermark.pixels_mut() {
        pixel.0[3] = 15; // Set transparency (0-255, lower = more transparent)
    }

    overlay(&mut dyn_img, &watermark, 0, 0); // Position in bottom-right corner

    // Draw text
    let mut x = 0;
    let mut y = 300;
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

    // draw_text_mut(&mut img, white, x +s , y, scale, &font, "NYAN/WSOL");
    // y += s;
    // draw_text_mut(&mut img, white, x +s , y, scale, &font, "is 12.34% along the bonding curve");
    // y += s;
    // draw_text_mut(&mut img, white, x +s , y, scale, &font, "and on its way to graduate to Raydium");
    // y += s;
    // y += s;
    draw_text_mut(&mut dyn_img, white, x, y, scale, &font, "Trades: All 1200");
    draw_text_mut(&mut dyn_img, green, 300, y, scale, &font, "(3.24%)");
    draw_text_mut(&mut dyn_img, white, 450, y, scale, &font, "Buy 1200");
    draw_text_mut(&mut dyn_img, green, 650, y, scale, &font, "(3.24%)");
    draw_text_mut(&mut dyn_img, white, 800, y, scale, &font, "Sell 1200");
    draw_text_mut(&mut dyn_img, green, 1000, y, scale, &font, "(3.24%)");
    y += s; // Move down for the next line
            // draw_text_mut(&mut img, white, x + s, y, scale, &font, "Volume: All 1200");
            // draw_text_mut(&mut img, green, 300 + s, y, scale, &font, "(3.24%)");
            // draw_text_mut(&mut img, white, 450 + s, y, scale, &font, "Buy 1200");
            // draw_text_mut(&mut img, green, 650 + s, y, scale, &font, "(3.24%)");
            // draw_text_mut(&mut img, white, 800 + s, y, scale, &font, "Sell 1200");
            // draw_text_mut(&mut img, green, 1000 + s, y, scale, &font, "(3.24%)");
            // }

    // Save image
    dyn_img.save(IMAGE_PATH)?;

    Ok(())
}
