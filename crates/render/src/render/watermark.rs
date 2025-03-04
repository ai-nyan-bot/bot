// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use image::imageops::{overlay, resize, FilterType};
use image::{GenericImageView, RgbaImage};

pub struct RenderWatermark {}

impl RenderWatermark {
    pub fn render(img: &mut RgbaImage) {
        // let mut dyn_img = DynamicImage::ImageRgba8(img); // Convert to DynamicImage

        let mut logo = image::load_from_memory(include_bytes!("../../asset/logo_1024.png"))
            .unwrap()
            .to_rgba8();
        let mut watermark = resize(&logo, img.width(), img.height(), FilterType::Lanczos3);

        // Reduce opacity to make it a watermark
        for pixel in watermark.pixels_mut() {
            pixel.0[3] = 40; // Set transparency (0-255, lower = more transparent)
        }

        overlay(img, &watermark, 0, 0); // Position in bottom-right corner
    }
}
