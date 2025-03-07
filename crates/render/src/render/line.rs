// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::Point;
use image::{Rgba, RgbaImage};

pub struct Line {
    pub start: Point,
    pub end: Point,
    pub color: Rgba<u8>,
    pub thickness: u8,
}

pub struct RenderLine {}

impl RenderLine {
    pub fn render(img: &mut RgbaImage, line: Line) {
        let x0 = line.start.x.0 as i32;
        let y0 = line.start.y.0 as i32;
        let x1 = line.end.x.0 as i32;
        let y1 = line.end.y.0 as i32;

        let dx = x1 - x0;
        let dy = y1 - y0;
        let length = ((dx * dx + dy * dy) as f64).sqrt();

        if length == 0.0 {
            return;
        }

        // Compute p    erpendicular direction (normalized)
        let perp_x = -(dy as f64) / length;
        let perp_y = (dx as f64) / length;

        let half_thickness = line.thickness as f64 / 2.0;

        // Draw multiple lines around the main line for thickness
        for i in -half_thickness as i32..=half_thickness as i32 {
            let offset_x = (i as f64 * perp_x).round() as i32;
            let offset_y = (i as f64 * perp_y).round() as i32;
            draw_line(
                img,
                x0 + offset_x,
                y0 + offset_y,
                x1 + offset_x,
                y1 + offset_y,
                line.color,
            );
        }
    }
}

fn draw_line(img: &mut RgbaImage, x0: i32, y0: i32, x1: i32, y1: i32, color: Rgba<u8>) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    let (mut x, mut y) = (x0, y0);

    while x != x1 || y != y1 {
        if x >= 0 && y >= 0 && x < img.width() as i32 && y < img.height() as i32 {
            img.put_pixel(x as u32, y as u32, color);
        }

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}
