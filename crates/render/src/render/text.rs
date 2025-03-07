// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::{Font, FontSize, Point, Size};
use image::{Rgba, RgbaImage};

pub struct Text {
    pub content: String,
    pub size: FontSize,
    pub color: Rgba<u8>,
}

impl Text {
    pub fn new(content: impl Into<String>, size: impl Into<FontSize>, color: Rgba<u8>) -> Self {
        Self {
            content: content.into(),
            size: size.into(),
            color,
        }
    }
}

pub struct RenderText {}

impl RenderText {
    pub fn render(
        img: &mut RgbaImage,
        font: &Font,
        start_x: u32,
        start_y: u32,
        text: Text,
    ) -> Size {
        // let mut x_offset = (start_x + self.padding.0) as u32;
        let x_offset = start_x as u32;
        // let y_offset = start_y + self.padding.1;
        let y_offset = start_y as u32;

        // for word in &self.words {
        // let scale = Scale::uniform(word.size);
        // let v_metrics = font.v_metrics(scale);
        let offset = Point::from((x_offset as u32, y_offset as u32));

        for glyph in font.layout(&text.content, text.size, offset) {
            if let Some(bb) = glyph.pixel_bounding_box() {
                glyph.draw(|gx, gy, v| {
                    let px = (bb.min.x + gx as i32) as u32;
                    let py = (bb.min.y + gy as i32) as u32;

                    if px < img.width() as u32 && py < img.height() as u32 {
                        let px = px as u32;
                        let py = py as u32;

                        let pixel = img.get_pixel_mut(px, py);
                        let blend =
                            |a, b, alpha| ((a as f32 * (1.0 - alpha)) + (b as f32 * alpha)) as u8;

                        *pixel = Rgba([
                            blend(pixel[0], text.color[0], v),
                            blend(pixel[1], text.color[1], v),
                            blend(pixel[2], text.color[2], v),
                            blend(pixel[3], text.color[3], v),
                        ]);
                    }
                });
            }
        }

        font.measure(text.content.as_str(), text.size)

        // let px_scale = PxScale::from(45.0);
        //
        // draw_text_mut(img, word.color, x_offset as u32, offset.y as u32, px_scale., &font, "");

        // x_offset += (word.size as u32 * word.text.len() as u32) / 2; // Adjust spacing
        // x_offset += font.measure(word.text.as_str(), FontSize::from(word.size as f32)).width.0 + 10;
        // }
    }
}
