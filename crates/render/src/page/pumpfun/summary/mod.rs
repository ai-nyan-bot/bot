// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

mod table;

use crate::page::pumpfun::summary::table::{
    draw_cell_center, draw_legend, draw_summary, draw_table,
};
use crate::{Font, FontType, RenderBackground, RenderWatermark, Text};
use image::{Rgba, RgbaImage};
use solana::model::Summary;

pub const PADDING_WIDTH: u32 = 40;
pub const CELL_WIDTH: u32 = 160;

pub const PADDING_HEIGHT: u32 = 40;
pub const CELL_HEIGHT: u32 = 112;

pub const DECREASED: Rgba<u8> = Rgba([150, 50, 50, 255]);
pub const INCREASED: Rgba<u8> = Rgba([50, 150, 50, 255]);

pub fn cell_x_start(idx: u32) -> u32 {
    PADDING_WIDTH + CELL_WIDTH * idx
}

pub fn cell_x_end(idx: u32) -> u32 {
    PADDING_WIDTH + CELL_WIDTH * (idx + 1)
}

pub fn cell_y_start(idx: u32) -> u32 {
    PADDING_HEIGHT + CELL_HEIGHT * idx
}

pub fn cell_y_end(idx: u32) -> u32 {
    PADDING_HEIGHT + CELL_HEIGHT * (idx + 1)
}

pub struct PumpfunSummary {
    pub m1: Option<Summary>,
    pub m5: Option<Summary>,
    pub m15: Option<Summary>,
    pub h1: Option<Summary>,
    pub h6: Option<Summary>,
    pub d1: Option<Summary>,
}

// struct Span {
//     words: Vec<Text>,
//     padding: (u32, u32), // (x, y) padding
// }
//
// impl Span {
//     fn render(&self, img: &mut RgbaImage, font: &Font, start_x: u32, start_y: u32) {
//         let mut x_offset = (start_x + self.padding.0) as u32;
//         let y_offset = (start_y + self.padding.1) as u32;
//         for word in &self.words {
//             let size = RenderText::render(img, font, x_offset, y_offset, word);
//             x_offset += size.width.0;
//         }
//     }
// }

pub fn pumpfun_summary(img: &mut RgbaImage, summary: PumpfunSummary) {
    RenderBackground::render(img);
    RenderWatermark::render(img);

    draw_table(img);

    let font = Font::new(FontType::DejaVuSans);

    draw_cell_center(
        img,
        &font,
        1,
        0,
        Text::new("1m", 36, Rgba([100, 100, 100, 255])),
    );
    draw_cell_center(
        img,
        &font,
        2,
        0,
        Text::new("5m", 36, Rgba([100, 100, 100, 255])),
    );
    draw_cell_center(
        img,
        &font,
        3,
        0,
        Text::new("15m", 36, Rgba([100, 100, 100, 255])),
    );
    draw_cell_center(
        img,
        &font,
        4,
        0,
        Text::new("1h", 36, Rgba([100, 100, 100, 255])),
    );
    draw_cell_center(
        img,
        &font,
        5,
        0,
        Text::new("6h", 36, Rgba([100, 100, 100, 255])),
    );
    draw_cell_center(
        img,
        &font,
        6,
        0,
        Text::new("1d", 36, Rgba([100, 100, 100, 255])),
    );

    draw_legend(img, &font, 1, "Bonding Curve");
    // draw_legend(img, &font, 2, "Price");
    // draw_legend(img, &font, 3, "Market Cap");
    // draw_legend(img, &font, 4, "Total Volume");
    // draw_legend(img, &font, 5, "Buy Volume");
    // draw_legend(img, &font, 6, "Sell Volume");
    // draw_legend(img, &font, 7, "Total Txs");
    // draw_legend(img, &font, 8, "Buy Txs");
    // draw_legend(img, &font, 9, "Sell Txs");

    if let Some(summary) = summary.m1 {
        draw_summary(img, &font, 1, summary)
    }

    if let Some(summary) = summary.m5 {
        draw_summary(img, &font, 2, summary)
    }

    if let Some(summary) = summary.m15 {
        draw_summary(img, &font, 3, summary)
    }

    if let Some(summary) = summary.h1 {
        draw_summary(img, &font, 4, summary)
    }

    if let Some(summary) = summary.h6 {
        draw_summary(img, &font, 5, summary)
    }

    if let Some(summary) = summary.d1 {
        draw_summary(img, &font, 6, summary)
    }
}
