// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::page::pumpfun::summary::{
    cell_x_end, cell_x_start, cell_y_end, cell_y_start, DECREASED, INCREASED, PADDING_HEIGHT,
};
use crate::{Font, FontSize, Line, Point, RenderLine, RenderText, Text};
use common::format::FormatPretty;
use image::{Rgba, RgbaImage};
use solana::model::{
    MarketCapWithChange, PriceWithChange, SummaryCurveProgress, SwapWithChange, TimeframeSummary,
    VolumeWithChange,
};
use std::ops::Div;

pub(crate) fn draw_table(img: &mut RgbaImage) {
    for idx in 0..6 {
        RenderLine::render(
            img,
            Line {
                start: Point::new(cell_x_end(idx), cell_y_start(0)),
                end: Point::new(cell_x_end(idx), cell_y_end(9)),
                color: Rgba([50, 50, 50, 255]),
                thickness: 1,
            },
        );
    }

    for idx in 0..9 {
        RenderLine::render(
            img,
            Line {
                start: Point::new(cell_x_start(0), cell_y_end(idx)),
                end: Point::new(cell_x_end(6), cell_y_end(idx)),
                color: Rgba([50, 50, 50, 255]),
                thickness: 1,
            },
        );
    }
}

pub(crate) fn draw_legend(img: &mut RgbaImage, font: &Font, y: u32, text: impl Into<String>) {
    let text = text.into();
    let text_size = FontSize::from(24);
    let size = font.measure(text.as_str(), text_size);
    RenderText::render(
        img,
        &font,
        cell_x_start(0) + (cell_x_end(0) - cell_x_start(0)).div(2) - size.width.0.div(2),
        cell_y_start(y) + (cell_y_end(y) - cell_y_start(y)).div(2) - size.height.0.div(2)
            + PADDING_HEIGHT / 2,
        Text {
            content: text,
            size: text_size,
            color: Rgba([255, 255, 255, 255]),
        },
    );
}

pub(crate) fn draw_cell_center(img: &mut RgbaImage, font: &Font, x: u32, y: u32, text: Text) {
    let size = font.measure(text.content.as_str(), text.size);
    let x_start = cell_x_start(x);
    let x_end = cell_x_end(x);
    let y_start = cell_y_start(y);
    let y_end = cell_y_end(y);

    let centered_x = (x_start + x_end) / 2 - size.width.0 / 2;
    let centered_y = (y_start + y_end) / 2 + size.height.0 / 2;

    RenderText::render(img, &font, centered_x, centered_y, text);
}

pub(crate) fn draw_cell_top(img: &mut RgbaImage, font: &Font, x: u32, y: u32, text: Text) {
    let size = font.measure(text.content.as_str(), text.size);
    let x_start = cell_x_start(x);
    let x_end = cell_x_end(x);
    let y_start = cell_y_start(y);

    let centered_x = (x_start + x_end) / 2 - size.width.0 / 2;
    let top_y = y_start + size.height.0 + size.height.0 / 2;

    RenderText::render(img, &font, centered_x, top_y, text);
}

pub(crate) fn draw_cell_bottom(img: &mut RgbaImage, font: &Font, x: u32, y: u32, text: Text) {
    let size = font.measure(text.content.as_str(), text.size);
    let x_start = cell_x_start(x);
    let x_end = cell_x_end(x);
    let y_end = cell_y_end(y);

    let centered_x = (x_start + x_end) / 2 - size.width.0 / 2;
    let bottom_y = y_end - size.height.0 / 2;

    RenderText::render(img, &font, centered_x, bottom_y, text);
}

pub(crate) fn draw_summary(img: &mut RgbaImage, font: &Font, x: u32, summary: TimeframeSummary) {
    draw_bonding_curve(img, font, x, summary.curve);
    draw_price(img, font, x, summary.price.close);
    draw_market_cap(img, font, x, summary.cap.close);
    draw_volume(img, font, x, 4, summary.volume.all);
    draw_volume(img, font, x, 5, summary.volume.buy);
    draw_volume(img, font, x, 6, summary.volume.sell);
    draw_txn(img, font, x, 7, summary.swap.all);
    draw_txn(img, font, x, 8, summary.swap.buy);
    draw_txn(img, font, x, 9, summary.swap.sell);
}

fn draw_bonding_curve(
    img: &mut RgbaImage,
    font: &Font,
    x: u32,
    curve_progress: SummaryCurveProgress,
) {
    let progress = curve_progress.close;
    if let Some(percent) = progress.progress {
        if let Some(change) = progress.change {
            draw_cell_top(
                img,
                font,
                x,
                1,
                Text::new(percent.pretty(), 28, Rgba([255, 255, 255, 255])),
            );

            let color = if change > 0.0 {
                INCREASED
            } else if change < 0.0 {
                DECREASED
            } else {
                Rgba([255, 255, 255, 255])
            };

            draw_cell_bottom(img, font, x, 1, Text::new(change.pretty(), 28, color));
        } else {
            draw_cell_center(
                img,
                font,
                x,
                1,
                Text::new(percent.pretty(), 28, Rgba([255, 255, 255, 255])),
            );
        }
    }
}

fn draw_market_cap(img: &mut RgbaImage, font: &Font, x: u32, cap: MarketCapWithChange) {
    if let Some(usd) = cap.usd {
        if let (Some(_change), Some(percent)) = (cap.usd_change, cap.percent) {
            draw_cell_top(
                img,
                font,
                x,
                3,
                Text::new(usd.pretty(), 28, Rgba([255, 255, 255, 255])),
            );

            let color = if percent > 0.0 {
                INCREASED
            } else if percent < 0.0 {
                DECREASED
            } else {
                Rgba([255, 255, 255, 255])
            };

            draw_cell_bottom(img, font, x, 3, Text::new(percent.pretty(), 28, color));
        } else {
            draw_cell_center(
                img,
                font,
                x,
                3,
                Text::new(usd.pretty(), 28, Rgba([255, 255, 255, 255])),
            );
        }
    }
}

fn draw_price(img: &mut RgbaImage, font: &Font, x: u32, cap: PriceWithChange) {
    if let Some(usd) = cap.usd {
        if let (Some(_change), Some(percent)) = (cap.usd_change, cap.percent) {
            draw_cell_top(
                img,
                font,
                x,
                2,
                Text::new(usd.pretty(), 28, Rgba([255, 255, 255, 255])),
            );

            let color = if percent > 0.0 {
                INCREASED
            } else if percent < 0.0 {
                DECREASED
            } else {
                Rgba([255, 255, 255, 255])
            };

            draw_cell_bottom(img, font, x, 2, Text::new(percent.pretty(), 28, color));
        } else {
            draw_cell_center(
                img,
                font,
                x,
                2,
                Text::new(usd.pretty(), 28, Rgba([255, 255, 255, 255])),
            );
        }
    }
}

fn draw_txn(img: &mut RgbaImage, font: &Font, x: u32, y: u32, swap: SwapWithChange) {
    if let Some(count) = swap.count {
        if let (Some(_change), Some(percent)) = (swap.change, swap.percent) {
            draw_cell_top(
                img,
                font,
                x,
                y,
                Text::new(count.pretty(), 28, Rgba([255, 255, 255, 255])),
            );

            let color = if percent > 0.0 {
                INCREASED
            } else if percent < 0.0 {
                DECREASED
            } else {
                Rgba([255, 255, 255, 255])
            };

            draw_cell_bottom(img, font, x, y, Text::new(percent.pretty(), 28, color));
        } else {
            draw_cell_center(
                img,
                font,
                x,
                y,
                Text::new(count.pretty(), 28, Rgba([255, 255, 255, 255])),
            );
        }
    }
}

fn draw_volume(img: &mut RgbaImage, font: &Font, x: u32, y: u32, volume: VolumeWithChange) {
    if let Some(usd) = volume.usd {
        if let (Some(_change), Some(percent)) = (volume.usd_change, volume.percent) {
            draw_cell_top(
                img,
                font,
                x,
                y,
                Text::new(usd.pretty(), 28, Rgba([255, 255, 255, 255])),
            );

            let color = if percent > 0.0 {
                INCREASED
            } else if percent < 0.0 {
                DECREASED
            } else {
                Rgba([255, 255, 255, 255])
            };

            draw_cell_bottom(img, font, x, y, Text::new(percent.pretty(), 28, color));
        } else {
            draw_cell_center(
                img,
                font,
                x,
                y,
                Text::new(usd.pretty(), 28, Rgba([255, 255, 255, 255])),
            );
        }
    }
}
