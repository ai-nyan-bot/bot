// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use image::{Rgba, RgbaImage};

pub struct RenderBackground {}

impl RenderBackground{
	pub fn render(img: &mut RgbaImage) {
		let space_color = Rgba([10, 10, 30, 255]); // Dark blue/black space

		for pixel in img.pixels_mut() {
			*pixel = space_color;
		}
	}
}